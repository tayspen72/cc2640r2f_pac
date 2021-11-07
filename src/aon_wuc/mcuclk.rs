#[doc = "Register `MCUCLK` reader"]
pub struct R(crate::R<MCUCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUCLK` writer"]
pub struct W(crate::W<MCUCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUCLK_SPEC>;
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
impl From<crate::W<MCUCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_R(crate::FieldReader<u32, u32>);
impl RESERVED3_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Field `RCOSC_HF_CAL_DONE` reader - 2:2\\]
MCU bootcode will set this bit when RCOSC_HF is calibrated. The FLASH can not be used until this bit is set. 1: RCOSC_HF is calibrated to 48 MHz, allowing FLASH to power up. 0: RCOSC_HF is not yet calibrated, ie FLASH must not assume that the SCLK_HF is safe"]
pub struct RCOSC_HF_CAL_DONE_R(crate::FieldReader<bool, bool>);
impl RCOSC_HF_CAL_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCOSC_HF_CAL_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCOSC_HF_CAL_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCOSC_HF_CAL_DONE` writer - 2:2\\]
MCU bootcode will set this bit when RCOSC_HF is calibrated. The FLASH can not be used until this bit is set. 1: RCOSC_HF is calibrated to 48 MHz, allowing FLASH to power up. 0: RCOSC_HF is not yet calibrated, ie FLASH must not assume that the SCLK_HF is safe"]
pub struct RCOSC_HF_CAL_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_HF_CAL_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "1:0\\]
Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWR_DWN_SRC_A {
    #[doc = "1: Use SCLK_LF in Powerdown"]
    SCLK_LF = 1,
    #[doc = "0: No clock in Powerdown"]
    NONE = 0,
}
impl From<PWR_DWN_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PWR_DWN_SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWR_DWN_SRC` reader - 1:0\\]
Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode."]
pub struct PWR_DWN_SRC_R(crate::FieldReader<u8, PWR_DWN_SRC_A>);
impl PWR_DWN_SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_DWN_SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWR_DWN_SRC_A> {
        match self.bits {
            1 => Some(PWR_DWN_SRC_A::SCLK_LF),
            0 => Some(PWR_DWN_SRC_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        **self == PWR_DWN_SRC_A::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == PWR_DWN_SRC_A::NONE
    }
}
impl core::ops::Deref for PWR_DWN_SRC_R {
    type Target = crate::FieldReader<u8, PWR_DWN_SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_DWN_SRC` writer - 1:0\\]
Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode."]
pub struct PWR_DWN_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_DWN_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWR_DWN_SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Use SCLK_LF in Powerdown"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(PWR_DWN_SRC_A::SCLK_LF)
    }
    #[doc = "No clock in Powerdown"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PWR_DWN_SRC_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
MCU bootcode will set this bit when RCOSC_HF is calibrated. The FLASH can not be used until this bit is set. 1: RCOSC_HF is calibrated to 48 MHz, allowing FLASH to power up. 0: RCOSC_HF is not yet calibrated, ie FLASH must not assume that the SCLK_HF is safe"]
    #[inline(always)]
    pub fn rcosc_hf_cal_done(&self) -> RCOSC_HF_CAL_DONE_R {
        RCOSC_HF_CAL_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode."]
    #[inline(always)]
    pub fn pwr_dwn_src(&self) -> PWR_DWN_SRC_R {
        PWR_DWN_SRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
MCU bootcode will set this bit when RCOSC_HF is calibrated. The FLASH can not be used until this bit is set. 1: RCOSC_HF is calibrated to 48 MHz, allowing FLASH to power up. 0: RCOSC_HF is not yet calibrated, ie FLASH must not assume that the SCLK_HF is safe"]
    #[inline(always)]
    pub fn rcosc_hf_cal_done(&mut self) -> RCOSC_HF_CAL_DONE_W {
        RCOSC_HF_CAL_DONE_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode."]
    #[inline(always)]
    pub fn pwr_dwn_src(&mut self) -> PWR_DWN_SRC_W {
        PWR_DWN_SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCU Clock Management This register contains bitfields related to the MCU clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcuclk](index.html) module"]
pub struct MCUCLK_SPEC;
impl crate::RegisterSpec for MCUCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcuclk::R](R) reader structure"]
impl crate::Readable for MCUCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcuclk::W](W) writer structure"]
impl crate::Writable for MCUCLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCUCLK to value 0"]
impl crate::Resettable for MCUCLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
