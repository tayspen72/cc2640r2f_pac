#[doc = "Register `AUXCLK` reader"]
pub struct R(crate::R<AUXCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXCLK` writer"]
pub struct W(crate::W<AUXCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXCLK_SPEC>;
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
impl From<crate::W<AUXCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED13` reader - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED13_R(crate::FieldReader<u32, u32>);
impl RESERVED13_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED13_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED13` writer - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED13_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0007_ffff << 13)) | ((value as u32 & 0x0007_ffff) << 13);
        self.w
    }
}
#[doc = "12:11\\]
When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode\n\nValue on reset: 0"]
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
#[doc = "Field `PWR_DWN_SRC` reader - 12:11\\]
When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode"]
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
#[doc = "Field `PWR_DWN_SRC` writer - 12:11\\]
When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode"]
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
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "10:8\\]
Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCLK_HF_DIV_A {
    #[doc = "7: Divide by 256"]
    DIV256 = 7,
    #[doc = "6: Divide by 128"]
    DIV128 = 6,
    #[doc = "5: Divide by 64"]
    DIV64 = 5,
    #[doc = "4: Divide by 32"]
    DIV32 = 4,
    #[doc = "3: Divide by 16"]
    DIV16 = 3,
    #[doc = "2: Divide by 8"]
    DIV8 = 2,
    #[doc = "1: Divide by 4"]
    DIV4 = 1,
    #[doc = "0: Divide by 2"]
    DIV2 = 0,
}
impl From<SCLK_HF_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLK_HF_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SCLK_HF_DIV` reader - 10:8\\]
Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX"]
pub struct SCLK_HF_DIV_R(crate::FieldReader<u8, SCLK_HF_DIV_A>);
impl SCLK_HF_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCLK_HF_DIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_HF_DIV_A {
        match self.bits {
            7 => SCLK_HF_DIV_A::DIV256,
            6 => SCLK_HF_DIV_A::DIV128,
            5 => SCLK_HF_DIV_A::DIV64,
            4 => SCLK_HF_DIV_A::DIV32,
            3 => SCLK_HF_DIV_A::DIV16,
            2 => SCLK_HF_DIV_A::DIV8,
            1 => SCLK_HF_DIV_A::DIV4,
            0 => SCLK_HF_DIV_A::DIV2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        **self == SCLK_HF_DIV_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == SCLK_HF_DIV_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == SCLK_HF_DIV_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == SCLK_HF_DIV_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == SCLK_HF_DIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == SCLK_HF_DIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == SCLK_HF_DIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == SCLK_HF_DIV_A::DIV2
    }
}
impl core::ops::Deref for SCLK_HF_DIV_R {
    type Target = crate::FieldReader<u8, SCLK_HF_DIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_HF_DIV` writer - 10:8\\]
Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX"]
pub struct SCLK_HF_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_HF_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLK_HF_DIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(SCLK_HF_DIV_A::DIV256)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(SCLK_HF_DIV_A::DIV128)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(SCLK_HF_DIV_A::DIV64)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(SCLK_HF_DIV_A::DIV32)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(SCLK_HF_DIV_A::DIV16)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(SCLK_HF_DIV_A::DIV8)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(SCLK_HF_DIV_A::DIV4)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(SCLK_HF_DIV_A::DIV2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `RESERVED3` reader - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_R(crate::FieldReader<u8, u8>);
impl RESERVED3_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "2:0\\]
Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "4: LF Clock (SCLK_LF)"]
    SCLK_LF = 4,
    #[doc = "1: HF Clock (SCLK_HF)"]
    SCLK_HF = 1,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRC` reader - 2:0\\]
Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless"]
pub struct SRC_R(crate::FieldReader<u8, SRC_A>);
impl SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRC_A> {
        match self.bits {
            4 => Some(SRC_A::SCLK_LF),
            1 => Some(SRC_A::SCLK_HF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_LF`"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        **self == SRC_A::SCLK_LF
    }
    #[doc = "Checks if the value of the field is `SCLK_HF`"]
    #[inline(always)]
    pub fn is_sclk_hf(&self) -> bool {
        **self == SRC_A::SCLK_HF
    }
}
impl core::ops::Deref for SRC_R {
    type Target = crate::FieldReader<u8, SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC` writer - 2:0\\]
Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LF Clock (SCLK_LF)"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut W {
        self.variant(SRC_A::SCLK_LF)
    }
    #[doc = "HF Clock (SCLK_HF)"]
    #[inline(always)]
    pub fn sclk_hf(self) -> &'a mut W {
        self.variant(SRC_A::SCLK_HF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new(((self.bits >> 13) & 0x0007_ffff) as u32)
    }
    #[doc = "Bits 11:12 - 12:11\\]
When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode"]
    #[inline(always)]
    pub fn pwr_dwn_src(&self) -> PWR_DWN_SRC_R {
        PWR_DWN_SRC_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX"]
    #[inline(always)]
    pub fn sclk_hf_div(&self) -> SCLK_HF_DIV_R {
        SCLK_HF_DIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&mut self) -> RESERVED13_W {
        RESERVED13_W { w: self }
    }
    #[doc = "Bits 11:12 - 12:11\\]
When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode"]
    #[inline(always)]
    pub fn pwr_dwn_src(&mut self) -> PWR_DWN_SRC_W {
        PWR_DWN_SRC_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\]
Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX"]
    #[inline(always)]
    pub fn sclk_hf_div(&mut self) -> SCLK_HF_DIV_W {
        SCLK_HF_DIV_W { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUX Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxclk](index.html) module"]
pub struct AUXCLK_SPEC;
impl crate::RegisterSpec for AUXCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auxclk::R](R) reader structure"]
impl crate::Readable for AUXCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxclk::W](W) writer structure"]
impl crate::Writable for AUXCLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUXCLK to value 0x01"]
impl crate::Resettable for AUXCLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
