#[doc = "Register `SSPSR` reader"]
pub struct R(crate::R<SSPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSPSR` writer"]
pub struct W(crate::W<SSPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSPSR_SPEC>;
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
impl From<crate::W<SSPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_R(crate::FieldReader<u32, u32>);
impl RESERVED4_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | ((value as u32 & 0x0fff_ffff) << 4);
        self.w
    }
}
#[doc = "Field `FOUR` reader - 3:3\\]
4-bit port size support 0x0: Not supported 0x1: Supported"]
pub struct FOUR_R(crate::FieldReader<bool, bool>);
impl FOUR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FOUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FOUR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FOUR` writer - 3:3\\]
4-bit port size support 0x0: Not supported 0x1: Supported"]
pub struct FOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> FOUR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `THREE` reader - 2:2\\]
3-bit port size support 0x0: Not supported 0x1: Supported"]
pub struct THREE_R(crate::FieldReader<bool, bool>);
impl THREE_R {
    pub(crate) fn new(bits: bool) -> Self {
        THREE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THREE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THREE` writer - 2:2\\]
3-bit port size support 0x0: Not supported 0x1: Supported"]
pub struct THREE_W<'a> {
    w: &'a mut W,
}
impl<'a> THREE_W<'a> {
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
#[doc = "Field `TWO` reader - 1:1\\]
2-bit port size support 0x0: Not supported 0x1: Supported"]
pub struct TWO_R(crate::FieldReader<bool, bool>);
impl TWO_R {
    pub(crate) fn new(bits: bool) -> Self {
        TWO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWO` writer - 1:1\\]
2-bit port size support 0x0: Not supported 0x1: Supported"]
pub struct TWO_W<'a> {
    w: &'a mut W,
}
impl<'a> TWO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `ONE` reader - 0:0\\]
1-bit port size support 0x0: Not supported 0x1: Supported"]
pub struct ONE_R(crate::FieldReader<bool, bool>);
impl ONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONE` writer - 0:0\\]
1-bit port size support 0x0: Not supported 0x1: Supported"]
pub struct ONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ONE_W<'a> {
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
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 3 - 3:3\\]
4-bit port size support 0x0: Not supported 0x1: Supported"]
    #[inline(always)]
    pub fn four(&self) -> FOUR_R {
        FOUR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
3-bit port size support 0x0: Not supported 0x1: Supported"]
    #[inline(always)]
    pub fn three(&self) -> THREE_R {
        THREE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
2-bit port size support 0x0: Not supported 0x1: Supported"]
    #[inline(always)]
    pub fn two(&self) -> TWO_R {
        TWO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
1-bit port size support 0x0: Not supported 0x1: Supported"]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
4-bit port size support 0x0: Not supported 0x1: Supported"]
    #[inline(always)]
    pub fn four(&mut self) -> FOUR_W {
        FOUR_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
3-bit port size support 0x0: Not supported 0x1: Supported"]
    #[inline(always)]
    pub fn three(&mut self) -> THREE_W {
        THREE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
2-bit port size support 0x0: Not supported 0x1: Supported"]
    #[inline(always)]
    pub fn two(&mut self) -> TWO_W {
        TWO_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
1-bit port size support 0x0: Not supported 0x1: Supported"]
    #[inline(always)]
    pub fn one(&mut self) -> ONE_W {
        ONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supported Sync Port Sizes This register represents a single port size that is supported on the device, that is, 4, 2 or 1. This is to ensure that tools do not attempt to select a port width that an attached TPA cannot capture.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspsr](index.html) module"]
pub struct SSPSR_SPEC;
impl crate::RegisterSpec for SSPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspsr::R](R) reader structure"]
impl crate::Readable for SSPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sspsr::W](W) writer structure"]
impl crate::Writable for SSPSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSPSR to value 0x0b"]
impl crate::Resettable for SSPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0b
    }
}
