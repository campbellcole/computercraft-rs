require "worker.controller"
require "worker.config"

local args = { ... }

local config = Config(args)

local controller = Controller(config)

controller:start()
