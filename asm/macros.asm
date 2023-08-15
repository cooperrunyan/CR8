@ref SNAKE = $1

@macro add! a b {
  adc *a *b
}

@macro jnz l h bool {
  mv %l *l
  mv %h *h
  jnz *bool
}

@macro jmp l h {
  jnz! *l *h $1
}
