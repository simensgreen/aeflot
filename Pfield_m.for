C*******************************************
C*******************************************
C***     P R O G R A M   A E F L O T     ***
C*******************************************
C*******************************************
C**** CEGMENTOB FUSELAGE PABHO 9 ***********
C****BAPIAHT C PROCHNOCTIA *****************
      IMPLICIT REAL*8 (A-H,O-Z)
C==============================================================
      DIMENSION XFUS(60),YB(30,30),ZB(30,30)
      COMMON /BLRR/ XXB(60),YYB(30,30),ZZB(30,30),FXPL(8),NXPL,NNNX,NNYZ
      COMMON /BL/ NDUM(55)
      EQUIVALENCE (BLOCK,XFUS),(BLOCK(271),YB),(BLOCK(2071),ZB)
C==============================================================
      COMMON /PARAM/ NBODY,NWING,NTAIL,LBC,THK,NN1,MACH,ALPHA,
     *REFA,SIDES,REF(6)
      COMMON ARRAY(6000),BLOCK(17600)
      COMMON /HEAD/ TITLE(16)
      COMMON /SEG/ NDUM1(41),NN2,DUM(60),NDUM2(20),DUM1(140)
     1,NCSUM
      COMMON /BTHET/ TB(600)
      COMMON /NEWCOM/ NDUM3(71),NN3,DUM2(40)
      COMMON /MATCOM/ MATIN
      COMMON /VELCOM/ N(5),NN4,EM,L(84)
      COMMON /PODOR/ K3,K6,NP,KPADX(9),KPODX(9),NPRADX(9),NPUSOR(9)
      COMMON /FIELD/ DUM3(10250),KFIELD,NDUM4(2),FIEL
      COMMON /TOLA/ ITT(600),NGRI
      COMMON /SOV/ EPSIL,NITER
      COMMON /SOPPA/ EPS2
      COMMON/ITER/ ITERM,MAXWTR,ITEMAX,GROUND,BET,DIVER,BELOYC,SHEK
      COMMON/MAXII/POPMAX(600),H
      REAL*8 MACH
      LOGICAL LBC,THK,FIEL,ITEMAX,GROUND,BET,DIVER,BELOYC,SHEK
      common /inp_file/ inpf,outf,outhl,outiz,outbl,outcp,outfd
      character *80 inpf,outf,outhl,outiz,outbl,outcp,outfd

C   Opened of the files for records
      call inpfl(inpf,outf,outhl,outiz,outbl,outcp,outfd)

      OPEN ( UNIT=1,STATUS='SCRATCH',
     *       ACCESS='SEQUENTIAL',FORM='UNFORMATTED')
      OPEN ( UNIT=2,STATUS='SCRATCH',
     *       ACCESS='SEQUENTIAL',FORM='UNFORMATTED')
      OPEN ( UNIT=9,STATUS='SCRATCH',
     *       ACCESS='SEQUENTIAL',FORM='UNFORMATTED')
      OPEN ( UNIT=10,STATUS='SCRATCH',
     *       ACCESS='SEQUENTIAL',FORM='UNFORMATTED')
      OPEN ( UNIT=11,STATUS='SCRATCH',
     *       ACCESS='SEQUENTIAL',FORM='UNFORMATTED')
      OPEN ( UNIT=12,STATUS='SCRATCH',
     *       ACCESS='SEQUENTIAL',FORM='UNFORMATTED')
      OPEN ( UNIT=13,STATUS='SCRATCH',
     *       ACCESS='SEQUENTIAL',FORM='UNFORMATTED')
      OPEN ( UNIT=14,STATUS='SCRATCH',
     *       ACCESS='SEQUENTIAL',FORM='UNFORMATTED')
      OPEN ( UNIT=15,STATUS='SCRATCH',
     *       ACCESS='SEQUENTIAL',FORM='UNFORMATTED')
      OPEN ( UNIT=19,STATUS='SCRATCH',
     *       ACCESS='SEQUENTIAL',FORM='UNFORMATTED')

c      WRITE (5,*) "INPUT FILE:"
      open (unit = 105, file = inpf, status = 'old',
     1 access = 'sequential')
c      WRITE (5,*) "OUTPUT FILE :"
    3 FORMAT(A20)
    2 CONTINUE
      open (unit = 108, file = outf, status = 'unknown',
     1 access = 'sequential')
      rewind 108
      open (unit = 109, file = outfd, status = 'unknown',
     1 access = 'sequential')
      rewind 109

      CALL ABZUG(105,108)
cc      REWIND 5
   10 CALL GEOM
c---------------------------------------------------------
      FNVB=1.0
      FNX=NDUM(20)
      FNYZ=NDUM(11)
      NNNX=NDUM(20)
      NNYZ=NDUM(11)
      FNXPL=8.0
      NXPL=FNXPL
      DO 270 IIJ=1,NNNX
      XXB(IIJ)=XFUS(IIJ)
      DO 270 JJJ=1,NNYZ
      YYB(IIJ,JJJ)=YB(IIJ,JJJ)
      ZZB(IIJ,JJJ)=ZB(IIJ,JJJ)
  270 CONTINUE
c---------------------------------------------------------
      READ(105,40) EPSIL,NITER
      READ(105,50) EPS2
   20 CALL VELCMP
      IF (FIEL) CALL FIELDS
      IF (.NOT.FIEL) GO TO 21
      FIEL=.FALSE.
      GO TO 20
  21  HC=MACH-.5d0
      IF(INT(HC).EQ.-1) GO TO 10
      IF(INT(HC).EQ.-2) GO TO 30
      CALL SULVE
      IF (KFIELD.EQ.1) FIEL=.TRUE.
      GOTO 20
  30  IF (.NOT.SHEK) GO TO 31
cc      REWIND 6
      WRITE (6) NGRI
cc      REWIND 6
C     OUTPUT NGRI
cc      REWIND 5
  31  STOP 'STOP AEFLOT'
  40  FORMAT(F10.0,I3)
  50  FORMAT(F10.0)
      END
C     ***********************
      SUBROUTINE ABZUG(IN,IS)
C     ***********************
      IMPLICIT REAL*8 (A-H,O-Z)
      INTEGER  TEXT,SCHLUS,OD
      DIMENSION TEXT(20)
      DATA SCHLUS /4H$   /
 100  FORMAT (10X,8(10H1234567890))
 101  FORMAT (20A4)
 102  FORMAT (1X,I3,6X,20A4)
 103  FORMAT (1H1)
 104  FORMAT (19X,1H1,9X,1H2,9X,1H3,9X,1H4,9X,1H5,9X,1H6,9X,1H7,9X,1H8)
      OD=IN
      IK=1
      IZ=0
      WRITE(IS,103)
  1   CONTINUE
      READ (IN,101) TEXT
      IF (IZ.NE.0) GO TO 4
      WRITE (IS,104)
      WRITE (IS,100)
   4  WRITE (IS,102) IK,TEXT
      IK=IK+1
      IZ=IZ+1
      IF (IZ.NE.50) GO TO 3
      IZ=0
      WRITE(IS,100)
      WRITE(IS,103)
   3  CONTINUE
      IF (TEXT(1).EQ.SCHLUS) GO TO 2
      GO TO 1
   2  CONTINUE
      WRITE(IS,103)
      REWIND OD
      RETURN
      END
C     *******************************
      SUBROUTINE FIELDS
C     *******************************
      IMPLICIT REAL*8 (A-H,O-Z)
      COMMON ARRAY(6000),BLOCK(17600)
      COMMON /PARAM/ NBODY,NWING,NTAIL,LBC,THK,NN1,XMACH,ALPHA,
     *REF1,SIDES,DUM(6)
      COMMON /NEWCOM/ K1,KWAF,KWAFOR,NDUM2(69),DUM1(20),YS(20)
      COMMON /VELCOM/ NDUM(6),EX,PRENT,NWTHK,NDUM1(82)
      COMMON /FIELD/ XFIELD(250),YFIELD(250,20),ZFIELD(250,20),KFIELD,
     1KFX,KFY,FIEL
      DIMENSION UC(600),VC(600),WC(600),G(600),U(600),V(600),W(600),
     1DZTDX(600),CPP(600),UCOM(600),GAM(600)
     2,YPT(600),GA(600)
      EQUIVALENCE (ARRAY(601),YPT)
      LOGICAL LBC,THK,FIEL
      INTEGER PRENT
      GAMMA(II,JJ)=(G(II+JJ)-G(II))*(YP-YPT(II))/(YPT(II+JJ)
     1-YPT(II))+G(II)
      ALP=ALPHA/57.2957795d0
      EPS=1.0d-8
      REWIND 14
      REWIND 15
      REWIND 1
      REWIND 2
      REWIND 9
      READ (9) ARRAY,UCOM,UCOM
    2 READ (2) XFIELD,YFIELD,ZFIELD
      READ (2) LFIELD,KFX,KFY
C     BETA=ASIN(1.d0/XMACH)
C     AK=TAN(BETA)
C     D=.8391d0/1.d0
C     A=D*D-AK*AK
      IF (LFIELD.EQ.0) GOTO 50
      NFPOIN=KFX*KFY
      IF (NBODY.EQ.0) GOTO 11
      NCPOIN=NBODY
      READ (1) (G(II),II=1,NBODY)
      DO 10 I=1,NFPOIN
      U(I)=0.d0
      V(I)=0.d0
      W(I)=0.d0
      READ (14) (UC(J),VC(J),WC(J),J=1,NBODY)
      DO 10 J=1,NBODY
      U(I)=U(I)+UC(J)*G(J)
      V(I)=V(I)+VC(J)*G(J)
      W(I)=W(I)+WC(J)*G(J)
   10 CONTINUE
   11 IF (NWING.EQ.0) GOTO 41
      NCPOIN=NWING
      READ (1) DZTDX
      READ (1) (G(II),II=1,NWING)
C***      READ (1) (G(II),II=1,NWING)
C***     NSTEP=KWAFOR ; NSTOP=NWTHK-KWAFOR+1
C***     DO 14 II=1,NWING
C***     GAM(II)=G(II)
C***  14 CONTINUE
C***     DO 13 II=1,NSTOP,NSTEP
C***     GAM(II)=0.d0
C***  13 CONTINUE
C***     DO 15 II=2,NSTOP,NSTEP
C***     GAM(II)=0.d0
C***  15 CONTINUE
C***     WRITE (108,140) (G(II),II=1,NWING)
C***     IF (NWING.EQ.0) GOTO 41
C***     J1=1
C**      J2=0
   12 DO 40 I=1,NFPOIN
      IF(NBODY.NE.0) GO TO 16
      U(I)=0.d0
      V(I)=0.d0
      W(I)=0.d0
C***    J2=J2+1
C***    IF (J2.GT.KFY) J1=J1+1 ; J2=1
C***    XSTART=XFIELD(J1)-ABS(ZFIELD(J1,J2))/AK
C***    IF (XSTART.LE.0.0d0) GOTO 16
C***    B=2.d0*D*(YFIELD(J1,J2)*D-XFIELD(J1)*AK*AK)
C***    C=D*D*(YFIELD(J1,J2)*YFIELD(J1,J2)+ZFIELD(J1,J2)*ZFIELD(J1,J2)
C***   1-XFIELD(J1)*XFIELD(J1)*AK*AK)
C***    YP=(B+SQRT(B*B-4.d0*A*C))/(2.d0*A)
   16 IF (.NOT.THK) GOTO 20
      READ (15) (UC(J),VC(J),WC(J),J=1,NWTHK)
      DO J=1,NWTHK
      U(I)=U(I)+UC(J)*DZTDX(J)
      V(I)=V(I)+VC(J)*DZTDX(J)
      W(I)=W(I)+WC(J)*DZTDX(J)
	enddo
   20 CONTINUE
      READ (15) (UC(J),VC(J),WC(J),J=1,NWING)
      DO 30 J=1,NWING
      GAM(J)=G(J)
      U(I)=U(I)+UC(J)*GAM(J)
      V(I)=V(I)+VC(J)*GAM(J)
      W(I)=W(I)+WC(J)*GAM(J)
   30 CONTINUE
C     JK=1 ; JY=2 ; J=1
C   1 IF (YP.GT.YS(JY).AND.J+NSTEP.LT.NWING) GA(J)=(G(J+NSTEP)-G
C    1(J))*(YS(JY)-YPT(J))/(YPT(J+NSTEP)-YPT(J))+G(J);GOTO 5
C     IF (YP.GT.YS(J).AND.J+NSTEP.GT.NWING) GA(J)=G(J);GOTO 5
C     IF (YP.LE.YS(JY).AND.J.EQ.1) GA(J)=GAMMA
C    1(J,NSTEP) ; GOTO 5
C     IF (YP.LE.YS(JY).AND.YP.LE.YPT(J)) GA(J)=
C    1GAMMA(J-NSTEP,NSTEP)
C     IF (YP.LE.YS(JY).AND.YP.GT.YPT(J)) GA(J)=
C    1GAMMA(J,NSTEP)
C     IF (YP.LE.YS(JY).AND.YP.GT.YPT(J).AND.J+NSTEP.GT.NWING) GA(J)=G(J)
C   5 U(I)=U(I)+UC(J)*GA(J)
C     V(I)=V(I)+VC(J)*GA(J)
C     W(I)=W(I)+WC(J)*GA(J)
C     IF (YP.LE.YS(JY)) GOTO 3
C     J=J+NSTEP ; JY=JY+1 ; JK=JK+NSTEP
C     IF (J.GT.NWING) GOTO 3
C     GOTO 1
C   3 CONTINUE
C     JK=1 ; JY=2 ; J=2
C 101 IF (YP.GT.YS(JY).AND.J+NSTEP.LT.NWING) GA(J)=(G(J+NSTEP)-G
C    1(J))*(YS(JY)-YPT(J))/(YPT(J+NSTEP)-YPT(J))+G(J);GOTO 105
C     IF (YP.GT.YS(JY).AND.J+NSTEP.GT.NWING) GA(J)=G(J);GOTO 105
C     IF (YP.LE.YS(JY).AND.J.EQ.2) GA(J)=GAMMA(J,
C    1NSTEP) ; GOTO 105
C     IF (YP.LE.YS(JY).AND.YP.LE.YPT(J)) GA(J)=
C    1GAMMA(J-NSTEP,NSTEP)
C     IF (YP.LE.YS(JY).AND.YP.GT.YPT(J)) GA(J)=
C    1GAMMA(J,NSTEP)
C     IF (YP.LE.YS(JY).AND.YP.GT.YPT(J).AND.J+NSTEP.GT.NWING) GA(J)=G(J)
C 105 U(I)=U(I)+UC(J)*GA(J)
C     V(I)=V(I)+VC(J)*GA(J)
C     W(I)=W(I)+WC(J)*GA(J)
C     IF (YP.LE.YS(JY)) GOTO 103
C     J=J+NSTEP ; JY=JY+1 ; JK=JK+NSTEP
C     IF (J.GT.NWING) GOTO 103
C     GOTO 101
C 103 CONTINUE
   40 CONTINUE
   41 DO 42 I=1,NFPOIN
      UCOM(I)=SQRT(U(I)*U(I)+V(I)*V(I)+W(I)*W(I))
   42 CONTINUE
C     IF (ABS(PRENT).LE.1) GOTO 46
      WRITE (108,120)
      WRITE (108,90)
      WRITE (108,120)
      J1=1
      J2=0
      DO 45 N=1,NFPOIN
      J2=J2+1
      IF (J2.GT.KFY) J1=J1+1
      IF (J2.GT.KFY) J2=1
      WRITE (108,110) N,XFIELD(J1),YFIELD(J1,J2),ZFIELD(J1,J2),U(N),V(
     1N),W(N),UCOM(N)
      WRITE (109,111) N,XFIELD(J1),YFIELD(J1,J2),ZFIELD(J1,J2),U(N),V(
     1N),W(N)
   45 CONTINUE
   46 CONTINUE
      DO 47 N=1,NFPOIN
      CPP(N)=-2.d0*U(N)
   47 CONTINUE
      WRITE (108,120)
      WRITE (108,130)
      WRITE (108,120)
      J1=1
      J2=0
      DO N=1,NFPOIN
      J2=J2+1
      IF (J2.GT.KFY) J1=J1+1
      IF (J2.GT.KFY) J2=1
      WRITE (108,110) N,XFIELD(J1),YFIELD(J1,J2),ZFIELD(J1,J2),CPP(N)
      enddo
   50 CONTINUE
      REWIND 1
      IF (LFIELD.NE.0) GOTO 2
      REWIND 14
      REWIND 15
      REWIND 2
   90 FORMAT (1X,5HPOINT,10X,1HX,10X,1HY,10X,1HZ,10X,1HU,10X,1HV,10X,
     11HW,8X,4HUCOM)
  110 FORMAT (1X,I6,10F11.5)
  111 FORMAT (1X,I3,10F7.5)
  120 FORMAT (//)
  130 FORMAT (1X,5HPOINT,9X,1HX,10X,1HY,10X,1HZ,9X,2HCP)
  140 FORMAT (1X,10F11.5)
      RETURN
      END
