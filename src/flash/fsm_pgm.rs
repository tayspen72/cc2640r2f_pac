#[doc = "Register `FSM_PGM` reader"]
pub struct R(crate::R<FSM_PGM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_PGM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_PGM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_PGM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_PGM` writer"]
pub struct W(crate::W<FSM_PGM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_PGM_SPEC>;
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
impl From<crate::W<FSM_PGM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_PGM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED26` reader - 31:26\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED26_R(crate::FieldReader<u8, u8>);
impl RESERVED26_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED26_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED26` writer - 31:26\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED26_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | ((value as u32 & 0x3f) << 26);
        self.w
    }
}
#[doc = "Field `PGM_BANK` reader - 25:23\\]
Internal. Only to be used through TI provided API."]
pub struct PGM_BANK_R(crate::FieldReader<u8, u8>);
impl PGM_BANK_R {
    pub(crate) fn new(bits: u8) -> Self {
        PGM_BANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGM_BANK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGM_BANK` writer - 25:23\\]
Internal. Only to be used through TI provided API."]
pub struct PGM_BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_BANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | ((value as u32 & 0x07) << 23);
        self.w
    }
}
#[doc = "Field `PGM_ADDR` reader - 22:0\\]
Internal. Only to be used through TI provided API."]
pub struct PGM_ADDR_R(crate::FieldReader<u32, u32>);
impl PGM_ADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        PGM_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGM_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGM_ADDR` writer - 22:0\\]
Internal. Only to be used through TI provided API."]
pub struct PGM_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x007f_ffff) | (value as u32 & 0x007f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved26(&self) -> RESERVED26_R {
        RESERVED26_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 23:25 - 25:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_bank(&self) -> PGM_BANK_R {
        PGM_BANK_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 0:22 - 22:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_addr(&self) -> PGM_ADDR_R {
        PGM_ADDR_R::new((self.bits & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved26(&mut self) -> RESERVED26_W {
        RESERVED26_W { w: self }
    }
    #[doc = "Bits 23:25 - 25:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_bank(&mut self) -> PGM_BANK_W {
        PGM_BANK_W { w: self }
    }
    #[doc = "Bits 0:22 - 22:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_addr(&mut self) -> PGM_ADDR_W {
        PGM_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_pgm](index.html) module"]
pub struct FSM_PGM_SPEC;
impl crate::RegisterSpec for FSM_PGM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_pgm::R](R) reader structure"]
impl crate::Readable for FSM_PGM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_pgm::W](W) writer structure"]
impl crate::Writable for FSM_PGM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_PGM to value 0"]
impl crate::Resettable for FSM_PGM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
