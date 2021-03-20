; strtolower:
; Copy a null-terminated ASCII string, converting
; all alphabetic characters to lower case.
;
; Entry parameters:
;   (SP+4): Source string address
;   (SP+8): Target string address

                org     $00100000       ;Start at 00100000
                move.l  #$00001000,(a7) ;Initialize $2000 with 'ABCD'
                move.l  #$00002000,-(a7)
                move.l  (a7),a0
                move.l  #$41424344,(a0) 
                subi.w  #$4,a7
strtolower      public
                link    a6,#0           ;Set up stack frame
                movea   8(a6),a0        ;A0 = src, from stack
                movea   12(a6),a1       ;A1 = dst, from stack
loop            move.b  (a0)+,d0        ;Load D0 from (src), incr src
                cmpi    #'A',d0         ;If D0 < 'A',
                blo     copy            ;skip
                cmpi    #'Z',d0         ;If D0 > 'Z',
                bhi     copy            ;skip
                addi    #'a'-'A',d0     ;D0 = lowercase(D0)
copy            move.b  d0,(a1)+        ;Store D0 to (dst), incr dst
                bne     loop            ;Repeat while D0 <> NUL
                unlk    a6              ;Restore stack frame
                rts                     ;Return
                end