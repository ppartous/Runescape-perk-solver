from ctypes import *
from enum import Enum
import time
import math
import json

class GizmoType(Enum):
    Weapon = 0
    Armour = 1
    Tool = 2

class SortType(Enum):
    Gizmo = 0
    Attempt = 1
    Price = 2

TwoUint8Arr = c_uint8 * 2

class FfiArgs(Structure):
    _fields_ = [
        ("ancient", c_bool),
        ("gizmo_type", c_int), # Enum value
        ("invention_level", TwoUint8Arr), # Array of 2 uint8 values. Represents a range of levels to search through
        ("perk", c_char_p), # null terminated
        ("rank", c_uint8),
        ("perk_two", c_char_p), # null terminated
        ("rank_two", c_uint8),
        ("fuzzy", c_bool),
        ("exclude", c_char_p), # null terminated
        ("sort_type", c_int), # Enum value
        ("out_file", c_char_p), # null terminated
        ("price_file", c_char_p), # null terminated
        ("alt_count", c_uint8) # Amount of alternative material combination results
    ]

class Response(Structure):
    _fields_ = [
        ("total_combination_count", c_uint64),
        ("bar_progress", POINTER(c_uint64)),
        ("materials", c_char_p),
        ("result", c_char_p)
    ]

# Set return types
lib = CDLL("./perk_solver.dll")
lib.perk_solver_ctypes.restype = Response

def format_progress(response):
    x = response.bar_progress[0]
    y = response.total_combination_count
    percent = math.floor(x / y * 100)
    print("{}/{} ({}%)".format(x, y, percent))

def perk_solver(args):
    response = lib.perk_solver_ctypes(args) # Async call

    if response.total_combination_count == 0 and response.result: # This means an error message was returned
        result = response.result # Python automatically copies c_char_p strings
        lib.free_response(response)
        return result

    print(json.dumps(json.loads(response.materials), indent=2))

    while response.bar_progress[0] != response.total_combination_count:
        format_progress(response)
        time.sleep(0.1)
    format_progress(response) # Show 100%

    lib.get_result(byref(response)) # Blocking call. Returns when the result is ready.
    result = response.result
    lib.free_response(response)
    return json.dumps(json.loads(result), indent=2)

# Call the functions
args = FfiArgs(
    ancient = c_bool(True),
    gizmo_type = GizmoType.Weapon.value,
    invention_level = TwoUint8Arr(40, 50),
    perk = create_string_buffer(b"equilibrium").raw,
    rank = 4,
    perk_two = create_string_buffer(b"mobile").raw,
    rank_two = 1,
    fuzzy = c_bool(False),
    exclude = create_string_buffer(b"connector,delicate,flexible").raw,
    sort_type = SortType.Price.value,
    out_file = create_string_buffer(b"false").raw,
    price_file = create_string_buffer(b"false").raw, # Always get fresh prices. Prices are still cached when making multiple calls without reloading the binary
    alt_count = 1
)
print(perk_solver(args))


# The mat_combination array in the result is in the order that materials fill the gizmo:
# 6 2 7
# 3 1 4
# 8 5 9

# Warning messages are currently still send over stderr