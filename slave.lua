require "slave.controller"

local args = { ... }

local host = args[1]

if not host then
    error("Usage: slave <hostname>")
end

local url = string.format("ws://%s:56552", host)

local controller = Controller()

controller:connect(url)

print("connected")

while true do
    if not controller:poll() then
        break
    end
end

print("finished")
