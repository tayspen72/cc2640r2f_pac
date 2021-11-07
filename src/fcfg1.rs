#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0xa0],
    #[doc = "0xa0 - Misc configurations"]
    pub misc_conf_1: crate::Reg<misc_conf_1::MISC_CONF_1_SPEC>,
    #[doc = "0xa4 - Internal. Only to be used through TI provided API."]
    pub misc_conf_2: crate::Reg<misc_conf_2::MISC_CONF_2_SPEC>,
    _reserved2: [u8; 0x1c],
    #[doc = "0xc4 - Internal. Only to be used through TI provided API."]
    pub config_rf_frontend_div5: crate::Reg<config_rf_frontend_div5::CONFIG_RF_FRONTEND_DIV5_SPEC>,
    #[doc = "0xc8 - Internal. Only to be used through TI provided API."]
    pub config_rf_frontend_div6: crate::Reg<config_rf_frontend_div6::CONFIG_RF_FRONTEND_DIV6_SPEC>,
    #[doc = "0xcc - Internal. Only to be used through TI provided API."]
    pub config_rf_frontend_div10:
        crate::Reg<config_rf_frontend_div10::CONFIG_RF_FRONTEND_DIV10_SPEC>,
    #[doc = "0xd0 - Internal. Only to be used through TI provided API."]
    pub config_rf_frontend_div12:
        crate::Reg<config_rf_frontend_div12::CONFIG_RF_FRONTEND_DIV12_SPEC>,
    #[doc = "0xd4 - Internal. Only to be used through TI provided API."]
    pub config_rf_frontend_div15:
        crate::Reg<config_rf_frontend_div15::CONFIG_RF_FRONTEND_DIV15_SPEC>,
    #[doc = "0xd8 - Internal. Only to be used through TI provided API."]
    pub config_rf_frontend_div30:
        crate::Reg<config_rf_frontend_div30::CONFIG_RF_FRONTEND_DIV30_SPEC>,
    #[doc = "0xdc - Internal. Only to be used through TI provided API."]
    pub config_synth_div5: crate::Reg<config_synth_div5::CONFIG_SYNTH_DIV5_SPEC>,
    #[doc = "0xe0 - Internal. Only to be used through TI provided API."]
    pub config_synth_div6: crate::Reg<config_synth_div6::CONFIG_SYNTH_DIV6_SPEC>,
    #[doc = "0xe4 - Internal. Only to be used through TI provided API."]
    pub config_synth_div10: crate::Reg<config_synth_div10::CONFIG_SYNTH_DIV10_SPEC>,
    #[doc = "0xe8 - Internal. Only to be used through TI provided API."]
    pub config_synth_div12: crate::Reg<config_synth_div12::CONFIG_SYNTH_DIV12_SPEC>,
    #[doc = "0xec - Internal. Only to be used through TI provided API."]
    pub config_synth_div15: crate::Reg<config_synth_div15::CONFIG_SYNTH_DIV15_SPEC>,
    #[doc = "0xf0 - Internal. Only to be used through TI provided API."]
    pub config_synth_div30: crate::Reg<config_synth_div30::CONFIG_SYNTH_DIV30_SPEC>,
    #[doc = "0xf4 - Internal. Only to be used through TI provided API."]
    pub config_misc_adc_div5: crate::Reg<config_misc_adc_div5::CONFIG_MISC_ADC_DIV5_SPEC>,
    #[doc = "0xf8 - Internal. Only to be used through TI provided API."]
    pub config_misc_adc_div6: crate::Reg<config_misc_adc_div6::CONFIG_MISC_ADC_DIV6_SPEC>,
    #[doc = "0xfc - Internal. Only to be used through TI provided API."]
    pub config_misc_adc_div10: crate::Reg<config_misc_adc_div10::CONFIG_MISC_ADC_DIV10_SPEC>,
    #[doc = "0x100 - Internal. Only to be used through TI provided API."]
    pub config_misc_adc_div12: crate::Reg<config_misc_adc_div12::CONFIG_MISC_ADC_DIV12_SPEC>,
    #[doc = "0x104 - Internal. Only to be used through TI provided API."]
    pub config_misc_adc_div15: crate::Reg<config_misc_adc_div15::CONFIG_MISC_ADC_DIV15_SPEC>,
    #[doc = "0x108 - Internal. Only to be used through TI provided API."]
    pub config_misc_adc_div30: crate::Reg<config_misc_adc_div30::CONFIG_MISC_ADC_DIV30_SPEC>,
    _reserved20: [u8; 0x0c],
    #[doc = "0x118 - Shadow of DIE_ID_0 register in eFuse"]
    pub shdw_die_id_0: crate::Reg<shdw_die_id_0::SHDW_DIE_ID_0_SPEC>,
    #[doc = "0x11c - Shadow of DIE_ID_1 register in eFuse"]
    pub shdw_die_id_1: crate::Reg<shdw_die_id_1::SHDW_DIE_ID_1_SPEC>,
    #[doc = "0x120 - Shadow of DIE_ID_2 register in eFuse"]
    pub shdw_die_id_2: crate::Reg<shdw_die_id_2::SHDW_DIE_ID_2_SPEC>,
    #[doc = "0x124 - Shadow of DIE_ID_3 register in eFuse"]
    pub shdw_die_id_3: crate::Reg<shdw_die_id_3::SHDW_DIE_ID_3_SPEC>,
    _reserved24: [u8; 0x10],
    #[doc = "0x138 - Internal. Only to be used through TI provided API."]
    pub shdw_osc_bias_ldo_trim: crate::Reg<shdw_osc_bias_ldo_trim::SHDW_OSC_BIAS_LDO_TRIM_SPEC>,
    #[doc = "0x13c - Internal. Only to be used through TI provided API."]
    pub shdw_ana_trim: crate::Reg<shdw_ana_trim::SHDW_ANA_TRIM_SPEC>,
    _reserved26: [u8; 0x24],
    #[doc = "0x164 - FLASH_NUMBER"]
    pub flash_number: crate::Reg<flash_number::FLASH_NUMBER_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0x16c - FLASH_COORDINATE"]
    pub flash_coordinate: crate::Reg<flash_coordinate::FLASH_COORDINATE_SPEC>,
    #[doc = "0x170 - Internal. Only to be used through TI provided API."]
    pub flash_e_p: crate::Reg<flash_e_p::FLASH_E_P_SPEC>,
    #[doc = "0x174 - Internal. Only to be used through TI provided API."]
    pub flash_c_e_p_r: crate::Reg<flash_c_e_p_r::FLASH_C_E_P_R_SPEC>,
    #[doc = "0x178 - Internal. Only to be used through TI provided API."]
    pub flash_p_r_pv: crate::Reg<flash_p_r_pv::FLASH_P_R_PV_SPEC>,
    #[doc = "0x17c - Internal. Only to be used through TI provided API."]
    pub flash_eh_seq: crate::Reg<flash_eh_seq::FLASH_EH_SEQ_SPEC>,
    #[doc = "0x180 - Internal. Only to be used through TI provided API."]
    pub flash_vhv_e: crate::Reg<flash_vhv_e::FLASH_VHV_E_SPEC>,
    #[doc = "0x184 - Internal. Only to be used through TI provided API."]
    pub flash_pp: crate::Reg<flash_pp::FLASH_PP_SPEC>,
    #[doc = "0x188 - Internal. Only to be used through TI provided API."]
    pub flash_prog_ep: crate::Reg<flash_prog_ep::FLASH_PROG_EP_SPEC>,
    #[doc = "0x18c - Internal. Only to be used through TI provided API."]
    pub flash_era_pw: crate::Reg<flash_era_pw::FLASH_ERA_PW_SPEC>,
    #[doc = "0x190 - Internal. Only to be used through TI provided API."]
    pub flash_vhv: crate::Reg<flash_vhv::FLASH_VHV_SPEC>,
    #[doc = "0x194 - Internal. Only to be used through TI provided API."]
    pub flash_vhv_pv: crate::Reg<flash_vhv_pv::FLASH_VHV_PV_SPEC>,
    #[doc = "0x198 - Internal. Only to be used through TI provided API."]
    pub flash_v: crate::Reg<flash_v::FLASH_V_SPEC>,
    _reserved39: [u8; 0xf8],
    #[doc = "0x294 - User Identification. Reading this register and the ICEPICK_DEVICE_ID register is the only support way of identifying a device. The value of this register will be written to AON_WUC:JTAGUSERCODE by boot FW while in safezone."]
    pub user_id: crate::Reg<user_id::USER_ID_SPEC>,
    _reserved40: [u8; 0x18],
    #[doc = "0x2b0 - Internal. Only to be used through TI provided API."]
    pub flash_otp_data3: crate::Reg<flash_otp_data3::FLASH_OTP_DATA3_SPEC>,
    #[doc = "0x2b4 - Internal. Only to be used through TI provided API."]
    pub ana2_trim: crate::Reg<ana2_trim::ANA2_TRIM_SPEC>,
    #[doc = "0x2b8 - Internal. Only to be used through TI provided API."]
    pub ldo_trim: crate::Reg<ldo_trim::LDO_TRIM_SPEC>,
    _reserved43: [u8; 0x2c],
    #[doc = "0x2e8 - MAC BLE Address 0"]
    pub mac_ble_0: crate::Reg<mac_ble_0::MAC_BLE_0_SPEC>,
    #[doc = "0x2ec - MAC BLE Address 1"]
    pub mac_ble_1: crate::Reg<mac_ble_1::MAC_BLE_1_SPEC>,
    #[doc = "0x2f0 - MAC IEEE 802.15.4 Address 0"]
    pub mac_15_4_0: crate::Reg<mac_15_4_0::MAC_15_4_0_SPEC>,
    #[doc = "0x2f4 - MAC IEEE 802.15.4 Address 1"]
    pub mac_15_4_1: crate::Reg<mac_15_4_1::MAC_15_4_1_SPEC>,
    _reserved47: [u8; 0x10],
    #[doc = "0x308 - Internal. Only to be used through TI provided API."]
    pub flash_otp_data4: crate::Reg<flash_otp_data4::FLASH_OTP_DATA4_SPEC>,
    #[doc = "0x30c - Miscellaneous Trim Parameters"]
    pub misc_trim: crate::Reg<misc_trim::MISC_TRIM_SPEC>,
    #[doc = "0x310 - Internal. Only to be used through TI provided API."]
    pub rcosc_hf_tempcomp: crate::Reg<rcosc_hf_tempcomp::RCOSC_HF_TEMPCOMP_SPEC>,
    _reserved50: [u8; 0x04],
    #[doc = "0x318 - IcePick Device Identification Reading this register and the USER_ID register is the only support way of identifying a device."]
    pub icepick_device_id: crate::Reg<icepick_device_id::ICEPICK_DEVICE_ID_SPEC>,
    #[doc = "0x31c - Factory Configuration (FCFG1) Revision"]
    pub fcfg1_revision: crate::Reg<fcfg1_revision::FCFG1_REVISION_SPEC>,
    #[doc = "0x320 - Misc OTP Data"]
    pub misc_otp_data: crate::Reg<misc_otp_data::MISC_OTP_DATA_SPEC>,
    _reserved53: [u8; 0x20],
    #[doc = "0x344 - IO Configuration"]
    pub ioconf: crate::Reg<ioconf::IOCONF_SPEC>,
    _reserved54: [u8; 0x04],
    #[doc = "0x34c - Internal. Only to be used through TI provided API."]
    pub config_if_adc: crate::Reg<config_if_adc::CONFIG_IF_ADC_SPEC>,
    #[doc = "0x350 - Internal. Only to be used through TI provided API."]
    pub config_osc_top: crate::Reg<config_osc_top::CONFIG_OSC_TOP_SPEC>,
    #[doc = "0x354 - Internal. Only to be used through TI provided API."]
    pub config_rf_frontend: crate::Reg<config_rf_frontend::CONFIG_RF_FRONTEND_SPEC>,
    #[doc = "0x358 - Internal. Only to be used through TI provided API."]
    pub config_synth: crate::Reg<config_synth::CONFIG_SYNTH_SPEC>,
    #[doc = "0x35c - AUX_ADC Gain in Absolute Reference Mode"]
    pub soc_adc_abs_gain: crate::Reg<soc_adc_abs_gain::SOC_ADC_ABS_GAIN_SPEC>,
    #[doc = "0x360 - AUX_ADC Gain in Relative Reference Mode"]
    pub soc_adc_rel_gain: crate::Reg<soc_adc_rel_gain::SOC_ADC_REL_GAIN_SPEC>,
    _reserved60: [u8; 0x04],
    #[doc = "0x368 - AUX_ADC Temperature Offsets in Absolute Reference Mode"]
    pub soc_adc_offset_int: crate::Reg<soc_adc_offset_int::SOC_ADC_OFFSET_INT_SPEC>,
    #[doc = "0x36c - Internal. Only to be used through TI provided API."]
    pub soc_adc_ref_trim_and_offset_ext:
        crate::Reg<soc_adc_ref_trim_and_offset_ext::SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC>,
    #[doc = "0x370 - Internal. Only to be used through TI provided API."]
    pub ampcomp_th1: crate::Reg<ampcomp_th1::AMPCOMP_TH1_SPEC>,
    #[doc = "0x374 - Internal. Only to be used through TI provided API."]
    pub ampcomp_th2: crate::Reg<ampcomp_th2::AMPCOMP_TH2_SPEC>,
    #[doc = "0x378 - Internal. Only to be used through TI provided API."]
    pub ampcomp_ctrl1: crate::Reg<ampcomp_ctrl1::AMPCOMP_CTRL1_SPEC>,
    #[doc = "0x37c - Internal. Only to be used through TI provided API."]
    pub anabypass_value2: crate::Reg<anabypass_value2::ANABYPASS_VALUE2_SPEC>,
    #[doc = "0x380 - Internal. Only to be used through TI provided API."]
    pub config_misc_adc: crate::Reg<config_misc_adc::CONFIG_MISC_ADC_SPEC>,
    _reserved67: [u8; 0x04],
    #[doc = "0x388 - Internal. Only to be used through TI provided API."]
    pub volt_trim: crate::Reg<volt_trim::VOLT_TRIM_SPEC>,
    #[doc = "0x38c - OSC Configuration"]
    pub osc_conf: crate::Reg<osc_conf::OSC_CONF_SPEC>,
    #[doc = "0x390 - Internal. Only to be used through TI provided API."]
    pub freq_offset: crate::Reg<freq_offset::FREQ_OFFSET_SPEC>,
    #[doc = "0x394 - Internal. Only to be used through TI provided API."]
    pub cap_trim: crate::Reg<cap_trim::CAP_TRIM_SPEC>,
    #[doc = "0x398 - Internal. Only to be used through TI provided API."]
    pub misc_otp_data_1: crate::Reg<misc_otp_data_1::MISC_OTP_DATA_1_SPEC>,
    #[doc = "0x39c - Power Down Current Control 20C"]
    pub pwd_curr_20c: crate::Reg<pwd_curr_20c::PWD_CURR_20C_SPEC>,
    #[doc = "0x3a0 - Power Down Current Control 35C"]
    pub pwd_curr_35c: crate::Reg<pwd_curr_35c::PWD_CURR_35C_SPEC>,
    #[doc = "0x3a4 - Power Down Current Control 50C"]
    pub pwd_curr_50c: crate::Reg<pwd_curr_50c::PWD_CURR_50C_SPEC>,
    #[doc = "0x3a8 - Power Down Current Control 65C"]
    pub pwd_curr_65c: crate::Reg<pwd_curr_65c::PWD_CURR_65C_SPEC>,
    #[doc = "0x3ac - Power Down Current Control 80C"]
    pub pwd_curr_80c: crate::Reg<pwd_curr_80c::PWD_CURR_80C_SPEC>,
    #[doc = "0x3b0 - Power Down Current Control 95C"]
    pub pwd_curr_95c: crate::Reg<pwd_curr_95c::PWD_CURR_95C_SPEC>,
    #[doc = "0x3b4 - Power Down Current Control 110C"]
    pub pwd_curr_110c: crate::Reg<pwd_curr_110c::PWD_CURR_110C_SPEC>,
    #[doc = "0x3b8 - Power Down Current Control 125C"]
    pub pwd_curr_125c: crate::Reg<pwd_curr_125c::PWD_CURR_125C_SPEC>,
}
#[doc = "MISC_CONF_1 register accessor: an alias for `Reg<MISC_CONF_1_SPEC>`"]
pub type MISC_CONF_1 = crate::Reg<misc_conf_1::MISC_CONF_1_SPEC>;
#[doc = "Misc configurations"]
pub mod misc_conf_1;
#[doc = "MISC_CONF_2 register accessor: an alias for `Reg<MISC_CONF_2_SPEC>`"]
pub type MISC_CONF_2 = crate::Reg<misc_conf_2::MISC_CONF_2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod misc_conf_2;
#[doc = "CONFIG_RF_FRONTEND_DIV5 register accessor: an alias for `Reg<CONFIG_RF_FRONTEND_DIV5_SPEC>`"]
pub type CONFIG_RF_FRONTEND_DIV5 =
    crate::Reg<config_rf_frontend_div5::CONFIG_RF_FRONTEND_DIV5_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_frontend_div5;
#[doc = "CONFIG_RF_FRONTEND_DIV6 register accessor: an alias for `Reg<CONFIG_RF_FRONTEND_DIV6_SPEC>`"]
pub type CONFIG_RF_FRONTEND_DIV6 =
    crate::Reg<config_rf_frontend_div6::CONFIG_RF_FRONTEND_DIV6_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_frontend_div6;
#[doc = "CONFIG_RF_FRONTEND_DIV10 register accessor: an alias for `Reg<CONFIG_RF_FRONTEND_DIV10_SPEC>`"]
pub type CONFIG_RF_FRONTEND_DIV10 =
    crate::Reg<config_rf_frontend_div10::CONFIG_RF_FRONTEND_DIV10_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_frontend_div10;
#[doc = "CONFIG_RF_FRONTEND_DIV12 register accessor: an alias for `Reg<CONFIG_RF_FRONTEND_DIV12_SPEC>`"]
pub type CONFIG_RF_FRONTEND_DIV12 =
    crate::Reg<config_rf_frontend_div12::CONFIG_RF_FRONTEND_DIV12_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_frontend_div12;
#[doc = "CONFIG_RF_FRONTEND_DIV15 register accessor: an alias for `Reg<CONFIG_RF_FRONTEND_DIV15_SPEC>`"]
pub type CONFIG_RF_FRONTEND_DIV15 =
    crate::Reg<config_rf_frontend_div15::CONFIG_RF_FRONTEND_DIV15_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_frontend_div15;
#[doc = "CONFIG_RF_FRONTEND_DIV30 register accessor: an alias for `Reg<CONFIG_RF_FRONTEND_DIV30_SPEC>`"]
pub type CONFIG_RF_FRONTEND_DIV30 =
    crate::Reg<config_rf_frontend_div30::CONFIG_RF_FRONTEND_DIV30_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_frontend_div30;
#[doc = "CONFIG_SYNTH_DIV5 register accessor: an alias for `Reg<CONFIG_SYNTH_DIV5_SPEC>`"]
pub type CONFIG_SYNTH_DIV5 = crate::Reg<config_synth_div5::CONFIG_SYNTH_DIV5_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div5;
#[doc = "CONFIG_SYNTH_DIV6 register accessor: an alias for `Reg<CONFIG_SYNTH_DIV6_SPEC>`"]
pub type CONFIG_SYNTH_DIV6 = crate::Reg<config_synth_div6::CONFIG_SYNTH_DIV6_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div6;
#[doc = "CONFIG_SYNTH_DIV10 register accessor: an alias for `Reg<CONFIG_SYNTH_DIV10_SPEC>`"]
pub type CONFIG_SYNTH_DIV10 = crate::Reg<config_synth_div10::CONFIG_SYNTH_DIV10_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div10;
#[doc = "CONFIG_SYNTH_DIV12 register accessor: an alias for `Reg<CONFIG_SYNTH_DIV12_SPEC>`"]
pub type CONFIG_SYNTH_DIV12 = crate::Reg<config_synth_div12::CONFIG_SYNTH_DIV12_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div12;
#[doc = "CONFIG_SYNTH_DIV15 register accessor: an alias for `Reg<CONFIG_SYNTH_DIV15_SPEC>`"]
pub type CONFIG_SYNTH_DIV15 = crate::Reg<config_synth_div15::CONFIG_SYNTH_DIV15_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div15;
#[doc = "CONFIG_SYNTH_DIV30 register accessor: an alias for `Reg<CONFIG_SYNTH_DIV30_SPEC>`"]
pub type CONFIG_SYNTH_DIV30 = crate::Reg<config_synth_div30::CONFIG_SYNTH_DIV30_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div30;
#[doc = "CONFIG_MISC_ADC_DIV5 register accessor: an alias for `Reg<CONFIG_MISC_ADC_DIV5_SPEC>`"]
pub type CONFIG_MISC_ADC_DIV5 = crate::Reg<config_misc_adc_div5::CONFIG_MISC_ADC_DIV5_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_misc_adc_div5;
#[doc = "CONFIG_MISC_ADC_DIV6 register accessor: an alias for `Reg<CONFIG_MISC_ADC_DIV6_SPEC>`"]
pub type CONFIG_MISC_ADC_DIV6 = crate::Reg<config_misc_adc_div6::CONFIG_MISC_ADC_DIV6_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_misc_adc_div6;
#[doc = "CONFIG_MISC_ADC_DIV10 register accessor: an alias for `Reg<CONFIG_MISC_ADC_DIV10_SPEC>`"]
pub type CONFIG_MISC_ADC_DIV10 = crate::Reg<config_misc_adc_div10::CONFIG_MISC_ADC_DIV10_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_misc_adc_div10;
#[doc = "CONFIG_MISC_ADC_DIV12 register accessor: an alias for `Reg<CONFIG_MISC_ADC_DIV12_SPEC>`"]
pub type CONFIG_MISC_ADC_DIV12 = crate::Reg<config_misc_adc_div12::CONFIG_MISC_ADC_DIV12_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_misc_adc_div12;
#[doc = "CONFIG_MISC_ADC_DIV15 register accessor: an alias for `Reg<CONFIG_MISC_ADC_DIV15_SPEC>`"]
pub type CONFIG_MISC_ADC_DIV15 = crate::Reg<config_misc_adc_div15::CONFIG_MISC_ADC_DIV15_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_misc_adc_div15;
#[doc = "CONFIG_MISC_ADC_DIV30 register accessor: an alias for `Reg<CONFIG_MISC_ADC_DIV30_SPEC>`"]
pub type CONFIG_MISC_ADC_DIV30 = crate::Reg<config_misc_adc_div30::CONFIG_MISC_ADC_DIV30_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_misc_adc_div30;
#[doc = "SHDW_DIE_ID_0 register accessor: an alias for `Reg<SHDW_DIE_ID_0_SPEC>`"]
pub type SHDW_DIE_ID_0 = crate::Reg<shdw_die_id_0::SHDW_DIE_ID_0_SPEC>;
#[doc = "Shadow of DIE_ID_0 register in eFuse"]
pub mod shdw_die_id_0;
#[doc = "SHDW_DIE_ID_1 register accessor: an alias for `Reg<SHDW_DIE_ID_1_SPEC>`"]
pub type SHDW_DIE_ID_1 = crate::Reg<shdw_die_id_1::SHDW_DIE_ID_1_SPEC>;
#[doc = "Shadow of DIE_ID_1 register in eFuse"]
pub mod shdw_die_id_1;
#[doc = "SHDW_DIE_ID_2 register accessor: an alias for `Reg<SHDW_DIE_ID_2_SPEC>`"]
pub type SHDW_DIE_ID_2 = crate::Reg<shdw_die_id_2::SHDW_DIE_ID_2_SPEC>;
#[doc = "Shadow of DIE_ID_2 register in eFuse"]
pub mod shdw_die_id_2;
#[doc = "SHDW_DIE_ID_3 register accessor: an alias for `Reg<SHDW_DIE_ID_3_SPEC>`"]
pub type SHDW_DIE_ID_3 = crate::Reg<shdw_die_id_3::SHDW_DIE_ID_3_SPEC>;
#[doc = "Shadow of DIE_ID_3 register in eFuse"]
pub mod shdw_die_id_3;
#[doc = "SHDW_OSC_BIAS_LDO_TRIM register accessor: an alias for `Reg<SHDW_OSC_BIAS_LDO_TRIM_SPEC>`"]
pub type SHDW_OSC_BIAS_LDO_TRIM = crate::Reg<shdw_osc_bias_ldo_trim::SHDW_OSC_BIAS_LDO_TRIM_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod shdw_osc_bias_ldo_trim;
#[doc = "SHDW_ANA_TRIM register accessor: an alias for `Reg<SHDW_ANA_TRIM_SPEC>`"]
pub type SHDW_ANA_TRIM = crate::Reg<shdw_ana_trim::SHDW_ANA_TRIM_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod shdw_ana_trim;
#[doc = "FLASH_NUMBER register accessor: an alias for `Reg<FLASH_NUMBER_SPEC>`"]
pub type FLASH_NUMBER = crate::Reg<flash_number::FLASH_NUMBER_SPEC>;
#[doc = "FLASH_NUMBER"]
pub mod flash_number;
#[doc = "FLASH_COORDINATE register accessor: an alias for `Reg<FLASH_COORDINATE_SPEC>`"]
pub type FLASH_COORDINATE = crate::Reg<flash_coordinate::FLASH_COORDINATE_SPEC>;
#[doc = "FLASH_COORDINATE"]
pub mod flash_coordinate;
#[doc = "FLASH_E_P register accessor: an alias for `Reg<FLASH_E_P_SPEC>`"]
pub type FLASH_E_P = crate::Reg<flash_e_p::FLASH_E_P_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_e_p;
#[doc = "FLASH_C_E_P_R register accessor: an alias for `Reg<FLASH_C_E_P_R_SPEC>`"]
pub type FLASH_C_E_P_R = crate::Reg<flash_c_e_p_r::FLASH_C_E_P_R_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_c_e_p_r;
#[doc = "FLASH_P_R_PV register accessor: an alias for `Reg<FLASH_P_R_PV_SPEC>`"]
pub type FLASH_P_R_PV = crate::Reg<flash_p_r_pv::FLASH_P_R_PV_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_p_r_pv;
#[doc = "FLASH_EH_SEQ register accessor: an alias for `Reg<FLASH_EH_SEQ_SPEC>`"]
pub type FLASH_EH_SEQ = crate::Reg<flash_eh_seq::FLASH_EH_SEQ_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_eh_seq;
#[doc = "FLASH_VHV_E register accessor: an alias for `Reg<FLASH_VHV_E_SPEC>`"]
pub type FLASH_VHV_E = crate::Reg<flash_vhv_e::FLASH_VHV_E_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_vhv_e;
#[doc = "FLASH_PP register accessor: an alias for `Reg<FLASH_PP_SPEC>`"]
pub type FLASH_PP = crate::Reg<flash_pp::FLASH_PP_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_pp;
#[doc = "FLASH_PROG_EP register accessor: an alias for `Reg<FLASH_PROG_EP_SPEC>`"]
pub type FLASH_PROG_EP = crate::Reg<flash_prog_ep::FLASH_PROG_EP_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_prog_ep;
#[doc = "FLASH_ERA_PW register accessor: an alias for `Reg<FLASH_ERA_PW_SPEC>`"]
pub type FLASH_ERA_PW = crate::Reg<flash_era_pw::FLASH_ERA_PW_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_era_pw;
#[doc = "FLASH_VHV register accessor: an alias for `Reg<FLASH_VHV_SPEC>`"]
pub type FLASH_VHV = crate::Reg<flash_vhv::FLASH_VHV_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_vhv;
#[doc = "FLASH_VHV_PV register accessor: an alias for `Reg<FLASH_VHV_PV_SPEC>`"]
pub type FLASH_VHV_PV = crate::Reg<flash_vhv_pv::FLASH_VHV_PV_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_vhv_pv;
#[doc = "FLASH_V register accessor: an alias for `Reg<FLASH_V_SPEC>`"]
pub type FLASH_V = crate::Reg<flash_v::FLASH_V_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_v;
#[doc = "USER_ID register accessor: an alias for `Reg<USER_ID_SPEC>`"]
pub type USER_ID = crate::Reg<user_id::USER_ID_SPEC>;
#[doc = "User Identification. Reading this register and the ICEPICK_DEVICE_ID register is the only support way of identifying a device. The value of this register will be written to AON_WUC:JTAGUSERCODE by boot FW while in safezone."]
pub mod user_id;
#[doc = "FLASH_OTP_DATA3 register accessor: an alias for `Reg<FLASH_OTP_DATA3_SPEC>`"]
pub type FLASH_OTP_DATA3 = crate::Reg<flash_otp_data3::FLASH_OTP_DATA3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_otp_data3;
#[doc = "ANA2_TRIM register accessor: an alias for `Reg<ANA2_TRIM_SPEC>`"]
pub type ANA2_TRIM = crate::Reg<ana2_trim::ANA2_TRIM_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ana2_trim;
#[doc = "LDO_TRIM register accessor: an alias for `Reg<LDO_TRIM_SPEC>`"]
pub type LDO_TRIM = crate::Reg<ldo_trim::LDO_TRIM_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ldo_trim;
#[doc = "MAC_BLE_0 register accessor: an alias for `Reg<MAC_BLE_0_SPEC>`"]
pub type MAC_BLE_0 = crate::Reg<mac_ble_0::MAC_BLE_0_SPEC>;
#[doc = "MAC BLE Address 0"]
pub mod mac_ble_0;
#[doc = "MAC_BLE_1 register accessor: an alias for `Reg<MAC_BLE_1_SPEC>`"]
pub type MAC_BLE_1 = crate::Reg<mac_ble_1::MAC_BLE_1_SPEC>;
#[doc = "MAC BLE Address 1"]
pub mod mac_ble_1;
#[doc = "MAC_15_4_0 register accessor: an alias for `Reg<MAC_15_4_0_SPEC>`"]
pub type MAC_15_4_0 = crate::Reg<mac_15_4_0::MAC_15_4_0_SPEC>;
#[doc = "MAC IEEE 802.15.4 Address 0"]
pub mod mac_15_4_0;
#[doc = "MAC_15_4_1 register accessor: an alias for `Reg<MAC_15_4_1_SPEC>`"]
pub type MAC_15_4_1 = crate::Reg<mac_15_4_1::MAC_15_4_1_SPEC>;
#[doc = "MAC IEEE 802.15.4 Address 1"]
pub mod mac_15_4_1;
#[doc = "FLASH_OTP_DATA4 register accessor: an alias for `Reg<FLASH_OTP_DATA4_SPEC>`"]
pub type FLASH_OTP_DATA4 = crate::Reg<flash_otp_data4::FLASH_OTP_DATA4_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_otp_data4;
#[doc = "MISC_TRIM register accessor: an alias for `Reg<MISC_TRIM_SPEC>`"]
pub type MISC_TRIM = crate::Reg<misc_trim::MISC_TRIM_SPEC>;
#[doc = "Miscellaneous Trim Parameters"]
pub mod misc_trim;
#[doc = "RCOSC_HF_TEMPCOMP register accessor: an alias for `Reg<RCOSC_HF_TEMPCOMP_SPEC>`"]
pub type RCOSC_HF_TEMPCOMP = crate::Reg<rcosc_hf_tempcomp::RCOSC_HF_TEMPCOMP_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod rcosc_hf_tempcomp;
#[doc = "ICEPICK_DEVICE_ID register accessor: an alias for `Reg<ICEPICK_DEVICE_ID_SPEC>`"]
pub type ICEPICK_DEVICE_ID = crate::Reg<icepick_device_id::ICEPICK_DEVICE_ID_SPEC>;
#[doc = "IcePick Device Identification Reading this register and the USER_ID register is the only support way of identifying a device."]
pub mod icepick_device_id;
#[doc = "FCFG1_REVISION register accessor: an alias for `Reg<FCFG1_REVISION_SPEC>`"]
pub type FCFG1_REVISION = crate::Reg<fcfg1_revision::FCFG1_REVISION_SPEC>;
#[doc = "Factory Configuration (FCFG1) Revision"]
pub mod fcfg1_revision;
#[doc = "MISC_OTP_DATA register accessor: an alias for `Reg<MISC_OTP_DATA_SPEC>`"]
pub type MISC_OTP_DATA = crate::Reg<misc_otp_data::MISC_OTP_DATA_SPEC>;
#[doc = "Misc OTP Data"]
pub mod misc_otp_data;
#[doc = "IOCONF register accessor: an alias for `Reg<IOCONF_SPEC>`"]
pub type IOCONF = crate::Reg<ioconf::IOCONF_SPEC>;
#[doc = "IO Configuration"]
pub mod ioconf;
#[doc = "CONFIG_IF_ADC register accessor: an alias for `Reg<CONFIG_IF_ADC_SPEC>`"]
pub type CONFIG_IF_ADC = crate::Reg<config_if_adc::CONFIG_IF_ADC_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_if_adc;
#[doc = "CONFIG_OSC_TOP register accessor: an alias for `Reg<CONFIG_OSC_TOP_SPEC>`"]
pub type CONFIG_OSC_TOP = crate::Reg<config_osc_top::CONFIG_OSC_TOP_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_osc_top;
#[doc = "CONFIG_RF_FRONTEND register accessor: an alias for `Reg<CONFIG_RF_FRONTEND_SPEC>`"]
pub type CONFIG_RF_FRONTEND = crate::Reg<config_rf_frontend::CONFIG_RF_FRONTEND_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_frontend;
#[doc = "CONFIG_SYNTH register accessor: an alias for `Reg<CONFIG_SYNTH_SPEC>`"]
pub type CONFIG_SYNTH = crate::Reg<config_synth::CONFIG_SYNTH_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth;
#[doc = "SOC_ADC_ABS_GAIN register accessor: an alias for `Reg<SOC_ADC_ABS_GAIN_SPEC>`"]
pub type SOC_ADC_ABS_GAIN = crate::Reg<soc_adc_abs_gain::SOC_ADC_ABS_GAIN_SPEC>;
#[doc = "AUX_ADC Gain in Absolute Reference Mode"]
pub mod soc_adc_abs_gain;
#[doc = "SOC_ADC_REL_GAIN register accessor: an alias for `Reg<SOC_ADC_REL_GAIN_SPEC>`"]
pub type SOC_ADC_REL_GAIN = crate::Reg<soc_adc_rel_gain::SOC_ADC_REL_GAIN_SPEC>;
#[doc = "AUX_ADC Gain in Relative Reference Mode"]
pub mod soc_adc_rel_gain;
#[doc = "SOC_ADC_OFFSET_INT register accessor: an alias for `Reg<SOC_ADC_OFFSET_INT_SPEC>`"]
pub type SOC_ADC_OFFSET_INT = crate::Reg<soc_adc_offset_int::SOC_ADC_OFFSET_INT_SPEC>;
#[doc = "AUX_ADC Temperature Offsets in Absolute Reference Mode"]
pub mod soc_adc_offset_int;
#[doc = "SOC_ADC_REF_TRIM_AND_OFFSET_EXT register accessor: an alias for `Reg<SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC>`"]
pub type SOC_ADC_REF_TRIM_AND_OFFSET_EXT =
    crate::Reg<soc_adc_ref_trim_and_offset_ext::SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod soc_adc_ref_trim_and_offset_ext;
#[doc = "AMPCOMP_TH1 register accessor: an alias for `Reg<AMPCOMP_TH1_SPEC>`"]
pub type AMPCOMP_TH1 = crate::Reg<ampcomp_th1::AMPCOMP_TH1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_th1;
#[doc = "AMPCOMP_TH2 register accessor: an alias for `Reg<AMPCOMP_TH2_SPEC>`"]
pub type AMPCOMP_TH2 = crate::Reg<ampcomp_th2::AMPCOMP_TH2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_th2;
#[doc = "AMPCOMP_CTRL1 register accessor: an alias for `Reg<AMPCOMP_CTRL1_SPEC>`"]
pub type AMPCOMP_CTRL1 = crate::Reg<ampcomp_ctrl1::AMPCOMP_CTRL1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_ctrl1;
#[doc = "ANABYPASS_VALUE2 register accessor: an alias for `Reg<ANABYPASS_VALUE2_SPEC>`"]
pub type ANABYPASS_VALUE2 = crate::Reg<anabypass_value2::ANABYPASS_VALUE2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod anabypass_value2;
#[doc = "CONFIG_MISC_ADC register accessor: an alias for `Reg<CONFIG_MISC_ADC_SPEC>`"]
pub type CONFIG_MISC_ADC = crate::Reg<config_misc_adc::CONFIG_MISC_ADC_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_misc_adc;
#[doc = "VOLT_TRIM register accessor: an alias for `Reg<VOLT_TRIM_SPEC>`"]
pub type VOLT_TRIM = crate::Reg<volt_trim::VOLT_TRIM_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod volt_trim;
#[doc = "OSC_CONF register accessor: an alias for `Reg<OSC_CONF_SPEC>`"]
pub type OSC_CONF = crate::Reg<osc_conf::OSC_CONF_SPEC>;
#[doc = "OSC Configuration"]
pub mod osc_conf;
#[doc = "FREQ_OFFSET register accessor: an alias for `Reg<FREQ_OFFSET_SPEC>`"]
pub type FREQ_OFFSET = crate::Reg<freq_offset::FREQ_OFFSET_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod freq_offset;
#[doc = "CAP_TRIM register accessor: an alias for `Reg<CAP_TRIM_SPEC>`"]
pub type CAP_TRIM = crate::Reg<cap_trim::CAP_TRIM_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cap_trim;
#[doc = "MISC_OTP_DATA_1 register accessor: an alias for `Reg<MISC_OTP_DATA_1_SPEC>`"]
pub type MISC_OTP_DATA_1 = crate::Reg<misc_otp_data_1::MISC_OTP_DATA_1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod misc_otp_data_1;
#[doc = "PWD_CURR_20C register accessor: an alias for `Reg<PWD_CURR_20C_SPEC>`"]
pub type PWD_CURR_20C = crate::Reg<pwd_curr_20c::PWD_CURR_20C_SPEC>;
#[doc = "Power Down Current Control 20C"]
pub mod pwd_curr_20c;
#[doc = "PWD_CURR_35C register accessor: an alias for `Reg<PWD_CURR_35C_SPEC>`"]
pub type PWD_CURR_35C = crate::Reg<pwd_curr_35c::PWD_CURR_35C_SPEC>;
#[doc = "Power Down Current Control 35C"]
pub mod pwd_curr_35c;
#[doc = "PWD_CURR_50C register accessor: an alias for `Reg<PWD_CURR_50C_SPEC>`"]
pub type PWD_CURR_50C = crate::Reg<pwd_curr_50c::PWD_CURR_50C_SPEC>;
#[doc = "Power Down Current Control 50C"]
pub mod pwd_curr_50c;
#[doc = "PWD_CURR_65C register accessor: an alias for `Reg<PWD_CURR_65C_SPEC>`"]
pub type PWD_CURR_65C = crate::Reg<pwd_curr_65c::PWD_CURR_65C_SPEC>;
#[doc = "Power Down Current Control 65C"]
pub mod pwd_curr_65c;
#[doc = "PWD_CURR_80C register accessor: an alias for `Reg<PWD_CURR_80C_SPEC>`"]
pub type PWD_CURR_80C = crate::Reg<pwd_curr_80c::PWD_CURR_80C_SPEC>;
#[doc = "Power Down Current Control 80C"]
pub mod pwd_curr_80c;
#[doc = "PWD_CURR_95C register accessor: an alias for `Reg<PWD_CURR_95C_SPEC>`"]
pub type PWD_CURR_95C = crate::Reg<pwd_curr_95c::PWD_CURR_95C_SPEC>;
#[doc = "Power Down Current Control 95C"]
pub mod pwd_curr_95c;
#[doc = "PWD_CURR_110C register accessor: an alias for `Reg<PWD_CURR_110C_SPEC>`"]
pub type PWD_CURR_110C = crate::Reg<pwd_curr_110c::PWD_CURR_110C_SPEC>;
#[doc = "Power Down Current Control 110C"]
pub mod pwd_curr_110c;
#[doc = "PWD_CURR_125C register accessor: an alias for `Reg<PWD_CURR_125C_SPEC>`"]
pub type PWD_CURR_125C = crate::Reg<pwd_curr_125c::PWD_CURR_125C_SPEC>;
#[doc = "Power Down Current Control 125C"]
pub mod pwd_curr_125c;
