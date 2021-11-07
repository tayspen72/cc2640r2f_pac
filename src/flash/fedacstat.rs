#[doc = "Register `FEDACSTAT` reader"]
pub struct R(crate::R<FEDACSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEDACSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEDACSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEDACSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEDACSTAT` writer"]
pub struct W(crate::W<FEDACSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEDACSTAT_SPEC>;
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
impl From<crate::W<FEDACSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEDACSTAT_SPEC>) -> Self {
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
#[doc = "Field `RVF_INT` reader - 25:25\\]
Internal. Only to be used through TI provided API."]
pub struct RVF_INT_R(crate::FieldReader<bool, bool>);
impl RVF_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RVF_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RVF_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RVF_INT` writer - 25:25\\]
Internal. Only to be used through TI provided API."]
pub struct RVF_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RVF_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `FSM_DONE` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub struct FSM_DONE_R(crate::FieldReader<bool, bool>);
impl FSM_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSM_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSM_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSM_DONE` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub struct FSM_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `ERR_PRF_FLG` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub struct ERR_PRF_FLG_R(crate::FieldReader<u32, u32>);
impl ERR_PRF_FLG_R {
    pub(crate) fn new(bits: u32) -> Self {
        ERR_PRF_FLG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_PRF_FLG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_PRF_FLG` writer - 23:0\\]
Internal. Only to be used through TI provided API."]
pub struct ERR_PRF_FLG_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_PRF_FLG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
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
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvf_int(&self) -> RVF_INT_R {
        RVF_INT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_done(&self) -> FSM_DONE_R {
        FSM_DONE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn err_prf_flg(&self) -> ERR_PRF_FLG_R {
        ERR_PRF_FLG_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved26(&mut self) -> RESERVED26_W {
        RESERVED26_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvf_int(&mut self) -> RVF_INT_W {
        RVF_INT_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_done(&mut self) -> FSM_DONE_W {
        FSM_DONE_W { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn err_prf_flg(&mut self) -> ERR_PRF_FLG_W {
        ERR_PRF_FLG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fedacstat](index.html) module"]
pub struct FEDACSTAT_SPEC;
impl crate::RegisterSpec for FEDACSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fedacstat::R](R) reader structure"]
impl crate::Readable for FEDACSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fedacstat::W](W) writer structure"]
impl crate::Writable for FEDACSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FEDACSTAT to value 0"]
impl crate::Resettable for FEDACSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
