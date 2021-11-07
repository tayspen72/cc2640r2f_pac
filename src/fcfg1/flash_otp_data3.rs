#[doc = "Register `FLASH_OTP_DATA3` reader"]
pub struct R(crate::R<FLASH_OTP_DATA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_OTP_DATA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_OTP_DATA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_OTP_DATA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_OTP_DATA3` writer"]
pub struct W(crate::W<FLASH_OTP_DATA3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_OTP_DATA3_SPEC>;
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
impl From<crate::W<FLASH_OTP_DATA3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_OTP_DATA3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EC_STEP_SIZE` reader - 31:23\\]
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
#[doc = "Field `EC_STEP_SIZE` writer - 31:23\\]
Internal. Only to be used through TI provided API."]
pub struct EC_STEP_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> EC_STEP_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 23)) | ((value as u32 & 0x01ff) << 23);
        self.w
    }
}
#[doc = "Field `DO_PRECOND` reader - 22:22\\]
Internal. Only to be used through TI provided API."]
pub struct DO_PRECOND_R(crate::FieldReader<bool, bool>);
impl DO_PRECOND_R {
    pub(crate) fn new(bits: bool) -> Self {
        DO_PRECOND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DO_PRECOND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DO_PRECOND` writer - 22:22\\]
Internal. Only to be used through TI provided API."]
pub struct DO_PRECOND_W<'a> {
    w: &'a mut W,
}
impl<'a> DO_PRECOND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `MAX_EC_LEVEL` reader - 21:18\\]
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
#[doc = "Field `MAX_EC_LEVEL` writer - 21:18\\]
Internal. Only to be used through TI provided API."]
pub struct MAX_EC_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_EC_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
#[doc = "Field `TRIM_1P7` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub struct TRIM_1P7_R(crate::FieldReader<u8, u8>);
impl TRIM_1P7_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM_1P7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_1P7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM_1P7` writer - 17:16\\]
Internal. Only to be used through TI provided API."]
pub struct TRIM_1P7_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_1P7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `FLASH_SIZE` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct FLASH_SIZE_R(crate::FieldReader<u8, u8>);
impl FLASH_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLASH_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_SIZE` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct FLASH_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `WAIT_SYSCODE` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct WAIT_SYSCODE_R(crate::FieldReader<u8, u8>);
impl WAIT_SYSCODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAIT_SYSCODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAIT_SYSCODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAIT_SYSCODE` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct WAIT_SYSCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_SYSCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 23:31 - 31:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ec_step_size(&self) -> EC_STEP_SIZE_R {
        EC_STEP_SIZE_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn do_precond(&self) -> DO_PRECOND_R {
        DO_PRECOND_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_ec_level(&self) -> MAX_EC_LEVEL_R {
        MAX_EC_LEVEL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_1p7(&self) -> TRIM_1P7_R {
        TRIM_1P7_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flash_size(&self) -> FLASH_SIZE_R {
        FLASH_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wait_syscode(&self) -> WAIT_SYSCODE_R {
        WAIT_SYSCODE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 23:31 - 31:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ec_step_size(&mut self) -> EC_STEP_SIZE_W {
        EC_STEP_SIZE_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn do_precond(&mut self) -> DO_PRECOND_W {
        DO_PRECOND_W { w: self }
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_ec_level(&mut self) -> MAX_EC_LEVEL_W {
        MAX_EC_LEVEL_W { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_1p7(&mut self) -> TRIM_1P7_W {
        TRIM_1P7_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flash_size(&mut self) -> FLASH_SIZE_W {
        FLASH_SIZE_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wait_syscode(&mut self) -> WAIT_SYSCODE_W {
        WAIT_SYSCODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_otp_data3](index.html) module"]
pub struct FLASH_OTP_DATA3_SPEC;
impl crate::RegisterSpec for FLASH_OTP_DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_otp_data3::R](R) reader structure"]
impl crate::Readable for FLASH_OTP_DATA3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_otp_data3::W](W) writer structure"]
impl crate::Writable for FLASH_OTP_DATA3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_OTP_DATA3 to value 0x0011_0003"]
impl crate::Resettable for FLASH_OTP_DATA3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0011_0003
    }
}
