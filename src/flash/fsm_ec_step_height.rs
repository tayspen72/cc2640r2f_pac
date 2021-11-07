#[doc = "Register `FSM_EC_STEP_HEIGHT` reader"]
pub struct R(crate::R<FSM_EC_STEP_HEIGHT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_EC_STEP_HEIGHT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_EC_STEP_HEIGHT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_EC_STEP_HEIGHT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_EC_STEP_HEIGHT` writer"]
pub struct W(crate::W<FSM_EC_STEP_HEIGHT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_EC_STEP_HEIGHT_SPEC>;
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
impl From<crate::W<FSM_EC_STEP_HEIGHT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_EC_STEP_HEIGHT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED4_R(crate::FieldReader<u32, u32>);
impl RESERVED4_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED4` writer - 31:4\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | ((value as u32 & 0x0fff_ffff) << 4);
        self.w
    }
}
#[doc = "Field `EC_STEP_HEIGHT` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct EC_STEP_HEIGHT_R(crate::FieldReader<u8, u8>);
impl EC_STEP_HEIGHT_R {
    pub(crate) fn new(bits: u8) -> Self {
        EC_STEP_HEIGHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EC_STEP_HEIGHT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EC_STEP_HEIGHT` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct EC_STEP_HEIGHT_W<'a> {
    w: &'a mut W,
}
impl<'a> EC_STEP_HEIGHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - 31:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ec_step_height(&self) -> EC_STEP_HEIGHT_R {
        EC_STEP_HEIGHT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ec_step_height(&mut self) -> EC_STEP_HEIGHT_W {
        EC_STEP_HEIGHT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_ec_step_height](index.html) module"]
pub struct FSM_EC_STEP_HEIGHT_SPEC;
impl crate::RegisterSpec for FSM_EC_STEP_HEIGHT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_ec_step_height::R](R) reader structure"]
impl crate::Readable for FSM_EC_STEP_HEIGHT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_ec_step_height::W](W) writer structure"]
impl crate::Writable for FSM_EC_STEP_HEIGHT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_EC_STEP_HEIGHT to value 0"]
impl crate::Resettable for FSM_EC_STEP_HEIGHT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
