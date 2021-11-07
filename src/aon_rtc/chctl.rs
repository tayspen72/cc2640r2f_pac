#[doc = "Register `CHCTL` reader"]
pub struct R(crate::R<CHCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTL` writer"]
pub struct W(crate::W<CHCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTL_SPEC>;
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
impl From<crate::W<CHCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED19` reader - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED19_R(crate::FieldReader<u16, u16>);
impl RESERVED19_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED19_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED19` writer - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED19_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 19)) | ((value as u32 & 0x1fff) << 19);
        self.w
    }
}
#[doc = "Field `CH2_CONT_EN` reader - 18:18\\]
Set to enable continuous operation of Channel 2"]
pub struct CH2_CONT_EN_R(crate::FieldReader<bool, bool>);
impl CH2_CONT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2_CONT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_CONT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_CONT_EN` writer - 18:18\\]
Set to enable continuous operation of Channel 2"]
pub struct CH2_CONT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_CONT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `RESERVED17` reader - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED17_R(crate::FieldReader<bool, bool>);
impl RESERVED17_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED17` writer - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED17_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED17_W<'a> {
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
#[doc = "Field `CH2_EN` reader - 16:16\\]
RTC Channel 2 Enable 0: Disable RTC Channel 2 1: Enable RTC Channel 2"]
pub struct CH2_EN_R(crate::FieldReader<bool, bool>);
impl CH2_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_EN` writer - 16:16\\]
RTC Channel 2 Enable 0: Disable RTC Channel 2 1: Enable RTC Channel 2"]
pub struct CH2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_EN_W<'a> {
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
#[doc = "Field `RESERVED10` reader - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED10_R(crate::FieldReader<u8, u8>);
impl RESERVED10_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED10` writer - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED10_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | ((value as u32 & 0x3f) << 10);
        self.w
    }
}
#[doc = "Field `CH1_CAPT_EN` reader - 9:9\\]
Set Channel 1 mode 0: Compare mode (default) 1: Capture mode"]
pub struct CH1_CAPT_EN_R(crate::FieldReader<bool, bool>);
impl CH1_CAPT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1_CAPT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_CAPT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_CAPT_EN` writer - 9:9\\]
Set Channel 1 mode 0: Compare mode (default) 1: Capture mode"]
pub struct CH1_CAPT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_CAPT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CH1_EN` reader - 8:8\\]
RTC Channel 1 Enable 0: Disable RTC Channel 1 1: Enable RTC Channel 1"]
pub struct CH1_EN_R(crate::FieldReader<bool, bool>);
impl CH1_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_EN` writer - 8:8\\]
RTC Channel 1 Enable 0: Disable RTC Channel 1 1: Enable RTC Channel 1"]
pub struct CH1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1_R(crate::FieldReader<u8, u8>);
impl RESERVED1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | ((value as u32 & 0x7f) << 1);
        self.w
    }
}
#[doc = "Field `CH0_EN` reader - 0:0\\]
RTC Channel 0 Enable 0: Disable RTC Channel 0 1: Enable RTC Channel 0"]
pub struct CH0_EN_R(crate::FieldReader<bool, bool>);
impl CH0_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_EN` writer - 0:0\\]
RTC Channel 0 Enable 0: Disable RTC Channel 0 1: Enable RTC Channel 0"]
pub struct CH0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> RESERVED19_R {
        RESERVED19_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
    #[doc = "Bit 18 - 18:18\\]
Set to enable continuous operation of Channel 2"]
    #[inline(always)]
    pub fn ch2_cont_en(&self) -> CH2_CONT_EN_R {
        CH2_CONT_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
RTC Channel 2 Enable 0: Disable RTC Channel 2 1: Enable RTC Channel 2"]
    #[inline(always)]
    pub fn ch2_en(&self) -> CH2_EN_R {
        CH2_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Set Channel 1 mode 0: Compare mode (default) 1: Capture mode"]
    #[inline(always)]
    pub fn ch1_capt_en(&self) -> CH1_CAPT_EN_R {
        CH1_CAPT_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
RTC Channel 1 Enable 0: Disable RTC Channel 1 1: Enable RTC Channel 1"]
    #[inline(always)]
    pub fn ch1_en(&self) -> CH1_EN_R {
        CH1_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
RTC Channel 0 Enable 0: Disable RTC Channel 0 1: Enable RTC Channel 0"]
    #[inline(always)]
    pub fn ch0_en(&self) -> CH0_EN_R {
        CH0_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&mut self) -> RESERVED19_W {
        RESERVED19_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Set to enable continuous operation of Channel 2"]
    #[inline(always)]
    pub fn ch2_cont_en(&mut self) -> CH2_CONT_EN_W {
        CH2_CONT_EN_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&mut self) -> RESERVED17_W {
        RESERVED17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
RTC Channel 2 Enable 0: Disable RTC Channel 2 1: Enable RTC Channel 2"]
    #[inline(always)]
    pub fn ch2_en(&mut self) -> CH2_EN_W {
        CH2_EN_W { w: self }
    }
    #[doc = "Bits 10:15 - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&mut self) -> RESERVED10_W {
        RESERVED10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Set Channel 1 mode 0: Compare mode (default) 1: Capture mode"]
    #[inline(always)]
    pub fn ch1_capt_en(&mut self) -> CH1_CAPT_EN_W {
        CH1_CAPT_EN_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
RTC Channel 1 Enable 0: Disable RTC Channel 1 1: Enable RTC Channel 1"]
    #[inline(always)]
    pub fn ch1_en(&mut self) -> CH1_EN_W {
        CH1_EN_W { w: self }
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
RTC Channel 0 Enable 0: Disable RTC Channel 0 1: Enable RTC Channel 0"]
    #[inline(always)]
    pub fn ch0_en(&mut self) -> CH0_EN_W {
        CH0_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl](index.html) module"]
pub struct CHCTL_SPEC;
impl crate::RegisterSpec for CHCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctl::R](R) reader structure"]
impl crate::Readable for CHCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctl::W](W) writer structure"]
impl crate::Writable for CHCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHCTL to value 0"]
impl crate::Resettable for CHCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
