# CR8

- 8 bit data width
- 16-bit address bus (64KB) + 8 bit memory bank
- Little endian
- Designed to be implemented with 74HC logic gates
- RISC architecture
- [Custom Assembly language](./asm/README.md) inspired by
  [Rust](https://rust-lang.org) and AT&T Assembly
- Supports communications with other devices by sending/recieving a byte
- Minimal native instruction set. Macro-supported development.
- 0.240488 Instructions per clock cycle (4.1582 cycles/instruction)
- Designed to run at 1-4 Mhz
- Not pipelined (Instructions are executed synchronously)

## Registers

Registers are written as `%x` where `x` is the id of the register to access.
Registers include:

| Id  | Usage    | Description                                                             |
| --- | -------- | ----------------------------------------------------------------------- |
| `A` | GP       | General purpose                                                         |
| `B` | GP       | General purpose                                                         |
| `C` | GP       | General purpose                                                         |
| `D` | GP       | General purpose                                                         |
| `F` | GP       | Flags regarding the last mathematical operation.                        |
| `X` | GP       | Low index byte (`X` + `Y` are used to read/write to memory).            |
| `Y` | GP       | High index byte (`X` + `Y` are used to read/write to memory).           |
| `Z` | GP       | General purpose                                                         |
| `K` | Internal | [Memory Bank](#memory) (the selected bank to use when reading/writing). |

> `K` register can only be modified with the `bank` instruction.

### Flags

LSB - MSB

- `Equal` (`EF`): `cmp` resulted in `arg1 == arg2`
- `Lessthan` (`LF`): `cmp` resulted in `arg1 < arg2`
- `Carry` (`CF`): Previous addition operation carried.
- `Borrow` (`BF`): Previous subtraction operation borrowed.

## Instructions

| Id | Mnemonic | Args               | Result                                         |
| -- | -------- | ------------------ | ---------------------------------------------- |
| 0  | `MOV`    | `reg`, `reg/imm8`  | `reg = reg/imm8`                               |
| 1  | `JNZ`    | `reg`, `XY/imm16`  | `if reg/imm8 != 0; PC = [XY/imm16]; else: NOP` |
| 2  | `JMP`    | `XY/imm16`         | `PC = [XY/imm16]`                              |
| 3  | `LW`     | `reg`, `XY/imm16`  | `reg = [XY/imm16]`                             |
| 4  | `SW`     | `XY/imm16`, `reg`, | `[XY/imm16] = reg`                             |
| 5  | `PUSH`   | `reg/imm8`         | `[SP++] = reg/imm8`                            |
| 6  | `POP`    | `reg`              | `reg = [SP--]`                                 |
| 7  | `IN`     | `reg`, `reg/imm8`  | `reg = PORT[reg/imm8]`                         |
| 8  | `OUT`    | `reg/imm8`, `reg`  | `PORT[reg/imm8] = reg`                         |
| 9  | `ADC*`   | `reg`, `reg/imm8`  | `reg = reg + reg/imm8 + CF`                    |
| A  | `SBB*`   | `reg`, `reg/imm8`  | `reg = reg + ~(reg/imm8 + BF)`                 |
| B  | `CMP*`   | `reg`, `reg/imm8`  | `reg - reg/imm8`                               |
| C  | `AND`    | `reg`, `reg/imm8`  | `reg = reg & reg/imm8`                         |
| D  | `OR`     | `reg`, `reg/imm8`  | `reg = reg \| reg/imm8`                        |
| E  | `NOR`    | `reg`, `reg/imm8`  | `reg = ~(reg \| reg/imm8)`                     |
| F  | `BANK`   | `reg/imm8`         | `%k = reg/imm8`                                |

> `*`: Updates FLAG register

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
[Memory Bank Register](#registers). If the `K` register is set to `0`, builtin
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

[asm](./asm/README.md) - Compiler for the custom Assembly language.
