gpio1    equ     $F0000

         org $400

start    move.b #1,d0
loop     move.b d0,$f0000
         addi.b #1,d0
         bra  loop

