#[doc = "Register `AMPCOMP_CTRL1` reader"]
pub struct R(crate::R<AMPCOMP_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPCOMP_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPCOMP_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPCOMP_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPCOMP_CTRL1` writer"]
pub struct W(crate::W<AMPCOMP_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPCOMP_CTRL1_SPEC>;
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
impl From<crate::W<AMPCOMP_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPCOMP_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED1` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED1_R(crate::FieldReader<bool, bool>);
impl RESERVED1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED1` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `AMPCOMP_REQ_MODE` reader - 30:30\\]
Internal. Only to be used through TI provided API."]
pub struct AMPCOMP_REQ_MODE_R(crate::FieldReader<bool, bool>);
impl AMPCOMP_REQ_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMPCOMP_REQ_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMPCOMP_REQ_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMPCOMP_REQ_MODE` writer - 30:30\\]
Internal. Only to be used through TI provided API."]
pub struct AMPCOMP_REQ_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPCOMP_REQ_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 29:24\\]
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
#[doc = "Field `RESERVED0` writer - 29:24\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `IBIAS_OFFSET` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub struct IBIAS_OFFSET_R(crate::FieldReader<u8, u8>);
impl IBIAS_OFFSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        IBIAS_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBIAS_OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBIAS_OFFSET` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub struct IBIAS_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIAS_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `IBIAS_INIT` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct IBIAS_INIT_R(crate::FieldReader<u8, u8>);
impl IBIAS_INIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        IBIAS_INIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBIAS_INIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBIAS_INIT` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct IBIAS_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIAS_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `LPM_IBIAS_WAIT_CNT_FINAL` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct LPM_IBIAS_WAIT_CNT_FINAL_R(crate::FieldReader<u8, u8>);
impl LPM_IBIAS_WAIT_CNT_FINAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPM_IBIAS_WAIT_CNT_FINAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_IBIAS_WAIT_CNT_FINAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM_IBIAS_WAIT_CNT_FINAL` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct LPM_IBIAS_WAIT_CNT_FINAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_IBIAS_WAIT_CNT_FINAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CAP_STEP` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub struct CAP_STEP_R(crate::FieldReader<u8, u8>);
impl CAP_STEP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAP_STEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP_STEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP_STEP` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub struct CAP_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `IBIASCAP_HPTOLP_OL_CNT` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct IBIASCAP_HPTOLP_OL_CNT_R(crate::FieldReader<u8, u8>);
impl IBIASCAP_HPTOLP_OL_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        IBIASCAP_HPTOLP_OL_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBIASCAP_HPTOLP_OL_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBIASCAP_HPTOLP_OL_CNT` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct IBIASCAP_HPTOLP_OL_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIASCAP_HPTOLP_OL_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_req_mode(&self) -> AMPCOMP_REQ_MODE_R {
        AMPCOMP_REQ_MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_offset(&self) -> IBIAS_OFFSET_R {
        IBIAS_OFFSET_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_init(&self) -> IBIAS_INIT_R {
        IBIAS_INIT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt_final(&self) -> LPM_IBIAS_WAIT_CNT_FINAL_R {
        LPM_IBIAS_WAIT_CNT_FINAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cap_step(&self) -> CAP_STEP_R {
        CAP_STEP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibiascap_hptolp_ol_cnt(&self) -> IBIASCAP_HPTOLP_OL_CNT_R {
        IBIASCAP_HPTOLP_OL_CNT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_req_mode(&mut self) -> AMPCOMP_REQ_MODE_W {
        AMPCOMP_REQ_MODE_W { w: self }
    }
    #[doc = "Bits 24:29 - 29:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_offset(&mut self) -> IBIAS_OFFSET_W {
        IBIAS_OFFSET_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_init(&mut self) -> IBIAS_INIT_W {
        IBIAS_INIT_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt_final(&mut self) -> LPM_IBIAS_WAIT_CNT_FINAL_W {
        LPM_IBIAS_WAIT_CNT_FINAL_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cap_step(&mut self) -> CAP_STEP_W {
        CAP_STEP_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibiascap_hptolp_ol_cnt(&mut self) -> IBIASCAP_HPTOLP_OL_CNT_W {
        IBIASCAP_HPTOLP_OL_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcomp_ctrl1](index.html) module"]
pub struct AMPCOMP_CTRL1_SPEC;
impl crate::RegisterSpec for AMPCOMP_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ampcomp_ctrl1::R](R) reader structure"]
impl crate::Readable for AMPCOMP_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ampcomp_ctrl1::W](W) writer structure"]
impl crate::Writable for AMPCOMP_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AMPCOMP_CTRL1 to value 0xff18_3f47"]
impl crate::Resettable for AMPCOMP_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff18_3f47
    }
}
