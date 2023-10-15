#[static(ROM: 0x0000)]
#[static(BRAM: 0x8000)]
#[dyn(&0xC000)]
#[static(GPRAM: 0xC000)]
#[static(STACK: 0xFC00)]
#[static(STACK_END: 0xFEFF)]


; Psuedo Register addresses
; Used for temporary data
#[static(PSR0: 0xFF00)]
#[static(PSR1: 0xFF01)]
#[static(PSR2: 0xFF02)]
#[static(PSR3: 0xFF03)]
#[static(PSR4: 0xFF04)]
#[static(PSR5: 0xFF05)]
#[static(PSR6: 0xFF06)]
#[static(PSR7: 0xFF07)]
#[static(PSR8: 0xFF08)]
#[static(PSR9: 0xFF09)]

#[static(CTRL: 0x00)]
#[static(SIGPING: 0x00)]
#[static(SIGHALT: 0x01)]
#[static(SIGDBG: 0x02)]
#[static(SIGBRKPT: 0x03)]

#[static(KB: 0x01)]

#[static(RNG: 0x02)]

