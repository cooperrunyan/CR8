# CR8

- 8 bit data width
- 16-bit address bus (64KB)
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

### Flags

- 0: `LF`: Less than flag
- 1: `EF`: Equal to flag
- 2: `CF`: Carry
- 4: `BF`: Borrow

## Instructions

> 0x00 is effectively a `NOP` because it moves A to A

| Code | Pnuemonic | Args                        | Result                                   |
| ---- | --------- | --------------------------- | ---------------------------------------- |
| 0    | `MOV`     | `reg`, `reg/imm8`           | `reg = reg/imm8`                         |
| 1    | `LW`      | `reg`, `HL/imm8`, `HL/imm8` | `reg` = `[HL/(imm8, imm8)]`              |
| 2    | `SW`      | `reg`, `HL/imm8`, `HL/imm8` | `[HL/(imm8, imm8)]` = `reg`              |
| 3    | `PUSH`    | `reg/imm8`                  | `[SP++] = reg/imm8`                      |
| 4    | `POP`     | `reg`                       | `reg = [SP--]`                           |
| 5    | `JNZ`     | `reg/imm1`,                 | `if reg/imm1 != 0; PC = [HL]; else: NOP` |
| 6    | `IN`      | `reg`, `reg/imm8`           | `reg = PORT[reg/imm8]`                   |
| 7    | `OUT`     | `reg/imm8`, `reg`           | `PORT[reg/imm8] = reg`                   |
| 8    | `CMP*`    | `reg`, `reg/imm8`           | `reg - reg/imm8`                         |
| 9    | `ADC*`    | `reg`, `reg/imm8`           | `reg = reg + reg/imm8 + CF`              |
| A    | `SBB*`    | `reg`, `reg/imm8`           | `reg = reg - (reg/imm8 + CF)`            |
| B    | `OR`      | `reg`, `reg/imm8`           | `reg = reg \| reg/imm8`                  |
| C    | `NOR`     | `reg`, `reg/imm8`           | `reg = !(reg \| reg/imm8)`               |
| D    | `AND`     | `reg`, `reg/imm8`           | `reg = reg & reg/imm8`                   |

## Built-in Macros

| Pnuemonic | Args                     | Result                                                  |
| --------- | ------------------------ | ------------------------------------------------------- |
| `LDA`     | `$0`, `$1`               | `MOV l, $0`, `MOV h, $1`                                |
| `SUB`     | `$0`, `$1`               | SBB with no borrow                                      |
| `ADD`     | `reg`, `reg`             | ADC with no carry                                       |
| `NAND`    | `reg`, `reg`             | Logical NAND                                            |
| `JNZA`    | `reg`, `reg`, `reg/imm8` | `LDA reg, reg`, `JNZ reg/imm8`                          |
| `JMP`     | `imm8`, `imm8`           | `JNZA imm8, imm8, 0x1`                                  |
| `JEQ`     | `imm8`, `imm8`           | `AND f, 0b0010`, `JNZA imm8, imm8, f`                   |
| `JLT`     | `imm8`, `imm8`           | `AND f, 0b0001`, `JNZA imm8, imm8, f`                   |
| `JLE`     | `imm8`, `imm8`           | `AND f, 0b0011`, `JNZA imm8, imm8, f`                   |
| `JGT`     | `imm8`, `imm8`           | `NOT f`, `AND f, 0b0001`, `JNZA imm8, imm8, f`          |
| `JGE`     | `imm8`, `imm8`           | `NAND f, 0b0001`, `AND f, 0b0011`, `JNZA imm8, imm8, f` |
| `JNE`     | `imm8`, `imm8`           | `NOT f`, `AND f, 0b0010`, `JNZA imm8, imm8, f`          |

> `*`: Sets FLAG register

### Instruction Layout

Instructions are 1-3 bytes long.

> JNZ is always 1 byte long

#### Header

First byte of the instruction.

| Length | Name      | Description                                                                        |
| ------ | --------- | ---------------------------------------------------------------------------------- |
| 4      | Operation | Instruction to run                                                                 |
| 1      | Immediate | Signifies whether the instruction is using an imm as an argument, instead of a reg |
| 3      | Register  | Register to run the operation on                                                   |

#### Tail

The 0 to 2 bytes succeeding the instruction header.

## Memory Layout

| Start Address | End Address | Size | Purpose         |
| ------------- | ----------- | ---- | --------------- |
| `0x0000`      | `0x7FFF`    | 32Kb | ROM             |
| `0x8000`      | `0xFDFF`    | 30Kb | RAM             |
| `0xFC00`      | `0xFEFF`    | 1Kb  | Stack           |
| `0xFF00`      | `0xFFFB`    | 256B | Empty           |
| `0xFFFC`      | `0xFFFD`    | 2B   | Stack Pointer   |
| `0xFFFE`      | `0xFFFF`    | 2B   | Program Counter |
