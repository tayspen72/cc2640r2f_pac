#[doc = "Register `FSM_FLES` reader"]
pub struct R(crate::R<FSM_FLES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_FLES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_FLES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_FLES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_FLES` writer"]
pub struct W(crate::W<FSM_FLES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_FLES_SPEC>;
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
impl From<crate::W<FSM_FLES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_FLES_SPEC>) -> Self {
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
#[doc = "Field `BLK_TIOTP` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub struct BLK_TIOTP_R(crate::FieldReader<u8, u8>);
impl BLK_TIOTP_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLK_TIOTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLK_TIOTP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLK_TIOTP` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub struct BLK_TIOTP_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK_TIOTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `BLK_OTP` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct BLK_OTP_R(crate::FieldReader<u8, u8>);
impl BLK_OTP_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLK_OTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLK_OTP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLK_OTP` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct BLK_OTP_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK_OTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
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
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn blk_tiotp(&self) -> BLK_TIOTP_R {
        BLK_TIOTP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn blk_otp(&self) -> BLK_OTP_R {
        BLK_OTP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn blk_tiotp(&mut self) -> BLK_TIOTP_W {
        BLK_TIOTP_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn blk_otp(&mut self) -> BLK_OTP_W {
        BLK_OTP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_fles](index.html) module"]
pub struct FSM_FLES_SPEC;
impl crate::RegisterSpec for FSM_FLES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_fles::R](R) reader structure"]
impl crate::Readable for FSM_FLES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_fles::W](W) writer structure"]
impl crate::Writable for FSM_FLES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_FLES to value 0"]
impl crate::Resettable for FSM_FLES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
