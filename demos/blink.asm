    .org $C000

reset:
    lda #$FF
    sta $8002 ; initialize blink counter
    lda #$50
    sta $8000 ; set blink pattern

loop:
    ror ; rotate blink pattern
    sta $8000 ; store blink pattern
    jmp loop ; loop

temp: .byte 0

    .org $FFFC
    .word reset