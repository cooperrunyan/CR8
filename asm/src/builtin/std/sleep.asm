
; Assuming the cpu requires n cycles to execute
; n bytes (fairly different in reality), 1 byte corresponds to
; 1 / CYCLE_HZ of time. At 4Mhz, this corresponds to 250ns.
; [sleep] is 512bytes long, which would be about 12.6µs.
; Sleep accepts a 32bit parameter for iterations to run which 
; computes to %abcd * 12.6µs.
; Arguments:
;     a - 12.6µs 
;     b - 3.23ms
;     c - 826ms
;     d - 211.39s
;
;   Note:
;     In a real CPU, each cycle will require several ticks,
;     depending on the operation and arguments. In the simulator,
;     the time spent on an operation is set to the tick speed duration,
;     1 / HZ (250ns at 4Mhz), multiplied by the operation length
;     so a 50-byte program (that doesn't jump at all) ran at 4Mhz would 
;     last 50 * 250ns (12.5µs).
;     
;     Also, the web simulator batches cycles together because of how JavaScript
;     timer durations work. At 4,000Khz (4Mhz), every 1ms, it will tick 4,000 
;     times. This can cause short pause-lengths to effectively round to the 
;     nearest millisecond. The web simulator is fairly weird, and the speed
;     of the sleep tends to be 2x what it would be calculated to be -- what 
;     should take 1s takes 2s.
sleep:
    .loop: ; 52 bytes / iteration
        dec %a
        sbb %b
        sbb %c
        sbb %d

        jnz .loop, %a
        jnz .loop, %b
        jnz .loop, %c
        jnz .loop, %d
        ret

