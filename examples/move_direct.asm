gpio1    equ     $F0000

         org $400

start    move.l #$12345678,d0
         move.b d0,$f0000
