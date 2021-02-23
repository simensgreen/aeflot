      subroutine inpfl(inpf,outf,outhl,outiz,outbl,outcp,outfd)
      character *80 inpf,outf,outhl,outiz,outbl,outcp,outfd
c----------------------------------
      iwr=6
 9000 format (a30)
      do i=1,30
        inpf(i:i)=' '
      enddo
c----------------------------------------------------------------
      narg=nargs()                              ! silicon grafic
c     write(iwr,*) narg                         ! silicon grafic
      if (narg.gt.1) then                       ! silicon grafic
        call getarg(1,inpf,ninpf)               ! silicon grafic
      else                                      ! silicon grafic
c----------------------------------------------------------------
        print *, 'enter the name of input file:'
        read (5,9000) inpf
        iflag = 0
        do i = 30,1,-1
          if (inpf(i:i) .eq. '.') then
            iflag=1
            go to 50
          endif
        enddo
c-------------------------------------
  50    if (iflag.eq.0) then
          inpf='p_car.dat'
          outf='p_car.out'
          outhl='p_car.hl'
          outiz='p_car.iz'
          outbl='p_car.bl'
          outcp='p_car.cp'
          outfd='p_car.fd'
          go to 500
        endif
      endif                                      ! silicon grafic
c-------------------------------------
      iflag = 0
      outf  = inpf
      outhl = inpf
      outiz = inpf
      outbl = inpf
      outcp = inpf
      outfd = inpf
      do i = 30,1,-1
        if (inpf(i:i) .eq. '.') then
          iflag=1
          go to 100
        endif
      enddo
 100  if (iflag.eq.1) then
        do j = i+1, 30
          outf(j:j) = ' '
          outhl(j:j) = ' '
          outiz(j:j) = ' '
          outbl(j:j)= ' '
          outcp(j:j) = ' '
          outfd(j:j) = ' '
        enddo
        outf (i+1 : i+3) = 'out'
        outhl(i+1 : i+2) = 'hl'
        outiz(i+1 : i+2) = 'iz'
        outbl(i+1 : i+2) = 'bl'
        outcp(i+1 : i+2) = 'cp'
        outfd(i+1 : i+2) = 'fd'
      else
        iflag = 0
        do j = 30,1,-1
          if (inpf(j:j) .ne. ' ') then
            iflag = 1
            go to 200
          endif
        end do
  200   if (iflag .eq. 1) then
          outf (j+1 : j+4) = '.out'
          outhl(j+1 : j+3) = '.hl'
          outiz(j+1 : j+3) = '.iz'
          outbl(j+1 : j+3) = '.bl'
          outcp(j+1 : j+3) = '.cp'
          outfd(j+1 : j+3) = '.fd'
        else
          print *, ' The name of input file defined uncorrected'
          stop
        endif
      endif
  500 continue
c      write(iwr,*) ' input file :', inpf
c      write(iwr,*) ' save  file :', savf
c      write(iwr,*) ' out   file :', outf
c      write(iwr,*) ' cp    file :',  cpf
c      write(iwr,*) ' pl4   file :', pl4f
c      write(iwr,*) ' pictg file :', pictg
      return
      end
