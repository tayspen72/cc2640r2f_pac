#[doc = "Register `PDCTL0` reader"]
pub struct R(crate::R<PDCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDCTL0` writer"]
pub struct W(crate::W<PDCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCTL0_SPEC>;
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
impl From<crate::W<PDCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCTL0_SPEC>) -> Self {
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
#[doc = "Field `PERIPH_ON` reader - 2:2\\]
PERIPH Power domain. 0: PERIPH power domain is powered down 1: PERIPH power domain is powered up"]
pub struct PERIPH_ON_R(crate::FieldReader<bool, bool>);
impl PERIPH_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERIPH_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIPH_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIPH_ON` writer - 2:2\\]
PERIPH Power domain. 0: PERIPH power domain is powered down 1: PERIPH power domain is powered up"]
pub struct PERIPH_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIPH_ON_W<'a> {
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
#[doc = "Field `SERIAL_ON` reader - 1:1\\]
SERIAL Power domain. 0: SERIAL power domain is powered down 1: SERIAL power domain is powered up"]
pub struct SERIAL_ON_R(crate::FieldReader<bool, bool>);
impl SERIAL_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        SERIAL_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERIAL_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERIAL_ON` writer - 1:1\\]
SERIAL Power domain. 0: SERIAL power domain is powered down 1: SERIAL power domain is powered up"]
pub struct SERIAL_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SERIAL_ON_W<'a> {
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
#[doc = "Field `RFC_ON` reader - 0:0\\]
0: RFC power domain powered off if also PDCTL1.RFC_ON = 0 1: RFC power domain powered on"]
pub struct RFC_ON_R(crate::FieldReader<bool, bool>);
impl RFC_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFC_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFC_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFC_ON` writer - 0:0\\]
0: RFC power domain powered off if also PDCTL1.RFC_ON = 0 1: RFC power domain powered on"]
pub struct RFC_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> RFC_ON_W<'a> {
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
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
PERIPH Power domain. 0: PERIPH power domain is powered down 1: PERIPH power domain is powered up"]
    #[inline(always)]
    pub fn periph_on(&self) -> PERIPH_ON_R {
        PERIPH_ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SERIAL Power domain. 0: SERIAL power domain is powered down 1: SERIAL power domain is powered up"]
    #[inline(always)]
    pub fn serial_on(&self) -> SERIAL_ON_R {
        SERIAL_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
0: RFC power domain powered off if also PDCTL1.RFC_ON = 0 1: RFC power domain powered on"]
    #[inline(always)]
    pub fn rfc_on(&self) -> RFC_ON_R {
        RFC_ON_R::new((self.bits & 0x01) != 0)
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
PERIPH Power domain. 0: PERIPH power domain is powered down 1: PERIPH power domain is powered up"]
    #[inline(always)]
    pub fn periph_on(&mut self) -> PERIPH_ON_W {
        PERIPH_ON_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
SERIAL Power domain. 0: SERIAL power domain is powered down 1: SERIAL power domain is powered up"]
    #[inline(always)]
    pub fn serial_on(&mut self) -> SERIAL_ON_W {
        SERIAL_ON_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: RFC power domain powered off if also PDCTL1.RFC_ON = 0 1: RFC power domain powered on"]
    #[inline(always)]
    pub fn rfc_on(&mut self) -> RFC_ON_W {
        RFC_ON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Domain Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdctl0](index.html) module"]
pub struct PDCTL0_SPEC;
impl crate::RegisterSpec for PDCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdctl0::R](R) reader structure"]
impl crate::Readable for PDCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdctl0::W](W) writer structure"]
impl crate::Writable for PDCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDCTL0 to value 0"]
impl crate::Resettable for PDCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
