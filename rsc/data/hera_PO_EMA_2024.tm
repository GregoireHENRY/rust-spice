KPL/MK

Meta-kernel for HERA Dataset v100 -- Studies 20210226_001
==========================================================================

   This meta-kernel lists the hera Studies SPICE kernels
   that provide information for the Studies scenario.

   The kernels listed in this meta-kernel and the order in which
   they are listed are picked to provide the best data available and
   the most complete coverage for the hera Studies scenario.


Usage of the Meta-kernel
-------------------------------------------------------------------------

   This file is used by the SPICE system as follows: programs that make use
   of this kernel must "load" the kernel normally during program
   initialization. Loading the kernel associates the data items with
   their names in a data structure called the "kernel pool". The SPICELIB
   routine FURNSH loads a kernel into the pool.

   The kernels listed below can be obtained from the ESA SPICE FTP server:

      ftp://spiftp.esac.esa.int/data/SPICE/hera/kernels/


Implementation Notes
-------------------------------------------------------------------------

   It is recommended that users make a local copy of this file and
   modify the value of the PATH_VALUES keyword to point to the actual
   location of the hera SPICE data set's ``data'' directory on
   their system. Replacing ``/'' with ``\'' and converting line
   terminators to the format native to the user's system may also be
   required if this meta-kernel is to be used on a non-UNIX workstation.


-------------------

   This file was created on February 26, 2021 by Alfredo Escalante ESAC/ESA.


   \begindata

     PATH_VALUES       = ( '/home/greg/rob/hera/kernels' )

     PATH_SYMBOLS      = ( 'KERNELS' )

     KERNELS_TO_LOAD   = (

                           '$KERNELS/fk/hera_v06.tf'
                           '$KERNELS/fk/hera_ops_v02.tf'
                           '$KERNELS/fk/hera_dsk_surfaces_v03.tf'

                           '$KERNELS/ik/hera_afc_v03.ti'
                           '$KERNELS/ik/hera_palt_v00.ti'
                           '$KERNELS/ik/hera_tira_v00.ti'

                           '$KERNELS/lsk/naif0010.tls'

                           '$KERNELS/pck/pck00010.tpc'
                           '$KERNELS/pck/de-403-masses.tpc'
                           '$KERNELS/pck/hera_didymos_v02.tpc'

                           '$KERNELS/sclk/hera_fict_20181203.tsc'

                           '$KERNELS/spk/de432s.bsp'
                           '$KERNELS/spk/didymos_hor_200101_400101_v01.bsp'
                           '$KERNELS/spk/didymos_gmv_270101_330623_v02.bsp'
                           '$KERNELS/spk/hera_dart_impact_site_v01.bsp'

                           '$KERNELS/spk/HERA_sc_PO_LPO_EMA_2024_v02.bsp'

                           '$KERNELS/ck/hera_sc_PO_EMA_20270209_20270807_f20181203_v02.bc'

                           '$KERNELS/dsk/g_50677mm_rad_obj_dida_0000n00000_v001.bds'
                           '$KERNELS/dsk/g_08438mm_lgt_obj_didb_0000n00000_v002.bds'

                         )

   \begintext


SPICE Kernel Dataset Version
------------------------------------------------------------------------

   The version of this SPICE Kernel Dataset is provided by the following
   keyword:

   \begindata

      SKD_VERSION = 'v100_20210226_001'

   \begintext


Contact Information
------------------------------------------------------------------------

   If you have any questions regarding this file contact the
   ESA SPICE Service at ESAC:

           Alfredo Escalante Lopez
           (+34) 91-8131-429
           spice@sciops.esa.int


End of MK file.
