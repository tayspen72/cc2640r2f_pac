#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Doorbell Command Register"]
    pub cmdr: crate::Reg<cmdr::CMDR_SPEC>,
    #[doc = "0x04 - Doorbell Command Status Register"]
    pub cmdsta: crate::Reg<cmdsta::CMDSTA_SPEC>,
    #[doc = "0x08 - Interrupt Flags From RF Hardware Modules"]
    pub rfhwifg: crate::Reg<rfhwifg::RFHWIFG_SPEC>,
    #[doc = "0x0c - Interrupt Enable For RF Hardware Modules"]
    pub rfhwien: crate::Reg<rfhwien::RFHWIEN_SPEC>,
    #[doc = "0x10 - Interrupt Flags For Command and Packet Engine Generated Interrupts"]
    pub rfcpeifg: crate::Reg<rfcpeifg::RFCPEIFG_SPEC>,
    #[doc = "0x14 - Interrupt Enable For Command and Packet Engine Generated Interrupts"]
    pub rfcpeien: crate::Reg<rfcpeien::RFCPEIEN_SPEC>,
    #[doc = "0x18 - Interrupt Vector Selection For Command and Packet Engine Generated Interrupts"]
    pub rfcpeisl: crate::Reg<rfcpeisl::RFCPEISL_SPEC>,
    #[doc = "0x1c - Doorbell Command Acknowledgement Interrupt Flag"]
    pub rfackifg: crate::Reg<rfackifg::RFACKIFG_SPEC>,
    #[doc = "0x20 - RF Core General Purpose Output Control"]
    pub sysgpoctl: crate::Reg<sysgpoctl::SYSGPOCTL_SPEC>,
}
#[doc = "CMDR register accessor: an alias for `Reg<CMDR_SPEC>`"]
pub type CMDR = crate::Reg<cmdr::CMDR_SPEC>;
#[doc = "Doorbell Command Register"]
pub mod cmdr;
#[doc = "CMDSTA register accessor: an alias for `Reg<CMDSTA_SPEC>`"]
pub type CMDSTA = crate::Reg<cmdsta::CMDSTA_SPEC>;
#[doc = "Doorbell Command Status Register"]
pub mod cmdsta;
#[doc = "RFHWIFG register accessor: an alias for `Reg<RFHWIFG_SPEC>`"]
pub type RFHWIFG = crate::Reg<rfhwifg::RFHWIFG_SPEC>;
#[doc = "Interrupt Flags From RF Hardware Modules"]
pub mod rfhwifg;
#[doc = "RFHWIEN register accessor: an alias for `Reg<RFHWIEN_SPEC>`"]
pub type RFHWIEN = crate::Reg<rfhwien::RFHWIEN_SPEC>;
#[doc = "Interrupt Enable For RF Hardware Modules"]
pub mod rfhwien;
#[doc = "RFCPEIFG register accessor: an alias for `Reg<RFCPEIFG_SPEC>`"]
pub type RFCPEIFG = crate::Reg<rfcpeifg::RFCPEIFG_SPEC>;
#[doc = "Interrupt Flags For Command and Packet Engine Generated Interrupts"]
pub mod rfcpeifg;
#[doc = "RFCPEIEN register accessor: an alias for `Reg<RFCPEIEN_SPEC>`"]
pub type RFCPEIEN = crate::Reg<rfcpeien::RFCPEIEN_SPEC>;
#[doc = "Interrupt Enable For Command and Packet Engine Generated Interrupts"]
pub mod rfcpeien;
#[doc = "RFCPEISL register accessor: an alias for `Reg<RFCPEISL_SPEC>`"]
pub type RFCPEISL = crate::Reg<rfcpeisl::RFCPEISL_SPEC>;
#[doc = "Interrupt Vector Selection For Command and Packet Engine Generated Interrupts"]
pub mod rfcpeisl;
#[doc = "RFACKIFG register accessor: an alias for `Reg<RFACKIFG_SPEC>`"]
pub type RFACKIFG = crate::Reg<rfackifg::RFACKIFG_SPEC>;
#[doc = "Doorbell Command Acknowledgement Interrupt Flag"]
pub mod rfackifg;
#[doc = "SYSGPOCTL register accessor: an alias for `Reg<SYSGPOCTL_SPEC>`"]
pub type SYSGPOCTL = crate::Reg<sysgpoctl::SYSGPOCTL_SPEC>;
#[doc = "RF Core General Purpose Output Control"]
pub mod sysgpoctl;
