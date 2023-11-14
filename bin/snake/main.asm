; 32 x 32 Grid
; coordinates are stored in 2-byte pairs where COORD is the x
; value and COORD + 1 is the y value
; coords are generally stored in %ab and iterations are usually
; in %cd

#[use(std::gfx::grid::block)]
#[use(std::gfx::grid::point)]
#[use(std::sleep)]

#[dyn(APPLE: 2)]
#[dyn(HEAD: 2)]
#[dyn(SNAKE_LEN: 2)]


; up - 0b00
; down - 0b01
; left - 0b10
; right - 0b11
#[dyn(DIRECTION: 1)]

#[dyn(SNAKE: 2048)] ; (32 * 32) * 2

#[main]
main:
  bank 1

  call erase

  ; new coords for apple
  rand_coord %a
  rand_coord %b
  sw APPLE, %a
  sw APPLE + 1, %b

  mov %a, 5
  mov %b, 0
  sw SNAKE_LEN, %a
  sw SNAKE_LEN + 1, %b


  mov %a, 5
  mov %b, 1
  ldxy SNAKE

  sw %a, %b

  inc %x, %y
  dec %a
  sw %a, %b

  inc %x, %y
  dec %a
  sw %a, %b

  inc %x, %y
  dec %a
  sw %a, %b

  inc %x, %y
  dec %a
  sw %a, %b

  mov %d, 0b11 ; right
  sw DIRECTION, %d

  call full_draw
  ; `loop` is right after main so as long as nothing new
  ; is put here, `jmp loop` is unnecessary.

; move snake
; requires shifting the snake in memory to pop the tail
; and push a new head
loop:
  lw %a, SNAKE
  lw %b, SNAKE + 1
  call update_direction
  call move
  push %a, %b ; store the next head for later
  call check_bounds


  lw %c, SNAKE_LEN
  lw %d, SNAKE_LEN + 1

  ldxy SNAKE
  lw %a ; old tail
  inc %x, %y
  lw %b
  dec %x, %y
  add %x, %y, %c, %d
  add %x, %y, %c, %d
  push %x, %y ; push the current addr
  sub %x, %y, 2, 0 ; comment this to make the snake grow on every tick
  lw %a ; old tail
  inc %x, %y
  lw %b
  block 0

  pop %x, %y
  pop %c, %d
  push %x, %y

  .shift:
    pop %x, %y ; get the address of the last iteration
    sub %x, %y, 4, 0
    lw %a, %b ; current block to be shifted in memory

    inc %x, %y ; move this block to the position of the next block
    sw %a, %b
    dec %x, %y
    push %x, %y

    ; if this iteration == start address of SNAKE
    cmp16 %z, %x, %y, SNAKE + 2
    ; then end
    jeq .end_shift

    ; Check if this block is equal to the next head coords (collision)
    cmp16 %z, %a, %b, %c, %d
    jeq main ; restart

    ; continue loop
    jmp .shift

  .end_shift:

  pop %f, %f ; address of the last iteration doesn't matter anymore
  mov %a, %c
  mov %b, %d
  push %a, %b

  sw SNAKE, %a
  sw SNAKE + 1, %b
  call check_apple

  pop %a, %b
  block 0b001100

  mov %a, 0
  mov %b, 24
  mov %c, 0
  mov %d, 0
  call sleep

  jmp loop

update_direction:
  in %z, KB
  lw %d, DIRECTION

  mov %f, %z
  and %f, 0b0001 ; up arrow
  jnz .up, %f

  mov %f, %z
  and %f, 0b0010 ; down arrow
  jnz .down, %f

  mov %f, %z
  and %f, 0b0100 ; left arrow
  jnz .left, %f

  mov %f, %z
  and %f, 0b1000 ; right arrow
  jnz .right, %f

  ret

  .up:
    cmp %d, 0b01 ; Don't update if current direction is down
    req
    mov %d, 0b00
    sw DIRECTION, %d
    ret

  .down:
    cmp %d, 0b00
    req
    mov %d, 0b01
    sw DIRECTION, %d
    ret

  .left:
    cmp %d, 0b11
    req
    mov %d, 0b10
    sw DIRECTION, %d
    ret

  .right:
    cmp %d, 0b10
    req
    mov %d, 0b11
    sw DIRECTION, %d
    ret

  ; inc/dec a or b depending on d (direction)
  move:
    and %d, 0b11
    jeq .up, %d, 0b00
    jeq .down, %d, 0b01
    jeq .left, %d, 0b10

    inc %a ; move right
    ret

    .up:
      dec %b
      ret

    .down:
      inc %b
      ret

    .left:
      dec %a
      ret


; caller will place coordinates in ab
check_bounds:
  ; Check if x coordinate is not 0-31
  and %a, 0b11100000
  jnz .gameover, %a

  ; Check if y coordinate is not 0-31
  and %b, 0b11100000
  jnz .gameover, %b

  ret

  .gameover:
    pop %f, %f ; forget the caller's address
    jmp main

; Check if the head of the snake == the apple
; if so, set a new apple.
; caller will place coordinates in ab
check_apple:
  lw %c, APPLE
  lw %d, APPLE + 1

  ; compare head and apple
  cmp16 %z, %a, %b, %c, %d
  rneq

  rand_coord %a
  rand_coord %b
  sw APPLE, %a
  sw APPLE + 1, %b
  block 0b110000

  lw %c, SNAKE_LEN
  lw %d, SNAKE_LEN + 1

  ldxy SNAKE
  add %x, %y, %c, %d
  add %x, %y, %c, %d
  lw %a, %b ; old tail
  inc %a, %b
  sw %a, %b
  inc %c, %d
  sw SNAKE_LEN, %c
  sw SNAKE_LEN + 1, %d

  ret

erase:
  lw %c, SNAKE_LEN
  lw %d, SNAKE_LEN + 1

  cmp16 %z, %c, %d, 0, 0
  req

  add %c, %d, %c, %d ; multiply by two (coordinate pairs are 2 bytes long)

  .iter:
    sub %c, %d, 2, 0
    ldxy SNAKE
    add %x, %y, %c, %d
    lw %a
    inc %x, %y
    lw %b

    push %c ; clear_box will modify %c
    block 0
    pop %c

    jnz .iter, %c, %d

  lw %a, APPLE
  lw %b, APPLE + 1
  block 0

  ret


full_draw:
  lw %c, SNAKE_LEN
  lw %d, SNAKE_LEN + 1

  cmp16 %z, %c, %d, 0, 0
  req

  add %c, %d, %c, %d ; multiply by two (coordinate pairs are 2 bytes long)

  .iter:
    sub %c, %d, 2, 0
    ldxy SNAKE
    add %x, %y, %c, %d
    lw %a
    inc %x, %y
    lw %b

    push %c ; clear_box will modify %c
    block 0b001100
    pop %c

    jnz .iter, %c, %d

  lw %a, APPLE
  lw %b, APPLE + 1
  block 0b110000

  ret

; Util stuff

#[macro] rand_coord: {
  ($r: reg) => {
    in $r, RNG
    and $r, 0b00011111
  }
}

; Compare two 16 bit numers with a designated register to trash
#[macro] cmp16: {
  ($inter: reg, $lhs_l: reg, $lhs_h: reg, $rhs_l: any, $rhs_h: any) => {
    cmp $lhs_l, $rhs_l
    mov $inter, %f
    cmp $lhs_h, $rhs_h
    and %f, $inter
  }
  ($inter: reg, $lhs_l: reg, $lhs_h: reg, $rhs: expr) => {
    cmp $lhs_l, $rhs.l
    mov $inter, %f
    cmp $lhs_h, $rhs.h
    and %f, $inter
  }
}

; Return if neq
#[macro] rneq: {
  () => {
    pop %x
    pop %y

    jneq

    push %y
    push %x
  }
}

; Return if eq
#[macro] req: {
  () => {
    pop %x
    pop %y

    jeq

    push %y
    push %x
  }
}


