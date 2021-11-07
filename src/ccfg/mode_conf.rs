#[doc = "Register `MODE_CONF` reader"]
pub struct R(crate::R<MODE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE_CONF` writer"]
pub struct W(crate::W<MODE_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MODE_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDDR_TRIM_SLEEP_DELTA` reader - 31:28\\]
Signed delta value to apply to the VDDR_TRIM_SLEEP target, minus one. See FCFG1:VOLT_TRIM.VDDR_TRIM_SLEEP_H. 0x8 (-8) : Delta = -7 ... 0xF (-1) : Delta = 0 0x0 (0) : Delta = +1 ... 0x7 (7) : Delta = +8"]
pub struct VDDR_TRIM_SLEEP_DELTA_R(crate::FieldReader<u8, u8>);
impl VDDR_TRIM_SLEEP_DELTA_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDDR_TRIM_SLEEP_DELTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDR_TRIM_SLEEP_DELTA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDR_TRIM_SLEEP_DELTA` writer - 31:28\\]
Signed delta value to apply to the VDDR_TRIM_SLEEP target, minus one. See FCFG1:VOLT_TRIM.VDDR_TRIM_SLEEP_H. 0x8 (-8) : Delta = -7 ... 0xF (-1) : Delta = 0 0x0 (0) : Delta = +1 ... 0x7 (7) : Delta = +8"]
pub struct VDDR_TRIM_SLEEP_DELTA_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_TRIM_SLEEP_DELTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `DCDC_RECHARGE` reader - 27:27\\]
DC/DC during recharge in powerdown. 0: Use the DC/DC during recharge in powerdown. 1: Do not use the DC/DC during recharge in powerdown (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub struct DCDC_RECHARGE_R(crate::FieldReader<bool, bool>);
impl DCDC_RECHARGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDC_RECHARGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC_RECHARGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDC_RECHARGE` writer - 27:27\\]
DC/DC during recharge in powerdown. 0: Use the DC/DC during recharge in powerdown. 1: Do not use the DC/DC during recharge in powerdown (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub struct DCDC_RECHARGE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_RECHARGE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `DCDC_ACTIVE` reader - 26:26\\]
DC/DC in active mode. 0: Use the DC/DC during active mode. 1: Do not use the DC/DC during active mode (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub struct DCDC_ACTIVE_R(crate::FieldReader<bool, bool>);
impl DCDC_ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDC_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDC_ACTIVE` writer - 26:26\\]
DC/DC in active mode. 0: Use the DC/DC during active mode. 1: Do not use the DC/DC during active mode (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub struct DCDC_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_ACTIVE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `VDDR_EXT_LOAD` reader - 25:25\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct VDDR_EXT_LOAD_R(crate::FieldReader<bool, bool>);
impl VDDR_EXT_LOAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        VDDR_EXT_LOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDR_EXT_LOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDR_EXT_LOAD` writer - 25:25\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct VDDR_EXT_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_EXT_LOAD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `VDDS_BOD_LEVEL` reader - 24:24\\]
VDDS BOD level. 0: VDDS BOD level is 2.0 V (necessary for maximum PA output power on CC13x0). 1: VDDS BOD level is 1.8 V (or 1.7 V for external regulator mode) (default)."]
pub struct VDDS_BOD_LEVEL_R(crate::FieldReader<bool, bool>);
impl VDDS_BOD_LEVEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VDDS_BOD_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDS_BOD_LEVEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDS_BOD_LEVEL` writer - 24:24\\]
VDDS BOD level. 0: VDDS BOD level is 2.0 V (necessary for maximum PA output power on CC13x0). 1: VDDS BOD level is 1.8 V (or 1.7 V for external regulator mode) (default)."]
pub struct VDDS_BOD_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDS_BOD_LEVEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "23:22\\]
Select source for SCLK_LF.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCLK_LF_OPTION_A {
    #[doc = "3: Low frequency RCOSC (default)"]
    RCOSC_LF = 3,
    #[doc = "2: 32.768kHz low frequency XOSC"]
    XOSC_LF = 2,
    #[doc = "1: External low frequency clock on DIO defined by EXT_LF_CLK.DIO. The RTC tick speed AON_RTC:SUBSECINC is updated to EXT_LF_CLK.RTC_INCREMENT (done in the trimDevice() xxWare boot function). External clock must always be running when the chip is in standby for VDDR recharge timing."]
    EXTERNAL_LF = 1,
    #[doc = "0: 31.25kHz clock derived from 24MHz XOSC (dividing by 768 in HW). The RTC tick speed \\[AON_RTC.SUBSECINC.*\\]
is updated to 0x8637BD, corresponding to a 31.25kHz clock (done in the trimDevice() xxWare boot function). Standby power mode is not supported when using this clock source."]
    XOSC_HF_DLF = 0,
}
impl From<SCLK_LF_OPTION_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLK_LF_OPTION_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SCLK_LF_OPTION` reader - 23:22\\]
Select source for SCLK_LF."]
pub struct SCLK_LF_OPTION_R(crate::FieldReader<u8, SCLK_LF_OPTION_A>);
impl SCLK_LF_OPTION_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCLK_LF_OPTION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_LF_OPTION_A {
        match self.bits {
            3 => SCLK_LF_OPTION_A::RCOSC_LF,
            2 => SCLK_LF_OPTION_A::XOSC_LF,
            1 => SCLK_LF_OPTION_A::EXTERNAL_LF,
            0 => SCLK_LF_OPTION_A::XOSC_HF_DLF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RCOSC_LF`"]
    #[inline(always)]
    pub fn is_rcosc_lf(&self) -> bool {
        **self == SCLK_LF_OPTION_A::RCOSC_LF
    }
    #[doc = "Checks if the value of the field is `XOSC_LF`"]
    #[inline(always)]
    pub fn is_xosc_lf(&self) -> bool {
        **self == SCLK_LF_OPTION_A::XOSC_LF
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_LF`"]
    #[inline(always)]
    pub fn is_external_lf(&self) -> bool {
        **self == SCLK_LF_OPTION_A::EXTERNAL_LF
    }
    #[doc = "Checks if the value of the field is `XOSC_HF_DLF`"]
    #[inline(always)]
    pub fn is_xosc_hf_dlf(&self) -> bool {
        **self == SCLK_LF_OPTION_A::XOSC_HF_DLF
    }
}
impl core::ops::Deref for SCLK_LF_OPTION_R {
    type Target = crate::FieldReader<u8, SCLK_LF_OPTION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_LF_OPTION` writer - 23:22\\]
Select source for SCLK_LF."]
pub struct SCLK_LF_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_LF_OPTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLK_LF_OPTION_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low frequency RCOSC (default)"]
    #[inline(always)]
    pub fn rcosc_lf(self) -> &'a mut W {
        self.variant(SCLK_LF_OPTION_A::RCOSC_LF)
    }
    #[doc = "32.768kHz low frequency XOSC"]
    #[inline(always)]
    pub fn xosc_lf(self) -> &'a mut W {
        self.variant(SCLK_LF_OPTION_A::XOSC_LF)
    }
    #[doc = "External low frequency clock on DIO defined by EXT_LF_CLK.DIO. The RTC tick speed AON_RTC:SUBSECINC is updated to EXT_LF_CLK.RTC_INCREMENT (done in the trimDevice() xxWare boot function). External clock must always be running when the chip is in standby for VDDR recharge timing."]
    #[inline(always)]
    pub fn external_lf(self) -> &'a mut W {
        self.variant(SCLK_LF_OPTION_A::EXTERNAL_LF)
    }
    #[doc = "31.25kHz clock derived from 24MHz XOSC (dividing by 768 in HW). The RTC tick speed \\[AON_RTC.SUBSECINC.*\\]
is updated to 0x8637BD, corresponding to a 31.25kHz clock (done in the trimDevice() xxWare boot function). Standby power mode is not supported when using this clock source."]
    #[inline(always)]
    pub fn xosc_hf_dlf(self) -> &'a mut W {
        self.variant(SCLK_LF_OPTION_A::XOSC_HF_DLF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `VDDR_TRIM_SLEEP_TC` reader - 21:21\\]
0x1: VDDR_TRIM_SLEEP_DELTA is not temperature compensated 0x0: RTOS/driver temperature compensates VDDR_TRIM_SLEEP_DELTA every time standby mode is entered. This improves low-temperature RCOSC_LF frequency stability in standby mode. When temperature compensation is performed, the delta is calculates this way: Delta = max (delta, min(8, floor(62-temp)/8)) Here, delta is given by VDDR_TRIM_SLEEP_DELTA, and temp is the current temperature in degrees C."]
pub struct VDDR_TRIM_SLEEP_TC_R(crate::FieldReader<bool, bool>);
impl VDDR_TRIM_SLEEP_TC_R {
    pub(crate) fn new(bits: bool) -> Self {
        VDDR_TRIM_SLEEP_TC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDR_TRIM_SLEEP_TC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDR_TRIM_SLEEP_TC` writer - 21:21\\]
0x1: VDDR_TRIM_SLEEP_DELTA is not temperature compensated 0x0: RTOS/driver temperature compensates VDDR_TRIM_SLEEP_DELTA every time standby mode is entered. This improves low-temperature RCOSC_LF frequency stability in standby mode. When temperature compensation is performed, the delta is calculates this way: Delta = max (delta, min(8, floor(62-temp)/8)) Here, delta is given by VDDR_TRIM_SLEEP_DELTA, and temp is the current temperature in degrees C."]
pub struct VDDR_TRIM_SLEEP_TC_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_TRIM_SLEEP_TC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `RTC_COMP` reader - 20:20\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct RTC_COMP_R(crate::FieldReader<bool, bool>);
impl RTC_COMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_COMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_COMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_COMP` writer - 20:20\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct RTC_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_COMP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "19:18\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XOSC_FREQ_A {
    #[doc = "3: 24 MHz XOSC_HF"]
    _24M = 3,
    #[doc = "2: 48 MHz XOSC_HF"]
    _48M = 2,
    #[doc = "1: HPOSC"]
    HPOSC = 1,
}
impl From<XOSC_FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: XOSC_FREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `XOSC_FREQ` reader - 19:18\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct XOSC_FREQ_R(crate::FieldReader<u8, XOSC_FREQ_A>);
impl XOSC_FREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        XOSC_FREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<XOSC_FREQ_A> {
        match self.bits {
            3 => Some(XOSC_FREQ_A::_24M),
            2 => Some(XOSC_FREQ_A::_48M),
            1 => Some(XOSC_FREQ_A::HPOSC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_24M`"]
    #[inline(always)]
    pub fn is_24m(&self) -> bool {
        **self == XOSC_FREQ_A::_24M
    }
    #[doc = "Checks if the value of the field is `_48M`"]
    #[inline(always)]
    pub fn is_48m(&self) -> bool {
        **self == XOSC_FREQ_A::_48M
    }
    #[doc = "Checks if the value of the field is `HPOSC`"]
    #[inline(always)]
    pub fn is_hposc(&self) -> bool {
        **self == XOSC_FREQ_A::HPOSC
    }
}
impl core::ops::Deref for XOSC_FREQ_R {
    type Target = crate::FieldReader<u8, XOSC_FREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC_FREQ` writer - 19:18\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct XOSC_FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_FREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XOSC_FREQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "24 MHz XOSC_HF"]
    #[inline(always)]
    pub fn _24m(self) -> &'a mut W {
        self.variant(XOSC_FREQ_A::_24M)
    }
    #[doc = "48 MHz XOSC_HF"]
    #[inline(always)]
    pub fn _48m(self) -> &'a mut W {
        self.variant(XOSC_FREQ_A::_48M)
    }
    #[doc = "HPOSC"]
    #[inline(always)]
    pub fn hposc(self) -> &'a mut W {
        self.variant(XOSC_FREQ_A::HPOSC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `XOSC_CAP_MOD` reader - 17:17\\]
Enable modification (delta) to XOSC cap-array. Value specified in XOSC_CAPARRAY_DELTA. 0: Apply cap-array delta 1: Do not apply cap-array delta (default)"]
pub struct XOSC_CAP_MOD_R(crate::FieldReader<bool, bool>);
impl XOSC_CAP_MOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        XOSC_CAP_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC_CAP_MOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC_CAP_MOD` writer - 17:17\\]
Enable modification (delta) to XOSC cap-array. Value specified in XOSC_CAPARRAY_DELTA. 0: Apply cap-array delta 1: Do not apply cap-array delta (default)"]
pub struct XOSC_CAP_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_CAP_MOD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `HF_COMP` reader - 16:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct HF_COMP_R(crate::FieldReader<bool, bool>);
impl HF_COMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        HF_COMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HF_COMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HF_COMP` writer - 16:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct HF_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> HF_COMP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `XOSC_CAPARRAY_DELTA` reader - 15:8\\]
Signed 8-bit value, directly modifying trimmed XOSC cap-array step value. Enabled by XOSC_CAP_MOD."]
pub struct XOSC_CAPARRAY_DELTA_R(crate::FieldReader<u8, u8>);
impl XOSC_CAPARRAY_DELTA_R {
    pub(crate) fn new(bits: u8) -> Self {
        XOSC_CAPARRAY_DELTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC_CAPARRAY_DELTA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC_CAPARRAY_DELTA` writer - 15:8\\]
Signed 8-bit value, directly modifying trimmed XOSC cap-array step value. Enabled by XOSC_CAP_MOD."]
pub struct XOSC_CAPARRAY_DELTA_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_CAPARRAY_DELTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `VDDR_CAP` reader - 7:0\\]
Unsigned 8-bit integer, representing the minimum decoupling capacitance (worst case) on VDDR, in units of 100nF. This should take into account capacitor tolerance and voltage dependent capacitance variation. This bit affects the recharge period calculation when going into powerdown or standby. NOTE! If using the following functions this field must be configured (used by TI RTOS): SysCtrlSetRechargeBeforePowerDown() SysCtrlAdjustRechargeAfterPowerDown()"]
pub struct VDDR_CAP_R(crate::FieldReader<u8, u8>);
impl VDDR_CAP_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDDR_CAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDR_CAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDR_CAP` writer - 7:0\\]
Unsigned 8-bit integer, representing the minimum decoupling capacitance (worst case) on VDDR, in units of 100nF. This should take into account capacitor tolerance and voltage dependent capacitance variation. This bit affects the recharge period calculation when going into powerdown or standby. NOTE! If using the following functions this field must be configured (used by TI RTOS): SysCtrlSetRechargeBeforePowerDown() SysCtrlAdjustRechargeAfterPowerDown()"]
pub struct VDDR_CAP_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_CAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Signed delta value to apply to the VDDR_TRIM_SLEEP target, minus one. See FCFG1:VOLT_TRIM.VDDR_TRIM_SLEEP_H. 0x8 (-8) : Delta = -7 ... 0xF (-1) : Delta = 0 0x0 (0) : Delta = +1 ... 0x7 (7) : Delta = +8"]
    #[inline(always)]
    pub fn vddr_trim_sleep_delta(&self) -> VDDR_TRIM_SLEEP_DELTA_R {
        VDDR_TRIM_SLEEP_DELTA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
DC/DC during recharge in powerdown. 0: Use the DC/DC during recharge in powerdown. 1: Do not use the DC/DC during recharge in powerdown (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn dcdc_recharge(&self) -> DCDC_RECHARGE_R {
        DCDC_RECHARGE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
DC/DC in active mode. 0: Use the DC/DC during active mode. 1: Do not use the DC/DC during active mode (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn dcdc_active(&self) -> DCDC_ACTIVE_R {
        DCDC_ACTIVE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_load(&self) -> VDDR_EXT_LOAD_R {
        VDDR_EXT_LOAD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
VDDS BOD level. 0: VDDS BOD level is 2.0 V (necessary for maximum PA output power on CC13x0). 1: VDDS BOD level is 1.8 V (or 1.7 V for external regulator mode) (default)."]
    #[inline(always)]
    pub fn vdds_bod_level(&self) -> VDDS_BOD_LEVEL_R {
        VDDS_BOD_LEVEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Select source for SCLK_LF."]
    #[inline(always)]
    pub fn sclk_lf_option(&self) -> SCLK_LF_OPTION_R {
        SCLK_LF_OPTION_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
0x1: VDDR_TRIM_SLEEP_DELTA is not temperature compensated 0x0: RTOS/driver temperature compensates VDDR_TRIM_SLEEP_DELTA every time standby mode is entered. This improves low-temperature RCOSC_LF frequency stability in standby mode. When temperature compensation is performed, the delta is calculates this way: Delta = max (delta, min(8, floor(62-temp)/8)) Here, delta is given by VDDR_TRIM_SLEEP_DELTA, and temp is the current temperature in degrees C."]
    #[inline(always)]
    pub fn vddr_trim_sleep_tc(&self) -> VDDR_TRIM_SLEEP_TC_R {
        VDDR_TRIM_SLEEP_TC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn rtc_comp(&self) -> RTC_COMP_R {
        RTC_COMP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn xosc_freq(&self) -> XOSC_FREQ_R {
        XOSC_FREQ_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17 - 17:17\\]
Enable modification (delta) to XOSC cap-array. Value specified in XOSC_CAPARRAY_DELTA. 0: Apply cap-array delta 1: Do not apply cap-array delta (default)"]
    #[inline(always)]
    pub fn xosc_cap_mod(&self) -> XOSC_CAP_MOD_R {
        XOSC_CAP_MOD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp(&self) -> HF_COMP_R {
        HF_COMP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Signed 8-bit value, directly modifying trimmed XOSC cap-array step value. Enabled by XOSC_CAP_MOD."]
    #[inline(always)]
    pub fn xosc_caparray_delta(&self) -> XOSC_CAPARRAY_DELTA_R {
        XOSC_CAPARRAY_DELTA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Unsigned 8-bit integer, representing the minimum decoupling capacitance (worst case) on VDDR, in units of 100nF. This should take into account capacitor tolerance and voltage dependent capacitance variation. This bit affects the recharge period calculation when going into powerdown or standby. NOTE! If using the following functions this field must be configured (used by TI RTOS): SysCtrlSetRechargeBeforePowerDown() SysCtrlAdjustRechargeAfterPowerDown()"]
    #[inline(always)]
    pub fn vddr_cap(&self) -> VDDR_CAP_R {
        VDDR_CAP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Signed delta value to apply to the VDDR_TRIM_SLEEP target, minus one. See FCFG1:VOLT_TRIM.VDDR_TRIM_SLEEP_H. 0x8 (-8) : Delta = -7 ... 0xF (-1) : Delta = 0 0x0 (0) : Delta = +1 ... 0x7 (7) : Delta = +8"]
    #[inline(always)]
    pub fn vddr_trim_sleep_delta(&mut self) -> VDDR_TRIM_SLEEP_DELTA_W {
        VDDR_TRIM_SLEEP_DELTA_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
DC/DC during recharge in powerdown. 0: Use the DC/DC during recharge in powerdown. 1: Do not use the DC/DC during recharge in powerdown (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn dcdc_recharge(&mut self) -> DCDC_RECHARGE_W {
        DCDC_RECHARGE_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
DC/DC in active mode. 0: Use the DC/DC during active mode. 1: Do not use the DC/DC during active mode (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn dcdc_active(&mut self) -> DCDC_ACTIVE_W {
        DCDC_ACTIVE_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_load(&mut self) -> VDDR_EXT_LOAD_W {
        VDDR_EXT_LOAD_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
VDDS BOD level. 0: VDDS BOD level is 2.0 V (necessary for maximum PA output power on CC13x0). 1: VDDS BOD level is 1.8 V (or 1.7 V for external regulator mode) (default)."]
    #[inline(always)]
    pub fn vdds_bod_level(&mut self) -> VDDS_BOD_LEVEL_W {
        VDDS_BOD_LEVEL_W { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\]
Select source for SCLK_LF."]
    #[inline(always)]
    pub fn sclk_lf_option(&mut self) -> SCLK_LF_OPTION_W {
        SCLK_LF_OPTION_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
0x1: VDDR_TRIM_SLEEP_DELTA is not temperature compensated 0x0: RTOS/driver temperature compensates VDDR_TRIM_SLEEP_DELTA every time standby mode is entered. This improves low-temperature RCOSC_LF frequency stability in standby mode. When temperature compensation is performed, the delta is calculates this way: Delta = max (delta, min(8, floor(62-temp)/8)) Here, delta is given by VDDR_TRIM_SLEEP_DELTA, and temp is the current temperature in degrees C."]
    #[inline(always)]
    pub fn vddr_trim_sleep_tc(&mut self) -> VDDR_TRIM_SLEEP_TC_W {
        VDDR_TRIM_SLEEP_TC_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn rtc_comp(&mut self) -> RTC_COMP_W {
        RTC_COMP_W { w: self }
    }
    #[doc = "Bits 18:19 - 19:18\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn xosc_freq(&mut self) -> XOSC_FREQ_W {
        XOSC_FREQ_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Enable modification (delta) to XOSC cap-array. Value specified in XOSC_CAPARRAY_DELTA. 0: Apply cap-array delta 1: Do not apply cap-array delta (default)"]
    #[inline(always)]
    pub fn xosc_cap_mod(&mut self) -> XOSC_CAP_MOD_W {
        XOSC_CAP_MOD_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp(&mut self) -> HF_COMP_W {
        HF_COMP_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Signed 8-bit value, directly modifying trimmed XOSC cap-array step value. Enabled by XOSC_CAP_MOD."]
    #[inline(always)]
    pub fn xosc_caparray_delta(&mut self) -> XOSC_CAPARRAY_DELTA_W {
        XOSC_CAPARRAY_DELTA_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Unsigned 8-bit integer, representing the minimum decoupling capacitance (worst case) on VDDR, in units of 100nF. This should take into account capacitor tolerance and voltage dependent capacitance variation. This bit affects the recharge period calculation when going into powerdown or standby. NOTE! If using the following functions this field must be configured (used by TI RTOS): SysCtrlSetRechargeBeforePowerDown() SysCtrlAdjustRechargeAfterPowerDown()"]
    #[inline(always)]
    pub fn vddr_cap(&mut self) -> VDDR_CAP_W {
        VDDR_CAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode_conf](index.html) module"]
pub struct MODE_CONF_SPEC;
impl crate::RegisterSpec for MODE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode_conf::R](R) reader structure"]
impl crate::Readable for MODE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode_conf::W](W) writer structure"]
impl crate::Writable for MODE_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE_CONF to value 0xffff_ffff"]
impl crate::Resettable for MODE_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
