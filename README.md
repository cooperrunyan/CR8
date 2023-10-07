# CR8

- 8 bit data width
- 16-bit address bus (64KB) + 8 bit memory bank
- Little endian
- Designed to be implemented with 74HC logic gates
- RISC-based architecture

## Registers

Represented with 4 bits.

| Number | Name  | Size  | Description          |
| ------ | ----- | ----- | -------------------- |
| 0      | `A`   | 8-bit | GP - arg 0           |
| 1      | `B`   | 8-bit | GP - arg 1           |
| 2      | `C`   | 8-bit | GP - arg 2           |
| 3      | `D`   | 8-bit | GP - arg 3           |
| 4      | `Z`   | 8-bit | GP - return          |
| 5      | `L`   | 8-bit | Low index byte       |
| 6      | `H`   | 8-bit | High index byte      |
| 7      | `F`   | 8-bit | Flags / Intermediate |
| 8      | `PCL` | 8-bit | Program counter LOW  |
| 9      | `PCH` | 8-bit | Program counter HIGH |
| A      | `SPL` | 8-bit | Stack pointer LOW    |
| B      | `SPH` | 8-bit | Stack pointer HIGH   |
| C      | `MB`  | 8-bit | Memory Bank          |

> `F` Is commonly used as an intermediate register in `std` macros, meaning
> certain macros will overwrite its state.

### Flags

- 0: `LF`: Less than flag
- 1: `EF`: Equal to flag
- 2: `CF`: Carry
- 4: `BF`: Borrow

## Instructions

| Code   | Pnuemonic | Args               | Result                                   |
| ------ | --------- | ------------------ | ---------------------------------------- |
| 000000 | `MOV`     | `reg`, `reg/imm8`  | `reg = reg/imm8`                         |
| 000001 | `JNZ`     | `reg/imm8`,        | `if reg/imm8 != 0; PC = [HL]; else: NOP` |
| 000010 | `LW`      | `reg`, `HL/imm16`  | `reg = [HL/imm16]`                       |
| 000011 | `SW`      | `HL/imm16`, `reg`, | `[HL/imm16] = reg`                       |
| 000100 | `PUSH`    | `reg/imm8`         | `[SP++] = reg/imm8`                      |
| 000101 | `POP`     | `reg`              | `reg = [SP--]`                           |
| 000110 | `IN`      | `reg`, `reg/imm8`  | `reg = PORT[reg/imm8]`                   |
| 000111 | `OUT`     | `reg/imm8`, `reg`  | `PORT[reg/imm8] = reg`                   |
| 001000 |           |                    |                                          |
| 001001 |           |                    |                                          |
| 001010 |           |                    |                                          |
| 001011 |           |                    |                                          |
| 001100 |           |                    |                                          |
| 001101 |           |                    |                                          |
| 001110 |           |                    |                                          |
| 001111 |           |                    |                                          |
| 010000 | `ADD*`    | `reg`, `reg/imm8`  | `reg = reg + reg/imm8`                   |
| 010001 | `ADC*`    | `reg`, `reg/imm8`  | `reg = reg + reg/imm8 + CF`              |
| 010010 | `SUB*`    | `reg`, `reg/imm8`  | `reg = reg + ~(reg/imm8 + 1)`            |
| 010011 | `SBB*`    | `reg`, `reg/imm8`  | `reg = reg + ~(reg/imm8 + BF)`           |
| 010100 |           |                    |                                          |
| 010101 |           |                    |                                          |
| 010110 |           |                    |                                          |
| 010111 |           |                    |                                          |
| 011000 | `CMP*`    | `reg`, `reg/imm8`  | `reg - reg/imm8`                         |
| 011001 | `NOT`     | `reg`              | `reg = ~reg`                             |
| 011010 | `AND`     | `reg`, `reg/imm8`  | `reg = reg & reg/imm8`                   |
| 011011 | `NAND`    | `reg`, `reg/imm8`  | `reg = ~(reg & reg/imm8)`                |
| 011100 | `OR`      | `reg`, `reg/imm8`  | `reg = reg \| reg/imm8`                  |
| 011101 | `NOR`     | `reg`, `reg/imm8`  | `reg = ~(reg \| reg/imm8)`               |
| 011110 | `XOR`     | `reg`, `reg/imm8`  | `reg = reg ^ reg/imm8`                   |
| 011111 | `XNOR`    | `reg`, `reg/imm8`  | `reg = ~(reg ^ reg/imm8)`                |

> `*`: Updates FLAG register

### Instruction Layout

- Instructions are 2-3 bytes long.
- First byte of the instruction looks like `XXXXXXYY`, where:
  - `X`: Instruction code (see above)
  - `Y`: Argument type
    - `00`: 1 Register + 0 Immediate
    - `01`: 1 Register + 1 Immediate
    - `10`: 2 Register + 0 Immediate
    - `11`: 0 Register + 1 Immediate

### Register/Immediate Encoding

Bytes following the byte at `[PC]`

```text
    reg1
    ┌──┐
00000000    00000000    00000000
└──┘        └──────┘    └──────┘
reg2        imm(low)    imm2(high)
```

## Memory Layout

| Start Address | End Address | Size  | Purpose    |
| ------------- | ----------- | ----- | ---------- |
| `0x0000`      | `0x7FFF`    | 32Kb  | ROM        |
| `0x8000`      | `0xBFFF`    | 16Kb  | Banked RAM |
| `0xC000`      | `0xFBFF`    | ~14Kb | GP RAM     |
| `0xFC00`      | `0xFEFF`    | ~1Kb  | Stack      |
| `0xFF00`      | `0xFFFF`    | ~256b | Misc       |

### Memory Banks

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

## Macros

Instruction-set is extremely minimal but the assembler offers extensibility with
rust-inspired macros. See `asm/std` for the `std` library which contains macros
and other functions.

```asm
#[macro] jnz: {
    ; Capture the use of `jnz arg0, arg1` where
    ; arg0 is a static, two-byte value and
    ; arg1 is either a static, one-byte value or a register
    ($addr: imm16, $if: imm8 | reg) => {
        mov %l, $addr.l    ; Move low-byte of $addr into %l
        mov %h, $addr.h    ; Move high-byte of $addr into %h
        jnz $if
    }

    ; Additional capture arm
    () => {
        ...
    }
}
```

> This is a snippet from `std::macro::jmp` that extends the native `jnz`
> instruction to also support known addresses (Instead of only being able to
> jump to `HL`).
