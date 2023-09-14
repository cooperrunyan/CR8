# CR8

- 8 bit data width
- 16-bit address bus (64KB) + 8 bit memory bank
- Little endian
- Designed to be implemented with 74HC logic gates

## Registers

| Number | Name | Size  | Description          |
| ------ | ---- | ----- | -------------------- |
| 0      | `A`  | 8-bit | GP - arg 0           |
| 1      | `B`  | 8-bit | GP - arg 1           |
| 2      | `C`  | 8-bit | GP - arg 2           |
| 3      | `D`  | 8-bit | GP - arg 3           |
| 4      | `Z`  | 8-bit | GP - return          |
| 5      | `L`  | 8-bit | Low index byte       |
| 6      | `H`  | 8-bit | High index byte      |
| 7      | `F`  | 8-bit | Flags / Intermediate |

### System Registers

| Name  | Size  | Description          |
| ----- | ----- | -------------------- |
| `PCL` | 8-bit | Program counter LOW  |
| `PCH` | 8-bit | Program counter HIGH |
| `SPL` | 8-bit | Stack pointer LOW    |
| `SPH` | 8-bit | Stack pointer HIGH   |
| `MB`  | 8-bit | Memory Bank          |

### Flags

- 0: `LF`: Less than flag
- 1: `EF`: Equal to flag
- 2: `CF`: Carry
- 4: `BF`: Borrow

## Instructions

> - 0x00 is effectively a `NOP` because it moves A to A
> - `JNZ` with the is-imm bit set to 1 is effectively `JMP`

| Code | Pnuemonic | Args                         | Result                                   |
| ---- | --------- | ---------------------------- | ---------------------------------------- |
| 0    | `MOV`     | `reg`, `reg/imm8`            | `reg = reg/imm8`                         |
| 1    | `LW`      | `reg`, `HL/imm8`, `HL/imm8`  | `reg` = `[HL/(imm8, imm8)]`              |
| 2    | `SW`      | `HL/imm8`, `HL/imm8`, `reg`, | `[HL/(imm8, imm8)]` = `reg`              |
| 3    | `PUSH`    | `reg/imm8`                   | `[SP++] = reg/imm8`                      |
| 4    | `POP`     | `reg`                        | `reg = [SP--]`                           |
| 5    | `JNZ`     | `reg/imm8`,                  | `if reg/imm8 != 0; PC = [HL]; else: NOP` |
| 6    | `IN`      | `reg`, `reg/imm8`            | `reg = PORT[reg/imm8]`                   |
| 7    | `OUT`     | `reg/imm8`, `reg`            | `PORT[reg/imm8] = reg`                   |
| 8    | `CMP*`    | `reg`, `reg/imm8`            | `reg - reg/imm8`                         |
| 9    | `ADC*`    | `reg`, `reg/imm8`            | `reg = reg + reg/imm8 + CF`              |
| A    | `SBB*`    | `reg`, `reg/imm8`            | `reg = reg - (reg/imm8 + CF)`            |
| B    | `OR`      | `reg`, `reg/imm8`            | `reg = reg \| reg/imm8`                  |
| C    | `NOR`     | `reg`, `reg/imm8`            | `reg = !(reg \| reg/imm8)`               |
| D    | `AND`     | `reg`, `reg/imm8`            | `reg = reg & reg/imm8`                   |
| E    | `MB`      | `imm8`                       | `SYSTEM_REGISTER[MB] = imm8`             |

## Built-in Macros

| Pnuemonic | Args       | Result                                    |
| --------- | ---------- | ----------------------------------------- |
| `LDA`     | `$0`, `$1` | `MOV l, $0`, `MOV h, $1`                  |
| `SUB`     | `$0`, `$1` | SBB with no borrow                        |
| `ADD`     | `$0`, `$1` | ADC with no carry                         |
| `INC`     | `$0`       | Increment                                 |
| `DEC`     | `$0`       | Decrement                                 |
| `NAND`    | `$0`, `$1` | Logical NAND                              |
| `NOT`     | `$0`       | Logical NOT                               |
| `XOR`     | `$0`, `$1` | Logical XOR                               |
| `XNOR`    | `$0`, `$1` | Logical XNOR                              |
| `JMP`     | `$0`, `$1` | Unconditional jump                        |
| `JEQ`     | `$0`, `$1` | Jump if Flags is equal to                 |
| `JLT`     | `$0`, `$1` | Jump if Flags is less than                |
| `JLE`     | `$0`, `$1` | Jump if Flags is less than or equal to    |
| `JGT`     | `$0`, `$1` | Jump if Flags is greater than             |
| `JGE`     | `$0`, `$1` | Jump if Flags is greater than or equal to |
| `JNE`     | `$0`, `$1` | Jump if Flags is not equal                |
| `CALL`    | `$0`, `$1` | Pushes PC to stack then jumps to `$0 $1`  |
| `RET`     | None       | Pops H and L from stack, then jumps       |
| `OUTB`    | `$0`, `$1` | `OUT` for immediates                      |
| `HALT`    | None       | Send `SIGHALT` to Control                 |

> `*`: Sets FLAG register

### Instruction Layout

Instructions are 1-4 bytes long. First byte of the instruction looks like:

`OOOOIRRR`

| Bits | Name          |
| ---- | ------------- |
| I    | Is-immediate? |
| O    | Operation     |
| R    | Register      |

> JNZ is always 1 byte long

## Memory Layout

| Start Address | End Address | Size  | Purpose                  |
| ------------- | ----------- | ----- | ------------------------ |
| `0x0000`      | `0x7FFF`    | 32Kb  | ROM                      |
| `0x8000`      | `0xBFFF`    | 16Kb  | Banked RAM               |
| `0xC000`      | `0xFBFF`    | ~13Kb | GP RAM                   |
| `0xFC00`      | `0xFEFF`    | ~2Kb  | Stack + Psuedo-registers |

## Memory Banks

- `0x00`: Builtin-memory
- `0x01`: VRAM
- Extensible

## Controller

The simulator has a Control device at port 0x00 to communicate with the CR8.
Right now it can only issue a HALT command to stop simulating.
