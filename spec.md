# `CR8`

- 8 bit data width
- 16-bit address bus (64KB)
- Little endian
- Designed to be implemented with 7-series ttl logic gates

## Registers

| Number | Name | Size   | Description     |
| ------ | ---- | ------ | --------------- |
| `*`    | `SP` | 16-bit | Stack Pointer   |
| `*`    | `PC` | 16-bit | Program Counter |
| 0      | `A`  | 8-bit  | GP - arg 0      |
| 1      | `B`  | 8-bit  | GP - arg 1      |
| 2      | `C`  | 8-bit  | GP - arg 2      |
| 3      | `D`  | 8-bit  | GP - arg 3      |
| 4      | `Z`  | 8-bit  | GP - return     |
| 5      | `L`  | 8-bit  | GP - Low index  |
| 6      | `H`  | 8-bit  | GP - High index |
| 7      | `F`  | 4-bit  | Flags           |

> `*`: [Memory mapped](#memory-layout) register

### Flags (LSB-MSB)

- `LF`: Less than flag
- `EF`: Equal to flag
- `CF`: Carry/No-Borrow flag
- `ZF`: Zero flag

## Instructions

| Code | Pnuemonic | Args              | Result                                   |
| ---- | --------- | ----------------- | ---------------------------------------- |
| 0    | `LW`      | `reg`, `HL/imm16` | `reg = [HL/imm16]`                       |
| 1    | `SW`      | `HL/imm16`, `reg` | `[HL/imm16]` = `reg`                     |
| 2    | `MOV`     | `reg`, `reg/imm8` | `reg = reg/imm8`                         |
| 3    | `PUSH`    | `reg/imm8`        | `[SP++] = reg/imm8`                      |
| 4    | `POP`     | `reg`             | `reg = [SP--]`                           |
| 5    | `JNZ`     | `reg/imm8`        | `if reg/imm8 != 0; PC = [HL]; else: NOP` |
| 6    | `INB`     | `reg`, `reg/imm8` | `reg = PORT[reg/imm8]`                   |
| 7    | `OUTB`    | `reg/imm8`, `reg` | `PORT[reg/imm8] = reg`                   |
| 8    | `CMP*`    | `reg`, `reg/imm8` | `reg - reg/imm8`                         |
| 9    | `ADC*`    | `reg`, `reg/imm8` | `reg = reg + reg/imm8 + CF`              |
| A    | `SBB*`    | `reg`, `reg/imm8` | `reg = reg - (reg/imm8 + CF)`            |
| B    | `OR`      | `reg`, `reg/imm8` | `reg = reg \| reg/imm8`                  |
| C    | `NOR`     | `reg`, `reg/imm8` | `reg = !(reg \| reg/imm8)`               |
| D    | `AND`     | `reg`, `reg/imm8` | `reg = reg & reg/imm8`                   |

> `*`: Sets [FLAG](#flags) register

### Instruction Layout

Instructions are 1-3 bytes long.

#### Header

First byte of the instruction.

| Length | Name      | Description                                                                        |
| ------ | --------- | ---------------------------------------------------------------------------------- |
| 4      | Operation | [Instruction](#instructions) to run                                                |
| 1      | Immediate | Signifies whether the instruction is using an imm as an argument, instead of a reg |
| 3      | Register  | [Register](#registers) to run the operation on                                     |

#### Tail

The 0 to 2 bytes succeeding the instruction header.

##### Case: `reg`

| Length | Name     | Description            |
| ------ | -------- | ---------------------- |
| 5      | EMPTY    | Empty space (unneeded) |
| 3      | Register | [Register](#registers) |

##### Case: `imm8`

| Length | Name | Description   |
| ------ | ---- | ------------- |
| 8      | Byte | Value of imm8 |

##### Case: `imm16`

| Length | Name      | Description        |
| ------ | --------- | ------------------ |
| 8      | Low Byte  | Low byte of imm16  |
| 8      | High Byte | High byte of imm16 |

### Example:

- Move contents of `A` register into `B` register:

```
MOV   B         A
┌──┐ ┌─┐       ┌─┐
00100001  00000000
    │     └───┘
   imm    EMPTY
```

- Compare contents of `C` to 145:

```
CMP   C
┌──┐ ┌─┐
01001010  01001101
    │     └──────┘
   imm      145
```

- Load 0x0100 from RAM, then push it to the stack:

```
MOV   L            │ MOV   H            │ LW    A  │ PUSH  A
┌──┐ ┌─┐           │ ┌──┐ ┌─┐           │ ┌──┐ ┌─┐ │ ┌──┐ ┌─┐
00101101  00000000 │ 00101110  00000001 │ 00000000 │ 00110000
    │     └──────┘ │     │     └──────┘ │     │    │     │
   imm       0     │    imm       1     │    imm   │    imm
```

## Memory Layout

| Start Address | End Address | Size | Purpose         |
| ------------- | ----------- | ---- | --------------- |
| `0x0000`      | `0x7FFF`    | 32Kb | ROM             |
| `0x8000`      | `0xFDFF`    | 30Kb | RAM             |
| `0xFC00`      | `0xFEFF`    | 1Kb  | Stack           |
| `0xFF00`      | `0xFFFB`    | 256B | Empty           |
| `0xFFFC`      | `0xFFFD`    | 2B   | Stack Pointer   |
| `0xFFFE`      | `0xFFFF`    | 2B   | Program Counter |
