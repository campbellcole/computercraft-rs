require "worker.serialize"

Controller = {
    ws = nil,
}

function Controller.__init__(base)
    local self = {
        ws = nil,
    }
    setmetatable(self, { __index = Controller })
    return self
end

setmetatable(Controller, { __call = Controller.__init__ })

function Controller:connect(url)
    while not self.ws do
        self.ws = http.websocket(url)
        if not self.ws then
            print("failed to connect, retrying in 5 seconds")
            sleep(5)
        end
    end
end

function Controller:__handle_request(request)
    if request.kind == "Echo" then
        return request
    elseif request.kind == "ConnectPeripheral" then
        local address = request.data
        return {
            kind = request.kind,
            data = peripheral.isPresent(address),
        }
    elseif request.kind == "CallPeripheral" then
        local address = request.data.address
        local method = request.data.method
        local args = request.data.args
        local ty = type(args)

        if ty == "nil" then
            args = {}
        elseif ty == "number" or ty == "string" or ty == "boolean" then
            args = { args }
        elseif ty ~= "table" then
            return {
                kind = request.kind,
                data = {
                    success = false,
                    error = "Invalid argument type. Must be nil, number, string, boolean, or array.",
                    result = nil,
                }
            }
        end
        local returns = { pcall(peripheral.call, address, method, table.unpack(args)) }
        local success = table.remove(returns, 1)
        local result = returns
        if #result == 0 then
            result = nil
        end
        if success then
            return {
                kind = request.kind,
                data = {
                    success = true,
                    error = nil,
                    result = result,
                }
            }
        else
            return {
                kind = request.kind,
                data = {
                    success = false,
                    error = result,
                    result = nil,
                }
            }
        end
    elseif request.kind == "GetPeripheralType" then
        local address = request.data
        return {
            kind = request.kind,
            data = peripheral.getType(address),
        }
    end
end

function Controller:poll()
    local msg = self.ws.receive()
    if msg == nil then
        -- the socket has closed, we're done here
        return false
    end
    print(msg)
    msg = textutils.unserializeJSON(msg, { parse_empty_array = false })

    local id = msg.id
    local request = msg.request

    local res_data = self:__handle_request(request)
    local res = {
        id = id,
        response = res_data,
    }
    local ser = serializeJSON(res)

    self.ws.send(ser)

    return true
end
