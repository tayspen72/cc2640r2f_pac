#[doc = "Register `FETCHSTAT` reader"]
pub struct R(crate::R<FETCHSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FETCHSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FETCHSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FETCHSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FETCHSTAT` writer"]
pub struct W(crate::W<FETCHSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FETCHSTAT_SPEC>;
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
impl From<crate::W<FETCHSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FETCHSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPCODE` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct OPCODE_R(crate::FieldReader<u16, u16>);
impl OPCODE_R {
    pub(crate) fn new(bits: u16) -> Self {
        OPCODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPCODE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPCODE` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct OPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `PC` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct PC_R(crate::FieldReader<u16, u16>);
impl PC_R {
    pub(crate) fn new(bits: u16) -> Self {
        PC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct PC_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_W<'a> {
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
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn opcode(&mut self) -> OPCODE_W {
        OPCODE_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W {
        PC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fetchstat](index.html) module"]
pub struct FETCHSTAT_SPEC;
impl crate::RegisterSpec for FETCHSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fetchstat::R](R) reader structure"]
impl crate::Readable for FETCHSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fetchstat::W](W) writer structure"]
impl crate::Writable for FETCHSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FETCHSTAT to value 0"]
impl crate::Resettable for FETCHSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
