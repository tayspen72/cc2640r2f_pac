#[doc = "Register `HWVER1` reader"]
pub struct R(crate::R<HWVER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWVER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWVER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWVER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWVER1` writer"]
pub struct W(crate::W<HWVER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWVER1_SPEC>;
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
impl From<crate::W<HWVER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWVER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED8_R(crate::FieldReader<u32, u32>);
impl RESERVED8_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED8_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Field `REV` reader - 7:0\\]
The revision number of this module is Rev 2.0."]
pub struct REV_R(crate::FieldReader<u8, u8>);
impl REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV` writer - 7:0\\]
The revision number of this module is Rev 2.0."]
pub struct REV_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\]
The revision number of this module is Rev 2.0."]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
The revision number of this module is Rev 2.0."]
    #[inline(always)]
    pub fn rev(&mut self) -> REV_W {
        REV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HW Version 1 TRNG Revision Number\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwver1](index.html) module"]
pub struct HWVER1_SPEC;
impl crate::RegisterSpec for HWVER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwver1::R](R) reader structure"]
impl crate::Readable for HWVER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwver1::W](W) writer structure"]
impl crate::Writable for HWVER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HWVER1 to value 0x20"]
impl crate::Resettable for HWVER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
