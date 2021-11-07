#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Slave Own Address This register consists of seven address bits that identify this I2C device on the I2C bus."]
    pub soar: crate::Reg<soar::SOAR_SPEC>,
    _reserved_1_sctl: [u8; 0x04],
    #[doc = "0x08 - Slave Data This register contains the data to be transmitted when in the Slave Transmit state, and the data received when in the Slave Receive state."]
    pub sdr: crate::Reg<sdr::SDR_SPEC>,
    #[doc = "0x0c - Slave Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
    pub simr: crate::Reg<simr::SIMR_SPEC>,
    #[doc = "0x10 - Slave Raw Interrupt Status This register shows the unmasked interrupt status."]
    pub sris: crate::Reg<sris::SRIS_SPEC>,
    #[doc = "0x14 - Slave Masked Interrupt Status This register show which interrupt is active (based on result from SRIS and SIMR)."]
    pub smis: crate::Reg<smis::SMIS_SPEC>,
    #[doc = "0x18 - Slave Interrupt Clear This register clears the raw interrupt SRIS."]
    pub sicr: crate::Reg<sicr::SICR_SPEC>,
    _reserved7: [u8; 0x07e4],
    #[doc = "0x800 - Master Salve Address This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit."]
    pub msa: crate::Reg<msa::MSA_SPEC>,
    _reserved_8_mctrl: [u8; 0x04],
    #[doc = "0x808 - Master Data This register contains the data to be transmitted when in the Master Transmit state and the data received when in the Master Receive state."]
    pub mdr: crate::Reg<mdr::MDR_SPEC>,
    #[doc = "0x80c - I2C Master Timer Period This register specifies the period of the SCL clock."]
    pub mtpr: crate::Reg<mtpr::MTPR_SPEC>,
    #[doc = "0x810 - Master Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
    pub mimr: crate::Reg<mimr::MIMR_SPEC>,
    #[doc = "0x814 - Master Raw Interrupt Status This register show the unmasked interrupt status."]
    pub mris: crate::Reg<mris::MRIS_SPEC>,
    #[doc = "0x818 - Master Masked Interrupt Status This register show which interrupt is active (based on result from MRIS and MIMR)."]
    pub mmis: crate::Reg<mmis::MMIS_SPEC>,
    #[doc = "0x81c - Master Interrupt Clear This register clears the raw and masked interrupt."]
    pub micr: crate::Reg<micr::MICR_SPEC>,
    #[doc = "0x820 - Master Configuration This register configures the mode (Master or Slave) and sets the interface for test mode loopback."]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x04 - Slave Control Note: This register shares address with SSTAT, meaning that this register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub fn sctl(&self) -> &crate::Reg<sctl::SCTL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<sctl::SCTL_SPEC>)
        }
    }
    #[doc = "0x04 - Slave Status Note: This register shares address with SCTL, meaning that this register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub fn sstat(&self) -> &crate::Reg<sstat::SSTAT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<sstat::SSTAT_SPEC>)
        }
    }
    #[doc = "0x804 - Master Control This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller as stated in MSTAT. When written, the control register configures the I2C controller operation. To generate a single transmit cycle, the I2C Master Slave Address (MSA) register is written with the desired address, the MSA.RS bit is cleared, and this register is written with * ACK=X (0 or 1), * STOP=1, * START=1, * RUN=1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the MDR register."]
    #[inline(always)]
    pub fn mctrl(&self) -> &crate::Reg<mctrl::MCTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2052usize)
                as *const crate::Reg<mctrl::MCTRL_SPEC>)
        }
    }
    #[doc = "0x804 - Master Status"]
    #[inline(always)]
    pub fn mstat(&self) -> &crate::Reg<mstat::MSTAT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2052usize)
                as *const crate::Reg<mstat::MSTAT_SPEC>)
        }
    }
}
#[doc = "SOAR register accessor: an alias for `Reg<SOAR_SPEC>`"]
pub type SOAR = crate::Reg<soar::SOAR_SPEC>;
#[doc = "Slave Own Address This register consists of seven address bits that identify this I2C device on the I2C bus."]
pub mod soar;
#[doc = "SSTAT register accessor: an alias for `Reg<SSTAT_SPEC>`"]
pub type SSTAT = crate::Reg<sstat::SSTAT_SPEC>;
#[doc = "Slave Status Note: This register shares address with SCTL, meaning that this register functions as a control register when written, and a status register when read."]
pub mod sstat;
#[doc = "SCTL register accessor: an alias for `Reg<SCTL_SPEC>`"]
pub type SCTL = crate::Reg<sctl::SCTL_SPEC>;
#[doc = "Slave Control Note: This register shares address with SSTAT, meaning that this register functions as a control register when written, and a status register when read."]
pub mod sctl;
#[doc = "SDR register accessor: an alias for `Reg<SDR_SPEC>`"]
pub type SDR = crate::Reg<sdr::SDR_SPEC>;
#[doc = "Slave Data This register contains the data to be transmitted when in the Slave Transmit state, and the data received when in the Slave Receive state."]
pub mod sdr;
#[doc = "SIMR register accessor: an alias for `Reg<SIMR_SPEC>`"]
pub type SIMR = crate::Reg<simr::SIMR_SPEC>;
#[doc = "Slave Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
pub mod simr;
#[doc = "SRIS register accessor: an alias for `Reg<SRIS_SPEC>`"]
pub type SRIS = crate::Reg<sris::SRIS_SPEC>;
#[doc = "Slave Raw Interrupt Status This register shows the unmasked interrupt status."]
pub mod sris;
#[doc = "SMIS register accessor: an alias for `Reg<SMIS_SPEC>`"]
pub type SMIS = crate::Reg<smis::SMIS_SPEC>;
#[doc = "Slave Masked Interrupt Status This register show which interrupt is active (based on result from SRIS and SIMR)."]
pub mod smis;
#[doc = "SICR register accessor: an alias for `Reg<SICR_SPEC>`"]
pub type SICR = crate::Reg<sicr::SICR_SPEC>;
#[doc = "Slave Interrupt Clear This register clears the raw interrupt SRIS."]
pub mod sicr;
#[doc = "MSA register accessor: an alias for `Reg<MSA_SPEC>`"]
pub type MSA = crate::Reg<msa::MSA_SPEC>;
#[doc = "Master Salve Address This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit."]
pub mod msa;
#[doc = "MSTAT register accessor: an alias for `Reg<MSTAT_SPEC>`"]
pub type MSTAT = crate::Reg<mstat::MSTAT_SPEC>;
#[doc = "Master Status"]
pub mod mstat;
#[doc = "MCTRL register accessor: an alias for `Reg<MCTRL_SPEC>`"]
pub type MCTRL = crate::Reg<mctrl::MCTRL_SPEC>;
#[doc = "Master Control This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller as stated in MSTAT. When written, the control register configures the I2C controller operation. To generate a single transmit cycle, the I2C Master Slave Address (MSA) register is written with the desired address, the MSA.RS bit is cleared, and this register is written with * ACK=X (0 or 1), * STOP=1, * START=1, * RUN=1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the MDR register."]
pub mod mctrl;
#[doc = "MDR register accessor: an alias for `Reg<MDR_SPEC>`"]
pub type MDR = crate::Reg<mdr::MDR_SPEC>;
#[doc = "Master Data This register contains the data to be transmitted when in the Master Transmit state and the data received when in the Master Receive state."]
pub mod mdr;
#[doc = "MTPR register accessor: an alias for `Reg<MTPR_SPEC>`"]
pub type MTPR = crate::Reg<mtpr::MTPR_SPEC>;
#[doc = "I2C Master Timer Period This register specifies the period of the SCL clock."]
pub mod mtpr;
#[doc = "MIMR register accessor: an alias for `Reg<MIMR_SPEC>`"]
pub type MIMR = crate::Reg<mimr::MIMR_SPEC>;
#[doc = "Master Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
pub mod mimr;
#[doc = "MRIS register accessor: an alias for `Reg<MRIS_SPEC>`"]
pub type MRIS = crate::Reg<mris::MRIS_SPEC>;
#[doc = "Master Raw Interrupt Status This register show the unmasked interrupt status."]
pub mod mris;
#[doc = "MMIS register accessor: an alias for `Reg<MMIS_SPEC>`"]
pub type MMIS = crate::Reg<mmis::MMIS_SPEC>;
#[doc = "Master Masked Interrupt Status This register show which interrupt is active (based on result from MRIS and MIMR)."]
pub mod mmis;
#[doc = "MICR register accessor: an alias for `Reg<MICR_SPEC>`"]
pub type MICR = crate::Reg<micr::MICR_SPEC>;
#[doc = "Master Interrupt Clear This register clears the raw and masked interrupt."]
pub mod micr;
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Master Configuration This register configures the mode (Master or Slave) and sets the interface for test mode loopback."]
pub mod mcr;
