@static byte REF = 1
@data byte x
@data dble y
@data word z


main:
  lda [t]
  jnz 1

t:
  adc %a, %b
