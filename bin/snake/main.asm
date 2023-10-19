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



#[static(DEFAULT_HEAD_X: 5)]
#[static(DEFAULT_HEAD_Y: 5)]

#[static(DEFAULT_SNAKE_LEN: 5)]

#[main]
main:
  mov %mb, 1

  ; new coords for apple
  call rand_coords
  sw APPLE, %a, %b

  mov %a, DEFAULT_SNAKE_LEN
  mov %b, 0
  sw SNAKE_LEN, %a, %b

  mov %a, 2
  mov %b, 1
  ldhl SNAKE

  sw %a, %b

  inc %l, %h
  inc %a
  sw %a, %b

  inc %l, %h
  inc %a
  sw %a, %b

  inc %l, %h
  inc %a
  sw %a, %b

  inc %l, %h
  inc %a
  sw %a, %b

  mov %d, 0b01 ; down
  sw %d, DIRECTION

  call render_clear
  call render_draw
  
  jmp loop

; Grab random numbers for x and y coordinates
rand_coords:
  in %a, RNG
  in %b, RNG
  and %a, 0b00011111 ; limit to 0-31
  and %b, 0b00011111
  ret

full_render:
  call render_draw
  call render_clear
  mov %a, 0
  mov %b, 255
  mov %c, 0
  mov %d, 0
  call sleep
  ret


render_clear:
  ; Length of VRAM render window
  ; mov %a, 0x00
  ; mov %b, 0x20
  ;
  ; .loop:
  ;   dec %a, %b
  ;   ldhl BRAM
  ;   add %l, %h, %a, %b
  ;   sw 0
  ;   jnz .loop, %a, %b
  ;   ret
  mov %a, %b, BRAM
  mov %c, %d, 0x2000
  mov %z, 0

  .loop:
    mov %l, %a
    mov %h, %b
    sw %z
    inc %a, %b
    dec %c, %d
    jnz .loop, %c
    jnz .loop, %d
    ret


render_draw:
  lw %a, %b, APPLE
  call thick_bordered_box
  ; inline_box 0b00000000, 0b00000000, 0b00111100, 0b00111100, 0b00111100, 0b00111100, 0b00000000, 0b00000000

  lw %c, %d, SNAKE_LEN
  
  .loop:
    dec %c, %d
    mov %l, %h, SNAKE
    add %l, %h, %c, %d
    add %l, %h, %c, %d
    lw %a, %b
    push %c, %d
    call filled_box
    pop %c, %d
    jnz .loop, %c, %d
    ret

; move snake
; requires shifting the snake in memory to pop the tail 
; and push a new head
loop:
  lw %a, %b, SNAKE
  call update_direction
  call move 
  call check
  push %a, %b ; store the next head for later


  lw %c, %d, SNAKE_LEN

  ldhl SNAKE
  lw %a, %b 
  dec %l, %h
  add %l, %h, %c, %d
  add %l, %h, %c, %d
  push %l, %h ; push the current addr
  dec %l, %h
  dec %l, %h
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

    ; if this iteration is not == start address of SNAKE
    cmp %l, (SNAKE + 2) & 0xFF
    mov %z, %f
    cmp %h, (SNAKE + 2) >> 8
    and %z, %f
    and %z, 0b0010
    ; then end
    jnz .end_shift, %z

    ; Check if this block is equal to the next head coords (collision)
    cmp %a, %c
    mov %z, %f
    cmp %b, %d
    and %f, %z
    and %f, 0b0010
    jnz main, %f ; restart

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
  mov %b, 5
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
  
  .done:
    ret

  .up:
    jeq .done, %d, 0b01 ; 
    mov %d, 0b00
    sw %d, DIRECTION
    jmp .done
    
  .down:
    jeq .done, %d, 0b00
    mov %d, 0b01
    sw %d, DIRECTION
    jmp .done
    
  .left:
    jeq .done, %d, 0b11
    mov %d, 0b10
    sw %d, DIRECTION
    jmp .done
    
  .right:
    jeq .done, %d, 0b10
    mov %d, 0b11
    sw %d, DIRECTION
    jmp .done
    
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
check:
  ; Check if x coordinate is not 0-31
  mov %z, %a
  and %z, 0b11100000 
  jnz .gameover, %z

  ; Check if y coordinate is not 0-31
  mov %z, %b
  and %z, 0b11100000 
  jnz .gameover, %z 

  ret

  .gameover:
    pop %f, %f ; forget the caller's address
    jmp main

; caller will place coordinates in ab
check_apple:
  lw %c, %d, APPLE

  ; compare head and apple
  cmp %a, %c
  mov %z, %f
  cmp %b, %d
  and %f, %z

  jeq .change
  ret
  .change:

  ; rand_coords will change ab
  call rand_coords
  sw APPLE, %a, %b
  call thick_bordered_box

  ret
  
