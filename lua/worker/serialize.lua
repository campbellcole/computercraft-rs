-- This is a modified version of the CC: Tweaked stdlib serializer implementation.
-- This version has the same API as 1.106.1, so it is also a polyfill.
--
-- The only thing that has been modified is the "recursion" handler that the old implementation had.
-- The old recursion handler will freak out when an object is used multiple times, which is extremely
-- common when listing large amounts of items (i.e. from refined storage or minecolonies).
-- Instead of instantly erroring when an object is seen twice, it will return an empty object
-- if and only if the current object has been seen more than 999 times. I chose this number arbitrarily,
-- and I've yet to see the limit get hit. Though, it is important to prevent putting any truly recursive
-- objects into this function because if the object contains itself it will be serialized 999 times.

local expect = dofile("rom/modules/main/cc/expect.lua")
local expect, field = expect.expect, expect.field

local function mk_tbl(str, name)
    local msg = "attempt to mutate textutils." .. name
    return setmetatable({}, {
        __newindex = function() error(msg, 2) end,
        __tostring = function() return str end,
    })
end

empty_json_array = mk_tbl("[]", "empty_json_array")

json_null = mk_tbl("null", "json_null")

local serializeJSONString
do
    local function hexify(c)
        return ("\\u00%02X"):format(c:byte())
    end

    local map = {
        ["\""] = "\\\"",
        ["\\"] = "\\\\",
        ["\b"] = "\\b",
        ["\f"] = "\\f",
        ["\n"] = "\\n",
        ["\r"] = "\\r",
        ["\t"] = "\\t",
    }
    for i = 0, 0x1f do
        local c = string.char(i)
        if map[c] == nil then map[c] = hexify(c) end
    end

    serializeJSONString = function(s, options)
        if options and options.unicode_strings and s:find("[\x80-\xff]") then
            local retval = '"'
            for _, code in utf8.codes(s) do
                if code > 0xFFFF then
                    -- Encode the codepoint as a UTF-16 surrogate pair
                    code = code - 0x10000
                    local high, low = bit32.extract(code, 10, 10) + 0xD800, bit32.extract(code, 0, 10) + 0xDC00
                    retval = retval .. ("\\u%04X\\u%04X"):format(high, low)
                elseif code <= 0x5C and map[string.char(code)] then -- 0x5C = `\`, don't run `string.char` if we don't need to
                    retval = retval .. map[string.char(code)]
                elseif code < 0x20 or code >= 0x7F then
                    retval = retval .. ("\\u%04X"):format(code)
                else
                    retval = retval .. string.char(code)
                end
            end
            return retval .. '"'
        else
            return ('"%s"'):format(s:gsub("[\0-\x1f\"\\]", map):gsub("[\x7f-\xff]", hexify))
        end
    end
end

local function serializeJSONImpl(t, tTracking, options)
    local sType = type(t)
    if t == empty_json_array then
        return "[]"
    elseif t == json_null then
        return "null"
    elseif sType == "table" then
        if tTracking[t] == nil then
            tTracking[t] = 1
        elseif tTracking[t] == 999 then
            print("stopping recursion at depth 999")
            return "{}"
        else
            tTracking[t] = tTracking[t] + 1
        end

        if next(t) == nil then
            -- Empty tables are simple
            return "{}"
        else
            -- Other tables take more work
            local sObjectResult = "{"
            local sArrayResult = "["
            local nObjectSize = 0
            local nArraySize = 0
            local largestArrayIndex = 0
            local bNBTStyle = options and options.nbt_style
            for k, v in pairs(t) do
                if type(k) == "string" then
                    local sEntry
                    if bNBTStyle then
                        sEntry = tostring(k) .. ":" .. serializeJSONImpl(v, tTracking, options)
                    else
                        sEntry = serializeJSONString(k, options) .. ":" .. serializeJSONImpl(v, tTracking, options)
                    end
                    if nObjectSize == 0 then
                        sObjectResult = sObjectResult .. sEntry
                    else
                        sObjectResult = sObjectResult .. "," .. sEntry
                    end
                    nObjectSize = nObjectSize + 1
                elseif type(k) == "number" and k > largestArrayIndex then --the largest index is kept to avoid losing half the array if there is any single nil in that array
                    largestArrayIndex = k
                end
            end
            for k = 1, largestArrayIndex, 1 do --the array is read up to the very last valid array index, ipairs() would stop at the first nil value and we would lose any data after.
                local sEntry
                if t[k] == nil then            --if the array is nil at index k the value is "null" as to keep the unused indexes in between used ones.
                    sEntry = "null"
                else                           -- if the array index does not point to a nil we serialise it's content.
                    sEntry = serializeJSONImpl(t[k], tTracking, options)
                end
                if nArraySize == 0 then
                    sArrayResult = sArrayResult .. sEntry
                else
                    sArrayResult = sArrayResult .. "," .. sEntry
                end
                nArraySize = nArraySize + 1
            end
            sObjectResult = sObjectResult .. "}"
            sArrayResult = sArrayResult .. "]"
            if nObjectSize > 0 or nArraySize == 0 then
                return sObjectResult
            else
                return sArrayResult
            end
        end
    elseif sType == "string" then
        return serializeJSONString(t, options)
    elseif sType == "number" or sType == "boolean" then
        return tostring(t)
    else
        error("Cannot serialize type " .. sType, 0)
    end
end

function serializeJSON(t, options)
    expect(1, t, "table", "string", "number", "boolean")
    expect(2, options, "table", "boolean", "nil")
    if type(options) == "boolean" then
        options = { nbt_style = options }
    elseif type(options) == "table" then
        field(options, "nbt_style", "boolean", "nil")
        field(options, "unicode_strings", "boolean", "nil")
    end

    local tTracking = {}
    return serializeJSONImpl(t, tTracking, options)
end
