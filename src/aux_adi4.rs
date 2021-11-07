#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    pub mux0: crate::Reg<mux0::MUX0_SPEC>,
    #[doc = "0x01 - Internal. Only to be used through TI provided API."]
    pub mux1: crate::Reg<mux1::MUX1_SPEC>,
    #[doc = "0x02 - Internal. Only to be used through TI provided API."]
    pub mux2: crate::Reg<mux2::MUX2_SPEC>,
    #[doc = "0x03 - Internal. Only to be used through TI provided API."]
    pub mux3: crate::Reg<mux3::MUX3_SPEC>,
    #[doc = "0x04 - Current Source Strength and trim control for current source. Only to be used through TI provided API."]
    pub isrc: crate::Reg<isrc::ISRC_SPEC>,
    #[doc = "0x05 - Comparator Control COMPA and COMPB comparators. Only to be used through TI provided API."]
    pub comp: crate::Reg<comp::COMP_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x07 - Internal. Only to be used through TI provided API."]
    pub mux4: crate::Reg<mux4::MUX4_SPEC>,
    #[doc = "0x08 - ADC Control 0 ADC Sample Control. Only to be used through TI provided API."]
    pub adc0: crate::Reg<adc0::ADC0_SPEC>,
    #[doc = "0x09 - ADC Control 1 ADC Comparator Control. Only to be used through TI provided API."]
    pub adc1: crate::Reg<adc1::ADC1_SPEC>,
    #[doc = "0x0a - ADC Reference 0 Control reference used by the ADC. Only to be used through TI provided API."]
    pub adcref0: crate::Reg<adcref0::ADCREF0_SPEC>,
    #[doc = "0x0b - ADC Reference 1 Control reference used by the ADC. Only to be used through TI provided API."]
    pub adcref1: crate::Reg<adcref1::ADCREF1_SPEC>,
}
#[doc = "MUX0 register accessor: an alias for `Reg<MUX0_SPEC>`"]
pub type MUX0 = crate::Reg<mux0::MUX0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux0;
#[doc = "MUX1 register accessor: an alias for `Reg<MUX1_SPEC>`"]
pub type MUX1 = crate::Reg<mux1::MUX1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux1;
#[doc = "MUX2 register accessor: an alias for `Reg<MUX2_SPEC>`"]
pub type MUX2 = crate::Reg<mux2::MUX2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux2;
#[doc = "MUX3 register accessor: an alias for `Reg<MUX3_SPEC>`"]
pub type MUX3 = crate::Reg<mux3::MUX3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux3;
#[doc = "ISRC register accessor: an alias for `Reg<ISRC_SPEC>`"]
pub type ISRC = crate::Reg<isrc::ISRC_SPEC>;
#[doc = "Current Source Strength and trim control for current source. Only to be used through TI provided API."]
pub mod isrc;
#[doc = "COMP register accessor: an alias for `Reg<COMP_SPEC>`"]
pub type COMP = crate::Reg<comp::COMP_SPEC>;
#[doc = "Comparator Control COMPA and COMPB comparators. Only to be used through TI provided API."]
pub mod comp;
#[doc = "MUX4 register accessor: an alias for `Reg<MUX4_SPEC>`"]
pub type MUX4 = crate::Reg<mux4::MUX4_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux4;
#[doc = "ADC0 register accessor: an alias for `Reg<ADC0_SPEC>`"]
pub type ADC0 = crate::Reg<adc0::ADC0_SPEC>;
#[doc = "ADC Control 0 ADC Sample Control. Only to be used through TI provided API."]
pub mod adc0;
#[doc = "ADC1 register accessor: an alias for `Reg<ADC1_SPEC>`"]
pub type ADC1 = crate::Reg<adc1::ADC1_SPEC>;
#[doc = "ADC Control 1 ADC Comparator Control. Only to be used through TI provided API."]
pub mod adc1;
#[doc = "ADCREF0 register accessor: an alias for `Reg<ADCREF0_SPEC>`"]
pub type ADCREF0 = crate::Reg<adcref0::ADCREF0_SPEC>;
#[doc = "ADC Reference 0 Control reference used by the ADC. Only to be used through TI provided API."]
pub mod adcref0;
#[doc = "ADCREF1 register accessor: an alias for `Reg<ADCREF1_SPEC>`"]
pub type ADCREF1 = crate::Reg<adcref1::ADCREF1_SPEC>;
#[doc = "ADC Reference 1 Control reference used by the ADC. Only to be used through TI provided API."]
pub mod adcref1;
