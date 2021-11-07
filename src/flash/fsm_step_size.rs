#[doc = "Register `FSM_STEP_SIZE` reader"]
pub struct R(crate::R<FSM_STEP_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_STEP_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_STEP_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_STEP_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_STEP_SIZE` writer"]
pub struct W(crate::W<FSM_STEP_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_STEP_SIZE_SPEC>;
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
impl From<crate::W<FSM_STEP_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_STEP_SIZE_SPEC>) -> Self {
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
#[doc = "Field `EC_STEP_SIZE` reader - 24:16\\]
Internal. Only to be used through TI provided API."]
pub struct EC_STEP_SIZE_R(crate::FieldReader<u16, u16>);
impl EC_STEP_SIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        EC_STEP_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC_STEP_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC_STEP_SIZE` writer - 24:16\\]
Internal. Only to be used through TI provided API."]
pub struct EC_STEP_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> EC_STEP_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED0_R(crate::FieldReader<u16, u16>);
impl RESERVED0_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
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
    pub fn ec_step_size(&self) -> EC_STEP_SIZE_R {
        EC_STEP_SIZE_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0xffff) as u16)
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
    pub fn ec_step_size(&mut self) -> EC_STEP_SIZE_W {
        EC_STEP_SIZE_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_step_size](index.html) module"]
pub struct FSM_STEP_SIZE_SPEC;
impl crate::RegisterSpec for FSM_STEP_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_step_size::R](R) reader structure"]
impl crate::Readable for FSM_STEP_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_step_size::W](W) writer structure"]
impl crate::Writable for FSM_STEP_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_STEP_SIZE to value 0"]
impl crate::Resettable for FSM_STEP_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
