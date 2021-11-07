#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - Status"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x08 - Result Result of last TDC conversion"]
    pub result: crate::Reg<result::RESULT_SPEC>,
    #[doc = "0x0c - Saturation Configuration"]
    pub satcfg: crate::Reg<satcfg::SATCFG_SPEC>,
    #[doc = "0x10 - Trigger Source Select source and polarity for TDC start and stop events. See the Technical Reference Manual for event timing requirements."]
    pub trigsrc: crate::Reg<trigsrc::TRIGSRC_SPEC>,
    #[doc = "0x14 - Trigger Counter Stop-counter control and status."]
    pub trigcnt: crate::Reg<trigcnt::TRIGCNT_SPEC>,
    #[doc = "0x18 - Trigger Counter Load Stop-counter load."]
    pub trigcntload: crate::Reg<trigcntload::TRIGCNTLOAD_SPEC>,
    #[doc = "0x1c - Trigger Counter Configuration Stop-counter configuration."]
    pub trigcntcfg: crate::Reg<trigcntcfg::TRIGCNTCFG_SPEC>,
    #[doc = "0x20 - Prescaler Control The prescaler can be used to count events that are faster than the AUX clock frequency. It can be used to: - count pulses on a specified event from the asynchronous event bus. - prescale a specified event from the asynchronous event bus. To use the prescaler output as an event source in TDC measurements you must set both TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to AUX_TDC_PRE. It is recommended to use the prescaler when the signal frequency to measure exceeds 1/10th of the AUX clock frequency."]
    pub prectl: crate::Reg<prectl::PRECTL_SPEC>,
    #[doc = "0x24 - Prescaler Counter"]
    pub precnt: crate::Reg<precnt::PRECNT_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status"]
pub mod stat;
#[doc = "RESULT register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Result Result of last TDC conversion"]
pub mod result;
#[doc = "SATCFG register accessor: an alias for `Reg<SATCFG_SPEC>`"]
pub type SATCFG = crate::Reg<satcfg::SATCFG_SPEC>;
#[doc = "Saturation Configuration"]
pub mod satcfg;
#[doc = "TRIGSRC register accessor: an alias for `Reg<TRIGSRC_SPEC>`"]
pub type TRIGSRC = crate::Reg<trigsrc::TRIGSRC_SPEC>;
#[doc = "Trigger Source Select source and polarity for TDC start and stop events. See the Technical Reference Manual for event timing requirements."]
pub mod trigsrc;
#[doc = "TRIGCNT register accessor: an alias for `Reg<TRIGCNT_SPEC>`"]
pub type TRIGCNT = crate::Reg<trigcnt::TRIGCNT_SPEC>;
#[doc = "Trigger Counter Stop-counter control and status."]
pub mod trigcnt;
#[doc = "TRIGCNTLOAD register accessor: an alias for `Reg<TRIGCNTLOAD_SPEC>`"]
pub type TRIGCNTLOAD = crate::Reg<trigcntload::TRIGCNTLOAD_SPEC>;
#[doc = "Trigger Counter Load Stop-counter load."]
pub mod trigcntload;
#[doc = "TRIGCNTCFG register accessor: an alias for `Reg<TRIGCNTCFG_SPEC>`"]
pub type TRIGCNTCFG = crate::Reg<trigcntcfg::TRIGCNTCFG_SPEC>;
#[doc = "Trigger Counter Configuration Stop-counter configuration."]
pub mod trigcntcfg;
#[doc = "PRECTL register accessor: an alias for `Reg<PRECTL_SPEC>`"]
pub type PRECTL = crate::Reg<prectl::PRECTL_SPEC>;
#[doc = "Prescaler Control The prescaler can be used to count events that are faster than the AUX clock frequency. It can be used to: - count pulses on a specified event from the asynchronous event bus. - prescale a specified event from the asynchronous event bus. To use the prescaler output as an event source in TDC measurements you must set both TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to AUX_TDC_PRE. It is recommended to use the prescaler when the signal frequency to measure exceeds 1/10th of the AUX clock frequency."]
pub mod prectl;
#[doc = "PRECNT register accessor: an alias for `Reg<PRECNT_SPEC>`"]
pub type PRECNT = crate::Reg<precnt::PRECNT_SPEC>;
#[doc = "Prescaler Counter"]
pub mod precnt;
