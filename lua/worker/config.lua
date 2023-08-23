local expect, field
if require then
    expect, field = require "cc.expect".expect, require "cc.expect".field
else
    local ok, did = pcall(dofile, "rom/modules/main/cc/expect.lua")
    if ok then field, expect = did.field, did.expect else field, expect = function(...) end, function(...) end end
end

DEFAULT_CONFIG = {
    -- hostname = "localhost",
    -- port = "56552",
    secure = false,
    name = nil,
    reconnect = true,
    debug = true,
}

Config = {
    hostname = "",
    port = "",
    secure = false,
    name = "",
    reconnect = true,
    debug = false,
}

function Config.__init__(base, args)
    expect(1, args, "table")
    local self = nil

    if #args < 1 then
        error(
            "\nUsage:\n  worker <hostname> <port> [secure] [name] [reconnect] [debug]\nOR\n  worker <path to config file>")
    elseif #args == 1 then
        local path = args[1]
        expect(1, path, "string")

        if not fs.exists(path) then
            error("config file does not exist")
        end

        local success, result = pcall(fs.open, path, "r")
        if not success then
            error("failed to open config file: " .. textutils.serialize(result))
        end

        local fileContents = result.readAll()
        if not fileContents then
            error("failed to read config file")
        end

        self = textutils.unserializeJSON(fileContents)
        if not self then
            error("failed to parse config file")
        end
    else
        -- put it in a table then take it back out so the error message is more intuitive
        local argTable = {
            hostname = args[1],
            port = args[2],
            secure = args[3],
            name = args[4],
            reconnect = args[5],
            debug = args[6],
        }

        field(argTable, "hostname", "string")
        field(argTable, "port", "string", "number")
        field(argTable, "secure", "boolean", "string", "nil")
        field(argTable, "name", "string", "nil")
        field(argTable, "reconnect", "boolean", "string", "nil")
        field(argTable, "debug", "boolean", "string", "nil")

        local hostname = argTable.hostname
        local port = argTable.port
        local secure = argTable.secure
        local name = argTable.name
        local reconnect = argTable.reconnect
        local debug = argTable.debug

        if type(port) == "number" then
            port = string.format("%d", port)
        end

        if not secure then
            secure = DEFAULT_CONFIG.secure
        end
        if type(secure) == "string" then
            secure = secure == "true"
        end

        if not reconnect then
            reconnect = DEFAULT_CONFIG.reconnect
        end
        if type(reconnect) == "string" then
            reconnect = reconnect == "true"
        end

        if not debug then
            debug = DEFAULT_CONFIG.debug
        end
        if type(debug) == "string" then
            debug = debug == "true"
        end

        self = {
            hostname = hostname,
            port = port,
            secure = secure,
            name = name,
            reconnect = reconnect,
            debug = debug,
        }
    end

    if self.debug then
        print("[debug] parsed config: " .. textutils.serialize(self))
    end

    setmetatable(self, { __index = Config })
    return self
end

setmetatable(Config, { __call = Config.__init__ })
