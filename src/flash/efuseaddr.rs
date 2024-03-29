#[doc = "Register `EFUSEADDR` reader"]
pub struct R(crate::R<EFUSEADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSEADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSEADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSEADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSEADDR` writer"]
pub struct W(crate::W<EFUSEADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSEADDR_SPEC>;
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
impl From<crate::W<EFUSEADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSEADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED16_R(crate::FieldReader<u16, u16>);
impl RESERVED16_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED16_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `BLOCK` reader - 15:11\\]
Internal. Only to be used through TI provided API."]
pub struct BLOCK_R(crate::FieldReader<u8, u8>);
impl BLOCK_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLOCK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCK` writer - 15:11\\]
Internal. Only to be used through TI provided API."]
pub struct BLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Field `ROW` reader - 10:0\\]
Internal. Only to be used through TI provided API."]
pub struct ROW_R(crate::FieldReader<u16, u16>);
impl ROW_R {
    pub(crate) fn new(bits: u16) -> Self {
        ROW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROW` writer - 10:0\\]
Internal. Only to be used through TI provided API."]
pub struct ROW_W<'a> {
    w: &'a mut W,
}
impl<'a> ROW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 0:10 - 10:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn row(&self) -> ROW_R {
        ROW_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn block(&mut self) -> BLOCK_W {
        BLOCK_W { w: self }
    }
    #[doc = "Bits 0:10 - 10:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn row(&mut self) -> ROW_W {
        ROW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuseaddr](index.html) module"]
pub struct EFUSEADDR_SPEC;
impl crate::RegisterSpec for EFUSEADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuseaddr::R](R) reader structure"]
impl crate::Readable for EFUSEADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuseaddr::W](W) writer structure"]
impl crate::Writable for EFUSEADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EFUSEADDR to value 0"]
impl crate::Resettable for EFUSEADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
