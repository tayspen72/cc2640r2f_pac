#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status Displays current VIMS mode and line buffer status"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x04 - Control Configure VIMS mode and line buffer settings"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
}
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status Displays current VIMS mode and line buffer status"]
pub mod stat;
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control Configure VIMS mode and line buffer settings"]
pub mod ctl;
