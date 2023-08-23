# computercraft-rs

A WebSocket layer for interacting with ComputerCraft from Rust. Currently only tested with [CC: Tweaked](https://tweaked.cc/) and [CraftOS-PC](https://www.craftos-pc.cc).

## Terminology

**CC**: An implementation of what is considered ComputerCraft (i.e. CC: Tweaked.) <br />
**Host**: The server hosting the websocket server and controlling the workers. <br />
**Worker**: A ComputerCraft computer running the `worker` program, connected to the Host.

## Notice

**This project is nowhere near completion. The fundamentals are in place, but there is a lot of work to be done.**

<details>
<summary>To-Do List</summary>

- [ ] Documentation
  - [ ] rustdoc
  - [x] README.md
- [x] Error handling
  - [x] Lua error handling (using `pcall`)
  - [x] Rust error handling
- [ ] Access to CC globals (`disk`, `fs`, `os`, etc.)
- [ ] Execution of arbitrary Lua code
  - [ ] Calling Lua files that are on the Worker
  - [ ] Executing Lua code stored in or generated by the Host
- [x] Named Workers
- [x] Multiple Workers
- [x] Two-way serialization
- [x] Async request/response protocol
- [x] Unwrapped peripheral access
  - [x] Attaching to arbitrary peripheral
  - [x] Calling arbitrary methods on peripherals
- [ ] Wrapped peripherals
  - [ ] Standard peripherals (CC)
    - [ ] Command block
    - [ ] Computer
    - [ ] Drive
    - [ ] Modem
    - [x] Monitor
    - [x] Printer
    - [ ] Speaker
  - [ ] Advanced Peripherals
    - [ ] Chat Box
    - [ ] Energy Detector
    - [ ] Environment Detector
    - [ ] Player Detector
    - [ ] Inventory Manager
    - [ ] NBT Storage
    - [ ] Block Reader
    - [ ] Geo Scanner
    - [ ] Redstone Integrator
    - [ ] AR Controller
    - [ ] ME Bridge
    - [ ] RS Bridge **(partially implemented)**
    - [ ] Colony Integrator **(partially implemented)**
  - [ ] Create: Crafts & Additions
    - [ ] Electric Motor
    - [ ] Accumulator
    - [ ] Portable Energy Interface
    - [ ] Redstone Relay
    - [ ] Digital Adapter

</details>

## Purpose

It is not hard to have a CC program to dramatically drop the TPS on a server.
This library is intended to enable offloading expensive computations from the Minecraft server onto a separate process.

It is also useful for people who, like me, are not fond of Lua, and find maintaining a substantial Lua program to be discouraging, to say the least.

## Usage

### Host

```rs
use computercraft::Server;
use computercraft::peripheral::IntoWrappedPeripheral;
use computercraft::wrappers::monitor::Monitor;

let server = Server::listen();

let computer = server.wait_for_connection().await;

let peripheral = computer.find_peripheral("monitor_0").await.unwrap();

let monitor: Monitor = peripheral.into_wrapped().await.unwrap();

monitor.write("Hello from Rust!").await;
```

### Worker

Copy the `worker.lua` file and `worker/` directory to the Worker(s). Decide if you wish to use a JSON config file or arguments (config file is recommended, see [`default_config.json`](/lua/default_config.json)).

##### Configuration Options

| Option    | Argument Pos. | JSON Key    | Description                                             | Default             |
| --------- | ------------- | ----------- | ------------------------------------------------------- | ------------------- |
| Hostname  | 1 (first)     | `hostname`  | The hostname to connect to                              | N/A                 |
| Port      | 2 (second)    | `port`      | The port the websocket is bound to                      | `56552`             |
| Secure    | 3 (third)     | `secure`    | Enables use of `wss` protocol                           | `false`             |
| Name      | 4 (fourth)    | `name`      | The Worker name to be used by the Host                  | `nil`/`null`/`None` |
| Reconnect | 5 (fifth)     | `reconnect` | Automatically reconnect when the socket closes or fails | `true`              |
| Debug     | 6 (sixth)     | `debug`     | Enable verbose logging                                  | `false`             |

The `worker` command has two forms:

1. `worker <arg list...>`: Argument form, the order is defined in the table above.
2. `worker <path to config JSON>`: Config file form, the JSON keys are also defined in the table above.

##### Helper Script

If you are using CraftOS-PC as your CC implementation, you can quickly initialize the computer (i.e. load the Lua code) by running the [`./scripts/init_craftos.sh`](./scripts/init_craftos.sh) script. There are a few options that you can read about by running `./scripts/init_craftos.sh --help`.

If you are running the Minecraft server and Host on the same computer, make sure you've
[configured local IP access](https://tweaked.cc/guide/local_ips.html). If you are having trouble using the hostname `localhost`, try `127.0.0.1` instead. I don't know why they're handled differently, but it reliably solves the issue for me.

**IMPORTANT:** If you are intending on sending large objects over the WebSocket (i.e. `rsBridge.listItems()`), make sure you increase the `websocket_message` value accordingly (**NOTE: See [CC: Tweaked issue #1566](https://github.com/cc-tweaked/CC-Tweaked/issues/1566) \[and [the fix](https://github.com/cc-tweaked/CC-Tweaked/issues/1566#issuecomment-1687052883)\]**). In my testing, using 4MiB (`4194304`) seems to work in most cases.
