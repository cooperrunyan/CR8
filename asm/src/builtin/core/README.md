# `core`

Builtin macros to effectively expand the machine's instruction-set.

| Mnemonic | Args                         | Size | Result                                               |
| -------- | ---------------------------- | ---- | ---------------------------------------------------- |
| `nop`    | None                         | 2    | Do nothing                                           |
| `mov`    | `reg`, `reg`                 | 2    | Move `(1) = (2)`                                     |
| `mov`    | `reg`, `imm8`                | 3    | Move `reg = imm8`                                    |
| `mov`    | `reg`, `reg`, `reg`, `reg`   | 4    | Move `(1, 2) = (3, 4)`                               |
| `mov`    | `reg`, `reg`, `imm8`, `imm8` | 6    | Move `(1, 2) = (3, 4)`                               |
| `mov`    | `reg`, `reg`, `imm16`        | 6    | Move `(1, 2) = imm16`                                |
| `ldhl`   | `imm16`                      | 6    | Move `(%l, %h) = imm16`                              |
| `jnz`    | `reg`                        | 2    | If `reg != 0`, set `PC` to `HL`                      |
| `jnz`    | `imm8`                       | 2    | If `imm8 != 0`, set `PC` to `HL`                     |
| `jnz`    | `imm16`, `imm8`              | 8    | Jump to `imm16` if `imm8 != 0`                       |
| `jnz`    | `imm16`, `reg`               | 8    | Jump to `imm16` if `reg != 0`                        |
| `jnz`    | `imm16`, `reg`, `reg`        | 14   | Jump to `imm16` if `(1, 2) != 0`                     |
| `jmp`    | `imm16`                      | 8    | Unconditional jump to `imm16`                        |
| `jmp`    | `reg`, `reg`                 | 8    | Unconditional jump to `(1, 2)`                       |
| `jmp`    | None                         | 2    | Unconditional jump to `HL`                           |
| `jeq`    | `imm16`                      | 12   | Jump to `imm16` if `F` has `Equal`                   |
| `jneq`   | `imm16`                      | 13   | Jump to `imm16` if `F` has no `Equal`                |
| `jle`    | `imm16`                      | 12   | Jump to `imm16` if `F` has `Equal` or `LessThan`     |
| `jlt`    | `imm16`                      | 12   | Jump to `imm16` if `F` has `LessThan`                |
| `jgt`    | `imm16`                      | 13   | Jump to `imm16` if `F` has no `Equal` nor `LessThan` |
| `jge`    | `imm16`                      | 14   | Jump to `imm16` if `F` has no `LessThan`             |
| `jz`     | `imm16`, `reg`               | 15   | Jump to `imm16` if `reg == 0`                        |
| `call`   | `imm16`                      | 11   | Push `PC` to stack and jump to `imm16`               |
| `call`   | `reg` , `reg`                | 11   | Push `PC` to stack and jump to `(1, 2)`              |
| `ret`    | None                         | 6    | Pop address (pushed by `call`) into `HL` and jump    |
| `lw`     | `reg`, `imm16`               | 4    | Store into `reg` the byte at address `imm16`         |
| `lw`     | `reg`                        | 2    | Store into `reg` the byte at address `HL`            |
| `lw`     | `reg`, `reg`                 | 12   | Read `HL` into `(1)` and `HL + 1` into `(2)`         |
| `lw`     | `reg`, `reg`, `imm16`        | 18   | Read `imm16` into `(1)` and `imm16 + 1` into `(2)`   |
| `sw`     | `reg`                        | 2    | Write to address `HL` the value in `reg`             |
| `sw`     | `imm8`                       | 5    | Write `imm8` to address `HL` and clear flags         |
| `sw`     | `imm16`, `reg`               | 4    | Write to address `imm16` the value in `reg`          |
| `sw`     | `imm16`, `reg`, `reg`        | 14   | Write `(1)` to `imm16` and `(2)` to `imm16 + 1`      |
| `sw`     | `reg`, `reg`                 | 8    | Write `(1)` to `HL` and `(2)` to `HL + 1`            |
| `sw`     | `imm16`, `imm8`              | 7    | Write `imm8` to address `imm16` and clear flags      |
| `push`   | `reg`                        | 2    | Push `reg` to the stack and increment `SP`           |
| `push`   | `imm8`                       | 2    | Push `imm8` to the stack and increment `SP`          |
| `push`   | `reg`, `reg`                 | 4    | Push `(1)` then `(2)` to the stack. `SP += 2`        |
| `push`   | `imm16`                      | 4    | Push low then high to the stack. `SP += 2`           |
| `pop`    | `reg`                        | 2    | Pop into `reg` the value that `SP` points to         |
| `pop`    | `reg`, `reg`                 | 4    | Pop from stack to `(2)`, pop to `(1)`. `SP -= 2`     |
| `in`     | `reg`, `reg`                 | 2    | Receive into `(1)` a byte from port `(2)`            |
| `in`     | `reg`, `imm8`                | 3    | Receive into `reg` a byte from `imm8`                |
| `out`    | `reg`, `reg`                 | 2    | Send to port `(1)`, the value of `(2)`               |
| `out`    | `imm8`, `reg`                | 3    | Send to port `imm8`, the value of `reg`              |
| `send`   | `imm8`, `imm8`               | 6    | Send to port `(1)`, the value of `(2)`               |
| `ping`   | None                         | 6    | Send `PING` to `CTRL` port                           |
| `halt`   | None                         | 6    | Send `HALT` to `CTRL` port                           |
| `brkpt`  | None                         | 6    | Send `BRKPT` to `CTRL` port                          |
| `dbg`    | None                         | 33   | Send `DBG` to `CTRL` port                            |
| `add`    | `reg`, `reg`                 | 2    | Add `(1) += (2)`                                     |
| `add`    | `reg`, `imm8`                | 3    | Add `reg += imm8`                                    |
| `add`    | `reg`, `reg`, `reg`, `reg`   | 4    | 16-bit add `(1, 2) += (3, 4)`                        |
| `add`    | `reg`, `reg`, `imm8`, `imm8` | 6    | 16-bit add `(1, 2) += (3, 4)`                        |
| `add`    | `reg`, `reg`, `imm16`        | 6    | 16-bit add `(1, 2) += imm16`                         |
| `adc`    | `reg`, `reg`                 | 2    | Add `(1) += (2) + CarryFlag`                         |
| `adc`    | `reg`, `imm8`                | 3    | Add `reg += imm8 + CarryFlag`                        |
| `adc`    | `reg`                        | 3    | Increment `reg` if previous operation carried        |
| `inc`    | `reg`                        | 3    | Increment `reg += 1`                                 |
| `inc`    | `reg`, `reg`                 | 6    | 16-bit Increment `(1, 2) += 1`                       |
| `sub`    | `reg`, `reg`                 | 2    | Subtract `(1) -= (2)`                                |
| `sub`    | `reg`, `imm8`                | 3    | Subtract `reg -= imm8`                               |
| `sub`    | `reg`, `reg`, `imm8`, `imm8` | 6    | 16-bit subtraction. `(1, 2) -= (3, 4)`               |
| `sub`    | `reg`, `reg`, `imm16`        | 6    | 16-bit subtraction. `(1, 2) -= imm16`                |
| `sbb`    | `reg`, `reg`                 | 2    | Subtraction. `(1) -= (2) + BorrowFlag`               |
| `sbb`    | `reg`, `imm8`                | 3    | Subtraction. `reg -= imm8 + BorrowFlag`              |
| `sbb`    | `reg`                        | 3    | Decrement `reg` if previous operation borrowed       |
| `dec`    | `reg`                        | 3    | Decrement `reg` by 1                                 |
| `dec`    | `reg`, `reg`                 | 6    | 16-bit Decrement `(1, 2) -= 1`                       |
| `cmp`    | `reg`, `reg`                 | 2    | Compare the two registers. Set `%f` to describe (1)  |
| `cmp`    | `reg`, `imm8`                | 3    | Compare the two values. Set `%f` to describe `reg`   |
| `not`    | `reg`                        | 2    | `reg = ~reg`                                         |
| `and`    | `reg`, `reg`                 | 2    | `(1) &= (2)`                                         |
| `and`    | `reg`, `imm8`                | 3    | `reg &= imm8`                                        |
| `nand`   | `reg`, `reg`                 | 2    | `(1) = ~(1 & 2)`                                     |
| `nand`   | `reg`, `imm8`                | 3    | `reg = ~(reg & imm8)`                                |
| `or`     | `reg`, `reg`                 | 2    | `(1) \|= (2)`                                        |
| `or`     | `reg`, `imm8`                | 3    | `reg \|= imm8`                                       |
| `nor`    | `reg`, `reg`                 | 2    | `(1) = ~(1 \| 2)`                                    |
| `nor`    | `reg`, `imm8`                | 3    | `reg = ~(reg \| imm8)`                               |
| `xor`    | `reg`, `reg`                 | 2    | `(1) ^= (2)`                                         |
| `xor`    | `reg`, `imm8`                | 3    | `reg ^= imm8`                                        |
| `xnor`   | `reg`, `reg`                 | 2    | `(1) = ~(1 ^ 2)`                                     |
| `xnor`   | `reg`, `imm8`                | 3    | `reg = ~(reg ^ imm8)`                                |
| `clrf`   | None                         | 3    | Clear the flags register                             |
| `clrfb`  | None                         | 3    | Clear the `borrow` flag                              |
| `clrfc`  | None                         | 3    | Clear the `carry` flag                               |
