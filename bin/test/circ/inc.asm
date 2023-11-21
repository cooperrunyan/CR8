; 4 bit increments abcd
; jumps back to 0x0000 to loop forever

main:
  adc %a, 1
  adc %b, 0
  adc %c, 0
  adc %d, 0
  jmp main
