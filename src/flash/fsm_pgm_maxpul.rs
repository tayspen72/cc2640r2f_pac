#[doc = "Register `FSM_PGM_MAXPUL` reader"]
pub struct R(crate::R<FSM_PGM_MAXPUL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_PGM_MAXPUL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_PGM_MAXPUL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_PGM_MAXPUL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_PGM_MAXPUL` writer"]
pub struct W(crate::W<FSM_PGM_MAXPUL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_PGM_MAXPUL_SPEC>;
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
impl From<crate::W<FSM_PGM_MAXPUL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_PGM_MAXPUL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED12` reader - 31:12\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED12_R(crate::FieldReader<u32, u32>);
impl RESERVED12_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED12_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED12` writer - 31:12\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | ((value as u32 & 0x000f_ffff) << 12);
        self.w
    }
}
#[doc = "Field `FSM_PGM_MAXPUL` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub struct FSM_PGM_MAXPUL_R(crate::FieldReader<u16, u16>);
impl FSM_PGM_MAXPUL_R {
    pub(crate) fn new(bits: u16) -> Self {
        FSM_PGM_MAXPUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSM_PGM_MAXPUL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSM_PGM_MAXPUL` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub struct FSM_PGM_MAXPUL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_PGM_MAXPUL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_pgm_maxpul(&self) -> FSM_PGM_MAXPUL_R {
        FSM_PGM_MAXPUL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_pgm_maxpul(&mut self) -> FSM_PGM_MAXPUL_W {
        FSM_PGM_MAXPUL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_pgm_maxpul](index.html) module"]
pub struct FSM_PGM_MAXPUL_SPEC;
impl crate::RegisterSpec for FSM_PGM_MAXPUL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_pgm_maxpul::R](R) reader structure"]
impl crate::Readable for FSM_PGM_MAXPUL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_pgm_maxpul::W](W) writer structure"]
impl crate::Writable for FSM_PGM_MAXPUL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_PGM_MAXPUL to value 0"]
impl crate::Resettable for FSM_PGM_MAXPUL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
