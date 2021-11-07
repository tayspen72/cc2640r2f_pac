#[doc = "Register `FLASH_PROG_EP` reader"]
pub struct R(crate::R<FLASH_PROG_EP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_PROG_EP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_PROG_EP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_PROG_EP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_PROG_EP` writer"]
pub struct W(crate::W<FLASH_PROG_EP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_PROG_EP_SPEC>;
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
impl From<crate::W<FLASH_PROG_EP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_PROG_EP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAX_EP` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct MAX_EP_R(crate::FieldReader<u16, u16>);
impl MAX_EP_R {
    pub(crate) fn new(bits: u16) -> Self {
        MAX_EP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAX_EP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAX_EP` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct MAX_EP_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_EP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `PROGRAM_PW` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct PROGRAM_PW_R(crate::FieldReader<u16, u16>);
impl PROGRAM_PW_R {
    pub(crate) fn new(bits: u16) -> Self {
        PROGRAM_PW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROGRAM_PW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROGRAM_PW` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct PROGRAM_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PROGRAM_PW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_ep(&self) -> MAX_EP_R {
        MAX_EP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn program_pw(&self) -> PROGRAM_PW_R {
        PROGRAM_PW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_ep(&mut self) -> MAX_EP_W {
        MAX_EP_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn program_pw(&mut self) -> PROGRAM_PW_W {
        PROGRAM_PW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_prog_ep](index.html) module"]
pub struct FLASH_PROG_EP_SPEC;
impl crate::RegisterSpec for FLASH_PROG_EP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_prog_ep::R](R) reader structure"]
impl crate::Readable for FLASH_PROG_EP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_prog_ep::W](W) writer structure"]
impl crate::Writable for FLASH_PROG_EP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_PROG_EP to value 0x0fa0_0010"]
impl crate::Resettable for FLASH_PROG_EP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fa0_0010
    }
}
