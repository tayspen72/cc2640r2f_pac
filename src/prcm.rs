#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Infrastructure Clock Division Factor For Run Mode"]
    pub infrclkdivr: crate::Reg<infrclkdivr::INFRCLKDIVR_SPEC>,
    #[doc = "0x04 - Infrastructure Clock Division Factor For Sleep Mode"]
    pub infrclkdivs: crate::Reg<infrclkdivs::INFRCLKDIVS_SPEC>,
    #[doc = "0x08 - Infrastructure Clock Division Factor For DeepSleep Mode"]
    pub infrclkdivds: crate::Reg<infrclkdivds::INFRCLKDIVDS_SPEC>,
    #[doc = "0x0c - MCU Voltage Domain Control"]
    pub vdctl: crate::Reg<vdctl::VDCTL_SPEC>,
    _reserved4: [u8; 0x18],
    #[doc = "0x28 - Load PRCM Settings To CLKCTRL Power Domain"]
    pub clkloadctl: crate::Reg<clkloadctl::CLKLOADCTL_SPEC>,
    #[doc = "0x2c - RFC Clock Gate"]
    pub rfcclkg: crate::Reg<rfcclkg::RFCCLKG_SPEC>,
    #[doc = "0x30 - VIMS Clock Gate"]
    pub vimsclkg: crate::Reg<vimsclkg::VIMSCLKG_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x3c - TRNG, CRYPTO And UDMA Clock Gate For Run Mode"]
    pub secdmaclkgr: crate::Reg<secdmaclkgr::SECDMACLKGR_SPEC>,
    #[doc = "0x40 - TRNG, CRYPTO And UDMA Clock Gate For Sleep Mode"]
    pub secdmaclkgs: crate::Reg<secdmaclkgs::SECDMACLKGS_SPEC>,
    #[doc = "0x44 - TRNG, CRYPTO And UDMA Clock Gate For Deep Sleep Mode"]
    pub secdmaclkgds: crate::Reg<secdmaclkgds::SECDMACLKGDS_SPEC>,
    #[doc = "0x48 - GPIO Clock Gate For Run Mode"]
    pub gpioclkgr: crate::Reg<gpioclkgr::GPIOCLKGR_SPEC>,
    #[doc = "0x4c - GPIO Clock Gate For Sleep Mode"]
    pub gpioclkgs: crate::Reg<gpioclkgs::GPIOCLKGS_SPEC>,
    #[doc = "0x50 - GPIO Clock Gate For Deep Sleep Mode"]
    pub gpioclkgds: crate::Reg<gpioclkgds::GPIOCLKGDS_SPEC>,
    #[doc = "0x54 - GPT Clock Gate For Run Mode"]
    pub gptclkgr: crate::Reg<gptclkgr::GPTCLKGR_SPEC>,
    #[doc = "0x58 - GPT Clock Gate For Sleep Mode"]
    pub gptclkgs: crate::Reg<gptclkgs::GPTCLKGS_SPEC>,
    #[doc = "0x5c - GPT Clock Gate For Deep Sleep Mode"]
    pub gptclkgds: crate::Reg<gptclkgds::GPTCLKGDS_SPEC>,
    #[doc = "0x60 - I2C Clock Gate For Run Mode"]
    pub i2cclkgr: crate::Reg<i2cclkgr::I2CCLKGR_SPEC>,
    #[doc = "0x64 - I2C Clock Gate For Sleep Mode"]
    pub i2cclkgs: crate::Reg<i2cclkgs::I2CCLKGS_SPEC>,
    #[doc = "0x68 - I2C Clock Gate For Deep Sleep Mode"]
    pub i2cclkgds: crate::Reg<i2cclkgds::I2CCLKGDS_SPEC>,
    #[doc = "0x6c - UART Clock Gate For Run Mode"]
    pub uartclkgr: crate::Reg<uartclkgr::UARTCLKGR_SPEC>,
    #[doc = "0x70 - UART Clock Gate For Sleep Mode"]
    pub uartclkgs: crate::Reg<uartclkgs::UARTCLKGS_SPEC>,
    #[doc = "0x74 - UART Clock Gate For Deep Sleep Mode"]
    pub uartclkgds: crate::Reg<uartclkgds::UARTCLKGDS_SPEC>,
    #[doc = "0x78 - SSI Clock Gate For Run Mode"]
    pub ssiclkgr: crate::Reg<ssiclkgr::SSICLKGR_SPEC>,
    #[doc = "0x7c - SSI Clock Gate For Sleep Mode"]
    pub ssiclkgs: crate::Reg<ssiclkgs::SSICLKGS_SPEC>,
    #[doc = "0x80 - SSI Clock Gate For Deep Sleep Mode"]
    pub ssiclkgds: crate::Reg<ssiclkgds::SSICLKGDS_SPEC>,
    #[doc = "0x84 - I2S Clock Gate For Run Mode"]
    pub i2sclkgr: crate::Reg<i2sclkgr::I2SCLKGR_SPEC>,
    #[doc = "0x88 - I2S Clock Gate For Sleep Mode"]
    pub i2sclkgs: crate::Reg<i2sclkgs::I2SCLKGS_SPEC>,
    #[doc = "0x8c - I2S Clock Gate For Deep Sleep Mode"]
    pub i2sclkgds: crate::Reg<i2sclkgds::I2SCLKGDS_SPEC>,
    _reserved28: [u8; 0x28],
    #[doc = "0xb8 - Internal. Only to be used through TI provided API."]
    pub cpuclkdiv: crate::Reg<cpuclkdiv::CPUCLKDIV_SPEC>,
    _reserved29: [u8; 0x04],
    #[doc = "0xc0 - Internal. Only to be used through TI provided API."]
    pub perbusdmaclkdiv: crate::Reg<perbusdmaclkdiv::PERBUSDMACLKDIV_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0xc8 - I2S Clock Control"]
    pub i2sbclksel: crate::Reg<i2sbclksel::I2SBCLKSEL_SPEC>,
    #[doc = "0xcc - GPT Scalar"]
    pub gptclkdiv: crate::Reg<gptclkdiv::GPTCLKDIV_SPEC>,
    #[doc = "0xd0 - I2S Clock Control"]
    pub i2sclkctl: crate::Reg<i2sclkctl::I2SCLKCTL_SPEC>,
    #[doc = "0xd4 - MCLK Division Ratio"]
    pub i2smclkdiv: crate::Reg<i2smclkdiv::I2SMCLKDIV_SPEC>,
    #[doc = "0xd8 - BCLK Division Ratio"]
    pub i2sbclkdiv: crate::Reg<i2sbclkdiv::I2SBCLKDIV_SPEC>,
    #[doc = "0xdc - WCLK Division Ratio"]
    pub i2swclkdiv: crate::Reg<i2swclkdiv::I2SWCLKDIV_SPEC>,
    _reserved36: [u8; 0x2c],
    #[doc = "0x10c - SW Initiated Resets"]
    pub swreset: crate::Reg<swreset::SWRESET_SPEC>,
    #[doc = "0x110 - WARM Reset Control And Status"]
    pub warmreset: crate::Reg<warmreset::WARMRESET_SPEC>,
    _reserved38: [u8; 0x18],
    #[doc = "0x12c - Power Domain Control"]
    pub pdctl0: crate::Reg<pdctl0::PDCTL0_SPEC>,
    #[doc = "0x130 - RFC Power Domain Control"]
    pub pdctl0rfc: crate::Reg<pdctl0rfc::PDCTL0RFC_SPEC>,
    #[doc = "0x134 - SERIAL Power Domain Control"]
    pub pdctl0serial: crate::Reg<pdctl0serial::PDCTL0SERIAL_SPEC>,
    #[doc = "0x138 - PERIPH Power Domain Control"]
    pub pdctl0periph: crate::Reg<pdctl0periph::PDCTL0PERIPH_SPEC>,
    _reserved42: [u8; 0x04],
    #[doc = "0x140 - Power Domain Status"]
    pub pdstat0: crate::Reg<pdstat0::PDSTAT0_SPEC>,
    #[doc = "0x144 - RFC Power Domain Status"]
    pub pdstat0rfc: crate::Reg<pdstat0rfc::PDSTAT0RFC_SPEC>,
    #[doc = "0x148 - SERIAL Power Domain Status"]
    pub pdstat0serial: crate::Reg<pdstat0serial::PDSTAT0SERIAL_SPEC>,
    #[doc = "0x14c - PERIPH Power Domain Status"]
    pub pdstat0periph: crate::Reg<pdstat0periph::PDSTAT0PERIPH_SPEC>,
    _reserved46: [u8; 0x2c],
    #[doc = "0x17c - Power Domain Control"]
    pub pdctl1: crate::Reg<pdctl1::PDCTL1_SPEC>,
    _reserved47: [u8; 0x04],
    #[doc = "0x184 - CPU Power Domain Direct Control"]
    pub pdctl1cpu: crate::Reg<pdctl1cpu::PDCTL1CPU_SPEC>,
    #[doc = "0x188 - RFC Power Domain Direct Control"]
    pub pdctl1rfc: crate::Reg<pdctl1rfc::PDCTL1RFC_SPEC>,
    #[doc = "0x18c - VIMS Mode Direct Control"]
    pub pdctl1vims: crate::Reg<pdctl1vims::PDCTL1VIMS_SPEC>,
    _reserved50: [u8; 0x04],
    #[doc = "0x194 - Power Manager Status"]
    pub pdstat1: crate::Reg<pdstat1::PDSTAT1_SPEC>,
    #[doc = "0x198 - BUS Power Domain Direct Read Status"]
    pub pdstat1bus: crate::Reg<pdstat1bus::PDSTAT1BUS_SPEC>,
    #[doc = "0x19c - RFC Power Domain Direct Read Status"]
    pub pdstat1rfc: crate::Reg<pdstat1rfc::PDSTAT1RFC_SPEC>,
    #[doc = "0x1a0 - CPU Power Domain Direct Read Status"]
    pub pdstat1cpu: crate::Reg<pdstat1cpu::PDSTAT1CPU_SPEC>,
    #[doc = "0x1a4 - VIMS Mode Direct Read Status"]
    pub pdstat1vims: crate::Reg<pdstat1vims::PDSTAT1VIMS_SPEC>,
    _reserved55: [u8; 0x24],
    #[doc = "0x1cc - Control To RFC"]
    pub rfcbits: crate::Reg<rfcbits::RFCBITS_SPEC>,
    #[doc = "0x1d0 - Selected RFC Mode"]
    pub rfcmodesel: crate::Reg<rfcmodesel::RFCMODESEL_SPEC>,
    #[doc = "0x1d4 - Allowed RFC Modes"]
    pub rfcmodehwopt: crate::Reg<rfcmodehwopt::RFCMODEHWOPT_SPEC>,
    _reserved58: [u8; 0x08],
    #[doc = "0x1e0 - Power Profiler Register"]
    pub pwrprofstat: crate::Reg<pwrprofstat::PWRPROFSTAT_SPEC>,
    _reserved59: [u8; 0x40],
    #[doc = "0x224 - Memory Retention Control"]
    pub ramreten: crate::Reg<ramreten::RAMRETEN_SPEC>,
}
#[doc = "INFRCLKDIVR register accessor: an alias for `Reg<INFRCLKDIVR_SPEC>`"]
pub type INFRCLKDIVR = crate::Reg<infrclkdivr::INFRCLKDIVR_SPEC>;
#[doc = "Infrastructure Clock Division Factor For Run Mode"]
pub mod infrclkdivr;
#[doc = "INFRCLKDIVS register accessor: an alias for `Reg<INFRCLKDIVS_SPEC>`"]
pub type INFRCLKDIVS = crate::Reg<infrclkdivs::INFRCLKDIVS_SPEC>;
#[doc = "Infrastructure Clock Division Factor For Sleep Mode"]
pub mod infrclkdivs;
#[doc = "INFRCLKDIVDS register accessor: an alias for `Reg<INFRCLKDIVDS_SPEC>`"]
pub type INFRCLKDIVDS = crate::Reg<infrclkdivds::INFRCLKDIVDS_SPEC>;
#[doc = "Infrastructure Clock Division Factor For DeepSleep Mode"]
pub mod infrclkdivds;
#[doc = "VDCTL register accessor: an alias for `Reg<VDCTL_SPEC>`"]
pub type VDCTL = crate::Reg<vdctl::VDCTL_SPEC>;
#[doc = "MCU Voltage Domain Control"]
pub mod vdctl;
#[doc = "CLKLOADCTL register accessor: an alias for `Reg<CLKLOADCTL_SPEC>`"]
pub type CLKLOADCTL = crate::Reg<clkloadctl::CLKLOADCTL_SPEC>;
#[doc = "Load PRCM Settings To CLKCTRL Power Domain"]
pub mod clkloadctl;
#[doc = "RFCCLKG register accessor: an alias for `Reg<RFCCLKG_SPEC>`"]
pub type RFCCLKG = crate::Reg<rfcclkg::RFCCLKG_SPEC>;
#[doc = "RFC Clock Gate"]
pub mod rfcclkg;
#[doc = "VIMSCLKG register accessor: an alias for `Reg<VIMSCLKG_SPEC>`"]
pub type VIMSCLKG = crate::Reg<vimsclkg::VIMSCLKG_SPEC>;
#[doc = "VIMS Clock Gate"]
pub mod vimsclkg;
#[doc = "SECDMACLKGR register accessor: an alias for `Reg<SECDMACLKGR_SPEC>`"]
pub type SECDMACLKGR = crate::Reg<secdmaclkgr::SECDMACLKGR_SPEC>;
#[doc = "TRNG, CRYPTO And UDMA Clock Gate For Run Mode"]
pub mod secdmaclkgr;
#[doc = "SECDMACLKGS register accessor: an alias for `Reg<SECDMACLKGS_SPEC>`"]
pub type SECDMACLKGS = crate::Reg<secdmaclkgs::SECDMACLKGS_SPEC>;
#[doc = "TRNG, CRYPTO And UDMA Clock Gate For Sleep Mode"]
pub mod secdmaclkgs;
#[doc = "SECDMACLKGDS register accessor: an alias for `Reg<SECDMACLKGDS_SPEC>`"]
pub type SECDMACLKGDS = crate::Reg<secdmaclkgds::SECDMACLKGDS_SPEC>;
#[doc = "TRNG, CRYPTO And UDMA Clock Gate For Deep Sleep Mode"]
pub mod secdmaclkgds;
#[doc = "GPIOCLKGR register accessor: an alias for `Reg<GPIOCLKGR_SPEC>`"]
pub type GPIOCLKGR = crate::Reg<gpioclkgr::GPIOCLKGR_SPEC>;
#[doc = "GPIO Clock Gate For Run Mode"]
pub mod gpioclkgr;
#[doc = "GPIOCLKGS register accessor: an alias for `Reg<GPIOCLKGS_SPEC>`"]
pub type GPIOCLKGS = crate::Reg<gpioclkgs::GPIOCLKGS_SPEC>;
#[doc = "GPIO Clock Gate For Sleep Mode"]
pub mod gpioclkgs;
#[doc = "GPIOCLKGDS register accessor: an alias for `Reg<GPIOCLKGDS_SPEC>`"]
pub type GPIOCLKGDS = crate::Reg<gpioclkgds::GPIOCLKGDS_SPEC>;
#[doc = "GPIO Clock Gate For Deep Sleep Mode"]
pub mod gpioclkgds;
#[doc = "GPTCLKGR register accessor: an alias for `Reg<GPTCLKGR_SPEC>`"]
pub type GPTCLKGR = crate::Reg<gptclkgr::GPTCLKGR_SPEC>;
#[doc = "GPT Clock Gate For Run Mode"]
pub mod gptclkgr;
#[doc = "GPTCLKGS register accessor: an alias for `Reg<GPTCLKGS_SPEC>`"]
pub type GPTCLKGS = crate::Reg<gptclkgs::GPTCLKGS_SPEC>;
#[doc = "GPT Clock Gate For Sleep Mode"]
pub mod gptclkgs;
#[doc = "GPTCLKGDS register accessor: an alias for `Reg<GPTCLKGDS_SPEC>`"]
pub type GPTCLKGDS = crate::Reg<gptclkgds::GPTCLKGDS_SPEC>;
#[doc = "GPT Clock Gate For Deep Sleep Mode"]
pub mod gptclkgds;
#[doc = "I2CCLKGR register accessor: an alias for `Reg<I2CCLKGR_SPEC>`"]
pub type I2CCLKGR = crate::Reg<i2cclkgr::I2CCLKGR_SPEC>;
#[doc = "I2C Clock Gate For Run Mode"]
pub mod i2cclkgr;
#[doc = "I2CCLKGS register accessor: an alias for `Reg<I2CCLKGS_SPEC>`"]
pub type I2CCLKGS = crate::Reg<i2cclkgs::I2CCLKGS_SPEC>;
#[doc = "I2C Clock Gate For Sleep Mode"]
pub mod i2cclkgs;
#[doc = "I2CCLKGDS register accessor: an alias for `Reg<I2CCLKGDS_SPEC>`"]
pub type I2CCLKGDS = crate::Reg<i2cclkgds::I2CCLKGDS_SPEC>;
#[doc = "I2C Clock Gate For Deep Sleep Mode"]
pub mod i2cclkgds;
#[doc = "UARTCLKGR register accessor: an alias for `Reg<UARTCLKGR_SPEC>`"]
pub type UARTCLKGR = crate::Reg<uartclkgr::UARTCLKGR_SPEC>;
#[doc = "UART Clock Gate For Run Mode"]
pub mod uartclkgr;
#[doc = "UARTCLKGS register accessor: an alias for `Reg<UARTCLKGS_SPEC>`"]
pub type UARTCLKGS = crate::Reg<uartclkgs::UARTCLKGS_SPEC>;
#[doc = "UART Clock Gate For Sleep Mode"]
pub mod uartclkgs;
#[doc = "UARTCLKGDS register accessor: an alias for `Reg<UARTCLKGDS_SPEC>`"]
pub type UARTCLKGDS = crate::Reg<uartclkgds::UARTCLKGDS_SPEC>;
#[doc = "UART Clock Gate For Deep Sleep Mode"]
pub mod uartclkgds;
#[doc = "SSICLKGR register accessor: an alias for `Reg<SSICLKGR_SPEC>`"]
pub type SSICLKGR = crate::Reg<ssiclkgr::SSICLKGR_SPEC>;
#[doc = "SSI Clock Gate For Run Mode"]
pub mod ssiclkgr;
#[doc = "SSICLKGS register accessor: an alias for `Reg<SSICLKGS_SPEC>`"]
pub type SSICLKGS = crate::Reg<ssiclkgs::SSICLKGS_SPEC>;
#[doc = "SSI Clock Gate For Sleep Mode"]
pub mod ssiclkgs;
#[doc = "SSICLKGDS register accessor: an alias for `Reg<SSICLKGDS_SPEC>`"]
pub type SSICLKGDS = crate::Reg<ssiclkgds::SSICLKGDS_SPEC>;
#[doc = "SSI Clock Gate For Deep Sleep Mode"]
pub mod ssiclkgds;
#[doc = "I2SCLKGR register accessor: an alias for `Reg<I2SCLKGR_SPEC>`"]
pub type I2SCLKGR = crate::Reg<i2sclkgr::I2SCLKGR_SPEC>;
#[doc = "I2S Clock Gate For Run Mode"]
pub mod i2sclkgr;
#[doc = "I2SCLKGS register accessor: an alias for `Reg<I2SCLKGS_SPEC>`"]
pub type I2SCLKGS = crate::Reg<i2sclkgs::I2SCLKGS_SPEC>;
#[doc = "I2S Clock Gate For Sleep Mode"]
pub mod i2sclkgs;
#[doc = "I2SCLKGDS register accessor: an alias for `Reg<I2SCLKGDS_SPEC>`"]
pub type I2SCLKGDS = crate::Reg<i2sclkgds::I2SCLKGDS_SPEC>;
#[doc = "I2S Clock Gate For Deep Sleep Mode"]
pub mod i2sclkgds;
#[doc = "CPUCLKDIV register accessor: an alias for `Reg<CPUCLKDIV_SPEC>`"]
pub type CPUCLKDIV = crate::Reg<cpuclkdiv::CPUCLKDIV_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cpuclkdiv;
#[doc = "PERBUSDMACLKDIV register accessor: an alias for `Reg<PERBUSDMACLKDIV_SPEC>`"]
pub type PERBUSDMACLKDIV = crate::Reg<perbusdmaclkdiv::PERBUSDMACLKDIV_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod perbusdmaclkdiv;
#[doc = "I2SBCLKSEL register accessor: an alias for `Reg<I2SBCLKSEL_SPEC>`"]
pub type I2SBCLKSEL = crate::Reg<i2sbclksel::I2SBCLKSEL_SPEC>;
#[doc = "I2S Clock Control"]
pub mod i2sbclksel;
#[doc = "GPTCLKDIV register accessor: an alias for `Reg<GPTCLKDIV_SPEC>`"]
pub type GPTCLKDIV = crate::Reg<gptclkdiv::GPTCLKDIV_SPEC>;
#[doc = "GPT Scalar"]
pub mod gptclkdiv;
#[doc = "I2SCLKCTL register accessor: an alias for `Reg<I2SCLKCTL_SPEC>`"]
pub type I2SCLKCTL = crate::Reg<i2sclkctl::I2SCLKCTL_SPEC>;
#[doc = "I2S Clock Control"]
pub mod i2sclkctl;
#[doc = "I2SMCLKDIV register accessor: an alias for `Reg<I2SMCLKDIV_SPEC>`"]
pub type I2SMCLKDIV = crate::Reg<i2smclkdiv::I2SMCLKDIV_SPEC>;
#[doc = "MCLK Division Ratio"]
pub mod i2smclkdiv;
#[doc = "I2SBCLKDIV register accessor: an alias for `Reg<I2SBCLKDIV_SPEC>`"]
pub type I2SBCLKDIV = crate::Reg<i2sbclkdiv::I2SBCLKDIV_SPEC>;
#[doc = "BCLK Division Ratio"]
pub mod i2sbclkdiv;
#[doc = "I2SWCLKDIV register accessor: an alias for `Reg<I2SWCLKDIV_SPEC>`"]
pub type I2SWCLKDIV = crate::Reg<i2swclkdiv::I2SWCLKDIV_SPEC>;
#[doc = "WCLK Division Ratio"]
pub mod i2swclkdiv;
#[doc = "SWRESET register accessor: an alias for `Reg<SWRESET_SPEC>`"]
pub type SWRESET = crate::Reg<swreset::SWRESET_SPEC>;
#[doc = "SW Initiated Resets"]
pub mod swreset;
#[doc = "WARMRESET register accessor: an alias for `Reg<WARMRESET_SPEC>`"]
pub type WARMRESET = crate::Reg<warmreset::WARMRESET_SPEC>;
#[doc = "WARM Reset Control And Status"]
pub mod warmreset;
#[doc = "PDCTL0 register accessor: an alias for `Reg<PDCTL0_SPEC>`"]
pub type PDCTL0 = crate::Reg<pdctl0::PDCTL0_SPEC>;
#[doc = "Power Domain Control"]
pub mod pdctl0;
#[doc = "PDCTL0RFC register accessor: an alias for `Reg<PDCTL0RFC_SPEC>`"]
pub type PDCTL0RFC = crate::Reg<pdctl0rfc::PDCTL0RFC_SPEC>;
#[doc = "RFC Power Domain Control"]
pub mod pdctl0rfc;
#[doc = "PDCTL0SERIAL register accessor: an alias for `Reg<PDCTL0SERIAL_SPEC>`"]
pub type PDCTL0SERIAL = crate::Reg<pdctl0serial::PDCTL0SERIAL_SPEC>;
#[doc = "SERIAL Power Domain Control"]
pub mod pdctl0serial;
#[doc = "PDCTL0PERIPH register accessor: an alias for `Reg<PDCTL0PERIPH_SPEC>`"]
pub type PDCTL0PERIPH = crate::Reg<pdctl0periph::PDCTL0PERIPH_SPEC>;
#[doc = "PERIPH Power Domain Control"]
pub mod pdctl0periph;
#[doc = "PDSTAT0 register accessor: an alias for `Reg<PDSTAT0_SPEC>`"]
pub type PDSTAT0 = crate::Reg<pdstat0::PDSTAT0_SPEC>;
#[doc = "Power Domain Status"]
pub mod pdstat0;
#[doc = "PDSTAT0RFC register accessor: an alias for `Reg<PDSTAT0RFC_SPEC>`"]
pub type PDSTAT0RFC = crate::Reg<pdstat0rfc::PDSTAT0RFC_SPEC>;
#[doc = "RFC Power Domain Status"]
pub mod pdstat0rfc;
#[doc = "PDSTAT0SERIAL register accessor: an alias for `Reg<PDSTAT0SERIAL_SPEC>`"]
pub type PDSTAT0SERIAL = crate::Reg<pdstat0serial::PDSTAT0SERIAL_SPEC>;
#[doc = "SERIAL Power Domain Status"]
pub mod pdstat0serial;
#[doc = "PDSTAT0PERIPH register accessor: an alias for `Reg<PDSTAT0PERIPH_SPEC>`"]
pub type PDSTAT0PERIPH = crate::Reg<pdstat0periph::PDSTAT0PERIPH_SPEC>;
#[doc = "PERIPH Power Domain Status"]
pub mod pdstat0periph;
#[doc = "PDCTL1 register accessor: an alias for `Reg<PDCTL1_SPEC>`"]
pub type PDCTL1 = crate::Reg<pdctl1::PDCTL1_SPEC>;
#[doc = "Power Domain Control"]
pub mod pdctl1;
#[doc = "PDCTL1CPU register accessor: an alias for `Reg<PDCTL1CPU_SPEC>`"]
pub type PDCTL1CPU = crate::Reg<pdctl1cpu::PDCTL1CPU_SPEC>;
#[doc = "CPU Power Domain Direct Control"]
pub mod pdctl1cpu;
#[doc = "PDCTL1RFC register accessor: an alias for `Reg<PDCTL1RFC_SPEC>`"]
pub type PDCTL1RFC = crate::Reg<pdctl1rfc::PDCTL1RFC_SPEC>;
#[doc = "RFC Power Domain Direct Control"]
pub mod pdctl1rfc;
#[doc = "PDCTL1VIMS register accessor: an alias for `Reg<PDCTL1VIMS_SPEC>`"]
pub type PDCTL1VIMS = crate::Reg<pdctl1vims::PDCTL1VIMS_SPEC>;
#[doc = "VIMS Mode Direct Control"]
pub mod pdctl1vims;
#[doc = "PDSTAT1 register accessor: an alias for `Reg<PDSTAT1_SPEC>`"]
pub type PDSTAT1 = crate::Reg<pdstat1::PDSTAT1_SPEC>;
#[doc = "Power Manager Status"]
pub mod pdstat1;
#[doc = "PDSTAT1BUS register accessor: an alias for `Reg<PDSTAT1BUS_SPEC>`"]
pub type PDSTAT1BUS = crate::Reg<pdstat1bus::PDSTAT1BUS_SPEC>;
#[doc = "BUS Power Domain Direct Read Status"]
pub mod pdstat1bus;
#[doc = "PDSTAT1RFC register accessor: an alias for `Reg<PDSTAT1RFC_SPEC>`"]
pub type PDSTAT1RFC = crate::Reg<pdstat1rfc::PDSTAT1RFC_SPEC>;
#[doc = "RFC Power Domain Direct Read Status"]
pub mod pdstat1rfc;
#[doc = "PDSTAT1CPU register accessor: an alias for `Reg<PDSTAT1CPU_SPEC>`"]
pub type PDSTAT1CPU = crate::Reg<pdstat1cpu::PDSTAT1CPU_SPEC>;
#[doc = "CPU Power Domain Direct Read Status"]
pub mod pdstat1cpu;
#[doc = "PDSTAT1VIMS register accessor: an alias for `Reg<PDSTAT1VIMS_SPEC>`"]
pub type PDSTAT1VIMS = crate::Reg<pdstat1vims::PDSTAT1VIMS_SPEC>;
#[doc = "VIMS Mode Direct Read Status"]
pub mod pdstat1vims;
#[doc = "RFCBITS register accessor: an alias for `Reg<RFCBITS_SPEC>`"]
pub type RFCBITS = crate::Reg<rfcbits::RFCBITS_SPEC>;
#[doc = "Control To RFC"]
pub mod rfcbits;
#[doc = "RFCMODESEL register accessor: an alias for `Reg<RFCMODESEL_SPEC>`"]
pub type RFCMODESEL = crate::Reg<rfcmodesel::RFCMODESEL_SPEC>;
#[doc = "Selected RFC Mode"]
pub mod rfcmodesel;
#[doc = "RFCMODEHWOPT register accessor: an alias for `Reg<RFCMODEHWOPT_SPEC>`"]
pub type RFCMODEHWOPT = crate::Reg<rfcmodehwopt::RFCMODEHWOPT_SPEC>;
#[doc = "Allowed RFC Modes"]
pub mod rfcmodehwopt;
#[doc = "PWRPROFSTAT register accessor: an alias for `Reg<PWRPROFSTAT_SPEC>`"]
pub type PWRPROFSTAT = crate::Reg<pwrprofstat::PWRPROFSTAT_SPEC>;
#[doc = "Power Profiler Register"]
pub mod pwrprofstat;
#[doc = "RAMRETEN register accessor: an alias for `Reg<RAMRETEN_SPEC>`"]
pub type RAMRETEN = crate::Reg<ramreten::RAMRETEN_SPEC>;
#[doc = "Memory Retention Control"]
pub mod ramreten;