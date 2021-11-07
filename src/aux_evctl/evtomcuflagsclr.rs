#[doc = "Register `EVTOMCUFLAGSCLR` reader"]
pub struct R(crate::R<EVTOMCUFLAGSCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVTOMCUFLAGSCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVTOMCUFLAGSCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVTOMCUFLAGSCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVTOMCUFLAGSCLR` writer"]
pub struct W(crate::W<EVTOMCUFLAGSCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVTOMCUFLAGSCLR_SPEC>;
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
impl From<crate::W<EVTOMCUFLAGSCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVTOMCUFLAGSCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED11_R(crate::FieldReader<u32, u32>);
impl RESERVED11_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED11_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED11` writer - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED11_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x001f_ffff << 11)) | ((value as u32 & 0x001f_ffff) << 11);
        self.w
    }
}
#[doc = "Field `ADC_IRQ` reader - 10:10\\]
Write 1 to clear EVTOMCUFLAGS.ADC_IRQ. Read value is 0."]
pub struct ADC_IRQ_R(crate::FieldReader<bool, bool>);
impl ADC_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_IRQ` writer - 10:10\\]
Write 1 to clear EVTOMCUFLAGS.ADC_IRQ. Read value is 0."]
pub struct ADC_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_IRQ_W<'a> {
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
#[doc = "Field `OBSMUX0` reader - 9:9\\]
Write 1 to clear EVTOMCUFLAGS.MCU_OBSMUX0. Read value is 0."]
pub struct OBSMUX0_R(crate::FieldReader<bool, bool>);
impl OBSMUX0_R {
    pub(crate) fn new(bits: bool) -> Self {
        OBSMUX0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OBSMUX0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OBSMUX0` writer - 9:9\\]
Write 1 to clear EVTOMCUFLAGS.MCU_OBSMUX0. Read value is 0."]
pub struct OBSMUX0_W<'a> {
    w: &'a mut W,
}
impl<'a> OBSMUX0_W<'a> {
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
#[doc = "Field `ADC_FIFO_ALMOST_FULL` reader - 8:8\\]
Write 1 to clear EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL. Read value is 0."]
pub struct ADC_FIFO_ALMOST_FULL_R(crate::FieldReader<bool, bool>);
impl ADC_FIFO_ALMOST_FULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_FIFO_ALMOST_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_FIFO_ALMOST_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_FIFO_ALMOST_FULL` writer - 8:8\\]
Write 1 to clear EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL. Read value is 0."]
pub struct ADC_FIFO_ALMOST_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_FIFO_ALMOST_FULL_W<'a> {
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
#[doc = "Field `ADC_DONE` reader - 7:7\\]
Write 1 to clear EVTOMCUFLAGS.ADC_DONE. Read value is 0."]
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
#[doc = "Field `ADC_DONE` writer - 7:7\\]
Write 1 to clear EVTOMCUFLAGS.ADC_DONE. Read value is 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `SMPH_AUTOTAKE_DONE` reader - 6:6\\]
Write 1 to clear EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE. Read value is 0."]
pub struct SMPH_AUTOTAKE_DONE_R(crate::FieldReader<bool, bool>);
impl SMPH_AUTOTAKE_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMPH_AUTOTAKE_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPH_AUTOTAKE_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPH_AUTOTAKE_DONE` writer - 6:6\\]
Write 1 to clear EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE. Read value is 0."]
pub struct SMPH_AUTOTAKE_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPH_AUTOTAKE_DONE_W<'a> {
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
#[doc = "Field `TIMER1_EV` reader - 5:5\\]
Write 1 to clear EVTOMCUFLAGS.TIMER1_EV. Read value is 0."]
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
#[doc = "Field `TIMER1_EV` writer - 5:5\\]
Write 1 to clear EVTOMCUFLAGS.TIMER1_EV. Read value is 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TIMER0_EV` reader - 4:4\\]
Write 1 to clear EVTOMCUFLAGS.TIMER0_EV. Read value is 0."]
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
#[doc = "Field `TIMER0_EV` writer - 4:4\\]
Write 1 to clear EVTOMCUFLAGS.TIMER0_EV. Read value is 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TDC_DONE` reader - 3:3\\]
Write 1 to clear EVTOMCUFLAGS.TDC_DONE. Read value is 0."]
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
#[doc = "Field `TDC_DONE` writer - 3:3\\]
Write 1 to clear EVTOMCUFLAGS.TDC_DONE. Read value is 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `AUX_COMPB` reader - 2:2\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPB. Read value is 0."]
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
#[doc = "Field `AUX_COMPB` writer - 2:2\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPB. Read value is 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `AUX_COMPA` reader - 1:1\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPA. Read value is 0."]
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
#[doc = "Field `AUX_COMPA` writer - 1:1\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPA. Read value is 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `AON_WU_EV` reader - 0:0\\]
Write 1 to clear EVTOMCUFLAGS.AON_WU_EV. Read value is 0."]
pub struct AON_WU_EV_R(crate::FieldReader<bool, bool>);
impl AON_WU_EV_R {
    pub(crate) fn new(bits: bool) -> Self {
        AON_WU_EV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AON_WU_EV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AON_WU_EV` writer - 0:0\\]
Write 1 to clear EVTOMCUFLAGS.AON_WU_EV. Read value is 0."]
pub struct AON_WU_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_WU_EV_W<'a> {
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
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 11) & 0x001f_ffff) as u32)
    }
    #[doc = "Bit 10 - 10:10\\]
Write 1 to clear EVTOMCUFLAGS.ADC_IRQ. Read value is 0."]
    #[inline(always)]
    pub fn adc_irq(&self) -> ADC_IRQ_R {
        ADC_IRQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Write 1 to clear EVTOMCUFLAGS.MCU_OBSMUX0. Read value is 0."]
    #[inline(always)]
    pub fn obsmux0(&self) -> OBSMUX0_R {
        OBSMUX0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Write 1 to clear EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL. Read value is 0."]
    #[inline(always)]
    pub fn adc_fifo_almost_full(&self) -> ADC_FIFO_ALMOST_FULL_R {
        ADC_FIFO_ALMOST_FULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Write 1 to clear EVTOMCUFLAGS.ADC_DONE. Read value is 0."]
    #[inline(always)]
    pub fn adc_done(&self) -> ADC_DONE_R {
        ADC_DONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Write 1 to clear EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE. Read value is 0."]
    #[inline(always)]
    pub fn smph_autotake_done(&self) -> SMPH_AUTOTAKE_DONE_R {
        SMPH_AUTOTAKE_DONE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Write 1 to clear EVTOMCUFLAGS.TIMER1_EV. Read value is 0."]
    #[inline(always)]
    pub fn timer1_ev(&self) -> TIMER1_EV_R {
        TIMER1_EV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Write 1 to clear EVTOMCUFLAGS.TIMER0_EV. Read value is 0."]
    #[inline(always)]
    pub fn timer0_ev(&self) -> TIMER0_EV_R {
        TIMER0_EV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Write 1 to clear EVTOMCUFLAGS.TDC_DONE. Read value is 0."]
    #[inline(always)]
    pub fn tdc_done(&self) -> TDC_DONE_R {
        TDC_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPB. Read value is 0."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPA. Read value is 0."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Write 1 to clear EVTOMCUFLAGS.AON_WU_EV. Read value is 0."]
    #[inline(always)]
    pub fn aon_wu_ev(&self) -> AON_WU_EV_R {
        AON_WU_EV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&mut self) -> RESERVED11_W {
        RESERVED11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Write 1 to clear EVTOMCUFLAGS.ADC_IRQ. Read value is 0."]
    #[inline(always)]
    pub fn adc_irq(&mut self) -> ADC_IRQ_W {
        ADC_IRQ_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Write 1 to clear EVTOMCUFLAGS.MCU_OBSMUX0. Read value is 0."]
    #[inline(always)]
    pub fn obsmux0(&mut self) -> OBSMUX0_W {
        OBSMUX0_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Write 1 to clear EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL. Read value is 0."]
    #[inline(always)]
    pub fn adc_fifo_almost_full(&mut self) -> ADC_FIFO_ALMOST_FULL_W {
        ADC_FIFO_ALMOST_FULL_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Write 1 to clear EVTOMCUFLAGS.ADC_DONE. Read value is 0."]
    #[inline(always)]
    pub fn adc_done(&mut self) -> ADC_DONE_W {
        ADC_DONE_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Write 1 to clear EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE. Read value is 0."]
    #[inline(always)]
    pub fn smph_autotake_done(&mut self) -> SMPH_AUTOTAKE_DONE_W {
        SMPH_AUTOTAKE_DONE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Write 1 to clear EVTOMCUFLAGS.TIMER1_EV. Read value is 0."]
    #[inline(always)]
    pub fn timer1_ev(&mut self) -> TIMER1_EV_W {
        TIMER1_EV_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Write 1 to clear EVTOMCUFLAGS.TIMER0_EV. Read value is 0."]
    #[inline(always)]
    pub fn timer0_ev(&mut self) -> TIMER0_EV_W {
        TIMER0_EV_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Write 1 to clear EVTOMCUFLAGS.TDC_DONE. Read value is 0."]
    #[inline(always)]
    pub fn tdc_done(&mut self) -> TDC_DONE_W {
        TDC_DONE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPB. Read value is 0."]
    #[inline(always)]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W {
        AUX_COMPB_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPA. Read value is 0."]
    #[inline(always)]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W {
        AUX_COMPA_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Write 1 to clear EVTOMCUFLAGS.AON_WU_EV. Read value is 0."]
    #[inline(always)]
    pub fn aon_wu_ev(&mut self) -> AON_WU_EV_W {
        AON_WU_EV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtomcuflagsclr](index.html) module"]
pub struct EVTOMCUFLAGSCLR_SPEC;
impl crate::RegisterSpec for EVTOMCUFLAGSCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evtomcuflagsclr::R](R) reader structure"]
impl crate::Readable for EVTOMCUFLAGSCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evtomcuflagsclr::W](W) writer structure"]
impl crate::Writable for EVTOMCUFLAGSCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVTOMCUFLAGSCLR to value 0"]
impl crate::Resettable for EVTOMCUFLAGSCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
