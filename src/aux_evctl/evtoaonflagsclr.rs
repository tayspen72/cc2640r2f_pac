#[doc = "Register `EVTOAONFLAGSCLR` reader"]
pub struct R(crate::R<EVTOAONFLAGSCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVTOAONFLAGSCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVTOAONFLAGSCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVTOAONFLAGSCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVTOAONFLAGSCLR` writer"]
pub struct W(crate::W<EVTOAONFLAGSCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVTOAONFLAGSCLR_SPEC>;
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
impl From<crate::W<EVTOAONFLAGSCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVTOAONFLAGSCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED9_R(crate::FieldReader<u32, u32>);
impl RESERVED9_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED9_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | ((value as u32 & 0x007f_ffff) << 9);
        self.w
    }
}
#[doc = "Field `TIMER1_EV` reader - 8:8\\]
Write 1 to clear EVTOAONFLAGS.TIMER1_EV. Read value is 0."]
pub struct TIMER1_EV_R(crate::FieldReader<bool, bool>);
impl TIMER1_EV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_EV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_EV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_EV` writer - 8:8\\]
Write 1 to clear EVTOAONFLAGS.TIMER1_EV. Read value is 0."]
pub struct TIMER1_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_EV_W<'a> {
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
#[doc = "Field `TIMER0_EV` reader - 7:7\\]
Write 1 to clear EVTOAONFLAGS.TIMER0_EV. Read value is 0."]
pub struct TIMER0_EV_R(crate::FieldReader<bool, bool>);
impl TIMER0_EV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_EV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_EV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_EV` writer - 7:7\\]
Write 1 to clear EVTOAONFLAGS.TIMER0_EV. Read value is 0."]
pub struct TIMER0_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_EV_W<'a> {
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
#[doc = "Field `TDC_DONE` reader - 6:6\\]
Write 1 to clear EVTOAONFLAGS.TDC_DONE. Read value is 0."]
pub struct TDC_DONE_R(crate::FieldReader<bool, bool>);
impl TDC_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDC_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDC_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDC_DONE` writer - 6:6\\]
Write 1 to clear EVTOAONFLAGS.TDC_DONE. Read value is 0."]
pub struct TDC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDC_DONE_W<'a> {
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
#[doc = "Field `ADC_DONE` reader - 5:5\\]
Write 1 to clear EVTOAONFLAGS.ADC_DONE. Read value is 0."]
pub struct ADC_DONE_R(crate::FieldReader<bool, bool>);
impl ADC_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_DONE` writer - 5:5\\]
Write 1 to clear EVTOAONFLAGS.ADC_DONE. Read value is 0."]
pub struct ADC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `AUX_COMPB` reader - 4:4\\]
Write 1 to clear EVTOAONFLAGS.AUX_COMPB. Read value is 0."]
pub struct AUX_COMPB_R(crate::FieldReader<bool, bool>);
impl AUX_COMPB_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUX_COMPB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUX_COMPB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUX_COMPB` writer - 4:4\\]
Write 1 to clear EVTOAONFLAGS.AUX_COMPB. Read value is 0."]
pub struct AUX_COMPB_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `AUX_COMPA` reader - 3:3\\]
Write 1 to clear EVTOAONFLAGS.AUX_COMPA. Read value is 0."]
pub struct AUX_COMPA_R(crate::FieldReader<bool, bool>);
impl AUX_COMPA_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUX_COMPA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUX_COMPA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUX_COMPA` writer - 3:3\\]
Write 1 to clear EVTOAONFLAGS.AUX_COMPA. Read value is 0."]
pub struct AUX_COMPA_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SWEV2` reader - 2:2\\]
Write 1 to clear EVTOAONFLAGS.SWEV2. Read value is 0."]
pub struct SWEV2_R(crate::FieldReader<bool, bool>);
impl SWEV2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWEV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWEV2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWEV2` writer - 2:2\\]
Write 1 to clear EVTOAONFLAGS.SWEV2. Read value is 0."]
pub struct SWEV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEV2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SWEV1` reader - 1:1\\]
Write 1 to clear EVTOAONFLAGS.SWEV1. Read value is 0."]
pub struct SWEV1_R(crate::FieldReader<bool, bool>);
impl SWEV1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWEV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWEV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWEV1` writer - 1:1\\]
Write 1 to clear EVTOAONFLAGS.SWEV1. Read value is 0."]
pub struct SWEV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEV1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SWEV0` reader - 0:0\\]
Write 1 to clear EVTOAONFLAGS.SWEV0. Read value is 0."]
pub struct SWEV0_R(crate::FieldReader<bool, bool>);
impl SWEV0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWEV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWEV0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWEV0` writer - 0:0\\]
Write 1 to clear EVTOAONFLAGS.SWEV0. Read value is 0."]
pub struct SWEV0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEV0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
    #[doc = "Bit 8 - 8:8\\]
Write 1 to clear EVTOAONFLAGS.TIMER1_EV. Read value is 0."]
    #[inline(always)]
    pub fn timer1_ev(&self) -> TIMER1_EV_R {
        TIMER1_EV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Write 1 to clear EVTOAONFLAGS.TIMER0_EV. Read value is 0."]
    #[inline(always)]
    pub fn timer0_ev(&self) -> TIMER0_EV_R {
        TIMER0_EV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Write 1 to clear EVTOAONFLAGS.TDC_DONE. Read value is 0."]
    #[inline(always)]
    pub fn tdc_done(&self) -> TDC_DONE_R {
        TDC_DONE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Write 1 to clear EVTOAONFLAGS.ADC_DONE. Read value is 0."]
    #[inline(always)]
    pub fn adc_done(&self) -> ADC_DONE_R {
        ADC_DONE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Write 1 to clear EVTOAONFLAGS.AUX_COMPB. Read value is 0."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Write 1 to clear EVTOAONFLAGS.AUX_COMPA. Read value is 0."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Write 1 to clear EVTOAONFLAGS.SWEV2. Read value is 0."]
    #[inline(always)]
    pub fn swev2(&self) -> SWEV2_R {
        SWEV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write 1 to clear EVTOAONFLAGS.SWEV1. Read value is 0."]
    #[inline(always)]
    pub fn swev1(&self) -> SWEV1_R {
        SWEV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Write 1 to clear EVTOAONFLAGS.SWEV0. Read value is 0."]
    #[inline(always)]
    pub fn swev0(&self) -> SWEV0_R {
        SWEV0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Write 1 to clear EVTOAONFLAGS.TIMER1_EV. Read value is 0."]
    #[inline(always)]
    pub fn timer1_ev(&mut self) -> TIMER1_EV_W {
        TIMER1_EV_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Write 1 to clear EVTOAONFLAGS.TIMER0_EV. Read value is 0."]
    #[inline(always)]
    pub fn timer0_ev(&mut self) -> TIMER0_EV_W {
        TIMER0_EV_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Write 1 to clear EVTOAONFLAGS.TDC_DONE. Read value is 0."]
    #[inline(always)]
    pub fn tdc_done(&mut self) -> TDC_DONE_W {
        TDC_DONE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Write 1 to clear EVTOAONFLAGS.ADC_DONE. Read value is 0."]
    #[inline(always)]
    pub fn adc_done(&mut self) -> ADC_DONE_W {
        ADC_DONE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Write 1 to clear EVTOAONFLAGS.AUX_COMPB. Read value is 0."]
    #[inline(always)]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W {
        AUX_COMPB_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Write 1 to clear EVTOAONFLAGS.AUX_COMPA. Read value is 0."]
    #[inline(always)]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W {
        AUX_COMPA_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Write 1 to clear EVTOAONFLAGS.SWEV2. Read value is 0."]
    #[inline(always)]
    pub fn swev2(&mut self) -> SWEV2_W {
        SWEV2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Write 1 to clear EVTOAONFLAGS.SWEV1. Read value is 0."]
    #[inline(always)]
    pub fn swev1(&mut self) -> SWEV1_W {
        SWEV1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Write 1 to clear EVTOAONFLAGS.SWEV0. Read value is 0."]
    #[inline(always)]
    pub fn swev0(&mut self) -> SWEV0_W {
        SWEV0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtoaonflagsclr](index.html) module"]
pub struct EVTOAONFLAGSCLR_SPEC;
impl crate::RegisterSpec for EVTOAONFLAGSCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evtoaonflagsclr::R](R) reader structure"]
impl crate::Readable for EVTOAONFLAGSCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evtoaonflagsclr::W](W) writer structure"]
impl crate::Writable for EVTOAONFLAGSCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVTOAONFLAGSCLR to value 0"]
impl crate::Resettable for EVTOAONFLAGSCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
