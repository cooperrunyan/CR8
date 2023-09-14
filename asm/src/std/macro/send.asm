#include "<std>/arch"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/macro/send

#macro
send [i0, i1]:
  push %f
  mov %f, $i1
  out $i0, %f
  pop %f

#macro
halt []:
  send &CTRL, &CTRLHALT

#macro
ping []:
  send &CTRL, &CTRLPING

#macro
peek [a0]:
  send &CTRL, &CTRLPEEK
  send &CTRL, $a0l
  send &CTRL, $a0h

#macro
dbg []:
  send &CTRL, &CTRLDBG
