#[doc = "Register `FSM_SECTOR` reader"]
pub struct R(crate::R<FSM_SECTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_SECTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_SECTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_SECTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_SECTOR` writer"]
pub struct W(crate::W<FSM_SECTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_SECTOR_SPEC>;
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
impl From<crate::W<FSM_SECTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_SECTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECT_ERASED` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct SECT_ERASED_R(crate::FieldReader<u16, u16>);
impl SECT_ERASED_R {
    pub(crate) fn new(bits: u16) -> Self {
        SECT_ERASED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECT_ERASED_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECT_ERASED` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct SECT_ERASED_W<'a> {
    w: &'a mut W,
}
impl<'a> SECT_ERASED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `FSM_SECTOR_EXTENSION` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct FSM_SECTOR_EXTENSION_R(crate::FieldReader<u8, u8>);
impl FSM_SECTOR_EXTENSION_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSM_SECTOR_EXTENSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSM_SECTOR_EXTENSION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSM_SECTOR_EXTENSION` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct FSM_SECTOR_EXTENSION_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_SECTOR_EXTENSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SECTOR` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub struct SECTOR_R(crate::FieldReader<u8, u8>);
impl SECTOR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SECTOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECTOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECTOR` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub struct SECTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `SEC_OUT` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct SEC_OUT_R(crate::FieldReader<u8, u8>);
impl SEC_OUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC_OUT` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct SEC_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sect_erased(&self) -> SECT_ERASED_R {
        SECT_ERASED_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_sector_extension(&self) -> FSM_SECTOR_EXTENSION_R {
        FSM_SECTOR_EXTENSION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sector(&self) -> SECTOR_R {
        SECTOR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sec_out(&self) -> SEC_OUT_R {
        SEC_OUT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sect_erased(&mut self) -> SECT_ERASED_W {
        SECT_ERASED_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_sector_extension(&mut self) -> FSM_SECTOR_EXTENSION_W {
        FSM_SECTOR_EXTENSION_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sector(&mut self) -> SECTOR_W {
        SECTOR_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sec_out(&mut self) -> SEC_OUT_W {
        SEC_OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_sector](index.html) module"]
pub struct FSM_SECTOR_SPEC;
impl crate::RegisterSpec for FSM_SECTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_sector::R](R) reader structure"]
impl crate::Readable for FSM_SECTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_sector::W](W) writer structure"]
impl crate::Writable for FSM_SECTOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_SECTOR to value 0xffff_0000"]
impl crate::Resettable for FSM_SECTOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_0000
    }
}
