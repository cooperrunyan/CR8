# CR8

- 8 bit data width
- 16-bit address bus (64KB) + 8 bit memory bank
- Little endian
- Designed to be implemented with 74HC logic gates
- RISC-based architecture
- [Custom Assembly language](./asm/README.md) inspired by
  [Rust](https://rust-lang.org) and AT&T Assembly
- Supports communications with other devices by sending/recieving a byte
- Minimal native instruction set. Macro-supported development.

## Registers

Registers are written as `%x` where `x` is the id of the register to access.
Registers include:

| Id    | Usage                                                                   |
| ----- | ----------------------------------------------------------------------- |
| `A`   | General purpose.                                                        |
| `B`   | General purpose.                                                        |
| `C`   | General purpose.                                                        |
| `D`   | General purpose.                                                        |
| `Z`   | General purpose.                                                        |
| `F`   | Flags regarding the last mathematical operation.                        |
| `L`   | Low index byte (`L` + `H` are used to read/write to memory).            |
| `H`   | High index byte (`L` + `H` are used to read/write to memory).           |
| `PCL` | Program Counter (low byte)                                              |
| `PCH` | Program Counter (high byte)                                             |
| `SPL` | Stack Pointer (low byte)                                                |
| `SPH` | Stack Pointer (high byte)                                               |
| `MB`  | [Memory Bank](#memory) (the selected bank to use when reading/writing). |

> It is EXTREMELY unsafe to mess with `PC` in a program, the computer takes care
> of it already. As well, it is ill-advised to change `SP` because `pop` and
> `push` manage it.

### Flags

LSB - MSB

- `Equal` (`EF`): `cmp` resulted in `arg1 == arg2`
- `Lessthan` (`LF`): `cmp` resulted in `arg1 < arg2`
- `Carry` (`CF`): Previous addition operation carried.
- `Borrow` (`BF`): Previous subtraction operation borrowed.

## Memory

- 16-bit address bus (64KB) + 8 bit [Memory Bank](#registers)
- Little endian

### Memory Layout

| Start Address | End Address | Size  | Purpose    |
| ------------- | ----------- | ----- | ---------- |
| `0x0000`      | `0x7FFF`    | 32Kb  | ROM        |
| `0x8000`      | `0xBFFF`    | 16Kb  | Banked RAM |
| `0xC000`      | `0xFBFF`    | ~14Kb | GP RAM     |
| `0xFC00`      | `0xFEFF`    | ~1Kb  | Stack      |
| `0xFF00`      | `0xFFFF`    | ~256b | Misc       |

### Memory Banks

Each bank is 16Kb long. Banks are accessed through the
[Memory Bank Register](#registers). If the `MB` register is set to `0`, builtin
ram is used.

- `0x00`: Builtin-memory
- `0x01`: VRAM
- `...`: Extensible

## Devices

Up to 256 devices can be connected at once. A device can either `send` a byte to
the system or `receive` a byte from the system.

### `0x00`: SysCtrl - _BUILTIN_

- `0x00`: `PING` - Ping the controller.
- `0x01`: `HALT` - Stop the clock.
- `0x02`: `DBG` - Debug the system's register state.
- `0x03`: `BRKPT` - Pause the runner until a line is sent to stdin. Only enabled
  if `--dbg` flag is true.

### `0x01`: Keyboard

- Stores state for 8 keys. Stores keys' pressed-state in 1 byte.
- Currently does not support receiving information.
- If it is asked to `send` it will flush its state to the receiver.
- When a key is pressed, it sets its corresponding bit into the state, which is
  stored until it is flushed.
- Key bit number (Can be modified):
  - `0`: `↑`
  - `1`: `↓`
  - `2`: `←`
  - `3`: `→`
  - `4`: `spacebar`
  - `5`: `r`
  - `6`: `+`
  - `7`: `-`

### `0x02`: Rng

- `send`: Sends a random byte
- `receive`: Does nothing

## Assembler

[asm](./asm/README.md) -- Compiler for the custom Assembly language.
