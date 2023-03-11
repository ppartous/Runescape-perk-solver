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

class FfiArgs(Structure):
    _fields_ = [
        ("ancient", c_bool),
        ("gizmo_type", c_int), # Enum value
        ("invention_level", c_uint8 * 2),
        ("perk", c_char_p), # null terminated
        ("rank", c_uint8),
        ("perk_two", c_char_p), # null terminated
        ("rank_two", c_uint8),
        ("fuzzy", c_bool),
        ("exclude", c_char_p), # null terminated
        ("sort_type", c_int), # Enum value
        ("out_file", c_char_p),
        ("price_file", c_char_p)
    ]

class Response(Structure):
    _fields_ = [
        ("bar_progress", POINTER(c_uint64)),
        ("total_combination_count", c_uint64)
    ]

def format_progress(response):
    x = response.bar_progress[0]
    y = response.total_combination_count
    percent = math.floor(x / y * 100)
    print("{}/{} ({}%)".format(x, y, percent))

def perk_solver(args):
    response = perk_solver_ctypes(args) # Async call
    while response.bar_progress[0] != response.total_combination_count:
        format_progress(response)
        time.sleep(0.1)
    format_progress(response) # Show 100%
    res = get_result_json() # Blocking call. Returns when the result is ready.
    return json.loads(res)

# Set return types
lib = CDLL("./perk_solver.dll")
perk_solver_ctypes = lib.perk_solver_ctypes
perk_solver_ctypes.restype = Response
get_result_json = lib.get_result_json
get_result_json.restype = c_char_p

# Call the functions
args = FfiArgs(
    ancient = c_bool(True),
    gizmo_type = GizmoType.Weapon.value,
    invention_level = (c_uint8 * 2)(1, 50),
    perk = create_string_buffer(b"equilibrium").raw,
    rank = 4,
    perk_two = create_string_buffer(b"mobile").raw,
    rank_two = 1,
    fuzzy = c_bool(False),
    exclude = create_string_buffer(b"connector,delicate,flexible").raw,
    sort_type = SortType.Price.value,
    out_file = c_char_p(0),
    price_file = c_char_p(0)
)
print(perk_solver(args))

# The mat_combination array in the result is in the order that materials fill the gizmo:
# 6 2 7
# 3 1 4
# 8 5 9