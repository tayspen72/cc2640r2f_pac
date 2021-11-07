#[doc = "Register `FSEQPMP` reader"]
pub struct R(crate::R<FSEQPMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSEQPMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSEQPMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSEQPMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSEQPMP` writer"]
pub struct W(crate::W<FSEQPMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSEQPMP_SPEC>;
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
impl From<crate::W<FSEQPMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSEQPMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED28` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED28_R(crate::FieldReader<u8, u8>);
impl RESERVED28_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED28_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED28` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED28_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `TRIM_3P4` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub struct TRIM_3P4_R(crate::FieldReader<u8, u8>);
impl TRIM_3P4_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM_3P4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_3P4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM_3P4` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub struct TRIM_3P4_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_3P4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `RESERVED22` reader - 23:22\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED22_R(crate::FieldReader<u8, u8>);
impl RESERVED22_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED22_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED22` writer - 23:22\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED22_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `TRIM_1P7` reader - 21:20\\]
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
#[doc = "Field `TRIM_1P7` writer - 21:20\\]
Internal. Only to be used through TI provided API."]
pub struct TRIM_1P7_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_1P7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `TRIM_0P8` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct TRIM_0P8_R(crate::FieldReader<u8, u8>);
impl TRIM_0P8_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM_0P8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_0P8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM_0P8` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct TRIM_0P8_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_0P8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED15` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED15_R(crate::FieldReader<bool, bool>);
impl RESERVED15_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED15` writer - 15:15\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED15_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `VIN_AT_X` reader - 14:12\\]
Internal. Only to be used through TI provided API."]
pub struct VIN_AT_X_R(crate::FieldReader<u8, u8>);
impl VIN_AT_X_R {
    pub(crate) fn new(bits: u8) -> Self {
        VIN_AT_X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VIN_AT_X_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VIN_AT_X` writer - 14:12\\]
Internal. Only to be used through TI provided API."]
pub struct VIN_AT_X_W<'a> {
    w: &'a mut W,
}
impl<'a> VIN_AT_X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `RESERVED9` reader - 11:9\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED9_R(crate::FieldReader<u8, u8>);
impl RESERVED9_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED9` writer - 11:9\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `VIN_BY_PASS` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct VIN_BY_PASS_R(crate::FieldReader<bool, bool>);
impl VIN_BY_PASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VIN_BY_PASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VIN_BY_PASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VIN_BY_PASS` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct VIN_BY_PASS_W<'a> {
    w: &'a mut W,
}
impl<'a> VIN_BY_PASS_W<'a> {
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
#[doc = "Field `SEQ_PUMP` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct SEQ_PUMP_R(crate::FieldReader<u8, u8>);
impl SEQ_PUMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEQ_PUMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEQ_PUMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQ_PUMP` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct SEQ_PUMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQ_PUMP_W<'a> {
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
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_3p4(&self) -> TRIM_3P4_R {
        TRIM_3P4_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved22(&self) -> RESERVED22_R {
        RESERVED22_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_1p7(&self) -> TRIM_1P7_R {
        TRIM_1P7_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_0p8(&self) -> TRIM_0P8_R {
        TRIM_0P8_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x(&self) -> VIN_AT_X_R {
        VIN_AT_X_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_by_pass(&self) -> VIN_BY_PASS_R {
        VIN_BY_PASS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn seq_pump(&self) -> SEQ_PUMP_R {
        SEQ_PUMP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&mut self) -> RESERVED28_W {
        RESERVED28_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_3p4(&mut self) -> TRIM_3P4_W {
        TRIM_3P4_W { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved22(&mut self) -> RESERVED22_W {
        RESERVED22_W { w: self }
    }
    #[doc = "Bits 20:21 - 21:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_1p7(&mut self) -> TRIM_1P7_W {
        TRIM_1P7_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_0p8(&mut self) -> TRIM_0P8_W {
        TRIM_0P8_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved15(&mut self) -> RESERVED15_W {
        RESERVED15_W { w: self }
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x(&mut self) -> VIN_AT_X_W {
        VIN_AT_X_W { w: self }
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_by_pass(&mut self) -> VIN_BY_PASS_W {
        VIN_BY_PASS_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn seq_pump(&mut self) -> SEQ_PUMP_W {
        SEQ_PUMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fseqpmp](index.html) module"]
pub struct FSEQPMP_SPEC;
impl crate::RegisterSpec for FSEQPMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fseqpmp::R](R) reader structure"]
impl crate::Readable for FSEQPMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fseqpmp::W](W) writer structure"]
impl crate::Writable for FSEQPMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSEQPMP to value 0x8508_0000"]
impl crate::Resettable for FSEQPMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8508_0000
    }
}
