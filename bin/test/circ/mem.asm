; Tests writing to memory and banks.
; Builtin RAM 0x8000 = 1
; Bank 1 at 0x0001 = 1
; Bank 2 at 0x0002 = 1

mov %a, 1
sw (0x8000), %a
bank 1
sw (0x8001), %a
bank 2
sw (0x8002), %a
