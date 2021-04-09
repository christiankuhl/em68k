
*  TOS 1.04  startup  disassembled code (machine code) 



     org    $FC0000




lFC0000	bra.s	lFC0030

* OS header
	DC.B	1,4    * TOS V Major, Minor

	dc.l   	$FC0030   *OS start exec. adr.

lFC0008	DC.L	$FC0000   *OS base adr

	dc.l	$611C   * First adr. not used by OS

	DC.L	$FC0030   *Reserved

lFC0014	DC.L	lFE81FE  * GEM mem. usage pb. pointer

	dc.l	$04061989   * OS creation date
	DC.B	0
lFC001D	DC.B	7     *Config bits - language ...

lFC001E	DC.B	$12,$86   *OS creation date in GEMDOS format

	DC.L	$378C    * GEMDOS pools base adr.
	DC.L	$E7D	* KBshift address
	DC.L	$5622	*Pointer to var. of actual GEMDOS proc.

	DC.B	0,0,0,0    *Reserved

lFC0030	move.w	#$2700,sr
	reset
	suba.l	a5,a5
	cmpi.l	#$FA52235F,$FA0000   * Cart #1
	bne.s	lFC004E
	lea	lFC004E(pc),a6
	jmp	$FA0004
lFC004E	lea	lFC0056(pc),a6
	bra	lFC066A
lFC0056	bne.s	lFC005E
	move.b	$424(a5),-$7FFF(a5)
lFC005E	cmpi.l	#$31415926,$426(a5)   * Resvalid
	bne.s	lFC0080
	move.l	$42A(a5),d0   * Resvector
	tst.b	$42A(a5)
	bne.s	lFC0080
	btst	#0,d0
	bne.s	lFC0080
	movea.l	d0,a0
	lea	lFC005E(pc),a6
	jmp	(a0)

lFC0080	suba.l	a5,a5
	lea	-$7800(a5),a0
	move.b	#7,(a0)
	move.b	#$C0,2(a0)
	move.b	#$E,(a0)
	move.b	#7,2(a0)
	btst	#0,lFC001D(pc)     
	beq.s	lFC00B0
	lea	lFC00AA(pc),a6
	bra	lFC0DC2

lFC00AA	move.b	#2,-$7DF6(a5)    * Vfreq = 50 Hz
lFC00B0	lea	-$7DC0(a5),a1 *  Palette base address
	move.w	#$F,d0
	lea	lFC068A(pc),a0   * Init palette
lFC00BC	move.w	(a0)+,(a1)+
	dbf	d0,lFC00BC
	move.b	#1,-$7DFF(a5) * VBase high.b = 1
	clr.b	-$7DFD(a5) * Vbase medium.b = 0
	move.b	$424(a5),d6 * Memctrl low nibble
	move.l	$42E(a5),d5 * phystop
	lea	lFC00DC(pc),a6
	bra	lFC066A

lFC00DC	beq	lFC01F2
	clr.w	d6
	move.b	#$A,-$7FFF(a5)
	movea.w	#8,a0
	lea	$200008,a1
	clr.w	d0
lFC00F4	move.w	d0,(a0)+
	move.w	d0,(a1)+
	add.w	#$FA54,d0
	cmpa.l	#$200,a0
	bne.s	lFC00F4
	move.l	#$200000,d1
lFC010A	lsr.w	#2,d6
	movea.w	#$208,a0
	lea	lFC0118(pc),a5
	bra	lFC0654

lFC0118	beq.s	lFC013A
	movea.w	#$408,a0
	lea	lFC0126(pc),a5
	bra	lFC0654

lFC0126	beq.s	lFC0138
	movea.w	#8,a0
	lea	lFC0134(pc),a5
	bra	lFC0654

lFC0134	bne.s	lFC013A
	addq.w	#4,d6
lFC0138	addq.w	#4,d6
lFC013A	sub.l	#$200000,d1
	beq.s	lFC010A
	move.b	d6,$FFFF8001
	lea	$8000,a7
	movea.l	8,a4
	lea	lFC018C(pc),a0
	move.l	a0,8
	move.w	#$FB55,d3
	move.l	#$20000,d7
	movea.l	d7,a0
lFC016A	movea.l	a0,a1
	move.w	d0,d2
	moveq	#$2A,d1
lFC0170	move.w	d2,-(a1)
	add.w	d3,d2
	dbf	d1,lFC0170
	movea.l	a0,a1
	moveq	#$2A,d1
lFC017C	cmp.w	-(a1),d0
	bne.s	lFC018C
	clr.w	(a1)
	add.w	d3,d0
	dbf	d1,lFC017C
	adda.l	d7,a0
	bra.s	lFC016A
lFC018C	suba.l	d7,a0
	move.l	a0,d5
	move.l	a4,8
	suba.l	a5,a5
	move.l	d5,d0
	sub.l	#$8000,d0
	lsr.w	#8,d0
	move.b	d0,-$7DFD(a5)
	swap	d0
	move.b	d0,-$7DFF(a5)
	movea.l	d5,a0
	move.l	#$400,d4
	moveq	#0,d0
	moveq	#0,d1
	moveq	#0,d2
	moveq	#0,d3
lFC01BC	movem.l	d0-d3,-(a0)
	movem.l	d0-d3,-(a0)
	movem.l	d0-d3,-(a0)
	movem.l	d0-d3,-(a0)
	cmpa.l	d4,a0
	bne.s	lFC01BC
	suba.l	a5,a5
	move.b	d6,$424(a5)
	move.l	d5,$42E(a5)
	move.l	#$752019F3,$420(a5)
	move.l	#$237698AA,$43A(a5)
	move.l	#$5555AAAA,$51A(a5)
lFC01F2	suba.l	a5,a5
	movea.l	#$980,a0
	movea.l	#$10000,a1 
	moveq	#0,d0 * Clear memory
lFC0202	move.w	d0,(a0)+
	cmpa.l	a0,a1
	bne.s	lFC0202
	movea.l	$42E(a5),a0 * phystop
	suba.l	#$8000,a0 * phystop - 0x8000
	move.w	#$7FF,d1 * 2047
	move.l	a0,$44E(a5) * screen memory base pointer = phystop - 0x8000
	move.b	$44F(a5),-$7DFF(a5) * Set this to the video card
	move.b	$450(a5),-$7DFD(a5) * ...
lFC0226	move.l	d0,(a0)+  * Clear screen memory
	move.l	d0,(a0)+ * ...
	move.l	d0,(a0)+ * ...
	move.l	d0,(a0)+ * ...
	dbf	d1,lFC0226
	movea.l	lFC0014(pc),a0      * AES descriptor
	cmpi.l	#$87654321,(a0)
	beq.s	lFC0242
	lea	lFC0008(pc),a0
lFC0242	move.l	4(a0),$4FA(a5)
	move.l	8(a0),$4FE(a5)
	move.l	#lFC16BA,$46A(a5) 
	move.l	#lFC1A24,$476(a5)
	move.l	#lFC173C,$472(a5)
	move.l	#lFC18EC,$47E(a5)
	move.l	#lFC1CC6,$47A(a5)
	move.l	#lFC3392,$506(a5)
	move.l	#lFC32F6,$50A(a5)
	move.l	#lFC3408,$50E(a5)
	move.l	#lFC3422,$512(a5)
	move.l	#lFC0D0C,$502(a5)
	move.l	$44E(a5),$436(a5)
	move.l	$4FA(a5),$432(a5)
	lea	$378A,a7
	move.w	#8,$454(a5)
	st	$444(a5)
	move.w	#3,$440(a5)
	move.l	#$181C,$4C6(a5)
	move.w	#$FFFF,$4EE(a5)
	move.l	#lFC0000,$4F2(a5)
	move.l	#$93A,$4A2(a5)
	move.l	#lFC0652,$46E(a5)
	clr.l	$4C2(a5)
	clr.w	$59E(a5)
	bsr	lFC0E62
	lea	lFC07A0(pc),a3
	lea	lFC0652(pc),a4
fc02fa	cmpi.l	#$FA52235F,$FA0000   * Cart
	beq.s	lFC032C
	lea	lFC0B0A(pc),a1
fc030a	adda.l	#$2000000,a1
	lea	8,a0
	move.w	#$3D,d0
lFC031A	move.l	a1,(a0)+
	adda.l	#$1000000,a1
	dbf	d0,lFC031A
	move.l	a3,$14
lFC032C	moveq	#6,d0
	lea	$64(a5),a1
lFC0332	move.l	#lFC07A0,(a1)+
	dbf	d0,lFC0332
	move.l	#lFC06C0,$70(a5)
	move.l	#lFC06AA,$68(a5)
	move.l	a3,$88(a5)
	move.l	#lFC07CA,$B4(a5)
	move.l	#lFC07C4,$B8(a5)
	move.l	#lFCAB3C,$28(a5)
	move.l	a4,$400(a5)
	move.l	#lFC07C0,$404(a5)
	move.l	a4,$408(a5)
	lea	$4CE(a5),a0
	move.l	a0,$456(a5)
	move.w	#7,d0
lFC0384	clr.l	(a0)+
	dbf	d0,lFC0384
	lea	lFC0978,a0
	movea.w	#$51E,a1
	moveq	#$1F,d0
lFC0396	move.l	(a0)+,(a1)+
	dbf	d0,lFC0396
	bsr	lFC34FC
	move.l	#lFC052E,-(a7)
	move.w	#1,-(a7)
	jsr	lFC3480
	addq.l	#6,a7
	move.l	#$7FFF,d0
lFC03B8	bsr	lFC054A
	dbf	d0,lFC03B8
	moveq	#2,d0
	bsr	lFC0628
	suba.l	a5,a5
	move.b	-$7DA0(a5),d0
	and.b	#3,d0
	cmp.b	#3,d0
	bne.s	lFC03D8
	moveq	#2,d0
lFC03D8	move.b	d0,$44C(a5)
	move.b	-$5FF(a5),d0
	bmi.s	lFC03F6
	lea	lFC03EA(pc),a6
	bra	lFC0DC2

lFC03EA	move.b	#2,-$7DA0(a5)
	move.b	#2,$44C(a5)
lFC03F6	bsr	lFC0EC6
	jsr	lFCB552
	jsr	lFCB4C8
	cmpi.b	#1,$44C(a5)
	bne.s	lFC0414
	move.w	-$7DA2(a5),-$7DBA(a5)
lFC0414	move.l	#lFC0030,$46E(a5)
	move.w	#1,$452(a5)
	clr.w	d0
	bsr	lFC0628
	move.w	#$2300,sr   * Enable interrups 


	moveq	#1,d0
	bsr	lFC0628
	jsr	lFC956E
	move.w	lFC001E,$60BE   * Initial date from TOS header
	jsr	lFC1F4C   * Read Ricoh RTC
	bcc.s	lFC0462
	bsr	lFC31A8   * Read IKBD chip date time, but it is skipped
	swap	d0
	tst.b	d0
	beq.s	lFC0462
	move.w	d0,$60BE
	swap	d0
	move.w	d0,$378A
lFC0462	bsr	lFC0530    * Floppy boot
	bsr	lFC054C    * Hard disk boot
	bsr	lFC0DFE
	tst.w	$482
	beq.s	lFC0494
	bsr	lFC0BF4    * AUTO exec  call
	move.l	#lFC0000,$4F2
	pea	lFC052B(pc)
	pea	lFC052B(pc)
	pea	lFC0518(pc)
	clr.w	-(a7)
	bra.s	lFC04FC

lFC0494	bsr	lFC0BF4
	move.l	#lFC0000,$4F2
	lea	lFC050C(pc),a0
	movea.l	#$840,a1
lFC04AC	cmpi.b	#$23,(a0)
	bne.s	lFC04B4
	movea.l	a1,a2
lFC04B4	move.b	(a0)+,(a1)+
	bpl.s	lFC04AC
	move.b	$446,d0
	add.b	#$41,d0
	move.b	d0,(a2)
	pea	$840
	pea	lFC052B
	pea	lFC052B(pc)
	move.w	#5,-(a7)
	move.w	#$4B,-(a7)
	trap	#1
	adda.w	#$E,a7
	movea.l	d0,a0
	move.l	$4FE,8(a0)
	pea	$840
	move.l	a0,-(a7)
	pea	lFC052B(pc)
	move.w	#4,-(a7)
lFC04FC	move.w	#$4B,-(a7)   * Starting AES/GEM
	trap	#1
	adda.w	#$E,a7
	jmp	lFC0030

lFC050C	DC.B	"PATH=",0
	DC.B	"#:\",0,0,255
lFC0518	DC.B	"COMMAND.PRG",0
	DC.B	"GEM.PRG"
lFC052B	DC.B	0,0,0
lFC052E	DC.B	$80,1

lFC0530	moveq	#3,d0
	bsr	lFC0628
	movea.l	$47A,a0
	jsr	(a0)
	tst.w	d0
	bne.s	lFC054A
	lea	$181C,a0
	jsr	(a0)
lFC054A	rts

lFC054C	moveq	#0,d7
lFC054E	suba.l	a4,a4
	bsr.s	lFC0578
	bne.s	lFC0570
	movea.l	$4C6(a4),a0
	move.w	#$FF,d1
	moveq	#0,d0
lFC055E	add.w	(a0)+,d0
	dbf	d1,lFC055E
	cmp.w	#$1234,d0
	bne.s	lFC0570
	movea.l	$4C6(a4),a0
	jsr	(a0)
lFC0570	add.b	#$20,d7
	bne.s	lFC054E
	rts

lFC0578	moveq	#1,d5
lFC057A	lea	-$79FA(a4),a6
	lea	-$79FC(a4),a5
	st	$43E(a4)
	move.l	$4C6(a4),-(a7)
	move.b	3(a7),-$79F3(a4)
	move.b	2(a7),-$79F5(a4)
	move.b	1(a7),-$79F7(a4)
	addq.w	#4,a7
	move.w	#$98,(a6)
	move.w	#$198,(a6)
	move.w	#$98,(a6)
	move.w	#1,(a5)
	move.w	#$88,(a6)
	move.b	d7,d0
	or.b	#8,d0
	swap	d0
	move.w	#$8A,d0
	bsr.s	lFC060E
	bne.s	lFC05F0
	moveq	#3,d6
	lea	lFC05FE(pc),a0
lFC05C8	move.l	(a0)+,d0
	bsr.s	lFC060E
	bne.s	lFC05F0
	dbf	d6,lFC05C8
	move.l	#$A,(a5)
	move.w	#$190,d1
	bsr.s	lFC0612
	bne.s	lFC05F0
	move.w	#$8A,(a6)
	move.w	(a5),d0
	and.w	#$FF,d0
	beq.s	lFC05F2
	dbf	d5,lFC057A
lFC05F0	moveq	#-1,d0
lFC05F2	move.w	#$80,(a6)
	tst.b	d0
	sf	$43E(a4)
	rts

lFC05FE	dc.l	$8A,$8A,$8A,$1008A

lFC060E	move.l	d0,(a5)
	moveq	#$A,d1
lFC0612	add.l	$4BA(a4),d1
lFC0616	btst	#5,-$5FF(a4)
	beq.s	lFC0626
	cmp.l	$4BA(a4),d1
	bne.s	lFC0616
	moveq	#-1,d1
lFC0626	rts

lFC0628	lea	$FA0000,a0
	cmpi.l	#$ABCDEF42,(a0)+
	bne.s	lFC0650
lFC0636	btst	d0,4(a0)
	beq.s	lFC064A
	movem.l	d0-d7/a0-a6,-(a7)
	movea.l	4(a0),a0
	jsr	(a0)
	movem.l	(a7)+,d0-d7/a0-a6
lFC064A	tst.l	(a0)
	movea.l	(a0),a0
	bne.s	lFC0636
lFC0650	rts

lFC0652	rts
lFC0654	adda.l	d1,a0
	clr.w	d0
	lea	$1F8(a0),a1
lFC065C	cmp.w	(a0)+,d0
	bne.s	lFC0668
	add.w	#$FA54,d0
	cmpa.l	a0,a1
	bne.s	lFC065C
lFC0668	jmp	(a5)

lFC066A	suba.l	a5,a5
	cmpi.l	#$752019F3,$420(a5)
	bne.s	lFC0688
	cmpi.l	#$237698AA,$43A(a5)
	bne.s	lFC0688
	cmpi.l	#$5555AAAA,$51A(a5)
lFC0688	jmp	(a6)

* Palette :
lFC068A	DC.B	7,$77,7,0,0,$70,7,$70
	DC.B	0,7,7,7,0,$77,5,$55
	DC.B	3,$33,7,$33,3,$73,7,$73
	DC.B	3,$37,7,$37,3,$77,0,0

lFC06AA	move.w	d0,-(a7)

	move.w	2(a7),d0
	and.w	#$700,d0
	bne.s	lFC06BC
	ori.w	#$300,2(a7)
lFC06BC	move.w	(a7)+,d0
	rte

lFC06C0	addq.l	#1,$466
	subq.w	#1,$452
	bmi	lFC079A
	movem.l	d0-d7/a0-a6,-(a7)
	addq.l	#1,$462
	suba.l	a5,a5
	move.b	-$5FF(a5),d1
	move.b	-$7DA0(a5),d0
	and.b	#3,d0
	cmp.b	#2,d0
	bge.s	lFC0702
	btst	#7,d1
	bne.s	lFC0722
	move.w	#$7D0,d0
lFC06F8	dbf	d0,lFC06F8
	move.b	#2,d0
	bra.s	lFC0714
lFC0702	btst	#7,d1
	beq.s	lFC0722
	move.b	$44A(a5),d0
	cmp.b	#2,d0
	blt.s	lFC0714
	clr.b	d0
lFC0714	move.b	d0,$44C(a5)
	move.b	d0,-$7DA0(a5)
	movea.l	$46E(a5),a0
	jsr	(a0)
lFC0722	jsr	lFCA694
	suba.l	a5,a5
	tst.l	$45A(a5)
	beq.s	lFC0746
	movea.l	$45A(a5),a0
	lea	-$7DC0(a5),a1
	move.w	#$F,d0
lFC073C	move.w	(a0)+,(a1)+
	dbf	d0,lFC073C
	clr.l	$45A(a5)
lFC0746	tst.l	$45E(a5)
	beq.s	lFC075E
	move.l	$45E(a5),$44E(a5)
	move.b	$450(a5),-$7DFD(a5)
	move.b	$44F(a5),-$7DFF(a5)
lFC075E	bsr	lFC1360
	move.w	$454,d7
	beq.s	lFC078A
	subq.l	#1,d7
	movea.l	$456,a0
lFC0772	movea.l	(a0)+,a1
	cmpa.l	#0,a1
	beq.s	lFC0786
	movem.l	d7/a0,-(a7)
	jsr	(a1)
	movem.l	(a7)+,d7/a0
lFC0786	dbf	d7,lFC0772
lFC078A	suba.l	a5,a5
	tst.w	$4EE(a5)
	bne.s	lFC0796
	bsr	lFC0CFA
lFC0796	movem.l	(a7)+,d0-d7/a0-a6
lFC079A	addq.w	#1,$452
lFC07A0	rte

lFC07A2	move	sr,-(a7)
	andi.w	#$F8FF,sr
	move.l	$466,d0
lFC07AE	cmp.l	$466,d0
	beq.s	lFC07AE
	move.w	(a7)+,sr
	rts

lFC07BA	move.l	$404,-(a7)
lFC07C0	moveq	#-1,d0
	rts

lFC07C4	lea	lFC084A(pc),a0
	bra.s	lFC07CE
lFC07CA	lea	lFC0818(pc),a0
lFC07CE	movea.l	$4A2,a1
	move.w	(a7)+,d0
	move.w	d0,-(a1)
	move.l	(a7)+,-(a1)
	movem.l	d3-d7/a3-a7,-(a1)
	move.l	a1,$4A2
	btst	#$D,d0
	bne.s	lFC07EC
	move.l	usp,a7
lFC07EC	move.w	(a7)+,d0
	cmp.w	(a0)+,d0
	bge.s	lFC0802
	lsl.w	#2,d0
	move.l	0(a0,d0.w),d0
	movea.l	d0,a0
	bpl.s	lFC07FE
	movea.l	(a0),a0
lFC07FE	suba.l	a5,a5
	jsr	(a0)
lFC0802	movea.l	$4A2,a1
	movem.l	(a1)+,d3-d7/a3-a7
	move.l	(a1)+,-(a7)
	move.w	(a1)+,-(a7)
	move.l	a1,$4A2
	rte

lFC0818	DC.B	0,$C
	DC.L	lFC0A10,lFC0956
	DC.L	lFC095C,lFC0968
	DC.L	$80000476,lFC0A3C
	DC.L	lFC0A54,$80000472
	DC.L	lFC0962,$8000047E
	DC.L	lFC09F8,lFC09FE
lFC084A	DC.B	0,$41

	DC.L	lFC40A8,lFC0652
	DC.L	lFC0A5C,lFC0A6C
	DC.L	lFC0A72,lFC0A7E
	DC.L	lFC0AC4,lFC0ACC
	DC.L	lFC0F38,lFC100A
	DC.L	lFC10C6,lFC1732
	DC.L	lFC3292,lFC3754
	DC.L	lFC39FE,lFC3A16
	DC.L	lFC41AC,lFC1C76
	DC.L	lFC1D42,lFC1286
	DC.L	lFC0CFA,lFCA6C6
	DC.L	lFC0E4C,lFC0E3E
	DC.L	lFC41D8,lFC3480
	DC.L	lFC377E,lFC37B8
	DC.L	lFC4024,lFC4082
	DC.L	lFC405C,lFC4172
	DC.L	lFC41F2,lFC4206
	DC.L	lFC423A,lFC4218
	DC.L	lFC215C,lFC07A2
	DC.L	lFC0950,lFC0AF0
	DC.L	lFC0652,lFC1692
	DC.L	lFC0652,lFC0652
	DC.L	lFC0652,lFC0652
	DC.L	lFC0652,lFC0652
	DC.L	lFC0652,lFC0652
	DC.L	lFC0652,lFC0652
	DC.L	lFC0652,lFC0652
	DC.L	lFC0652,lFC0652
	DC.L	lFC0652,lFC0652
	DC.L	lFC0652,lFC0652
	DC.L	lFC0652,lFC0652
	DC.L	lFC0652,lFC0652
	DC.L	lFC0E9E

LFC0950		movea.l	4(a7),a0
	jmp	(a0)

lFC0956	lea	$51E(a5),a0
	bra.s	lFC096C
lFC095C	lea	$53E(a5),a0
	bra.s	lFC096C
lFC0962	lea	$55E(a5),a0
	bra.s	lFC096C
lFC0968	lea	$57E(a5),a0
lFC096C	move.w	4(a7),d0
	lsl.w	#2,d0
	movea.l	0(a0,d0.w),a0
	jmp	(a0)

lFC0978	DC.L	lFC0652,lFC33A6
	DC.L	lFC3494,lFC32A6
	DC.L	lFC0652,lFC0652
	DC.L	lFC0652,lFC0652
	DC.L	lFC3372,lFC33BE
	DC.L	lFC34AA,lFC32C0
	DC.L	lFC0652,lFC0652
	DC.L	lFC0652,lFC0652
	DC.L	lFC3392,lFC3408
	DC.L	lFC34E0,lFC344A
	DC.L	lFC326A,lFC0652
	DC.L	lFC0652,lFC0652
	DC.L	lFC32F6,lFC3422
	DC.L	lFCA30A,lFC327A
	DC.L	lFC345C,lFCA2FE
	DC.L	lFC0652,lFC0652
lFC09F8	move.l	$4C2(a5),d0
	rts

lFC09FE	moveq	#0,d0
	move.b	$E7D(a5),d0
	move.w	4(a7),d1
	bmi.s	lFC0A0E
	move.b	d1,$E7D(a5)
lFC0A0E	rts

lFC0A10	movea.l	4(a7),a0
	lea	$48E(a5),a1
	move.l	a1,(a0)
	clr.l	4(a0)
	move.l	a1,8(a0)
	clr.l	(a1)
	move.l	$432(a5),4(a1)
	move.l	$436(a5),d0
	sub.l	$432(a5),d0
	move.l	d0,8(a1)
	clr.l	$C(a1)
	rts

lFC0A3C	move.w	4(a7),d0
	lsl.w	#2,d0
	suba.l	a0,a0
	lea	0(a0,d0.w),a0
	move.l	(a0),d0
	move.l	6(a7),d1
	bmi.s	lFC0A52
	move.l	d1,(a0)
lFC0A52	rts

lFC0A54	moveq	#0,d0
	move.w	$442(a5),d0
	rts

lFC0A5C	moveq	#0,d0
	move.b	-$7DFF(a5),d0
	lsl.w	#8,d0
	move.b	-$7DFD(a5),d0
	lsl.l	#8,d0
	rts

LFC0A6C 	move.l	$44E(a5),d0
	rts

LFC0A72	moveq	#0,d0
	move.b	-$7DA0(a5),d0
	and.b	#3,d0
	rts

LFC0A7E 	tst.l	4(a7)
	bmi.s	lFC0A8A
	move.l	4(a7),$44E(a5)
lFC0A8A	tst.l	8(a7)
	bmi.s	lFC0A9C
	move.b	9(a7),-$7DFF(a5)
	move.b	$A(a7),-$7DFD(a5)
lFC0A9C	tst.w	$C(a7)
	bmi.s	lFC0AC2
	move.b	$D(a7),$44C(a5)
	bsr	lFC07A2
	move.b	$44C(a5),-$7DA0(a5)
	clr.w	$452(a5)
	jsr	lFCB4C8
	move.w	#1,$452(a5)
lFC0AC2	rts

LFC0AC4	 	move.l	4(a7),$45A(a5)
	rts

LFC0ACC 	move.w	4(a7),d1
	add.w	d1,d1
	and.w	#$1F,d1
	lea	-$7DC0(a5),a0
	move.w	0(a0,d1.w),d0
	and.w	#$777,d0
	tst.w	6(a7)
	bmi.s	lFC0AEE
	move.w	6(a7),0(a0,d1.w)
lFC0AEE	rts

LFC0AF0		movea.l	lFC0014(pc),a0
	cmpi.l	#$87654321,(a0)
	bne.s	lFC0B08
	cmpa.l	$42E(a5),a0
	bge.s	lFC0B08
	clr.l	(a0)
	bra	lFC0030
lFC0B08	rts

lFC0B0A	bsr.s	lFC0B0E
	nop
lFC0B0E	suba.l	a5,a5
	move.l	(a7)+,$3C4(a5)
	movem.l	d0-d7/a0-a7,$384(a5)
	move.l	usp,a0
	move.l	a0,$3C8(a5)
	moveq	#$F,d0
	lea	$3CC(a5),a0
	movea.l	a7,a1
lFC0B28	move.w	(a1)+,(a0)+
	dbf	d0,lFC0B28
	move.l	#$12345678,$380(a5)
	moveq	#0,d1
	move.b	$3C4(a5),d1
	subq.w	#1,d1
	bsr.s	lFC0B56
	move.l	#$93A,$4A2(a5)
	move.w	#$FFFF,-(a7)
	move.w	#$4C,-(a7)
	trap	#1
	bra	lFC0030

lFC0B56	move.b	-$7DA0(a5),d7
	and.w	#3,d7
	add.w	d7,d7
	moveq	#0,d0
	move.b	-$7DFF(a5),d0
	lsl.w	#8,d0
	move.b	-$7DFD(a5),d0
	lsl.l	#8,d0
	movea.l	d0,a0
	adda.w	lFC0B9E(pc,d7.w),a0
	lea	lFC0DA2,a1
	move.w	#$F,d6
lFC0B7E	move.w	d1,d2
	movea.l	a0,a2
lFC0B82	move.w	lFC0BA6(pc,d7.w),d5
lFC0B86	move.w	(a1),(a0)+
	dbf	d5,lFC0B86
	dbf	d2,lFC0B82
	addq.w	#2,a1
	adda.w	lFC0BAE(pc,d7.w),a2
	movea.l	a2,a0
	dbf	d6,lFC0B7E
	rts

lFC0B9E	DC.B	$3E,$80,$3E,$80,$3E,$80,$3E,$80
lFC0BA6	DC.B	0,3,0,1,0,0,0,0
lFC0BAE	DC.B	0,$A0,0,$A0,0,$50,0,$50
lFC0BB6	movea.l	4(a7),a0
	movea.l	8(a7),a1
	move.w	#$3F,d0
lFC0BC2	move.b	(a0)+,(a1)+
	move.b	(a0)+,(a1)+
	move.b	(a0)+,(a1)+
	move.b	(a0)+,(a1)+
	move.b	(a0)+,(a1)+
	move.b	(a0)+,(a1)+
	move.b	(a0)+,(a1)+
	move.b	(a0)+,(a1)+
	dbf	d0,lFC0BC2
	rts

lFC0BD8	move.l	$46A,-(a7)
	rts

lFC0BE0	DC.B	"\AUTO\"
lFC0BE6	DC.B	"*.PRG",0,$12,$34
	DC.B	$56,$78,$9A,$BC,$DE,$F0



* Executing  \AUTO\*.PRG :

lFC0BF4	lea	lFC0BE0(pc),a0
	lea	lFC0BE6(pc),a1
	move.l	(a7)+,$980
	suba.l	a5,a5
	move.l	a0,$984(a5)
	move.l	a1,$988(a5)
	move.l	$4C2(a5),d0
	move.w	$446,d1
	btst	d1,d0
	beq.s	lFC0C50
	lea	lFC052B(pc),a0
	move.l	a0,-(a7)
	move.l	a0,-(a7)
	move.l	a0,-(a7)
	move.w	#5,-(a7)
	move.w	#$4B,-(a7)
	trap	#1
	adda.w	#$10,a7
	movea.l	d0,a0
	move.l	#lFC0C58,8(a0)
	move.l	a3,-(a7)
	move.l	d0,-(a7)
	move.l	a3,-(a7)
	move.w	#4,-(a7)
	move.w	#$4B,-(a7)
	trap	#1
	adda.w	#$10,a7
lFC0C50	move.l	$980,-(a7)
	rts

































