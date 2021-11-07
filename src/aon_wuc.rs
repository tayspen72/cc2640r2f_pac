#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Clock Management This register contains bitfields related to the MCU clock."]
    pub mcuclk: crate::Reg<mcuclk::MCUCLK_SPEC>,
    #[doc = "0x04 - AUX Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
    pub auxclk: crate::Reg<auxclk::AUXCLK_SPEC>,
    #[doc = "0x08 - MCU Configuration This register contains power management related bitfields for the MCU domain."]
    pub mcucfg: crate::Reg<mcucfg::MCUCFG_SPEC>,
    #[doc = "0x0c - AUX Configuration This register contains power management related signals for the AUX domain."]
    pub auxcfg: crate::Reg<auxcfg::AUXCFG_SPEC>,
    #[doc = "0x10 - AUX Control This register contains events and control signals for the AUX domain."]
    pub auxctl: crate::Reg<auxctl::AUXCTL_SPEC>,
    #[doc = "0x14 - Power Status This register is used to monitor various power management related signals in AON. Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up."]
    pub pwrstat: crate::Reg<pwrstat::PWRSTAT_SPEC>,
    #[doc = "0x18 - Shutdown Control This register contains bitfields required for entering shutdown mode"]
    pub shutdown: crate::Reg<shutdown::SHUTDOWN_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Control 0 This register contains various chip level control and debug bitfields."]
    pub ctl0: crate::Reg<ctl0::CTL0_SPEC>,
    #[doc = "0x24 - Control 1 This register contains various chip level control and debug bitfields."]
    pub ctl1: crate::Reg<ctl1::CTL1_SPEC>,
    _reserved9: [u8; 0x08],
    #[doc = "0x30 - Recharge Controller Configuration This register sets all relevant patameters for controlling the recharge algorithm."]
    pub rechargecfg: crate::Reg<rechargecfg::RECHARGECFG_SPEC>,
    #[doc = "0x34 - Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
    pub rechargestat: crate::Reg<rechargestat::RECHARGESTAT_SPEC>,
    #[doc = "0x38 - Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
    pub osccfg: crate::Reg<osccfg::OSCCFG_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x40 - JTAG Configuration This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP."]
    pub jtagcfg: crate::Reg<jtagcfg::JTAGCFG_SPEC>,
    #[doc = "0x44 - JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
    pub jtagusercode: crate::Reg<jtagusercode::JTAGUSERCODE_SPEC>,
}
#[doc = "MCUCLK register accessor: an alias for `Reg<MCUCLK_SPEC>`"]
pub type MCUCLK = crate::Reg<mcuclk::MCUCLK_SPEC>;
#[doc = "MCU Clock Management This register contains bitfields related to the MCU clock."]
pub mod mcuclk;
#[doc = "AUXCLK register accessor: an alias for `Reg<AUXCLK_SPEC>`"]
pub type AUXCLK = crate::Reg<auxclk::AUXCLK_SPEC>;
#[doc = "AUX Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
pub mod auxclk;
#[doc = "MCUCFG register accessor: an alias for `Reg<MCUCFG_SPEC>`"]
pub type MCUCFG = crate::Reg<mcucfg::MCUCFG_SPEC>;
#[doc = "MCU Configuration This register contains power management related bitfields for the MCU domain."]
pub mod mcucfg;
#[doc = "AUXCFG register accessor: an alias for `Reg<AUXCFG_SPEC>`"]
pub type AUXCFG = crate::Reg<auxcfg::AUXCFG_SPEC>;
#[doc = "AUX Configuration This register contains power management related signals for the AUX domain."]
pub mod auxcfg;
#[doc = "AUXCTL register accessor: an alias for `Reg<AUXCTL_SPEC>`"]
pub type AUXCTL = crate::Reg<auxctl::AUXCTL_SPEC>;
#[doc = "AUX Control This register contains events and control signals for the AUX domain."]
pub mod auxctl;
#[doc = "PWRSTAT register accessor: an alias for `Reg<PWRSTAT_SPEC>`"]
pub type PWRSTAT = crate::Reg<pwrstat::PWRSTAT_SPEC>;
#[doc = "Power Status This register is used to monitor various power management related signals in AON. Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up."]
pub mod pwrstat;
#[doc = "SHUTDOWN register accessor: an alias for `Reg<SHUTDOWN_SPEC>`"]
pub type SHUTDOWN = crate::Reg<shutdown::SHUTDOWN_SPEC>;
#[doc = "Shutdown Control This register contains bitfields required for entering shutdown mode"]
pub mod shutdown;
#[doc = "CTL0 register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Control 0 This register contains various chip level control and debug bitfields."]
pub mod ctl0;
#[doc = "CTL1 register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "Control 1 This register contains various chip level control and debug bitfields."]
pub mod ctl1;
#[doc = "RECHARGECFG register accessor: an alias for `Reg<RECHARGECFG_SPEC>`"]
pub type RECHARGECFG = crate::Reg<rechargecfg::RECHARGECFG_SPEC>;
#[doc = "Recharge Controller Configuration This register sets all relevant patameters for controlling the recharge algorithm."]
pub mod rechargecfg;
#[doc = "RECHARGESTAT register accessor: an alias for `Reg<RECHARGESTAT_SPEC>`"]
pub type RECHARGESTAT = crate::Reg<rechargestat::RECHARGESTAT_SPEC>;
#[doc = "Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
pub mod rechargestat;
#[doc = "OSCCFG register accessor: an alias for `Reg<OSCCFG_SPEC>`"]
pub type OSCCFG = crate::Reg<osccfg::OSCCFG_SPEC>;
#[doc = "Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
pub mod osccfg;
#[doc = "JTAGCFG register accessor: an alias for `Reg<JTAGCFG_SPEC>`"]
pub type JTAGCFG = crate::Reg<jtagcfg::JTAGCFG_SPEC>;
#[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP."]
pub mod jtagcfg;
#[doc = "JTAGUSERCODE register accessor: an alias for `Reg<JTAGUSERCODE_SPEC>`"]
pub type JTAGUSERCODE = crate::Reg<jtagusercode::JTAGUSERCODE_SPEC>;
#[doc = "JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
pub mod jtagusercode;
