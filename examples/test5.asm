gpio1     equ $f0000

          org $400

start     move.l #service_level2,$68
          move.l #stack,a7  ; reinit stack pointer
          move.w #$2100,sr  ; enable level2 interrupt

here      bra   here

service_level2
          addi.b  #1,d0
          cmpi.b  #100,d0
          blt     skip
          clr.b   d0
          ;addi.b  #1,d1
          eor.b  #$ff,d1
          move.b  d1,gpio1
skip      rte

ram     ds.b    32
stack   equ      *
