#[doc = "Register `FSM_ERA_PUL` reader"]
pub struct R(crate::R<FSM_ERA_PUL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_ERA_PUL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_ERA_PUL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_ERA_PUL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_ERA_PUL` writer"]
pub struct W(crate::W<FSM_ERA_PUL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_ERA_PUL_SPEC>;
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
impl From<crate::W<FSM_ERA_PUL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_ERA_PUL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED20` reader - 31:20\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED20_R(crate::FieldReader<u16, u16>);
impl RESERVED20_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED20_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED20` writer - 31:20\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED20_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | ((value as u32 & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Field `MAX_EC_LEVEL` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct MAX_EC_LEVEL_R(crate::FieldReader<u8, u8>);
impl MAX_EC_LEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAX_EC_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAX_EC_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAX_EC_LEVEL` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct MAX_EC_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_EC_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
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
#[doc = "Field `MAX_ERA_PUL` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub struct MAX_ERA_PUL_R(crate::FieldReader<u16, u16>);
impl MAX_ERA_PUL_R {
    pub(crate) fn new(bits: u16) -> Self {
        MAX_ERA_PUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAX_ERA_PUL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAX_ERA_PUL` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub struct MAX_ERA_PUL_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_ERA_PUL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_ec_level(&self) -> MAX_EC_LEVEL_R {
        MAX_EC_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
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
    pub fn max_era_pul(&self) -> MAX_ERA_PUL_R {
        MAX_ERA_PUL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_ec_level(&mut self) -> MAX_EC_LEVEL_W {
        MAX_EC_LEVEL_W { w: self }
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
    pub fn max_era_pul(&mut self) -> MAX_ERA_PUL_W {
        MAX_ERA_PUL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_era_pul](index.html) module"]
pub struct FSM_ERA_PUL_SPEC;
impl crate::RegisterSpec for FSM_ERA_PUL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_era_pul::R](R) reader structure"]
impl crate::Readable for FSM_ERA_PUL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_era_pul::W](W) writer structure"]
impl crate::Writable for FSM_ERA_PUL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_ERA_PUL to value 0x0004_0bb8"]
impl crate::Resettable for FSM_ERA_PUL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_0bb8
    }
}
