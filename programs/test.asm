#include "<std>/arch"
#include "<std>/macro"

jmp [main]

#include "<std>/math"

main:
    mov %a, 12
    mov %b, 14

    #marker call
    call [mul]

    #marker debug
    dbg

    #marker halt
    halt

