; Tests the stack operations
; Result:
;   a = 2
;   b = 4
;   SP increments then goes back to previous state
;   0xFC00 = 4

mov %a, 4
push %a
mov %a, 2
pop %b
