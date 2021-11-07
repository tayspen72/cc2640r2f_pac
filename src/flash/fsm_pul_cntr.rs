#[doc = "Register `FSM_PUL_CNTR` reader"]
pub struct R(crate::R<FSM_PUL_CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_PUL_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_PUL_CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_PUL_CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_PUL_CNTR` writer"]
pub struct W(crate::W<FSM_PUL_CNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_PUL_CNTR_SPEC>;
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
impl From<crate::W<FSM_PUL_CNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_PUL_CNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED25` reader - 31:25\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED25_R(crate::FieldReader<u8, u8>);
impl RESERVED25_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED25_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED25` writer - 31:25\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED25_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
#[doc = "Field `CUR_EC_LEVEL` reader - 24:16\\]
Internal. Only to be used through TI provided API."]
pub struct CUR_EC_LEVEL_R(crate::FieldReader<u16, u16>);
impl CUR_EC_LEVEL_R {
    pub(crate) fn new(bits: u16) -> Self {
        CUR_EC_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CUR_EC_LEVEL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CUR_EC_LEVEL` writer - 24:16\\]
Internal. Only to be used through TI provided API."]
pub struct CUR_EC_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CUR_EC_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED12` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED12_R(crate::FieldReader<u8, u8>);
impl RESERVED12_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED12` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PUL_CNTR` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub struct PUL_CNTR_R(crate::FieldReader<u16, u16>);
impl PUL_CNTR_R {
    pub(crate) fn new(bits: u16) -> Self {
        PUL_CNTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUL_CNTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUL_CNTR` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub struct PUL_CNTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PUL_CNTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cur_ec_level(&self) -> CUR_EC_LEVEL_R {
        CUR_EC_LEVEL_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pul_cntr(&self) -> PUL_CNTR_R {
        PUL_CNTR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&mut self) -> RESERVED25_W {
        RESERVED25_W { w: self }
    }
    #[doc = "Bits 16:24 - 24:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cur_ec_level(&mut self) -> CUR_EC_LEVEL_W {
        CUR_EC_LEVEL_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pul_cntr(&mut self) -> PUL_CNTR_W {
        PUL_CNTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_pul_cntr](index.html) module"]
pub struct FSM_PUL_CNTR_SPEC;
impl crate::RegisterSpec for FSM_PUL_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_pul_cntr::R](R) reader structure"]
impl crate::Readable for FSM_PUL_CNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_pul_cntr::W](W) writer structure"]
impl crate::Writable for FSM_PUL_CNTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_PUL_CNTR to value 0"]
impl crate::Resettable for FSM_PUL_CNTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
