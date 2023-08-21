require "worker.controller"

local args = { ... }

local host = args[1]
local port = args[2]

if not host then
    error("Usage: worker <hostname> [port]")
end

if not port then
    port = "56552"
end

local url = string.format("ws://%s:%s", host, port)

local controller = Controller()

controller:connect(url)

print("connected")

while true do
    if not controller:poll() then
        break
    end
end

print("finished")
