    ; org $400
    move.b #$45,d1
    move.b #$17,d2
    abcd d1,d2
    bcs abcd_fail
    cmpi.b #$62,d2
    bne abcd_fail
    abcd d1,d2
    bcc abcd_fail
    cmpi.b #$07,d2
    bne abcd_fail
    abcd d2,d3
    bcs abcd_fail
    cmpi.b #$08,d3
    bne abcd_fail
    move.l d6,d1
    ori.b #$10,CCR 
    nop
    nop

abcd_fail: bra abcd_fail    