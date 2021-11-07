#[doc = "Register `MISC_OTP_DATA` reader"]
pub struct R(crate::R<MISC_OTP_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_OTP_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_OTP_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_OTP_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_OTP_DATA` writer"]
pub struct W(crate::W<MISC_OTP_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_OTP_DATA_SPEC>;
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
impl From<crate::W<MISC_OTP_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_OTP_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCOSC_HF_ITUNE` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSC_HF_ITUNE_R(crate::FieldReader<u8, u8>);
impl RCOSC_HF_ITUNE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCOSC_HF_ITUNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCOSC_HF_ITUNE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCOSC_HF_ITUNE` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSC_HF_ITUNE_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_HF_ITUNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `RCOSC_HF_CRIM` reader - 27:20\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSC_HF_CRIM_R(crate::FieldReader<u8, u8>);
impl RCOSC_HF_CRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCOSC_HF_CRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCOSC_HF_CRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCOSC_HF_CRIM` writer - 27:20\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSC_HF_CRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_HF_CRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | ((value as u32 & 0xff) << 20);
        self.w
    }
}
#[doc = "Field `PER_M` reader - 19:15\\]
Internal. Only to be used through TI provided API."]
pub struct PER_M_R(crate::FieldReader<u8, u8>);
impl PER_M_R {
    pub(crate) fn new(bits: u8) -> Self {
        PER_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER_M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER_M` writer - 19:15\\]
Internal. Only to be used through TI provided API."]
pub struct PER_M_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | ((value as u32 & 0x1f) << 15);
        self.w
    }
}
#[doc = "Field `PER_E` reader - 14:12\\]
Internal. Only to be used through TI provided API."]
pub struct PER_E_R(crate::FieldReader<u8, u8>);
impl PER_E_R {
    pub(crate) fn new(bits: u8) -> Self {
        PER_E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER_E_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER_E` writer - 14:12\\]
Internal. Only to be used through TI provided API."]
pub struct PER_E_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_E_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `PO_TAIL_RES_TRIM` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub struct PO_TAIL_RES_TRIM_R(crate::FieldReader<u8, u8>);
impl PO_TAIL_RES_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PO_TAIL_RES_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PO_TAIL_RES_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PO_TAIL_RES_TRIM` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub struct PO_TAIL_RES_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PO_TAIL_RES_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TEST_PROGRAM_REV` reader - 7:0\\]
The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
pub struct TEST_PROGRAM_REV_R(crate::FieldReader<u8, u8>);
impl TEST_PROGRAM_REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        TEST_PROGRAM_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_PROGRAM_REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_PROGRAM_REV` writer - 7:0\\]
The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
pub struct TEST_PROGRAM_REV_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_PROGRAM_REV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_hf_itune(&self) -> RCOSC_HF_ITUNE_R {
        RCOSC_HF_ITUNE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - 27:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_hf_crim(&self) -> RCOSC_HF_CRIM_R {
        RCOSC_HF_CRIM_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 15:19 - 19:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_m(&self) -> PER_M_R {
        PER_M_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_e(&self) -> PER_E_R {
        PER_E_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn po_tail_res_trim(&self) -> PO_TAIL_RES_TRIM_R {
        PO_TAIL_RES_TRIM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
    #[inline(always)]
    pub fn test_program_rev(&self) -> TEST_PROGRAM_REV_R {
        TEST_PROGRAM_REV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_hf_itune(&mut self) -> RCOSC_HF_ITUNE_W {
        RCOSC_HF_ITUNE_W { w: self }
    }
    #[doc = "Bits 20:27 - 27:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_hf_crim(&mut self) -> RCOSC_HF_CRIM_W {
        RCOSC_HF_CRIM_W { w: self }
    }
    #[doc = "Bits 15:19 - 19:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_m(&mut self) -> PER_M_W {
        PER_M_W { w: self }
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_e(&mut self) -> PER_E_W {
        PER_E_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn po_tail_res_trim(&mut self) -> PO_TAIL_RES_TRIM_W {
        PO_TAIL_RES_TRIM_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
    #[inline(always)]
    pub fn test_program_rev(&mut self) -> TEST_PROGRAM_REV_W {
        TEST_PROGRAM_REV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Misc OTP Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_otp_data](index.html) module"]
pub struct MISC_OTP_DATA_SPEC;
impl crate::RegisterSpec for MISC_OTP_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_otp_data::R](R) reader structure"]
impl crate::Readable for MISC_OTP_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_otp_data::W](W) writer structure"]
impl crate::Writable for MISC_OTP_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC_OTP_DATA to value 0xc600"]
impl crate::Resettable for MISC_OTP_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc600
    }
}
