; 32 x 32 (byte) Grid
; 8 x 8 px boxes
; coordinates are stored in 2-byte pairs where COORD is the x 
; value and COORD + 1 is the y value
; coords are generally stored in %ab and iterations are usually
; in %cd

#[use(std::gfx::grid::block::filled)]
#[use(std::gfx::grid::block::thick_bordered)]
#[use(std::gfx::grid::block::clear)]
#[use(std::gfx::grid::point)]
#[use(std::gfx::grid::inline_box)]
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
  mov %mb, 1

  call erase

  ; new coords for apple
  rand_coord %a
  rand_coord %b
  sw APPLE, %a, %b

  mov %a, 5
  mov %b, 0
  sw SNAKE_LEN, %a, %b

  mov %a, 5
  mov %b, 1
  ldhl SNAKE

  sw %a, %b

  inc %l, %h
  dec %a
  sw %a, %b

  inc %l, %h
  dec %a
  sw %a, %b

  inc %l, %h
  dec %a
  sw %a, %b

  inc %l, %h
  dec %a
  sw %a, %b

  mov %d, 0b11 ; right
  sw %d, DIRECTION
  
  call full_draw
  ; `loop` is right after main so as long as nothing new
  ; is put here, `jmp loop` is unnecessary.
  
; move snake
; requires shifting the snake in memory to pop the tail 
; and push a new head
loop:
  lw %a, %b, SNAKE
  call update_direction
  call move 
  push %a, %b ; store the next head for later
  ; call check_bounds


  lw %c, %d, SNAKE_LEN

  ldhl SNAKE
  lw %a, %b 
  dec %l, %h
  add %l, %h, %c, %d
  add %l, %h, %c, %d
  push %l, %h ; push the current addr
  sub %l, %h, 2, 0 ; comment this to make the snake grow on every tick
  lw %a, %b ; old tail
  call clear_box
  
  pop %l, %h
  pop %c, %d
  push %l, %h

  .shift:
    pop %l, %h ; get the address of the last iteration
    sub %l, %h, 4, 0
    lw %a, %b ; current block to be shifted in memory
    
    inc %l, %h ; move this block to the position of the next block
    sw %a, %b 
    dec %l, %h 
    push %l, %h

    ; if this iteration == start address of SNAKE
    cmp16 %z, %l, %h, SNAKE + 2
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
  
  sw SNAKE, %a, %b
  call check_apple

  pop %a, %b
  call filled_box

  mov %a, 0
  mov %b, 1
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
    sw %d, DIRECTION
    ret
    
  .down:
    cmp %d, 0b00
    req
    mov %d, 0b01
    sw %d, DIRECTION
    ret
    
  .left:
    cmp %d, 0b11
    req
    mov %d, 0b10
    sw %d, DIRECTION
    ret
    
  .right:
    cmp %d, 0b10
    req
    mov %d, 0b11
    sw %d, DIRECTION
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
  lw %c, %d, APPLE

  ; compare head and apple
  cmp16 %z, %a, %b, %c, %d
  rneq

  rand_coord %a
  rand_coord %b
  sw APPLE, %a, %b
  call thick_bordered_box
  
  lw %c, %d, SNAKE_LEN

  ldhl SNAKE
  add %l, %h, %c, %d
  add %l, %h, %c, %d
  lw %a, %b ; old tail
  inc %a, %b
  sw %a, %b
  inc %c, %d
  sw SNAKE_LEN, %c, %d 

  ret
  
erase:
  lw %c, %d, SNAKE_LEN

  cmp16 %z, %c, %d, 0, 0
  req

  add %c, %d, %c, %d ; multiply by two (coordinate pairs are 2 bytes long)

  .iter:
    sub %c, %d, 2, 0
    ldhl SNAKE
    add %l, %h, %c, %d
    lw %a 
    inc %l, %h
    lw %b

    push %c ; clear_box will modify %c
    call clear_box
    pop %c 

    jnz .iter, %c, %d
  
  lw %a, %b, APPLE
  call clear_box

  ret


full_draw:
  lw %c, %d, SNAKE_LEN

  cmp16 %z, %c, %d, 0, 0
  req

  add %c, %d, %c, %d ; multiply by two (coordinate pairs are 2 bytes long)

  .iter:
    sub %c, %d, 2, 0
    ldhl SNAKE
    add %l, %h, %c, %d
    lw %a 
    inc %l, %h
    lw %b

    push %c ; clear_box will modify %c
    call filled_box
    pop %c 

    jnz .iter, %c, %d
  
  lw %a, %b, APPLE
  call thick_bordered_box

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
  ($inter: reg, $lhs_l: reg, $lhs_h: reg, $rhs_l: lit | reg, $rhs_h: lit | reg) => {
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
    pop %l
    pop %h

    jneq

    push %h
    push %l
  }
}

; Return if eq
#[macro] req: {
  () => {
    pop %l
    pop %h

    jeq

    push %h
    push %l
  }
}

  
