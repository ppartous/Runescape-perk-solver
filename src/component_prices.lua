local comps = require("Module:Component costs")
local mats = require("Module:Disassemble/mats")
for _, mat in pairs(mats) do
    print(string.format("%s: %.1f", mat, comps._price(mat)))
end
