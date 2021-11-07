#[doc = "Register `FSM_PE_VSU` reader"]
pub struct R(crate::R<FSM_PE_VSU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_PE_VSU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_PE_VSU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_PE_VSU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_PE_VSU` writer"]
pub struct W(crate::W<FSM_PE_VSU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_PE_VSU_SPEC>;
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
impl From<crate::W<FSM_PE_VSU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_PE_VSU_SPEC>) -> Self {
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
#[doc = "Field `PGM_VSU` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct PGM_VSU_R(crate::FieldReader<u8, u8>);
impl PGM_VSU_R {
    pub(crate) fn new(bits: u8) -> Self {
        PGM_VSU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGM_VSU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGM_VSU` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct PGM_VSU_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_VSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ERA_VSU` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct ERA_VSU_R(crate::FieldReader<u8, u8>);
impl ERA_VSU_R {
    pub(crate) fn new(bits: u8) -> Self {
        ERA_VSU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERA_VSU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERA_VSU` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct ERA_VSU_W<'a> {
    w: &'a mut W,
}
impl<'a> ERA_VSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
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
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_vsu(&self) -> PGM_VSU_R {
        PGM_VSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_vsu(&self) -> ERA_VSU_R {
        ERA_VSU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_vsu(&mut self) -> PGM_VSU_W {
        PGM_VSU_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_vsu(&mut self) -> ERA_VSU_W {
        ERA_VSU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_pe_vsu](index.html) module"]
pub struct FSM_PE_VSU_SPEC;
impl crate::RegisterSpec for FSM_PE_VSU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_pe_vsu::R](R) reader structure"]
impl crate::Readable for FSM_PE_VSU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_pe_vsu::W](W) writer structure"]
impl crate::Writable for FSM_PE_VSU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_PE_VSU to value 0"]
impl crate::Resettable for FSM_PE_VSU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
