#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Channel 0 Control"]
    pub dmach0ctl: crate::Reg<dmach0ctl::DMACH0CTL_SPEC>,
    #[doc = "0x04 - DMA Channel 0 External Address"]
    pub dmach0extaddr: crate::Reg<dmach0extaddr::DMACH0EXTADDR_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - DMA Channel 0 Length"]
    pub dmach0len: crate::Reg<dmach0len::DMACH0LEN_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x18 - DMA Controller Status"]
    pub dmastat: crate::Reg<dmastat::DMASTAT_SPEC>,
    #[doc = "0x1c - DMA Controller Software Reset"]
    pub dmaswreset: crate::Reg<dmaswreset::DMASWRESET_SPEC>,
    #[doc = "0x20 - DMA Channel 1 Control"]
    pub dmach1ctl: crate::Reg<dmach1ctl::DMACH1CTL_SPEC>,
    #[doc = "0x24 - DMA Channel 1 External Address"]
    pub dmach1extaddr: crate::Reg<dmach1extaddr::DMACH1EXTADDR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x2c - DMA Channel 1 Length"]
    pub dmach1len: crate::Reg<dmach1len::DMACH1LEN_SPEC>,
    _reserved8: [u8; 0x48],
    #[doc = "0x78 - DMA Controller Master Configuration"]
    pub dmabuscfg: crate::Reg<dmabuscfg::DMABUSCFG_SPEC>,
    #[doc = "0x7c - DMA Controller Port Error"]
    pub dmaporterr: crate::Reg<dmaporterr::DMAPORTERR_SPEC>,
    _reserved10: [u8; 0x7c],
    #[doc = "0xfc - DMA Controller Version"]
    pub dmahwver: crate::Reg<dmahwver::DMAHWVER_SPEC>,
    _reserved11: [u8; 0x0300],
    #[doc = "0x400 - Key Write Area"]
    pub keywritearea: crate::Reg<keywritearea::KEYWRITEAREA_SPEC>,
    #[doc = "0x404 - Key Written Area Status This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and will result in an error."]
    pub keywrittenarea: crate::Reg<keywrittenarea::KEYWRITTENAREA_SPEC>,
    #[doc = "0x408 - Key Size This register defines the size of the keys that are written with DMA."]
    pub keysize: crate::Reg<keysize::KEYSIZE_SPEC>,
    #[doc = "0x40c - Key Read Area"]
    pub keyreadarea: crate::Reg<keyreadarea::KEYREADAREA_SPEC>,
    _reserved15: [u8; 0xf0],
    #[doc = "0x500 - Clear AES_KEY2/GHASH Key"]
    pub aeskey2: crate::Reg<aeskey2::AESKEY2_SPEC>,
    _reserved16: [u8; 0x0c],
    #[doc = "0x510 - Clear AES_KEY3"]
    pub aeskey3: crate::Reg<aeskey3::AESKEY3_SPEC>,
    _reserved17: [u8; 0x2c],
    #[doc = "0x540 - AES Initialization Vector"]
    pub aesiv: crate::Reg<aesiv::AESIV_SPEC>,
    _reserved18: [u8; 0x0c],
    #[doc = "0x550 - AES Input/Output Buffer Control"]
    pub aesctl: crate::Reg<aesctl::AESCTL_SPEC>,
    #[doc = "0x554 - Crypto Data Length LSW"]
    pub aesdatalen0: crate::Reg<aesdatalen0::AESDATALEN0_SPEC>,
    #[doc = "0x558 - Crypto Data Length MSW"]
    pub aesdatalen1: crate::Reg<aesdatalen1::AESDATALEN1_SPEC>,
    #[doc = "0x55c - AES Authentication Length"]
    pub aesauthlen: crate::Reg<aesauthlen::AESAUTHLEN_SPEC>,
    _reserved_22_aesdatain0: [u8; 0x04],
    _reserved_23_aesdatain1: [u8; 0x04],
    _reserved_24_aesdatain2: [u8; 0x04],
    _reserved_25_aesdatain3: [u8; 0x04],
    #[doc = "0x570 - AES Tag Output"]
    pub aestagout: crate::Reg<aestagout::AESTAGOUT_SPEC>,
    _reserved27: [u8; 0x018c],
    #[doc = "0x700 - Master Algorithm Select This register configures the internal destination of the DMA controller."]
    pub algsel: crate::Reg<algsel::ALGSEL_SPEC>,
    #[doc = "0x704 - Master Protection Control"]
    pub dmaprotctl: crate::Reg<dmaprotctl::DMAPROTCTL_SPEC>,
    _reserved29: [u8; 0x38],
    #[doc = "0x740 - Software Reset"]
    pub swreset: crate::Reg<swreset::SWRESET_SPEC>,
    _reserved30: [u8; 0x3c],
    #[doc = "0x780 - Control Interrupt Configuration"]
    pub irqtype: crate::Reg<irqtype::IRQTYPE_SPEC>,
    #[doc = "0x784 - Interrupt Enable"]
    pub irqen: crate::Reg<irqen::IRQEN_SPEC>,
    #[doc = "0x788 - Interrupt Clear"]
    pub irqclr: crate::Reg<irqclr::IRQCLR_SPEC>,
    #[doc = "0x78c - Interrupt Set"]
    pub irqset: crate::Reg<irqset::IRQSET_SPEC>,
    #[doc = "0x790 - Interrupt Status"]
    pub irqstat: crate::Reg<irqstat::IRQSTAT_SPEC>,
    _reserved35: [u8; 0x68],
    #[doc = "0x7fc - CTRL Module Version"]
    pub hwver: crate::Reg<hwver::HWVER_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x560 - AES Data Input/Output 0"]
    #[inline(always)]
    pub fn aesdatain0(&self) -> &crate::Reg<aesdatain0::AESDATAIN0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const crate::Reg<aesdatain0::AESDATAIN0_SPEC>)
        }
    }
    #[doc = "0x560 - Data Input/Output"]
    #[inline(always)]
    pub fn aesdataout0(&self) -> &crate::Reg<aesdataout0::AESDATAOUT0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const crate::Reg<aesdataout0::AESDATAOUT0_SPEC>)
        }
    }
    #[doc = "0x564 - AES Data Input/Output 1"]
    #[inline(always)]
    pub fn aesdatain1(&self) -> &crate::Reg<aesdatain1::AESDATAIN1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1380usize)
                as *const crate::Reg<aesdatain1::AESDATAIN1_SPEC>)
        }
    }
    #[doc = "0x564 - AES Data Input/Output 3"]
    #[inline(always)]
    pub fn aesdataout1(&self) -> &crate::Reg<aesdataout1::AESDATAOUT1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1380usize)
                as *const crate::Reg<aesdataout1::AESDATAOUT1_SPEC>)
        }
    }
    #[doc = "0x568 - AES Data Input/Output 2"]
    #[inline(always)]
    pub fn aesdatain2(&self) -> &crate::Reg<aesdatain2::AESDATAIN2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1384usize)
                as *const crate::Reg<aesdatain2::AESDATAIN2_SPEC>)
        }
    }
    #[doc = "0x568 - AES Data Input/Output 2"]
    #[inline(always)]
    pub fn aesdataout2(&self) -> &crate::Reg<aesdataout2::AESDATAOUT2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1384usize)
                as *const crate::Reg<aesdataout2::AESDATAOUT2_SPEC>)
        }
    }
    #[doc = "0x56c - Data Input/Output"]
    #[inline(always)]
    pub fn aesdatain3(&self) -> &crate::Reg<aesdatain3::AESDATAIN3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1388usize)
                as *const crate::Reg<aesdatain3::AESDATAIN3_SPEC>)
        }
    }
    #[doc = "0x56c - AES Data Input/Output 3"]
    #[inline(always)]
    pub fn aesdataout3(&self) -> &crate::Reg<aesdataout3::AESDATAOUT3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1388usize)
                as *const crate::Reg<aesdataout3::AESDATAOUT3_SPEC>)
        }
    }
}
#[doc = "DMACH0CTL register accessor: an alias for `Reg<DMACH0CTL_SPEC>`"]
pub type DMACH0CTL = crate::Reg<dmach0ctl::DMACH0CTL_SPEC>;
#[doc = "DMA Channel 0 Control"]
pub mod dmach0ctl;
#[doc = "DMACH0EXTADDR register accessor: an alias for `Reg<DMACH0EXTADDR_SPEC>`"]
pub type DMACH0EXTADDR = crate::Reg<dmach0extaddr::DMACH0EXTADDR_SPEC>;
#[doc = "DMA Channel 0 External Address"]
pub mod dmach0extaddr;
#[doc = "DMACH0LEN register accessor: an alias for `Reg<DMACH0LEN_SPEC>`"]
pub type DMACH0LEN = crate::Reg<dmach0len::DMACH0LEN_SPEC>;
#[doc = "DMA Channel 0 Length"]
pub mod dmach0len;
#[doc = "DMASTAT register accessor: an alias for `Reg<DMASTAT_SPEC>`"]
pub type DMASTAT = crate::Reg<dmastat::DMASTAT_SPEC>;
#[doc = "DMA Controller Status"]
pub mod dmastat;
#[doc = "DMASWRESET register accessor: an alias for `Reg<DMASWRESET_SPEC>`"]
pub type DMASWRESET = crate::Reg<dmaswreset::DMASWRESET_SPEC>;
#[doc = "DMA Controller Software Reset"]
pub mod dmaswreset;
#[doc = "DMACH1CTL register accessor: an alias for `Reg<DMACH1CTL_SPEC>`"]
pub type DMACH1CTL = crate::Reg<dmach1ctl::DMACH1CTL_SPEC>;
#[doc = "DMA Channel 1 Control"]
pub mod dmach1ctl;
#[doc = "DMACH1EXTADDR register accessor: an alias for `Reg<DMACH1EXTADDR_SPEC>`"]
pub type DMACH1EXTADDR = crate::Reg<dmach1extaddr::DMACH1EXTADDR_SPEC>;
#[doc = "DMA Channel 1 External Address"]
pub mod dmach1extaddr;
#[doc = "DMACH1LEN register accessor: an alias for `Reg<DMACH1LEN_SPEC>`"]
pub type DMACH1LEN = crate::Reg<dmach1len::DMACH1LEN_SPEC>;
#[doc = "DMA Channel 1 Length"]
pub mod dmach1len;
#[doc = "DMABUSCFG register accessor: an alias for `Reg<DMABUSCFG_SPEC>`"]
pub type DMABUSCFG = crate::Reg<dmabuscfg::DMABUSCFG_SPEC>;
#[doc = "DMA Controller Master Configuration"]
pub mod dmabuscfg;
#[doc = "DMAPORTERR register accessor: an alias for `Reg<DMAPORTERR_SPEC>`"]
pub type DMAPORTERR = crate::Reg<dmaporterr::DMAPORTERR_SPEC>;
#[doc = "DMA Controller Port Error"]
pub mod dmaporterr;
#[doc = "DMAHWVER register accessor: an alias for `Reg<DMAHWVER_SPEC>`"]
pub type DMAHWVER = crate::Reg<dmahwver::DMAHWVER_SPEC>;
#[doc = "DMA Controller Version"]
pub mod dmahwver;
#[doc = "KEYWRITEAREA register accessor: an alias for `Reg<KEYWRITEAREA_SPEC>`"]
pub type KEYWRITEAREA = crate::Reg<keywritearea::KEYWRITEAREA_SPEC>;
#[doc = "Key Write Area"]
pub mod keywritearea;
#[doc = "KEYWRITTENAREA register accessor: an alias for `Reg<KEYWRITTENAREA_SPEC>`"]
pub type KEYWRITTENAREA = crate::Reg<keywrittenarea::KEYWRITTENAREA_SPEC>;
#[doc = "Key Written Area Status This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and will result in an error."]
pub mod keywrittenarea;
#[doc = "KEYSIZE register accessor: an alias for `Reg<KEYSIZE_SPEC>`"]
pub type KEYSIZE = crate::Reg<keysize::KEYSIZE_SPEC>;
#[doc = "Key Size This register defines the size of the keys that are written with DMA."]
pub mod keysize;
#[doc = "KEYREADAREA register accessor: an alias for `Reg<KEYREADAREA_SPEC>`"]
pub type KEYREADAREA = crate::Reg<keyreadarea::KEYREADAREA_SPEC>;
#[doc = "Key Read Area"]
pub mod keyreadarea;
#[doc = "AESKEY2 register accessor: an alias for `Reg<AESKEY2_SPEC>`"]
pub type AESKEY2 = crate::Reg<aeskey2::AESKEY2_SPEC>;
#[doc = "Clear AES_KEY2/GHASH Key"]
pub mod aeskey2;
#[doc = "AESKEY3 register accessor: an alias for `Reg<AESKEY3_SPEC>`"]
pub type AESKEY3 = crate::Reg<aeskey3::AESKEY3_SPEC>;
#[doc = "Clear AES_KEY3"]
pub mod aeskey3;
#[doc = "AESIV register accessor: an alias for `Reg<AESIV_SPEC>`"]
pub type AESIV = crate::Reg<aesiv::AESIV_SPEC>;
#[doc = "AES Initialization Vector"]
pub mod aesiv;
#[doc = "AESCTL register accessor: an alias for `Reg<AESCTL_SPEC>`"]
pub type AESCTL = crate::Reg<aesctl::AESCTL_SPEC>;
#[doc = "AES Input/Output Buffer Control"]
pub mod aesctl;
#[doc = "AESDATALEN0 register accessor: an alias for `Reg<AESDATALEN0_SPEC>`"]
pub type AESDATALEN0 = crate::Reg<aesdatalen0::AESDATALEN0_SPEC>;
#[doc = "Crypto Data Length LSW"]
pub mod aesdatalen0;
#[doc = "AESDATALEN1 register accessor: an alias for `Reg<AESDATALEN1_SPEC>`"]
pub type AESDATALEN1 = crate::Reg<aesdatalen1::AESDATALEN1_SPEC>;
#[doc = "Crypto Data Length MSW"]
pub mod aesdatalen1;
#[doc = "AESAUTHLEN register accessor: an alias for `Reg<AESAUTHLEN_SPEC>`"]
pub type AESAUTHLEN = crate::Reg<aesauthlen::AESAUTHLEN_SPEC>;
#[doc = "AES Authentication Length"]
pub mod aesauthlen;
#[doc = "AESDATAOUT0 register accessor: an alias for `Reg<AESDATAOUT0_SPEC>`"]
pub type AESDATAOUT0 = crate::Reg<aesdataout0::AESDATAOUT0_SPEC>;
#[doc = "Data Input/Output"]
pub mod aesdataout0;
#[doc = "AESDATAIN0 register accessor: an alias for `Reg<AESDATAIN0_SPEC>`"]
pub type AESDATAIN0 = crate::Reg<aesdatain0::AESDATAIN0_SPEC>;
#[doc = "AES Data Input/Output 0"]
pub mod aesdatain0;
#[doc = "AESDATAOUT1 register accessor: an alias for `Reg<AESDATAOUT1_SPEC>`"]
pub type AESDATAOUT1 = crate::Reg<aesdataout1::AESDATAOUT1_SPEC>;
#[doc = "AES Data Input/Output 3"]
pub mod aesdataout1;
#[doc = "AESDATAIN1 register accessor: an alias for `Reg<AESDATAIN1_SPEC>`"]
pub type AESDATAIN1 = crate::Reg<aesdatain1::AESDATAIN1_SPEC>;
#[doc = "AES Data Input/Output 1"]
pub mod aesdatain1;
#[doc = "AESDATAOUT2 register accessor: an alias for `Reg<AESDATAOUT2_SPEC>`"]
pub type AESDATAOUT2 = crate::Reg<aesdataout2::AESDATAOUT2_SPEC>;
#[doc = "AES Data Input/Output 2"]
pub mod aesdataout2;
#[doc = "AESDATAIN2 register accessor: an alias for `Reg<AESDATAIN2_SPEC>`"]
pub type AESDATAIN2 = crate::Reg<aesdatain2::AESDATAIN2_SPEC>;
#[doc = "AES Data Input/Output 2"]
pub mod aesdatain2;
#[doc = "AESDATAOUT3 register accessor: an alias for `Reg<AESDATAOUT3_SPEC>`"]
pub type AESDATAOUT3 = crate::Reg<aesdataout3::AESDATAOUT3_SPEC>;
#[doc = "AES Data Input/Output 3"]
pub mod aesdataout3;
#[doc = "AESDATAIN3 register accessor: an alias for `Reg<AESDATAIN3_SPEC>`"]
pub type AESDATAIN3 = crate::Reg<aesdatain3::AESDATAIN3_SPEC>;
#[doc = "Data Input/Output"]
pub mod aesdatain3;
#[doc = "AESTAGOUT register accessor: an alias for `Reg<AESTAGOUT_SPEC>`"]
pub type AESTAGOUT = crate::Reg<aestagout::AESTAGOUT_SPEC>;
#[doc = "AES Tag Output"]
pub mod aestagout;
#[doc = "ALGSEL register accessor: an alias for `Reg<ALGSEL_SPEC>`"]
pub type ALGSEL = crate::Reg<algsel::ALGSEL_SPEC>;
#[doc = "Master Algorithm Select This register configures the internal destination of the DMA controller."]
pub mod algsel;
#[doc = "DMAPROTCTL register accessor: an alias for `Reg<DMAPROTCTL_SPEC>`"]
pub type DMAPROTCTL = crate::Reg<dmaprotctl::DMAPROTCTL_SPEC>;
#[doc = "Master Protection Control"]
pub mod dmaprotctl;
#[doc = "SWRESET register accessor: an alias for `Reg<SWRESET_SPEC>`"]
pub type SWRESET = crate::Reg<swreset::SWRESET_SPEC>;
#[doc = "Software Reset"]
pub mod swreset;
#[doc = "IRQTYPE register accessor: an alias for `Reg<IRQTYPE_SPEC>`"]
pub type IRQTYPE = crate::Reg<irqtype::IRQTYPE_SPEC>;
#[doc = "Control Interrupt Configuration"]
pub mod irqtype;
#[doc = "IRQEN register accessor: an alias for `Reg<IRQEN_SPEC>`"]
pub type IRQEN = crate::Reg<irqen::IRQEN_SPEC>;
#[doc = "Interrupt Enable"]
pub mod irqen;
#[doc = "IRQCLR register accessor: an alias for `Reg<IRQCLR_SPEC>`"]
pub type IRQCLR = crate::Reg<irqclr::IRQCLR_SPEC>;
#[doc = "Interrupt Clear"]
pub mod irqclr;
#[doc = "IRQSET register accessor: an alias for `Reg<IRQSET_SPEC>`"]
pub type IRQSET = crate::Reg<irqset::IRQSET_SPEC>;
#[doc = "Interrupt Set"]
pub mod irqset;
#[doc = "IRQSTAT register accessor: an alias for `Reg<IRQSTAT_SPEC>`"]
pub type IRQSTAT = crate::Reg<irqstat::IRQSTAT_SPEC>;
#[doc = "Interrupt Status"]
pub mod irqstat;
#[doc = "HWVER register accessor: an alias for `Reg<HWVER_SPEC>`"]
pub type HWVER = crate::Reg<hwver::HWVER_SPEC>;
#[doc = "CTRL Module Version"]
pub mod hwver;
