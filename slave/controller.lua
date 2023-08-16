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
    end
end

function Controller:poll()
    local msg = self.ws.receive()
    if msg == nil then
        -- the socket has closed, we're done here
        return false
    end
    print(msg)
    msg = textutils.unserialiseJSON(msg, { parse_empty_array = false })

    local id = msg.id
    local request = msg.request

    local res_data = self:__handle_request(request)
    local res = {
        id = id,
        response = res_data,
    }
    local ser = textutils.serialiseJSON(res, { unicode_strings = true })

    self.ws.send(ser)

    return true
end
