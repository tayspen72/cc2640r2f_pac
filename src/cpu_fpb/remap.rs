#[doc = "Register `REMAP` reader"]
pub struct R(crate::R<REMAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REMAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REMAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REMAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REMAP` writer"]
pub struct W(crate::W<REMAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REMAP_SPEC>;
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
impl From<crate::W<REMAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REMAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED29` reader - 31:29\\]
This field always reads 3'b001. Writing to this field is ignored."]
pub struct RESERVED29_R(crate::FieldReader<u8, u8>);
impl RESERVED29_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED29_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED29` writer - 31:29\\]
This field always reads 3'b001. Writing to this field is ignored."]
pub struct RESERVED29_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
#[doc = "Field `REMAP` reader - 28:5\\]
Remap base address field."]
pub struct REMAP_R(crate::FieldReader<u32, u32>);
impl REMAP_R {
    pub(crate) fn new(bits: u32) -> Self {
        REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REMAP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REMAP` writer - 28:5\\]
Remap base address field."]
pub struct REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> REMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 5)) | ((value as u32 & 0x00ff_ffff) << 5);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 4:0\\]
This field always reads 0. Writing to this field is ignored."]
pub struct RESERVED0_R(crate::FieldReader<u8, u8>);
impl RESERVED0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - 4:0\\]
This field always reads 0. Writing to this field is ignored."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31 - 31:29\\]
This field always reads 3'b001. Writing to this field is ignored."]
    #[inline(always)]
    pub fn reserved29(&self) -> RESERVED29_R {
        RESERVED29_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bits 5:28 - 28:5\\]
Remap base address field."]
    #[inline(always)]
    pub fn remap(&self) -> REMAP_R {
        REMAP_R::new(((self.bits >> 5) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:4 - 4:0\\]
This field always reads 0. Writing to this field is ignored."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 29:31 - 31:29\\]
This field always reads 3'b001. Writing to this field is ignored."]
    #[inline(always)]
    pub fn reserved29(&mut self) -> RESERVED29_W {
        RESERVED29_W { w: self }
    }
    #[doc = "Bits 5:28 - 28:5\\]
Remap base address field."]
    #[inline(always)]
    pub fn remap(&mut self) -> REMAP_W {
        REMAP_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\]
This field always reads 0. Writing to this field is ignored."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Remap This register provides the remap base address location where a matched addresses are remapped. The three most significant bits and the five least significant bits of the remap base address are hard-coded to 3'b001 and 5'b00000 respectively. The remap base address must be in system space and is it required to be 8-word aligned, with one word allocated to each of the eight FPB comparators.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [remap](index.html) module"]
pub struct REMAP_SPEC;
impl crate::RegisterSpec for REMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [remap::R](R) reader structure"]
impl crate::Readable for REMAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [remap::W](W) writer structure"]
impl crate::Writable for REMAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REMAP to value 0x2000_0000"]
impl crate::Resettable for REMAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_0000
    }
}
