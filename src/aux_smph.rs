#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Semaphore 0"]
    pub smph0: crate::Reg<smph0::SMPH0_SPEC>,
    #[doc = "0x04 - Semaphore 1"]
    pub smph1: crate::Reg<smph1::SMPH1_SPEC>,
    #[doc = "0x08 - Semaphore 2"]
    pub smph2: crate::Reg<smph2::SMPH2_SPEC>,
    #[doc = "0x0c - Semaphore 3"]
    pub smph3: crate::Reg<smph3::SMPH3_SPEC>,
    #[doc = "0x10 - Semaphore 4"]
    pub smph4: crate::Reg<smph4::SMPH4_SPEC>,
    #[doc = "0x14 - Semaphore 5"]
    pub smph5: crate::Reg<smph5::SMPH5_SPEC>,
    #[doc = "0x18 - Semaphore 6"]
    pub smph6: crate::Reg<smph6::SMPH6_SPEC>,
    #[doc = "0x1c - Semaphore 7"]
    pub smph7: crate::Reg<smph7::SMPH7_SPEC>,
    #[doc = "0x20 - Auto Take Sticky Request for Single Semaphore."]
    pub autotake: crate::Reg<autotake::AUTOTAKE_SPEC>,
}
#[doc = "SMPH0 register accessor: an alias for `Reg<SMPH0_SPEC>`"]
pub type SMPH0 = crate::Reg<smph0::SMPH0_SPEC>;
#[doc = "Semaphore 0"]
pub mod smph0;
#[doc = "SMPH1 register accessor: an alias for `Reg<SMPH1_SPEC>`"]
pub type SMPH1 = crate::Reg<smph1::SMPH1_SPEC>;
#[doc = "Semaphore 1"]
pub mod smph1;
#[doc = "SMPH2 register accessor: an alias for `Reg<SMPH2_SPEC>`"]
pub type SMPH2 = crate::Reg<smph2::SMPH2_SPEC>;
#[doc = "Semaphore 2"]
pub mod smph2;
#[doc = "SMPH3 register accessor: an alias for `Reg<SMPH3_SPEC>`"]
pub type SMPH3 = crate::Reg<smph3::SMPH3_SPEC>;
#[doc = "Semaphore 3"]
pub mod smph3;
#[doc = "SMPH4 register accessor: an alias for `Reg<SMPH4_SPEC>`"]
pub type SMPH4 = crate::Reg<smph4::SMPH4_SPEC>;
#[doc = "Semaphore 4"]
pub mod smph4;
#[doc = "SMPH5 register accessor: an alias for `Reg<SMPH5_SPEC>`"]
pub type SMPH5 = crate::Reg<smph5::SMPH5_SPEC>;
#[doc = "Semaphore 5"]
pub mod smph5;
#[doc = "SMPH6 register accessor: an alias for `Reg<SMPH6_SPEC>`"]
pub type SMPH6 = crate::Reg<smph6::SMPH6_SPEC>;
#[doc = "Semaphore 6"]
pub mod smph6;
#[doc = "SMPH7 register accessor: an alias for `Reg<SMPH7_SPEC>`"]
pub type SMPH7 = crate::Reg<smph7::SMPH7_SPEC>;
#[doc = "Semaphore 7"]
pub mod smph7;
#[doc = "AUTOTAKE register accessor: an alias for `Reg<AUTOTAKE_SPEC>`"]
pub type AUTOTAKE = crate::Reg<autotake::AUTOTAKE_SPEC>;
#[doc = "Auto Take Sticky Request for Single Semaphore."]
pub mod autotake;
