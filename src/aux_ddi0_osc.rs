#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control 0 Controls clock source selects"]
    pub ctl0: crate::Reg<ctl0::CTL0_SPEC>,
    #[doc = "0x04 - Control 1 This register contains OSC_DIG configuration"]
    pub ctl1: crate::Reg<ctl1::CTL1_SPEC>,
    #[doc = "0x08 - RADC External Configuration"]
    pub radcextcfg: crate::Reg<radcextcfg::RADCEXTCFG_SPEC>,
    #[doc = "0x0c - Amplitude Compensation Control"]
    pub ampcompctl: crate::Reg<ampcompctl::AMPCOMPCTL_SPEC>,
    #[doc = "0x10 - Amplitude Compensation Threshold 1 This register contains threshold values for amplitude compensation algorithm"]
    pub ampcompth1: crate::Reg<ampcompth1::AMPCOMPTH1_SPEC>,
    #[doc = "0x14 - Amplitude Compensation Threshold 2 This register contains threshold values for amplitude compensation algorithm."]
    pub ampcompth2: crate::Reg<ampcompth2::AMPCOMPTH2_SPEC>,
    #[doc = "0x18 - Analog Bypass Values 1"]
    pub anabypassval1: crate::Reg<anabypassval1::ANABYPASSVAL1_SPEC>,
    #[doc = "0x1c - Internal. Only to be used through TI provided API."]
    pub anabypassval2: crate::Reg<anabypassval2::ANABYPASSVAL2_SPEC>,
    #[doc = "0x20 - Analog Test Control"]
    pub atestctl: crate::Reg<atestctl::ATESTCTL_SPEC>,
    #[doc = "0x24 - ADC Doubler Nanoamp Control"]
    pub adcdoublernanoampctl: crate::Reg<adcdoublernanoampctl::ADCDOUBLERNANOAMPCTL_SPEC>,
    #[doc = "0x28 - XOSCHF Control"]
    pub xoschfctl: crate::Reg<xoschfctl::XOSCHFCTL_SPEC>,
    #[doc = "0x2c - Low Frequency Oscillator Control"]
    pub lfoscctl: crate::Reg<lfoscctl::LFOSCCTL_SPEC>,
    #[doc = "0x30 - RCOSCHF Control"]
    pub rcoschfctl: crate::Reg<rcoschfctl::RCOSCHFCTL_SPEC>,
    #[doc = "0x34 - Status 0 This register contains status signals from OSC_DIG"]
    pub stat0: crate::Reg<stat0::STAT0_SPEC>,
    #[doc = "0x38 - Status 1 This register contains status signals from OSC_DIG"]
    pub stat1: crate::Reg<stat1::STAT1_SPEC>,
    #[doc = "0x3c - Status 2 This register contains status signals from AMPCOMP FSM"]
    pub stat2: crate::Reg<stat2::STAT2_SPEC>,
}
#[doc = "CTL0 register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Control 0 Controls clock source selects"]
pub mod ctl0;
#[doc = "CTL1 register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "Control 1 This register contains OSC_DIG configuration"]
pub mod ctl1;
#[doc = "RADCEXTCFG register accessor: an alias for `Reg<RADCEXTCFG_SPEC>`"]
pub type RADCEXTCFG = crate::Reg<radcextcfg::RADCEXTCFG_SPEC>;
#[doc = "RADC External Configuration"]
pub mod radcextcfg;
#[doc = "AMPCOMPCTL register accessor: an alias for `Reg<AMPCOMPCTL_SPEC>`"]
pub type AMPCOMPCTL = crate::Reg<ampcompctl::AMPCOMPCTL_SPEC>;
#[doc = "Amplitude Compensation Control"]
pub mod ampcompctl;
#[doc = "AMPCOMPTH1 register accessor: an alias for `Reg<AMPCOMPTH1_SPEC>`"]
pub type AMPCOMPTH1 = crate::Reg<ampcompth1::AMPCOMPTH1_SPEC>;
#[doc = "Amplitude Compensation Threshold 1 This register contains threshold values for amplitude compensation algorithm"]
pub mod ampcompth1;
#[doc = "AMPCOMPTH2 register accessor: an alias for `Reg<AMPCOMPTH2_SPEC>`"]
pub type AMPCOMPTH2 = crate::Reg<ampcompth2::AMPCOMPTH2_SPEC>;
#[doc = "Amplitude Compensation Threshold 2 This register contains threshold values for amplitude compensation algorithm."]
pub mod ampcompth2;
#[doc = "ANABYPASSVAL1 register accessor: an alias for `Reg<ANABYPASSVAL1_SPEC>`"]
pub type ANABYPASSVAL1 = crate::Reg<anabypassval1::ANABYPASSVAL1_SPEC>;
#[doc = "Analog Bypass Values 1"]
pub mod anabypassval1;
#[doc = "ANABYPASSVAL2 register accessor: an alias for `Reg<ANABYPASSVAL2_SPEC>`"]
pub type ANABYPASSVAL2 = crate::Reg<anabypassval2::ANABYPASSVAL2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod anabypassval2;
#[doc = "ATESTCTL register accessor: an alias for `Reg<ATESTCTL_SPEC>`"]
pub type ATESTCTL = crate::Reg<atestctl::ATESTCTL_SPEC>;
#[doc = "Analog Test Control"]
pub mod atestctl;
#[doc = "ADCDOUBLERNANOAMPCTL register accessor: an alias for `Reg<ADCDOUBLERNANOAMPCTL_SPEC>`"]
pub type ADCDOUBLERNANOAMPCTL = crate::Reg<adcdoublernanoampctl::ADCDOUBLERNANOAMPCTL_SPEC>;
#[doc = "ADC Doubler Nanoamp Control"]
pub mod adcdoublernanoampctl;
#[doc = "XOSCHFCTL register accessor: an alias for `Reg<XOSCHFCTL_SPEC>`"]
pub type XOSCHFCTL = crate::Reg<xoschfctl::XOSCHFCTL_SPEC>;
#[doc = "XOSCHF Control"]
pub mod xoschfctl;
#[doc = "LFOSCCTL register accessor: an alias for `Reg<LFOSCCTL_SPEC>`"]
pub type LFOSCCTL = crate::Reg<lfoscctl::LFOSCCTL_SPEC>;
#[doc = "Low Frequency Oscillator Control"]
pub mod lfoscctl;
#[doc = "RCOSCHFCTL register accessor: an alias for `Reg<RCOSCHFCTL_SPEC>`"]
pub type RCOSCHFCTL = crate::Reg<rcoschfctl::RCOSCHFCTL_SPEC>;
#[doc = "RCOSCHF Control"]
pub mod rcoschfctl;
#[doc = "STAT0 register accessor: an alias for `Reg<STAT0_SPEC>`"]
pub type STAT0 = crate::Reg<stat0::STAT0_SPEC>;
#[doc = "Status 0 This register contains status signals from OSC_DIG"]
pub mod stat0;
#[doc = "STAT1 register accessor: an alias for `Reg<STAT1_SPEC>`"]
pub type STAT1 = crate::Reg<stat1::STAT1_SPEC>;
#[doc = "Status 1 This register contains status signals from OSC_DIG"]
pub mod stat1;
#[doc = "STAT2 register accessor: an alias for `Reg<STAT2_SPEC>`"]
pub type STAT2 = crate::Reg<stat2::STAT2_SPEC>;
#[doc = "Status 2 This register contains status signals from AMPCOMP FSM"]
pub mod stat2;
