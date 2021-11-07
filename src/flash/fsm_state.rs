#[doc = "Register `FSM_STATE` reader"]
pub struct R(crate::R<FSM_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_STATE` writer"]
pub struct W(crate::W<FSM_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_STATE_SPEC>;
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
impl From<crate::W<FSM_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_STATE_SPEC>) -> Self {
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
#[doc = "Field `CTRLENZ` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub struct CTRLENZ_R(crate::FieldReader<bool, bool>);
impl CTRLENZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTRLENZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLENZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLENZ` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub struct CTRLENZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLENZ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `EXECUTEZ` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub struct EXECUTEZ_R(crate::FieldReader<bool, bool>);
impl EXECUTEZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXECUTEZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXECUTEZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXECUTEZ` writer - 10:10\\]
Internal. Only to be used through TI provided API."]
pub struct EXECUTEZ_W<'a> {
    w: &'a mut W,
}
impl<'a> EXECUTEZ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `RESERVED9` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED9_R(crate::FieldReader<bool, bool>);
impl RESERVED9_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED9` writer - 9:9\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `FSM_ACT` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct FSM_ACT_R(crate::FieldReader<bool, bool>);
impl FSM_ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSM_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSM_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSM_ACT` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct FSM_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_ACT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `TIOTP_ACT` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub struct TIOTP_ACT_R(crate::FieldReader<bool, bool>);
impl TIOTP_ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIOTP_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIOTP_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIOTP_ACT` writer - 7:7\\]
Internal. Only to be used through TI provided API."]
pub struct TIOTP_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIOTP_ACT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `OTP_ACT` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub struct OTP_ACT_R(crate::FieldReader<bool, bool>);
impl OTP_ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTP_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTP_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTP_ACT` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub struct OTP_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> OTP_ACT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED0_R(crate::FieldReader<u8, u8>);
impl RESERVED0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
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
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrlenz(&self) -> CTRLENZ_R {
        CTRLENZ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn executez(&self) -> EXECUTEZ_R {
        EXECUTEZ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_act(&self) -> FSM_ACT_R {
        FSM_ACT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tiotp_act(&self) -> TIOTP_ACT_R {
        TIOTP_ACT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn otp_act(&self) -> OTP_ACT_R {
        OTP_ACT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrlenz(&mut self) -> CTRLENZ_W {
        CTRLENZ_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn executez(&mut self) -> EXECUTEZ_W {
        EXECUTEZ_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_act(&mut self) -> FSM_ACT_W {
        FSM_ACT_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tiotp_act(&mut self) -> TIOTP_ACT_W {
        TIOTP_ACT_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn otp_act(&mut self) -> OTP_ACT_W {
        OTP_ACT_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_state](index.html) module"]
pub struct FSM_STATE_SPEC;
impl crate::RegisterSpec for FSM_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_state::R](R) reader structure"]
impl crate::Readable for FSM_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_state::W](W) writer structure"]
impl crate::Writable for FSM_STATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_STATE to value 0x0c00"]
impl crate::Resettable for FSM_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c00
    }
}
