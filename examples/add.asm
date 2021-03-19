gpio1    equ     $F0000

         org $400

start    move.l #$45DFED78,d0
         move.l #$DDEA7654,d1
         add.l  d1,d0
         trap #0

