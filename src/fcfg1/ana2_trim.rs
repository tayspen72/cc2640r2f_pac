#[doc = "Register `ANA2_TRIM` reader"]
pub struct R(crate::R<ANA2_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA2_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA2_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA2_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA2_TRIM` writer"]
pub struct W(crate::W<ANA2_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA2_TRIM_SPEC>;
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
impl From<crate::W<ANA2_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA2_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCOSCHFCTRIMFRACT_EN` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSCHFCTRIMFRACT_EN_R(crate::FieldReader<bool, bool>);
impl RCOSCHFCTRIMFRACT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCOSCHFCTRIMFRACT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCOSCHFCTRIMFRACT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCOSCHFCTRIMFRACT_EN` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSCHFCTRIMFRACT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHFCTRIMFRACT_EN_W<'a> {
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
#[doc = "Field `RCOSCHFCTRIMFRACT` reader - 30:26\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSCHFCTRIMFRACT_R(crate::FieldReader<u8, u8>);
impl RCOSCHFCTRIMFRACT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCOSCHFCTRIMFRACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCOSCHFCTRIMFRACT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCOSCHFCTRIMFRACT` writer - 30:26\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSCHFCTRIMFRACT_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHFCTRIMFRACT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | ((value as u32 & 0x1f) << 26);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 25:25\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED0_R(crate::FieldReader<bool, bool>);
impl RESERVED0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - 25:25\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
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
#[doc = "Field `SET_RCOSC_HF_FINE_RESISTOR` reader - 24:23\\]
Internal. Only to be used through TI provided API."]
pub struct SET_RCOSC_HF_FINE_RESISTOR_R(crate::FieldReader<u8, u8>);
impl SET_RCOSC_HF_FINE_RESISTOR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SET_RCOSC_HF_FINE_RESISTOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SET_RCOSC_HF_FINE_RESISTOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SET_RCOSC_HF_FINE_RESISTOR` writer - 24:23\\]
Internal. Only to be used through TI provided API."]
pub struct SET_RCOSC_HF_FINE_RESISTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_RCOSC_HF_FINE_RESISTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | ((value as u32 & 0x03) << 23);
        self.w
    }
}
#[doc = "Field `ATESTLF_UDIGLDO_IBIAS_TRIM` reader - 22:22\\]
Internal. Only to be used through TI provided API."]
pub struct ATESTLF_UDIGLDO_IBIAS_TRIM_R(crate::FieldReader<bool, bool>);
impl ATESTLF_UDIGLDO_IBIAS_TRIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATESTLF_UDIGLDO_IBIAS_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATESTLF_UDIGLDO_IBIAS_TRIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATESTLF_UDIGLDO_IBIAS_TRIM` writer - 22:22\\]
Internal. Only to be used through TI provided API."]
pub struct ATESTLF_UDIGLDO_IBIAS_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ATESTLF_UDIGLDO_IBIAS_TRIM_W<'a> {
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
#[doc = "Field `NANOAMP_RES_TRIM` reader - 21:16\\]
Internal. Only to be used through TI provided API."]
pub struct NANOAMP_RES_TRIM_R(crate::FieldReader<u8, u8>);
impl NANOAMP_RES_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        NANOAMP_RES_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NANOAMP_RES_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NANOAMP_RES_TRIM` writer - 21:16\\]
Internal. Only to be used through TI provided API."]
pub struct NANOAMP_RES_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> NANOAMP_RES_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED1` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED1_R(crate::FieldReader<u8, u8>);
impl RESERVED1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED1` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `DITHER_EN` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub struct DITHER_EN_R(crate::FieldReader<bool, bool>);
impl DITHER_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DITHER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DITHER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DITHER_EN` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub struct DITHER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHER_EN_W<'a> {
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
#[doc = "Field `DCDC_IPEAK` reader - 10:8\\]
Internal. Only to be used through TI provided API."]
pub struct DCDC_IPEAK_R(crate::FieldReader<u8, u8>);
impl DCDC_IPEAK_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCDC_IPEAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC_IPEAK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDC_IPEAK` writer - 10:8\\]
Internal. Only to be used through TI provided API."]
pub struct DCDC_IPEAK_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_IPEAK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `DEAD_TIME_TRIM` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub struct DEAD_TIME_TRIM_R(crate::FieldReader<u8, u8>);
impl DEAD_TIME_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEAD_TIME_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEAD_TIME_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEAD_TIME_TRIM` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub struct DEAD_TIME_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DEAD_TIME_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `DCDC_LOW_EN_SEL` reader - 5:3\\]
Internal. Only to be used through TI provided API."]
pub struct DCDC_LOW_EN_SEL_R(crate::FieldReader<u8, u8>);
impl DCDC_LOW_EN_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCDC_LOW_EN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC_LOW_EN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDC_LOW_EN_SEL` writer - 5:3\\]
Internal. Only to be used through TI provided API."]
pub struct DCDC_LOW_EN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_LOW_EN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `DCDC_HIGH_EN_SEL` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub struct DCDC_HIGH_EN_SEL_R(crate::FieldReader<u8, u8>);
impl DCDC_HIGH_EN_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCDC_HIGH_EN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC_HIGH_EN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDC_HIGH_EN_SEL` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub struct DCDC_HIGH_EN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_HIGH_EN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract_en(&self) -> RCOSCHFCTRIMFRACT_EN_R {
        RCOSCHFCTRIMFRACT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 26:30 - 30:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract(&self) -> RCOSCHFCTRIMFRACT_R {
        RCOSCHFCTRIMFRACT_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 23:24 - 24:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn set_rcosc_hf_fine_resistor(&self) -> SET_RCOSC_HF_FINE_RESISTOR_R {
        SET_RCOSC_HF_FINE_RESISTOR_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atestlf_udigldo_ibias_trim(&self) -> ATESTLF_UDIGLDO_IBIAS_TRIM_R {
        ATESTLF_UDIGLDO_IBIAS_TRIM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nanoamp_res_trim(&self) -> NANOAMP_RES_TRIM_R {
        NANOAMP_RES_TRIM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dither_en(&self) -> DITHER_EN_R {
        DITHER_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_ipeak(&self) -> DCDC_IPEAK_R {
        DCDC_IPEAK_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dead_time_trim(&self) -> DEAD_TIME_TRIM_R {
        DEAD_TIME_TRIM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_low_en_sel(&self) -> DCDC_LOW_EN_SEL_R {
        DCDC_LOW_EN_SEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_high_en_sel(&self) -> DCDC_HIGH_EN_SEL_R {
        DCDC_HIGH_EN_SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract_en(&mut self) -> RCOSCHFCTRIMFRACT_EN_W {
        RCOSCHFCTRIMFRACT_EN_W { w: self }
    }
    #[doc = "Bits 26:30 - 30:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract(&mut self) -> RCOSCHFCTRIMFRACT_W {
        RCOSCHFCTRIMFRACT_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Bits 23:24 - 24:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn set_rcosc_hf_fine_resistor(&mut self) -> SET_RCOSC_HF_FINE_RESISTOR_W {
        SET_RCOSC_HF_FINE_RESISTOR_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atestlf_udigldo_ibias_trim(&mut self) -> ATESTLF_UDIGLDO_IBIAS_TRIM_W {
        ATESTLF_UDIGLDO_IBIAS_TRIM_W { w: self }
    }
    #[doc = "Bits 16:21 - 21:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nanoamp_res_trim(&mut self) -> NANOAMP_RES_TRIM_W {
        NANOAMP_RES_TRIM_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dither_en(&mut self) -> DITHER_EN_W {
        DITHER_EN_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_ipeak(&mut self) -> DCDC_IPEAK_W {
        DCDC_IPEAK_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dead_time_trim(&mut self) -> DEAD_TIME_TRIM_W {
        DEAD_TIME_TRIM_W { w: self }
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_low_en_sel(&mut self) -> DCDC_LOW_EN_SEL_W {
        DCDC_LOW_EN_SEL_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcdc_high_en_sel(&mut self) -> DCDC_HIGH_EN_SEL_W {
        DCDC_HIGH_EN_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana2_trim](index.html) module"]
pub struct ANA2_TRIM_SPEC;
impl crate::RegisterSpec for ANA2_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana2_trim::R](R) reader structure"]
impl crate::Readable for ANA2_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana2_trim::W](W) writer structure"]
impl crate::Writable for ANA2_TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANA2_TRIM to value 0x8240_f47f"]
impl crate::Resettable for ANA2_TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8240_f47f
    }
}
