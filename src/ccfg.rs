#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved_0: crate::Reg<reserved_0::RESERVED_0_SPEC>,
    _reserved1: [u8; 0x0fa4],
    #[doc = "0xfa8 - Extern LF clock configuration"]
    pub ext_lf_clk: crate::Reg<ext_lf_clk::EXT_LF_CLK_SPEC>,
    #[doc = "0xfac - Mode Configuration 1"]
    pub mode_conf_1: crate::Reg<mode_conf_1::MODE_CONF_1_SPEC>,
    #[doc = "0xfb0 - CCFG Size and Disable Flags"]
    pub size_and_dis_flags: crate::Reg<size_and_dis_flags::SIZE_AND_DIS_FLAGS_SPEC>,
    #[doc = "0xfb4 - Mode Configuration 0"]
    pub mode_conf: crate::Reg<mode_conf::MODE_CONF_SPEC>,
    #[doc = "0xfb8 - Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
    pub volt_load_0: crate::Reg<volt_load_0::VOLT_LOAD_0_SPEC>,
    #[doc = "0xfbc - Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
    pub volt_load_1: crate::Reg<volt_load_1::VOLT_LOAD_1_SPEC>,
    #[doc = "0xfc0 - Real Time Clock Offset Enabled by MODE_CONF.RTC_COMP."]
    pub rtc_offset: crate::Reg<rtc_offset::RTC_OFFSET_SPEC>,
    #[doc = "0xfc4 - Frequency Offset"]
    pub freq_offset: crate::Reg<freq_offset::FREQ_OFFSET_SPEC>,
    #[doc = "0xfc8 - IEEE MAC Address 0"]
    pub ieee_mac_0: crate::Reg<ieee_mac_0::IEEE_MAC_0_SPEC>,
    #[doc = "0xfcc - IEEE MAC Address 1"]
    pub ieee_mac_1: crate::Reg<ieee_mac_1::IEEE_MAC_1_SPEC>,
    #[doc = "0xfd0 - IEEE BLE Address 0"]
    pub ieee_ble_0: crate::Reg<ieee_ble_0::IEEE_BLE_0_SPEC>,
    #[doc = "0xfd4 - IEEE BLE Address 1"]
    pub ieee_ble_1: crate::Reg<ieee_ble_1::IEEE_BLE_1_SPEC>,
    #[doc = "0xfd8 - Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash."]
    pub bl_config: crate::Reg<bl_config::BL_CONFIG_SPEC>,
    #[doc = "0xfdc - Erase Configuration"]
    pub erase_conf: crate::Reg<erase_conf::ERASE_CONF_SPEC>,
    #[doc = "0xfe0 - TI Options"]
    pub ccfg_ti_options: crate::Reg<ccfg_ti_options::CCFG_TI_OPTIONS_SPEC>,
    #[doc = "0xfe4 - Test Access Points Enable 0"]
    pub ccfg_tap_dap_0: crate::Reg<ccfg_tap_dap_0::CCFG_TAP_DAP_0_SPEC>,
    #[doc = "0xfe8 - Test Access Points Enable 1"]
    pub ccfg_tap_dap_1: crate::Reg<ccfg_tap_dap_1::CCFG_TAP_DAP_1_SPEC>,
    #[doc = "0xfec - Image Valid"]
    pub image_valid_conf: crate::Reg<image_valid_conf::IMAGE_VALID_CONF_SPEC>,
    #[doc = "0xff0 - Protect Sectors 0-31 Each bit write protects one 4KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
    pub ccfg_prot_31_0: crate::Reg<ccfg_prot_31_0::CCFG_PROT_31_0_SPEC>,
    #[doc = "0xff4 - Protect Sectors 32-63 Each bit write protects one 4KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0."]
    pub ccfg_prot_63_32: crate::Reg<ccfg_prot_63_32::CCFG_PROT_63_32_SPEC>,
    #[doc = "0xff8 - Protect Sectors 64-95 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0."]
    pub ccfg_prot_95_64: crate::Reg<ccfg_prot_95_64::CCFG_PROT_95_64_SPEC>,
    #[doc = "0xffc - Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0."]
    pub ccfg_prot_127_96: crate::Reg<ccfg_prot_127_96::CCFG_PROT_127_96_SPEC>,
}
#[doc = "RESERVED_0 register accessor: an alias for `Reg<RESERVED_0_SPEC>`"]
pub type RESERVED_0 = crate::Reg<reserved_0::RESERVED_0_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved_0;
#[doc = "EXT_LF_CLK register accessor: an alias for `Reg<EXT_LF_CLK_SPEC>`"]
pub type EXT_LF_CLK = crate::Reg<ext_lf_clk::EXT_LF_CLK_SPEC>;
#[doc = "Extern LF clock configuration"]
pub mod ext_lf_clk;
#[doc = "MODE_CONF_1 register accessor: an alias for `Reg<MODE_CONF_1_SPEC>`"]
pub type MODE_CONF_1 = crate::Reg<mode_conf_1::MODE_CONF_1_SPEC>;
#[doc = "Mode Configuration 1"]
pub mod mode_conf_1;
#[doc = "SIZE_AND_DIS_FLAGS register accessor: an alias for `Reg<SIZE_AND_DIS_FLAGS_SPEC>`"]
pub type SIZE_AND_DIS_FLAGS = crate::Reg<size_and_dis_flags::SIZE_AND_DIS_FLAGS_SPEC>;
#[doc = "CCFG Size and Disable Flags"]
pub mod size_and_dis_flags;
#[doc = "MODE_CONF register accessor: an alias for `Reg<MODE_CONF_SPEC>`"]
pub type MODE_CONF = crate::Reg<mode_conf::MODE_CONF_SPEC>;
#[doc = "Mode Configuration 0"]
pub mod mode_conf;
#[doc = "VOLT_LOAD_0 register accessor: an alias for `Reg<VOLT_LOAD_0_SPEC>`"]
pub type VOLT_LOAD_0 = crate::Reg<volt_load_0::VOLT_LOAD_0_SPEC>;
#[doc = "Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
pub mod volt_load_0;
#[doc = "VOLT_LOAD_1 register accessor: an alias for `Reg<VOLT_LOAD_1_SPEC>`"]
pub type VOLT_LOAD_1 = crate::Reg<volt_load_1::VOLT_LOAD_1_SPEC>;
#[doc = "Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
pub mod volt_load_1;
#[doc = "RTC_OFFSET register accessor: an alias for `Reg<RTC_OFFSET_SPEC>`"]
pub type RTC_OFFSET = crate::Reg<rtc_offset::RTC_OFFSET_SPEC>;
#[doc = "Real Time Clock Offset Enabled by MODE_CONF.RTC_COMP."]
pub mod rtc_offset;
#[doc = "FREQ_OFFSET register accessor: an alias for `Reg<FREQ_OFFSET_SPEC>`"]
pub type FREQ_OFFSET = crate::Reg<freq_offset::FREQ_OFFSET_SPEC>;
#[doc = "Frequency Offset"]
pub mod freq_offset;
#[doc = "IEEE_MAC_0 register accessor: an alias for `Reg<IEEE_MAC_0_SPEC>`"]
pub type IEEE_MAC_0 = crate::Reg<ieee_mac_0::IEEE_MAC_0_SPEC>;
#[doc = "IEEE MAC Address 0"]
pub mod ieee_mac_0;
#[doc = "IEEE_MAC_1 register accessor: an alias for `Reg<IEEE_MAC_1_SPEC>`"]
pub type IEEE_MAC_1 = crate::Reg<ieee_mac_1::IEEE_MAC_1_SPEC>;
#[doc = "IEEE MAC Address 1"]
pub mod ieee_mac_1;
#[doc = "IEEE_BLE_0 register accessor: an alias for `Reg<IEEE_BLE_0_SPEC>`"]
pub type IEEE_BLE_0 = crate::Reg<ieee_ble_0::IEEE_BLE_0_SPEC>;
#[doc = "IEEE BLE Address 0"]
pub mod ieee_ble_0;
#[doc = "IEEE_BLE_1 register accessor: an alias for `Reg<IEEE_BLE_1_SPEC>`"]
pub type IEEE_BLE_1 = crate::Reg<ieee_ble_1::IEEE_BLE_1_SPEC>;
#[doc = "IEEE BLE Address 1"]
pub mod ieee_ble_1;
#[doc = "BL_CONFIG register accessor: an alias for `Reg<BL_CONFIG_SPEC>`"]
pub type BL_CONFIG = crate::Reg<bl_config::BL_CONFIG_SPEC>;
#[doc = "Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash."]
pub mod bl_config;
#[doc = "ERASE_CONF register accessor: an alias for `Reg<ERASE_CONF_SPEC>`"]
pub type ERASE_CONF = crate::Reg<erase_conf::ERASE_CONF_SPEC>;
#[doc = "Erase Configuration"]
pub mod erase_conf;
#[doc = "CCFG_TI_OPTIONS register accessor: an alias for `Reg<CCFG_TI_OPTIONS_SPEC>`"]
pub type CCFG_TI_OPTIONS = crate::Reg<ccfg_ti_options::CCFG_TI_OPTIONS_SPEC>;
#[doc = "TI Options"]
pub mod ccfg_ti_options;
#[doc = "CCFG_TAP_DAP_0 register accessor: an alias for `Reg<CCFG_TAP_DAP_0_SPEC>`"]
pub type CCFG_TAP_DAP_0 = crate::Reg<ccfg_tap_dap_0::CCFG_TAP_DAP_0_SPEC>;
#[doc = "Test Access Points Enable 0"]
pub mod ccfg_tap_dap_0;
#[doc = "CCFG_TAP_DAP_1 register accessor: an alias for `Reg<CCFG_TAP_DAP_1_SPEC>`"]
pub type CCFG_TAP_DAP_1 = crate::Reg<ccfg_tap_dap_1::CCFG_TAP_DAP_1_SPEC>;
#[doc = "Test Access Points Enable 1"]
pub mod ccfg_tap_dap_1;
#[doc = "IMAGE_VALID_CONF register accessor: an alias for `Reg<IMAGE_VALID_CONF_SPEC>`"]
pub type IMAGE_VALID_CONF = crate::Reg<image_valid_conf::IMAGE_VALID_CONF_SPEC>;
#[doc = "Image Valid"]
pub mod image_valid_conf;
#[doc = "CCFG_PROT_31_0 register accessor: an alias for `Reg<CCFG_PROT_31_0_SPEC>`"]
pub type CCFG_PROT_31_0 = crate::Reg<ccfg_prot_31_0::CCFG_PROT_31_0_SPEC>;
#[doc = "Protect Sectors 0-31 Each bit write protects one 4KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
pub mod ccfg_prot_31_0;
#[doc = "CCFG_PROT_63_32 register accessor: an alias for `Reg<CCFG_PROT_63_32_SPEC>`"]
pub type CCFG_PROT_63_32 = crate::Reg<ccfg_prot_63_32::CCFG_PROT_63_32_SPEC>;
#[doc = "Protect Sectors 32-63 Each bit write protects one 4KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0."]
pub mod ccfg_prot_63_32;
#[doc = "CCFG_PROT_95_64 register accessor: an alias for `Reg<CCFG_PROT_95_64_SPEC>`"]
pub type CCFG_PROT_95_64 = crate::Reg<ccfg_prot_95_64::CCFG_PROT_95_64_SPEC>;
#[doc = "Protect Sectors 64-95 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0."]
pub mod ccfg_prot_95_64;
#[doc = "CCFG_PROT_127_96 register accessor: an alias for `Reg<CCFG_PROT_127_96_SPEC>`"]
pub type CCFG_PROT_127_96 = crate::Reg<ccfg_prot_127_96::CCFG_PROT_127_96_SPEC>;
#[doc = "Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0."]
pub mod ccfg_prot_127_96;
