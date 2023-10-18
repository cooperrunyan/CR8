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

; forms a linked list
; SNAKE = [ SEGMENT_X, SEGMENT_Y, NEXT_L, NEXT_H  ]
; Ex (starting at addr 0x0000)  forms (0,0) -> (1, 0) -> (2, 0)
;     0x0000 01 02   03   04 05 06   07   08 09 0A   0B   
;     0      0  0x04 0x00 1  0  0x08 0x00 2  0  0x0C 0x00
;
; Why?
;    To move the snake, we will need to remove the tail and add a new head
#[dyn(SNAKE: 2048)] ; (32 * 32) * 4



#[static(DEFAULT_HEAD_X: 5)]
#[static(DEFAULT_HEAD_Y: 5)]

#[static(DEFAULT_SNAKE_LEN: 5)]

#[boot]
main:
  jmp game_start

; Grab random numbers for x and y coordinates
rand_coords:
  in %a, RNG
  in %b, RNG
  and %a, 0b00011111 ; limit to 0-31
  and %b, 0b00011111
  ret

game_start:
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

  call render_clear
  call render_draw
  
  jmp game_loop

full_render:
  call render_draw
  mov %a, 0
  mov %b, 255
  mov %c, 0
  mov %d, 0
  call sleep
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
move:
  lw %c, %d, SNAKE_LEN

  ldhl SNAKE
  lw %a, %b ; push the current head to stack to duplicate later
  dec %l, %h
  push %a, %b
  add %l, %h, %c, %d
  add %l, %h, %c, %d
  push %l, %h ; push the current addr
  dec %l, %h
  dec %l, %h
  lw %a, %b
  call clear_box

  .loop:
    pop %l, %h ; get the address of the last iteration
    sub %l, %h, 4, 0
    lw %a, %b
    inc %l, %h ; move this block to the position of the next block
    sw %a, %b 
    dec %l, %h 
    push %l, %h

    ; if this iteration is not == start address of SNAKE
    cmp %l, (SNAKE + 2) & 0xFF
    mov %d, %f
    cmp %h, (SNAKE + 2) >> 8
    and %f, %d

    ; continue loop
    jneq .loop

  pop %f, %f ; address of the last iteration doesn't matter anymore
  pop %a, %b ; original head from before the .loop

  inc %b   ; move down
  sw SNAKE, %a, %b
  call filled_box
  ret
  
    
game_loop:
  call move
  mov %a, 0
  mov %b, 10
  mov %c, 0
  mov %d, 0
  call sleep
  jmp game_loop
