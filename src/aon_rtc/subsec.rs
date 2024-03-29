#[doc = "Register `SUBSEC` reader"]
pub struct R(crate::R<SUBSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUBSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUBSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUBSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUBSEC` writer"]
pub struct W(crate::W<SUBSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUBSEC_SPEC>;
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
impl From<crate::W<SUBSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUBSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - 31:0\\]
Unsigned integer representing Real Time Clock in fractions of a second (VALUE/2^32 seconds) at the time when SEC register was read. Examples : - 0x0000_0000 = 0.0 sec - 0x4000_0000 = 0.25 sec - 0x8000_0000 = 0.5 sec - 0xC000_0000 = 0.75 sec"]
pub struct VALUE_R(crate::FieldReader<u32, u32>);
impl VALUE_R {
    pub(crate) fn new(bits: u32) -> Self {
        VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALUE` writer - 31:0\\]
Unsigned integer representing Real Time Clock in fractions of a second (VALUE/2^32 seconds) at the time when SEC register was read. Examples : - 0x0000_0000 = 0.0 sec - 0x4000_0000 = 0.25 sec - 0x8000_0000 = 0.5 sec - 0xC000_0000 = 0.75 sec"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Unsigned integer representing Real Time Clock in fractions of a second (VALUE/2^32 seconds) at the time when SEC register was read. Examples : - 0x0000_0000 = 0.0 sec - 0x4000_0000 = 0.25 sec - 0x8000_0000 = 0.5 sec - 0xC000_0000 = 0.75 sec"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Unsigned integer representing Real Time Clock in fractions of a second (VALUE/2^32 seconds) at the time when SEC register was read. Examples : - 0x0000_0000 = 0.0 sec - 0x4000_0000 = 0.25 sec - 0x8000_0000 = 0.5 sec - 0xC000_0000 = 0.75 sec"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Second Counter Value, Fractional Part\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subsec](index.html) module"]
pub struct SUBSEC_SPEC;
impl crate::RegisterSpec for SUBSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [subsec::R](R) reader structure"]
impl crate::Readable for SUBSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [subsec::W](W) writer structure"]
impl crate::Writable for SUBSEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUBSEC to value 0"]
impl crate::Resettable for SUBSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
