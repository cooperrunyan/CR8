#[use(std)]

#[main]
main:
  bank 1

  .loop:
    test 0, 0
    test 1, 1
    test 0, 31
    test 31, 31
    test 31, 0
    jmp .loop

#[macro] test: {
  ($a: lit, $b: lit) => {
    mov %a, $a
    mov %b, $b
    block 0b110011
  }
}
