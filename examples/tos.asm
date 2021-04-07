00fc0030  46fc 2700                 move #$2700,sr                  
00fc0034  4e70                      reset                           
00fc0036  9bcd                      suba.l a5,a5                    
00fc0038  0cb9 fa52 235f 00fa 0000  cmpi.l #$fa52235f,(00fa0000).w  
00fc0042  660a                      bne $00fc004e                   
00fc0044  4dfa 0008                 lea (0008,pc)[00fc004e],a6      
00fc0048  4ef9 00fa 0004            jmp (00fa0004).w                
00fc004e  4dfa 0006                 lea (0006,pc)[00fc0056],a6      
00fc0052  6000 0616                 bra $00fc066a                   
00fc0056  6606                      bne $00fc005e                   
00fc0058  1b6d 0424 8001            move.b 424(a5),-7fff(a5)        
00fc005e  0cad 3141 5926 0426       cmpi.l #$31415926,426(a5)       
00fc0066  6618                      bne $00fc0080                   
00fc0068  202d 042a                 move.l 42a(a5),d0               
00fc006c  4a2d 042a                 tst.b 42a(a5)                   
00fc0070  660e                      bne $00fc0080                   
00fc0072  0800 0000                 btst #0,d0                      
00fc0076  6608                      bne $00fc0080                   
00fc0078  2040                      movea.l d0,a0                   
00fc007a  4dfa ffe2                 lea (-01e,pc)[00fc005e],a6      
00fc007e  4ed0                      jmp (a0)                        
00fc0080  9bcd                      suba.l a5,a5                    
00fc0082  41ed 8800                 lea -7800(a5),a0                
00fc0086  10bc 0007                 move.b #$07,(a0)                
00fc008a  117c 00c0 0002            move.b #$c0,2(a0)               
00fc0090  10bc 000e                 move.b #$0e,(a0)                
00fc0094  117c 0007 0002            move.b #$07,2(a0)               
00fc009a  083a 0000 ff7f            btst #0,(-081,pc)[00fc001d]     
00fc00a0  670e                      beq $00fc00b0                   
00fc00a2  4dfa 0006                 lea (0006,pc)[00fc00aa],a6      
00fc00a6  6000 0d1a                 bra $00fc0dc2                   
00fc00aa  1b7c 0002 820a            move.b #$02,-7df6(a5)           
00fc00b0  43ed 8240                 lea -7dc0(a5),a1                
00fc00b4  303c 000f                 move.w #$000f,d0                
00fc00b8  41fa 05d0                 lea (05d0,pc)[00fc068a],a0      
00fc00bc  32d8                      move.w (a0)+,(a1)+              
00fc00be  51c8 fffc                 dbf d0,$00fc00bc                
00fc00c2  1b7c 0001 8201            move.b #$01,-7dff(a5)           
00fc00c8  422d 8203                 clr.b -7dfd(a5)                 
00fc00cc  1c2d 0424                 move.b 424(a5),d6               
00fc00d0  2a2d 042e                 move.l 42e(a5),d5               
00fc00d4  4dfa 0006                 lea (0006,pc)[00fc00dc],a6      
00fc00d8  6000 0590                 bra $00fc066a                   
00fc00dc  6700 0114                 beq $00fc01f2                   
00fc00e0  4246                      clr.w d6                        
00fc00e2  1b7c 000a 8001            move.b #$0a,-7fff(a5)           
00fc00e8  307c 0008                 movea.w #$0008,a0               
00fc00ec  43f9 0020 0008            lea (00200008).w,a1             
00fc00f2  4240                      clr.w d0                        
00fc00f4  30c0                      move.w d0,(a0)+                 
00fc00f6  32c0                      move.w d0,(a1)+                 
00fc00f8  d07c fa54                 add.w #$fa54,d0                 
00fc00fc  b1fc 0000 0200            cmpa.l #$00000200,a0            
00fc0102  66f0                      bne $00fc00f4                   
00fc0104  223c 0020 0000            move.l #$00200000,d1            
00fc010a  e44e                      lsr.w 2,d6                      
00fc010c  307c 0208                 movea.w #$0208,a0               
00fc0110  4bfa 0006                 lea (0006,pc)[00fc0118],a5      
00fc0114  6000 053e                 bra $00fc0654                   
00fc0118  6720                      beq $00fc013a                   
00fc011a  307c 0408                 movea.w #$0408,a0               
00fc011e  4bfa 0006                 lea (0006,pc)[00fc0126],a5      
00fc0122  6000 0530                 bra $00fc0654                   
00fc0126  6710                      beq $00fc0138                   
00fc0128  307c 0008                 movea.w #$0008,a0               
00fc012c  4bfa 0006                 lea (0006,pc)[00fc0134],a5      
00fc0130  6000 0522                 bra $00fc0654                   
00fc0134  6604                      bne $00fc013a                   
00fc0136  5846                      addq.w #$4,d6                   
00fc0138  5846                      addq.w #$4,d6                   
00fc013a  92bc 0020 0000            sub.l #$00200000,d1             
00fc0140  67c8                      beq $00fc010a                   
00fc0142  13c6 ffff 8001            move.b d6,(ffff8001).w          
00fc0148  4ff9 0000 8000            lea (00008000).w,a7             
00fc014e  2879 0000 0008            movea.l (00000008).w,a4         
00fc0154  41fa 0036                 lea (0036,pc)[00fc018c],a0      
00fc0158  23c8 0000 0008            move.l a0,(00000008).w          
00fc015e  363c fb55                 move.w #$fb55,d3                
00fc0162  2e3c 0002 0000            move.l #$00020000,d7            
00fc0168  2047                      movea.l d7,a0                   
00fc016a  2248                      movea.l a0,a1                   
00fc016c  3400                      move.w d0,d2                    
00fc016e  722a                      moveq #$2a,d1                   
00fc0170  3302                      move.w d2,-(a1)                 
00fc0172  d443                      add.w d3,d2                     
00fc0174  51c9 fffa                 dbf d1,$00fc0170                
00fc0178  2248                      movea.l a0,a1                   
00fc017a  722a                      moveq #$2a,d1                   
00fc017c  b061                      cmp.w -(a1),d0                  
00fc017e  660c                      bne $00fc018c                   
00fc0180  4251                      clr.w (a1)                      
00fc0182  d043                      add.w d3,d0                     
00fc0184  51c9 fff6                 dbf d1,$00fc017c                
00fc0188  d1c7                      dc                              
00fc018a  60de                      bra $00fc016a                   
00fc018c  91c7                      suba.l d7,a0                    
00fc018e  2a08                      move.l a0,d5                    
00fc0190  23cc 0000 0008            move.l a4,(00000008).w          
00fc0196  9bcd                      suba.l a5,a5                    
00fc0198  2005                      move.l d5,d0                    
00fc019a  90bc 0000 8000            sub.l #$00008000,d0             
00fc01a0  e048                      lsr.w 0,d0                      
00fc01a2  1b40 8203                 move.b d0,-7dfd(a5)             
00fc01a6  4840                      swap d0                         
00fc01a8  1b40 8201                 move.b d0,-7dff(a5)             
00fc01ac  2045                      movea.l d5,a0                   
00fc01ae  283c 0000 0400            move.l #$00000400,d4            
00fc01b4  7000                      moveq #$00,d0                   
00fc01b6  7200                      moveq #$00,d1                   
00fc01b8  7400                      moveq #$00,d2                   
00fc01ba  7600                      moveq #$00,d3                   
00fc01bc  48e0 f000                 movem.l d3/d2/d1/d0,-(a0)       
00fc01c0  48e0 f000                 movem.l d3/d2/d1/d0,-(a0)       
00fc01c4  48e0 f000                 movem.l d3/d2/d1/d0,-(a0)       
00fc01c8  48e0 f000                 movem.l d3/d2/d1/d0,-(a0)       
00fc01cc  b1c4                      cmpa.l d4,a0                    
00fc01ce  66ec                      bne $00fc01bc                   
00fc01d0  9bcd                      suba.l a5,a5                    
00fc01d2  1b46 0424                 move.b d6,424(a5)               
00fc01d6  2b45 042e                 move.l d5,42e(a5)               
00fc01da  2b7c 7520 19f3 0420       move.l #$752019f3,420(a5)       
00fc01e2  2b7c 2376 98aa 043a       move.l #$237698aa,43a(a5)       
00fc01ea  2b7c 5555 aaaa 051a       move.l #$5555aaaa,51a(a5)       
00fc01f2  9bcd                      suba.l a5,a5                    
00fc01f4  207c 0000 0980            movea.l #$00000980,a0           
00fc01fa  227c 0001 0000            movea.l #$00010000,a1           
00fc0200  7000                      moveq #$00,d0                   
00fc0202  30c0                      move.w d0,(a0)+                 
00fc0204  b3c8                      cmpa.l a0,a1                    
00fc0206  66fa                      bne $00fc0202                   
00fc0208  206d 042e                 movea.l 42e(a5),a0              
00fc020c  91fc 0000 8000            suba.l #$00008000,a0            
00fc0212  323c 07ff                 move.w #$07ff,d1                
00fc0216  2b48 044e                 move.l a0,44e(a5)               
00fc021a  1b6d 044f 8201            move.b 44f(a5),-7dff(a5)        
00fc0220  1b6d 0450 8203            move.b 450(a5),-7dfd(a5)        
00fc0226  20c0                      move.l d0,(a0)+                 
00fc0228  20c0                      move.l d0,(a0)+                 
00fc022a  20c0                      move.l d0,(a0)+                 
00fc022c  20c0                      move.l d0,(a0)+                 
00fc022e  51c9 fff6                 dbf d1,$00fc0226                
00fc0232  207a fde0                 movea.l (-220,pc)[00fc0014],a0  
00fc0236  0c90 8765 4321            cmpi.l #$87654321,(a0)          
00fc023c  6704                      beq $00fc0242                   
00fc023e  41fa fdc8                 lea (-238,pc)[00fc0008],a0      
00fc0242  2b68 0004 04fa            move.l 4(a0),4fa(a5)            
00fc0248  2b68 0008 04fe            move.l 8(a0),4fe(a5)            
00fc024e  2b7c 00fc 16ba 046a       move.l #$00fc16ba,46a(a5)       
00fc0256  2b7c 00fc 1a24 0476       move.l #$00fc1a24,476(a5)       
00fc025e  2b7c 00fc 173c 0472       move.l #$00fc173c,472(a5)       
00fc0266  2b7c 00fc 18ec 047e       move.l #$00fc18ec,47e(a5)       
00fc026e  2b7c 00fc 1cc6 047a       move.l #$00fc1cc6,47a(a5)       
00fc0276  2b7c 00fc 3392 0506       move.l #$00fc3392,506(a5)       
00fc027e  2b7c 00fc 32f6 050a       move.l #$00fc32f6,50a(a5)       
00fc0286  2b7c 00fc 3408 050e       move.l #$00fc3408,50e(a5)       
00fc028e  2b7c 00fc 3422 0512       move.l #$00fc3422,512(a5)       
00fc0296  2b7c 00fc 0d0c 0502       move.l #$00fc0d0c,502(a5)       
00fc029e  2b6d 044e 0436            move.l 44e(a5),436(a5)          
00fc02a4  2b6d 04fa 0432            move.l 4fa(a5),432(a5)          
00fc02aa  4ff9 0000 378a            lea (0000378a).w,a7             
00fc02b0  3b7c 0008 0454            move.w #$0008,454(a5)           
00fc02b6  50ed 0444                 st 444(a5)                      
00fc02ba  3b7c 0003 0440            move.w #$0003,440(a5)           
00fc02c0  2b7c 0000 181c 04c6       move.l #$0000181c,4c6(a5)       
00fc02c8  3b7c ffff 04ee            move.w #$ffff,4ee(a5)           
00fc02ce  2b7c 00fc 0000 04f2       move.l #$00fc0000,4f2(a5)       
00fc02d6  2b7c 0000 093a 04a2       move.l #$0000093a,4a2(a5)       
00fc02de  2b7c 00fc 0652 046e       move.l #$00fc0652,46e(a5)       
00fc02e6  42ad 04c2                 clr.l 4c2(a5)                   
00fc02ea  426d 059e                 clr.w 59e(a5)                   
00fc02ee  6100 0b72                 bsr $00fc0e62                   
00fc02f2  47fa 04ac                 lea (04ac,pc)[00fc07a0],a3      
00fc02f6  49fa 035a                 lea (035a,pc)[00fc0652],a4      
00fc02fa  0cb9 fa52 235f 00fa 0000  cmpi.l #$fa52235f,(00fa0000).w  
00fc0304  6726                      beq $00fc032c                   
00fc0306  43fa 0802                 lea (0802,pc)[00fc0b0a],a1      
00fc030a  d3fc                      dc                              
00fc030c  0200 0000                 andi.b #$00,d0                  
00fc0310  41f9 0000 0008            lea (00000008).w,a0             
00fc0316  303c 003d                 move.w #$003d,d0                
00fc031a  20c9                      move.l a1,(a0)+                 
00fc031c  d3fc                      dc                              
00fc031e  0100                      btst d0,d0                      
00fc0320  0000 51c8                 ori.b #$c8,d0                   
00fc0324  fff6                      dc                              
00fc0326  23cb 0000 0014            move.l a3,(00000014).w          
00fc032c  7006                      moveq #$06,d0                   
00fc032e  43ed 0064                 lea 64(a5),a1                   
00fc0332  22fc 00fc 07a0            move.l #$00fc07a0,(a1)+         
00fc0338  51c8 fff8                 dbf d0,$00fc0332                
00fc033c  2b7c 00fc 06c0 0070       move.l #$00fc06c0,70(a5)        
00fc0344  2b7c 00fc 06aa 0068       move.l #$00fc06aa,68(a5)        
00fc034c  2b4b 0088                 move.l a3,88(a5)                
00fc0350  2b7c 00fc 07ca 00b4       move.l #$00fc07ca,b4(a5)        
00fc0358  2b7c 00fc 07c4 00b8       move.l #$00fc07c4,b8(a5)        
00fc0360  2b7c 00fc ab96 0028       move.l #$00fcab96,28(a5)        
00fc0368  2b4c 0400                 move.l a4,400(a5)               
00fc036c  2b7c 00fc 07c0 0404       move.l #$00fc07c0,404(a5)       
00fc0374  2b4c 0408                 move.l a4,408(a5)               
00fc0378  41ed 04ce                 lea 4ce(a5),a0                  
00fc037c  2b48 0456                 move.l a0,456(a5)               
00fc0380  303c 0007                 move.w #$0007,d0                
00fc0384  4298                      clr.l (a0)+                     
00fc0386  51c8 fffc                 dbf d0,$00fc0384                
00fc038a  41f9 00fc 0978            lea (00fc0978).w,a0             
00fc0390  327c 051e                 movea.w #$051e,a1               
00fc0394  701f                      moveq #$1f,d0                   
00fc0396  22d8                      move.l (a0)+,(a1)+              
00fc0398  51c8 fffc                 dbf d0,$00fc0396                
00fc039c  6100 315e                 bsr $00fc34fc                   
00fc03a0  2f3c 00fc 052e            move.l #$00fc052e,-(a7)         
00fc03a6  3f3c 0001                 move.w #$0001,-(a7)             
00fc03aa  4eb9 00fc 3480            jsr (00fc3480).w                
00fc03b0  5c8f                      addq.l #$6,a7                   
00fc03b2  203c 0000 7fff            move.l #$00007fff,d0            
00fc03b8  6100 0190                 bsr $00fc054a                   
00fc03bc  51c8 fffa                 dbf d0,$00fc03b8                
00fc03c0  7002                      moveq #$02,d0                   
00fc03c2  6100 0264                 bsr $00fc0628                   
00fc03c6  9bcd                      suba.l a5,a5                    
00fc03c8  102d 8260                 move.b -7da0(a5),d0             
00fc03cc  c03c 0003                 and.b #$03,d0                   
00fc03d0  b03c 0003                 cmp.b #$03,d0                   
00fc03d4  6602                      bne $00fc03d8                   
00fc03d6  7002                      moveq #$02,d0                   
00fc03d8  1b40 044c                 move.b d0,44c(a5)               
00fc03dc  102d fa01                 move.b -5ff(a5),d0              
00fc03e0  6b14                      bmi $00fc03f6                   
00fc03e2  4dfa 0006                 lea (0006,pc)[00fc03ea],a6      
00fc03e6  6000 09da                 bra $00fc0dc2                   
00fc03ea  1b7c 0002 8260            move.b #$02,-7da0(a5)           
00fc03f0  1b7c 0002 044c            move.b #$02,44c(a5)             
00fc03f6  6100 0ace                 bsr $00fc0ec6                   
00fc03fa  4eb9 00fc b5ac            jsr (00fcb5ac).w                
00fc0400  4eb9 00fc b522            jsr (00fcb522).w                
00fc0406  0c2d 0001 044c            cmpi.b #$01,44c(a5)             
00fc040c  6606                      bne $00fc0414                   
00fc040e  3b6d 825e 8246            move.w -7da2(a5),-7dba(a5)      
00fc0414  2b7c 00fc 0030 046e       move.l #$00fc0030,46e(a5)       
00fc041c  3b7c 0001 0452            move.w #$0001,452(a5)           
00fc0422  4240                      clr.w d0                        
00fc0424  6100 0202                 bsr $00fc0628                   
00fc0428  46fc 2300                 move #$2300,sr                  
00fc042c  7001                      moveq #$01,d0                   
00fc042e  6100 01f8                 bsr $00fc0628                   
00fc0432  4eb9 00fc 95c8            jsr (00fc95c8).w                
00fc0438  33f9 00fc 001e 0000 60be  move.w (00fc001e).w,(000060be).w
00fc0442  4eb9 00fc 1f4c            jsr (00fc1f4c).w                
00fc0448  6418                      bcc $00fc0462                   
00fc044a  6100 2d5c                 bsr $00fc31a8                   
00fc044e  4840                      swap d0                         
00fc0450  4a00                      tst.b d0                        
00fc0452  670e                      beq $00fc0462                   
00fc0454  33c0 0000 60be            move.w d0,(000060be).w          
00fc045a  4840                      swap d0                         
00fc045c  33c0 0000 378a            move.w d0,(0000378a).w          
00fc0462  6100 00cc                 bsr $00fc0530                   
00fc0466  6100 00e4                 bsr $00fc054c                   
00fc046a  6100 0992                 bsr $00fc0dfe                   
00fc046e  4a79 0000 0482            tst.w (00000482).w              
00fc0474  671e                      beq $00fc0494                   
00fc0476  6100 077c                 bsr $00fc0bf4                   
00fc047a  23fc 00fc 0000 0000 04f2  move.l #$00fc0000,(000004f2).w  
00fc0484  487a 00a5                 pea (00a5,pc)[00fc052b]         
00fc0488  487a 00a1                 pea (00a1,pc)[00fc052b]         
00fc048c  487a 008a                 pea (008a,pc)[00fc0518]         
00fc0490  4267                      clr.w -(a7)                     
00fc0492  6068                      bra $00fc04fc                   
00fc0494  6100 075e                 bsr $00fc0bf4                   
00fc0498  23fc 00fc 0000 0000 04f2  move.l #$00fc0000,(000004f2).w  
00fc04a2  41fa 0068                 lea (0068,pc)[00fc050c],a0      
00fc04a6  227c 0000 0840            movea.l #$00000840,a1           
00fc04ac  0c10 0023                 cmpi.b #$23,(a0)                
00fc04b0  6602                      bne $00fc04b4                   
00fc04b2  2449                      movea.l a1,a2                   
00fc04b4  12d8                      move.b (a0)+,(a1)+              
00fc04b6  6af4                      bpl $00fc04ac                   
00fc04b8  1039 0000 0446            move.b (00000446).w,d0          
00fc04be  d03c 0041                 add.b #$41,d0                   
00fc04c2  1480                      move.b d0,(a2)                  
00fc04c4  4879 0000 0840            pea (00000840).w                
00fc04ca  4879 00fc 052b            pea (00fc052b).w                
00fc04d0  487a 0059                 pea (0059,pc)[00fc052b]         
00fc04d4  3f3c 0005                 move.w #$0005,-(a7)             
00fc04d8  3f3c 004b                 move.w #$004b,-(a7)             
00fc04dc  4e41                      trap #33                        
00fc04de  defc                      dc                              
00fc04e0  000e 2040                 ori.b #$40,a6                   
00fc04e4  2179 0000 04fe 0008       move.l (000004fe).w,8(a0)       
00fc04ec  4879 0000 0840            pea (00000840).w                
00fc04f2  2f08                      move.l a0,-(a7)                 
00fc04f4  487a 0035                 pea (0035,pc)[00fc052b]         
00fc04f8  3f3c 0004                 move.w #$0004,-(a7)             
00fc04fc  3f3c 004b                 move.w #$004b,-(a7)             
00fc0500  4e41                      trap #33                        
00fc0502  defc                      dc                              
00fc0504  000e 4ef9                 ori.b #$f9,a6                   
00fc0508  00fc                      dc                              
00fc050a  0030 5041 5448            ori.b #$41,(48a0,d5.w*4)        
00fc0510  3d00                      move.w d0,-(a6)                 
00fc0512  233a 5c00                 move.l (5c00,pc)[00fc6114],-(a1)
00fc0516  00ff                      dc                              
00fc0518  434f                      dc                              
00fc051a  4d4d                      dc                              
00fc051c  414e                      dc                              
00fc051e  442e 5052                 neg.b 5052(a6)                  
00fc0522  4700                      chk.l d0,d3                     
00fc0524  4745                      dc                              
00fc0526  4d2e 5052                 chk.l 5052(a6),d6               
00fc052a  4700                      chk.l d0,d3                     
00fc052c  0000 8001                 ori.b #$01,d0                   
00fc0530  7003                      moveq #$03,d0                   
00fc0532  6100 00f4                 bsr $00fc0628                   
00fc0536  2079 0000 047a            movea.l (0000047a).w,a0         
00fc053c  4e90                      jsr (a0)                        
00fc053e  4a40                      tst.w d0                        
00fc0540  6608                      bne $00fc054a                   
00fc0542  41f9 0000 181c            lea (0000181c).w,a0             
00fc0548  4e90                      jsr (a0)                        
00fc054a  4e75                      rts                             
00fc054c  7e00                      moveq #$00,d7                   
00fc054e  99cc                      suba.l a4,a4                    
00fc0550  6126                      bsr $00fc0578                   
00fc0552  661c                      bne $00fc0570                   
00fc0554  206c 04c6                 movea.l 4c6(a4),a0              
00fc0558  323c 00ff                 move.w #$00ff,d1                
00fc055c  7000                      moveq #$00,d0                   
00fc055e  d058                      add.w (a0)+,d0                  
00fc0560  51c9 fffc                 dbf d1,$00fc055e                
00fc0564  b07c 1234                 cmp.w #$1234,d0                 
00fc0568  6606                      bne $00fc0570                   
00fc056a  206c 04c6                 movea.l 4c6(a4),a0              
00fc056e  4e90                      jsr (a0)                        
00fc0570  de3c 0020                 add.b #$20,d7                   
00fc0574  66d8                      bne $00fc054e                   
00fc0576  4e75                      rts                             
00fc0578  7a01                      moveq #$01,d5                   
00fc057a  4dec 8606                 lea -79fa(a4),a6                
00fc057e  4bec 8604                 lea -79fc(a4),a5                
00fc0582  50ec 043e                 st 43e(a4)                      
00fc0586  2f2c 04c6                 move.l 4c6(a4),-(a7)            
00fc058a  196f 0003 860d            move.b 3(a7),-79f3(a4)          
00fc0590  196f 0002 860b            move.b 2(a7),-79f5(a4)          
00fc0596  196f 0001 8609            move.b 1(a7),-79f7(a4)          
00fc059c  584f                      addq.w #$4,a7                   
00fc059e  3cbc 0098                 move.w #$0098,(a6)              
00fc05a2  3cbc 0198                 move.w #$0198,(a6)              
00fc05a6  3cbc 0098                 move.w #$0098,(a6)              
00fc05aa  3abc 0001                 move.w #$0001,(a5)              
00fc05ae  3cbc 0088                 move.w #$0088,(a6)              
00fc05b2  1007                      move.b d7,d0                    
00fc05b4  803c 0008                 or.b #$08,d0                    
00fc05b8  4840                      swap d0                         
00fc05ba  303c 008a                 move.w #$008a,d0                
00fc05be  614e                      bsr $00fc060e                   
00fc05c0  662e                      bne $00fc05f0                   
00fc05c2  7c03                      moveq #$03,d6                   
00fc05c4  41fa 0038                 lea (0038,pc)[00fc05fe],a0      
00fc05c8  2018                      move.l (a0)+,d0                 
00fc05ca  6142                      bsr $00fc060e                   
00fc05cc  6622                      bne $00fc05f0                   
00fc05ce  51ce fff8                 dbf d6,$00fc05c8                
00fc05d2  2abc 0000 000a            move.l #$0000000a,(a5)          
00fc05d8  323c 0190                 move.w #$0190,d1                
00fc05dc  6134                      bsr $00fc0612                   
00fc05de  6610                      bne $00fc05f0                   
00fc05e0  3cbc 008a                 move.w #$008a,(a6)              
00fc05e4  3015                      move.w (a5),d0                  
00fc05e6  c07c 00ff                 and.w #$00ff,d0                 
00fc05ea  6706                      beq $00fc05f2                   
00fc05ec  51cd ff8c                 dbf d5,$00fc057a                
00fc05f0  70ff                      moveq #$ff,d0                   
00fc05f2  3cbc 0080                 move.w #$0080,(a6)              
00fc05f6  4a00                      tst.b d0                        
00fc05f8  51ec 043e                 sf 43e(a4)                      
00fc05fc  4e75                      rts                             
00fc05fe  0000 008a                 ori.b #$8a,d0                   
00fc0602  0000 008a                 ori.b #$8a,d0                   
00fc0606  0000 008a                 ori.b #$8a,d0                   
00fc060a  0001 008a                 ori.b #$8a,d1                   
00fc060e  2a80                      move.l d0,(a5)                  
00fc0610  720a                      moveq #$0a,d1                   
00fc0612  d2ac 04ba                 add.l 4ba(a4),d1                
00fc0616  082c 0005 fa01            btst #5,-5ff(a4)                
00fc061c  6708                      beq $00fc0626                   
00fc061e  b2ac 04ba                 cmp.l 4ba(a4),d1                
00fc0622  66f2                      bne $00fc0616                   
00fc0624  72ff                      moveq #$ff,d1                   
00fc0626  4e75                      rts                             
00fc0628  41f9 00fa 0000            lea (00fa0000).w,a0             
00fc062e  0c98 abcd ef42            cmpi.l #$abcdef42,(a0)+         
00fc0634  661a                      bne $00fc0650                   
00fc0636  0128 0004                 btst d0,4(a0)                   
00fc063a  670e                      beq $00fc064a                   
00fc063c  48e7 fffe                 movem.l a6/a5/a4/a3/a2/a1/a0/d7/d6/d5/d4/d3/d2/d1/d0,-(a7)
00fc0640  2068 0004                 movea.l 4(a0),a0                
00fc0644  4e90                      jsr (a0)                        
00fc0646  4cdf 7fff                 movem.l (a7)+,d0/d1/d2/d3/d4/d5/d6/d7/a0/a1/a2/a3/a4/a5/a6
00fc064a  4a90                      tst.l (a0)                      
00fc064c  2050                      movea.l (a0),a0                 
00fc064e  66e6                      bne $00fc0636                   
00fc0650  4e75                      rts                             
00fc0652  4e75                      rts                             
00fc0654  d1c1                      dc                              
00fc0656  4240                      clr.w d0                        
00fc0658  43e8 01f8                 lea 1f8(a0),a1                  
00fc065c  b058                      cmp.w (a0)+,d0                  
00fc065e  6608                      bne $00fc0668                   
00fc0660  d07c fa54                 add.w #$fa54,d0                 
00fc0664  b3c8                      cmpa.l a0,a1                    
00fc0666  66f4                      bne $00fc065c                   
00fc0668  4ed5                      jmp (a5)                        
00fc066a  9bcd                      suba.l a5,a5                    
00fc066c  0cad 7520 19f3 0420       cmpi.l #$752019f3,420(a5)       
00fc0674  6612                      bne $00fc0688                   
00fc0676  0cad 2376 98aa 043a       cmpi.l #$237698aa,43a(a5)       
00fc067e  6608                      bne $00fc0688                   
00fc0680  0cad 5555 aaaa 051a       cmpi.l #$5555aaaa,51a(a5)       
00fc0688  4ed6                      jmp (a6)                        
00fc068a  0777 0700                 bchg d3,(0a7,d0.w*8)            
00fc068e  0070 0770 0007            ori.w #$0770,(7a0,d0.w*1)       
00fc0694  0707                      btst d3,d7                      
00fc0696  0077 0555 0333 0733 0373  ori.w #$0555,(73673a7,d0.w*2)   
00fc06a0  0773 0337 0737 0377       bchg d3,(73a77a3,d0.w*2)        
00fc06a8  0000 3f00                 ori.b #$00,d0                   
00fc06ac  302f 0002                 move.w 2(a7),d0                 
00fc06b0  c07c 0700                 and.w #$0700,d0                 
00fc06b4  6606                      bne $00fc06bc                   
00fc06b6  006f 0300 0002            ori.w #$0300,2(a7)              
00fc06bc  301f                      move.w (a7)+,d0                 
00fc06be  4e73                      rte                             
00fc06c0  52b9 0000 0466            addq.l #$1,(00000466).w         
00fc06c6  5379 0000 0452            subq.w #$1,(00000452).w         
00fc06cc  6b00 00cc                 bmi $00fc079a                   
00fc06d0  48e7 fffe                 movem.l a6/a5/a4/a3/a2/a1/a0/d7/d6/d5/d4/d3/d2/d1/d0,-(a7)
00fc06d4  52b9 0000 0462            addq.l #$1,(00000462).w         
00fc06da  9bcd                      suba.l a5,a5                    
00fc06dc  122d fa01                 move.b -5ff(a5),d1              
00fc06e0  102d 8260                 move.b -7da0(a5),d0             
00fc06e4  c03c 0003                 and.b #$03,d0                   
00fc06e8  b03c 0002                 cmp.b #$02,d0                   
00fc06ec  6c14                      bge $00fc0702                   
00fc06ee  0801 0007                 btst #7,d1                      
00fc06f2  662e                      bne $00fc0722                   
00fc06f4  303c 07d0                 move.w #$07d0,d0                
00fc06f8  51c8 fffe                 dbf d0,$00fc06f8                
00fc06fc  103c 0002                 move.b #$02,d0                  
00fc0700  6012                      bra $00fc0714                   
00fc0702  0801 0007                 btst #7,d1                      
00fc0706  671a                      beq $00fc0722                   
00fc0708  102d 044a                 move.b 44a(a5),d0               
00fc070c  b03c 0002                 cmp.b #$02,d0                   
00fc0710  6d02                      blt $00fc0714                   
00fc0712  4200                      clr.b d0                        
00fc0714  1b40 044c                 move.b d0,44c(a5)               
00fc0718  1b40 8260                 move.b d0,-7da0(a5)             
00fc071c  206d 046e                 movea.l 46e(a5),a0              
00fc0720  4e90                      jsr (a0)                        
00fc0722  4eb9 00fc a6ee            jsr (00fca6ee).w                
00fc0728  9bcd                      suba.l a5,a5                    
00fc072a  4aad 045a                 tst.l 45a(a5)                   
00fc072e  6716                      beq $00fc0746                   
00fc0730  206d 045a                 movea.l 45a(a5),a0              
00fc0734  43ed 8240                 lea -7dc0(a5),a1                
00fc0738  303c 000f                 move.w #$000f,d0                
00fc073c  32d8                      move.w (a0)+,(a1)+              
00fc073e  51c8 fffc                 dbf d0,$00fc073c                
00fc0742  42ad 045a                 clr.l 45a(a5)                   
00fc0746  4aad 045e                 tst.l 45e(a5)                   
00fc074a  6712                      beq $00fc075e                   
00fc074c  2b6d 045e 044e            move.l 45e(a5),44e(a5)          
00fc0752  1b6d 0450 8203            move.b 450(a5),-7dfd(a5)        
00fc0758  1b6d 044f 8201            move.b 44f(a5),-7dff(a5)        
00fc075e  6100 0c00                 bsr $00fc1360                   
00fc0762  3e39 0000 0454            move.w (00000454).w,d7          
00fc0768  6720                      beq $00fc078a                   
00fc076a  5387                      subq.l #$1,d7                   
00fc076c  2079 0000 0456            movea.l (00000456).w,a0         
00fc0772  2258                      movea.l (a0)+,a1                
00fc0774  b3fc 0000 0000            cmpa.l #$00000000,a1            
00fc077a  670a                      beq $00fc0786                   
00fc077c  48e7 0180                 movem.l a0/d7,-(a7)             
00fc0780  4e91                      jsr (a1)                        
00fc0782  4cdf 0180                 movem.l (a7)+,d7/a0             
00fc0786  51cf ffea                 dbf d7,$00fc0772                
00fc078a  9bcd                      suba.l a5,a5                    
00fc078c  4a6d 04ee                 tst.w 4ee(a5)                   
00fc0790  6604                      bne $00fc0796                   
00fc0792  6100 0566                 bsr $00fc0cfa                   
00fc0796  4cdf 7fff                 movem.l (a7)+,d0/d1/d2/d3/d4/d5/d6/d7/a0/a1/a2/a3/a4/a5/a6
00fc079a  5279 0000 0452            addq.w #$1,(00000452).w         
00fc07a0  4e73                      rte                             
00fc07a2  40e7                      move sr,-(a7)                   
00fc07a4  027c f8ff                 andi #$f8ff,sr                  
00fc07a8  2039 0000 0466            move.l (00000466).w,d0          
00fc07ae  b0b9 0000 0466            cmp.l (00000466).w,d0           
00fc07b4  67f8                      beq $00fc07ae                   
00fc07b6  46df                      move (a7)+,sr                   
00fc07b8  4e75                      rts                             
00fc07ba  2f39 0000 0404            move.l (00000404).w,-(a7)       
00fc07c0  70ff                      moveq #$ff,d0                   
00fc07c2  4e75                      rts                             
00fc07c4  41fa 0084                 lea (0084,pc)[00fc084a],a0      
00fc07c8  6004                      bra $00fc07ce                   
00fc07ca  41fa 004c                 lea (004c,pc)[00fc0818],a0      
00fc07ce  2279 0000 04a2            movea.l (000004a2).w,a1         
00fc07d4  301f                      move.w (a7)+,d0                 
00fc07d6  3300                      move.w d0,-(a1)                 
00fc07d8  231f                      move.l (a7)+,-(a1)              
00fc07da  48e1 1f1f                 movem.l a7/a6/a5/a4/a3/d7/d6/d5/d4/d3,-(a1)
00fc07de  23c9 0000 04a2            move.l a1,(000004a2).w          
00fc07e4  0800 000d                 btst #13,d0                     
00fc07e8  6602                      bne $00fc07ec                   
00fc07ea  4e6f                      move usp,a7                     
00fc07ec  301f                      move.w (a7)+,d0                 
00fc07ee  b058                      cmp.w (a0)+,d0                  
00fc07f0  6c10                      bge $00fc0802                   
00fc07f2  e548                      lsl.w 2,d0                      
00fc07f4  2030 0000                 move.l (0a0,d0.w*1),d0          
00fc07f8  2040                      movea.l d0,a0                   
00fc07fa  6a02                      bpl $00fc07fe                   
00fc07fc  2050                      movea.l (a0),a0                 
00fc07fe  9bcd                      suba.l a5,a5                    
00fc0800  4e90                      jsr (a0)                        
00fc0802  2279 0000 04a2            movea.l (000004a2).w,a1         
00fc0808  4cd9 f8f8                 movem.l (a1)+,d3/d4/d5/d6/d7/a3/a4/a5/a6/a7
00fc080c  2f19                      move.l (a1)+,-(a7)              
00fc080e  3f19                      move.w (a1)+,-(a7)              
00fc0810  23c9 0000 04a2            move.l a1,(000004a2).w          
00fc0816  4e73                      rte                             
00fc0818  000c 00fc                 ori.b #$fc,a4                   
00fc081c  0a10 00fc                 eori.b #$fc,(a0)                
00fc0820  0956                      bchg d4,(a6)                    
00fc0822  00fc                      dc                              
00fc0824  095c                      bchg d4,(a4)+                   
00fc0826  00fc                      dc                              
00fc0828  0968 8000                 bchg d4,-8000(a0)               
00fc082c  0476 00fc 0a3c            subi.w #$00fc,(3ca6,d0.l*2)     
00fc0832  00fc                      dc                              
00fc0834  0a54 8000                 eori.w #$8000,(a4)              
00fc0838  0472 00fc 0962 8000       subi.w #$00fc,(8000a2,d0.l*1)   
00fc0840  047e                      dc                              
00fc0842  00fc                      dc                              
00fc0844  09f8 00fc                 bset d4,(00fc).w                
00fc0848  09fe                      dc                              
00fc084a  0041 00fc                 ori.w #$00fc,d1                 
00fc084e  4102                      chk.l d2,d0                     
00fc0850  00fc                      dc                              
00fc0852  0652 00fc                 addi.w #$00fc,(a2)              
00fc0856  0a5c 00fc                 eori.w #$00fc,(a4)+             
00fc085a  0a6c 00fc 0a72            eori.w #$00fc,a72(a4)           
00fc0860  00fc                      dc                              
00fc0862  0a7e                      dc                              
00fc0864  00fc                      dc                              
00fc0866  0ac4                      dc                              
00fc0868  00fc                      dc                              
00fc086a  0acc                      dc                              
00fc086c  00fc                      dc                              
00fc086e  0f38 00fc                 btst d7,(00fc).w                
00fc0872  100a                      move.b a2,d0                    
00fc0874  00fc                      dc                              
00fc0876  10c6                      move.b d6,(a0)+                 
00fc0878  00fc                      dc                              
00fc087a  1732 00fc                 move.b (-4a2,d0.w*1),-(a3)      
00fc087e  3292                      move.w (a2),(a1)                
00fc0880  00fc                      dc                              
00fc0882  3754 00fc                 move.w (a4),fc(a3)              
00fc0886  39fe                      dc                              
00fc0888  00fc                      dc                              
00fc088a  3a16                      move.w (a6),d5                  
00fc088c  00fc                      dc                              
00fc088e  4206                      clr.b d6                        
00fc0890  00fc                      dc                              
00fc0892  1c76 00fc                 move.b (-4a6,d0.w*1),a6         
00fc0896  1d42 00fc                 move.b d2,fc(a6)                
00fc089a  1286                      move.b d6,(a1)                  
00fc089c  00fc                      dc                              
00fc089e  0cfa                      dc                              
00fc08a0  00fc                      dc                              
00fc08a2  a720                      dc                              
00fc08a4  00fc                      dc                              
00fc08a6  0e4c                      dc                              
00fc08a8  00fc                      dc                              
00fc08aa  0e3e                      dc                              
00fc08ac  00fc                      dc                              
00fc08ae  4232 00fc                 clr.b (-4a2,d0.w*1)             
00fc08b2  3480                      move.w d0,(a2)                  
00fc08b4  00fc                      dc                              
00fc08b6  377e                      dc                              
00fc08b8  00fc                      dc                              
00fc08ba  37b8 00fc 407e            move.w (00fc).w,(7ea3,d4.w*1)   
00fc08c0  00fc                      dc                              
00fc08c2  40dc                      move sr,(a4)+                   
00fc08c4  00fc                      dc                              
00fc08c6  40b6 00fc                 negx.l (-4a6,d0.w*1)            
00fc08ca  41cc                      lea a4,a0                       
00fc08cc  00fc                      dc                              
00fc08ce  424c                      clr.w a4                        
00fc08d0  00fc                      dc                              
00fc08d2  4260                      clr.w -(a0)                     
00fc08d4  00fc                      dc                              
00fc08d6  4294                      clr.l (a4)                      
00fc08d8  00fc                      dc                              
00fc08da  4272 00fc                 clr.w (-4a2,d0.w*1)             
00fc08de  215c 00fc                 move.l (a4)+,fc(a0)             
00fc08e2  07a2                      bclr d3,-(a2)                   
00fc08e4  00fc                      dc                              
00fc08e6  0950                      bchg d4,(a0)                    
00fc08e8  00fc                      dc                              
00fc08ea  0af0                      dc                              
00fc08ec  00fc                      dc                              
00fc08ee  0652 00fc                 addi.w #$00fc,(a2)              
00fc08f2  1692                      move.b (a2),(a3)                
00fc08f4  00fc                      dc                              
00fc08f6  0652 00fc                 addi.w #$00fc,(a2)              
00fc08fa  0652 00fc                 addi.w #$00fc,(a2)              
00fc08fe  0652 00fc                 addi.w #$00fc,(a2)              
00fc0902  0652 00fc                 addi.w #$00fc,(a2)              
00fc0906  0652 00fc                 addi.w #$00fc,(a2)              
00fc090a  0652 00fc                 addi.w #$00fc,(a2)              
00fc090e  0652 00fc                 addi.w #$00fc,(a2)              
00fc0912  0652 00fc                 addi.w #$00fc,(a2)              
00fc0916  0652 00fc                 addi.w #$00fc,(a2)              
00fc091a  0652 00fc                 addi.w #$00fc,(a2)              
00fc091e  0652 00fc                 addi.w #$00fc,(a2)              
00fc0922  0652 00fc                 addi.w #$00fc,(a2)              
00fc0926  0652 00fc                 addi.w #$00fc,(a2)              
00fc092a  0652 00fc                 addi.w #$00fc,(a2)              
00fc092e  0652 00fc                 addi.w #$00fc,(a2)              
00fc0932  0652 00fc                 addi.w #$00fc,(a2)              
00fc0936  0652 00fc                 addi.w #$00fc,(a2)              
00fc093a  0652 00fc                 addi.w #$00fc,(a2)              
00fc093e  0652 00fc                 addi.w #$00fc,(a2)              
00fc0942  0652 00fc                 addi.w #$00fc,(a2)              
00fc0946  0652 00fc                 addi.w #$00fc,(a2)              
00fc094a  0652 00fc                 addi.w #$00fc,(a2)              
00fc094e  0e9e                      dc                              
00fc0950  206f 0004                 movea.l 4(a7),a0                
00fc0954  4ed0                      jmp (a0)                        
00fc0956  41ed 051e                 lea 51e(a5),a0                  
00fc095a  6010                      bra $00fc096c                   
00fc095c  41ed 053e                 lea 53e(a5),a0                  
00fc0960  600a                      bra $00fc096c                   
00fc0962  41ed 055e                 lea 55e(a5),a0                  
00fc0966  6004                      bra $00fc096c                   
00fc0968  41ed 057e                 lea 57e(a5),a0                  
00fc096c  302f 0004                 move.w 4(a7),d0                 
00fc0970  e548                      lsl.w 2,d0                      
00fc0972  2070 0000                 movea.l (0a0,d0.w*1),a0         
00fc0976  4ed0                      jmp (a0)                        
00fc0978  00fc                      dc                              
00fc097a  0652 00fc                 addi.w #$00fc,(a2)              
00fc097e  33a6 00fc                 move.w -(a6),(-4a1,d0.w*1)      
00fc0982  3494                      move.w (a4),(a2)                
00fc0984  00fc                      dc                              
00fc0986  32a6                      move.w -(a6),(a1)               
00fc0988  00fc                      dc                              
00fc098a  0652 00fc                 addi.w #$00fc,(a2)              
00fc098e  0652 00fc                 addi.w #$00fc,(a2)              
00fc0992  0652 00fc                 addi.w #$00fc,(a2)              
00fc0996  0652 00fc                 addi.w #$00fc,(a2)              
00fc099a  3372 00fc 33be            move.w (-4a2,d0.w*1),33be(a1)   
00fc09a0  00fc                      dc                              
00fc09a2  34aa 00fc                 move.w fc(a2),(a2)              
00fc09a6  32c0                      move.w d0,(a1)+                 
00fc09a8  00fc                      dc                              
00fc09aa  0652 00fc                 addi.w #$00fc,(a2)              
00fc09ae  0652 00fc                 addi.w #$00fc,(a2)              
00fc09b2  0652 00fc                 addi.w #$00fc,(a2)              
00fc09b6  0652 00fc                 addi.w #$00fc,(a2)              
00fc09ba  3392 00fc                 move.w (a2),(-4a1,d0.w*1)       
00fc09be  3408                      move.w a0,d2                    
00fc09c0  00fc                      dc                              
00fc09c2  34e0                      move.w -(a0),(a2)+              
00fc09c4  00fc                      dc                              
00fc09c6  344a                      movea.w a2,a2                   
00fc09c8  00fc                      dc                              
00fc09ca  326a 00fc                 movea.w fc(a2),a1               
00fc09ce  0652 00fc                 addi.w #$00fc,(a2)              
00fc09d2  0652 00fc                 addi.w #$00fc,(a2)              
00fc09d6  0652 00fc                 addi.w #$00fc,(a2)              
00fc09da  32f6 00fc                 move.w (-4a6,d0.w*1),(a1)+      
00fc09de  3422                      move.w -(a2),d2                 
00fc09e0  00fc                      dc                              
00fc09e2  a364                      dc                              
00fc09e4  00fc                      dc                              
00fc09e6  327a 00fc                 movea.w (00fc,pc)[00fc0ae4],a1  
00fc09ea  345c                      movea.w (a4)+,a2                
00fc09ec  00fc                      dc                              
00fc09ee  a358                      dc                              
00fc09f0  00fc                      dc                              
00fc09f2  0652 00fc                 addi.w #$00fc,(a2)              
00fc09f6  0652 202d                 addi.w #$202d,(a2)              
00fc09fa  04c2                      dc                              
00fc09fc  4e75                      rts                             
00fc09fe  7000                      moveq #$00,d0                   
00fc0a00  102d 0e7d                 move.b e7d(a5),d0               
00fc0a04  322f 0004                 move.w 4(a7),d1                 
00fc0a08  6b04                      bmi $00fc0a0e                   
00fc0a0a  1b41 0e7d                 move.b d1,e7d(a5)               
00fc0a0e  4e75                      rts                             
00fc0a10  206f 0004                 movea.l 4(a7),a0                
00fc0a14  43ed 048e                 lea 48e(a5),a1                  
00fc0a18  2089                      move.l a1,(a0)                  
00fc0a1a  42a8 0004                 clr.l 4(a0)                     
00fc0a1e  2149 0008                 move.l a1,8(a0)                 
00fc0a22  4291                      clr.l (a1)                      
00fc0a24  236d 0432 0004            move.l 432(a5),4(a1)            
00fc0a2a  202d 0436                 move.l 436(a5),d0               
00fc0a2e  90ad 0432                 sub.l 432(a5),d0                
00fc0a32  2340 0008                 move.l d0,8(a1)                 
00fc0a36  42a9 000c                 clr.l c(a1)                     
00fc0a3a  4e75                      rts                             
00fc0a3c  302f 0004                 move.w 4(a7),d0                 
00fc0a40  e548                      lsl.w 2,d0                      
00fc0a42  91c8                      suba.l a0,a0                    
00fc0a44  41f0 0000                 lea (0a0,d0.w*1),a0             
00fc0a48  2010                      move.l (a0),d0                  
00fc0a4a  222f 0006                 move.l 6(a7),d1                 
00fc0a4e  6b02                      bmi $00fc0a52                   
00fc0a50  2081                      move.l d1,(a0)                  
00fc0a52  4e75                      rts                             
00fc0a54  7000                      moveq #$00,d0                   
00fc0a56  302d 0442                 move.w 442(a5),d0               
00fc0a5a  4e75                      rts                             
00fc0a5c  7000                      moveq #$00,d0                   
00fc0a5e  102d 8201                 move.b -7dff(a5),d0             
00fc0a62  e148                      lsl.w 0,d0                      
00fc0a64  102d 8203                 move.b -7dfd(a5),d0             
00fc0a68  e188                      lsl.l 0,d0                      
00fc0a6a  4e75                      rts                             
00fc0a6c  202d 044e                 move.l 44e(a5),d0               
00fc0a70  4e75                      rts                             
00fc0a72  7000                      moveq #$00,d0                   
00fc0a74  102d 8260                 move.b -7da0(a5),d0             
00fc0a78  c03c 0003                 and.b #$03,d0                   
00fc0a7c  4e75                      rts                             
00fc0a7e  4aaf 0004                 tst.l 4(a7)                     
00fc0a82  6b06                      bmi $00fc0a8a                   
00fc0a84  2b6f 0004 044e            move.l 4(a7),44e(a5)            
00fc0a8a  4aaf 0008                 tst.l 8(a7)                     
00fc0a8e  6b0c                      bmi $00fc0a9c                   
00fc0a90  1b6f 0009 8201            move.b 9(a7),-7dff(a5)          
00fc0a96  1b6f 000a 8203            move.b a(a7),-7dfd(a5)          
00fc0a9c  4a6f 000c                 tst.w c(a7)                     
00fc0aa0  6b20                      bmi $00fc0ac2                   
00fc0aa2  1b6f 000d 044c            move.b d(a7),44c(a5)            
00fc0aa8  6100 fcf8                 bsr $00fc07a2                   
00fc0aac  1b6d 044c 8260            move.b 44c(a5),-7da0(a5)        
00fc0ab2  426d 0452                 clr.w 452(a5)                   
00fc0ab6  4eb9 00fc b522            jsr (00fcb522).w                
00fc0abc  3b7c 0001 0452            move.w #$0001,452(a5)           
00fc0ac2  4e75                      rts                             
00fc0ac4  2b6f 0004 045a            move.l 4(a7),45a(a5)            
00fc0aca  4e75                      rts                             
00fc0acc  322f 0004                 move.w 4(a7),d1                 
00fc0ad0  d241                      add.w d1,d1                     
00fc0ad2  c27c 001f                 and.w #$001f,d1                 
00fc0ad6  41ed 8240                 lea -7dc0(a5),a0                
00fc0ada  3030 1000                 move.w (0a0,d1.w*1),d0          
00fc0ade  c07c 0777                 and.w #$0777,d0                 
00fc0ae2  4a6f 0006                 tst.w 6(a7)                     
00fc0ae6  6b06                      bmi $00fc0aee                   
00fc0ae8  31af 0006 1000            move.w 6(a7),(0a0,d1.w*1)       
00fc0aee  4e75                      rts                             
00fc0af0  207a f522                 movea.l (-ade,pc)[00fc0014],a0  
00fc0af4  0c90 8765 4321            cmpi.l #$87654321,(a0)          
00fc0afa  660c                      bne $00fc0b08                   
00fc0afc  b1ed 042e                 cmpa.l 42e(a5),a0               
00fc0b00  6c06                      bge $00fc0b08                   
00fc0b02  4290                      clr.l (a0)                      
00fc0b04  6000 f52a                 bra $00fc0030                   
00fc0b08  4e75                      rts                             
00fc0b0a  6102                      bsr $00fc0b0e                   
00fc0b0c  4e71                      nop                             
00fc0b0e  9bcd                      suba.l a5,a5                    
00fc0b10  2b5f 03c4                 move.l (a7)+,3c4(a5)            
00fc0b14  48ed ffff 0384            movem.l d0/d1/d2/d3/d4/d5/d6/d7/a0/a1/a2/a3/a4/a5/a6/a7,384(a5)
00fc0b1a  4e68                      move usp,a0                     
00fc0b1c  2b48 03c8                 move.l a0,3c8(a5)               
00fc0b20  700f                      moveq #$0f,d0                   
00fc0b22  41ed 03cc                 lea 3cc(a5),a0                  
00fc0b26  224f                      movea.l a7,a1                   
00fc0b28  30d9                      move.w (a1)+,(a0)+              
00fc0b2a  51c8 fffc                 dbf d0,$00fc0b28                
00fc0b2e  2b7c 1234 5678 0380       move.l #$12345678,380(a5)       
00fc0b36  7200                      moveq #$00,d1                   
00fc0b38  122d 03c4                 move.b 3c4(a5),d1               
00fc0b3c  5341                      subq.w #$1,d1                   
00fc0b3e  6116                      bsr $00fc0b56                   
00fc0b40  2b7c 0000 093a 04a2       move.l #$0000093a,4a2(a5)       
00fc0b48  3f3c ffff                 move.w #$ffff,-(a7)             
00fc0b4c  3f3c 004c                 move.w #$004c,-(a7)             
00fc0b50  4e41                      trap #33                        
00fc0b52  6000 f4dc                 bra $00fc0030                   
00fc0b56  1e2d 8260                 move.b -7da0(a5),d7             
00fc0b5a  ce7c 0003                 and.w #$0003,d7                 
00fc0b5e  de47                      add.w d7,d7                     
00fc0b60  7000                      moveq #$00,d0                   
00fc0b62  102d 8201                 move.b -7dff(a5),d0             
