#[doc = "Register `EVSTAT0` reader"]
pub struct R(crate::R<EVSTAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVSTAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVSTAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVSTAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVSTAT0` writer"]
pub struct W(crate::W<EVSTAT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVSTAT0_SPEC>;
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
impl From<crate::W<EVSTAT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVSTAT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUXIO2` reader - 15:15\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
pub struct AUXIO2_R(crate::FieldReader<bool, bool>);
impl AUXIO2_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXIO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXIO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXIO2` writer - 15:15\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
pub struct AUXIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO2_W<'a> {
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
#[doc = "Field `AUXIO1` reader - 14:14\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
pub struct AUXIO1_R(crate::FieldReader<bool, bool>);
impl AUXIO1_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXIO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXIO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXIO1` writer - 14:14\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
pub struct AUXIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `AUXIO0` reader - 13:13\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
pub struct AUXIO0_R(crate::FieldReader<bool, bool>);
impl AUXIO0_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXIO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXIO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXIO0` writer - 13:13\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
pub struct AUXIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `AON_PROG_WU` reader - 12:12\\]
AON_EVENT:AUXWUSEL.WU2_EV OR AON_EVENT:AUXWUSEL.WU1_EV OR AON_EVENT:AUXWUSEL.WU0_EV"]
pub struct AON_PROG_WU_R(crate::FieldReader<bool, bool>);
impl AON_PROG_WU_R {
    pub(crate) fn new(bits: bool) -> Self {
        AON_PROG_WU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AON_PROG_WU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AON_PROG_WU` writer - 12:12\\]
AON_EVENT:AUXWUSEL.WU2_EV OR AON_EVENT:AUXWUSEL.WU1_EV OR AON_EVENT:AUXWUSEL.WU0_EV"]
pub struct AON_PROG_WU_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_PROG_WU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `AON_SW` reader - 11:11\\]
AON_WUC:AUXCTL.SWEV"]
pub struct AON_SW_R(crate::FieldReader<bool, bool>);
impl AON_SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        AON_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AON_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AON_SW` writer - 11:11\\]
AON_WUC:AUXCTL.SWEV"]
pub struct AON_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_SW_W<'a> {
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
#[doc = "Field `OBSMUX1` reader - 10:10\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
pub struct OBSMUX1_R(crate::FieldReader<bool, bool>);
impl OBSMUX1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OBSMUX1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OBSMUX1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OBSMUX1` writer - 10:10\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
pub struct OBSMUX1_W<'a> {
    w: &'a mut W,
}
impl<'a> OBSMUX1_W<'a> {
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
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
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
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
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
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
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
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
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
AUX_ANAIF ADC conversion done event."]
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
AUX_ANAIF ADC conversion done event."]
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
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
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
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
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
AUX_TIMER1_EV event, see AUX_TIMER:T1TARGET for description."]
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
AUX_TIMER1_EV event, see AUX_TIMER:T1TARGET for description."]
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
AUX_TIMER0_EV event, see AUX_TIMER:T0TARGET for description."]
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
AUX_TIMER0_EV event, see AUX_TIMER:T0TARGET for description."]
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
AUX_TDC:STAT.DONE"]
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
AUX_TDC:STAT.DONE"]
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
Comparator B output"]
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
Comparator B output"]
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
Comparator A output"]
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
Comparator A output"]
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
#[doc = "Field `AON_RTC_CH2` reader - 0:0\\]
AON_RTC:EVFLAGS.CH2"]
pub struct AON_RTC_CH2_R(crate::FieldReader<bool, bool>);
impl AON_RTC_CH2_R {
    pub(crate) fn new(bits: bool) -> Self {
        AON_RTC_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AON_RTC_CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AON_RTC_CH2` writer - 0:0\\]
AON_RTC:EVFLAGS.CH2"]
pub struct AON_RTC_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_RTC_CH2_W<'a> {
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
    #[doc = "Bit 15 - 15:15\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio2(&self) -> AUXIO2_R {
        AUXIO2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio1(&self) -> AUXIO1_R {
        AUXIO1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio0(&self) -> AUXIO0_R {
        AUXIO0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
AON_EVENT:AUXWUSEL.WU2_EV OR AON_EVENT:AUXWUSEL.WU1_EV OR AON_EVENT:AUXWUSEL.WU0_EV"]
    #[inline(always)]
    pub fn aon_prog_wu(&self) -> AON_PROG_WU_R {
        AON_PROG_WU_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
AON_WUC:AUXCTL.SWEV"]
    #[inline(always)]
    pub fn aon_sw(&self) -> AON_SW_R {
        AON_SW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
    #[inline(always)]
    pub fn obsmux1(&self) -> OBSMUX1_R {
        OBSMUX1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
    #[inline(always)]
    pub fn obsmux0(&self) -> OBSMUX0_R {
        OBSMUX0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
    #[inline(always)]
    pub fn adc_fifo_almost_full(&self) -> ADC_FIFO_ALMOST_FULL_R {
        ADC_FIFO_ALMOST_FULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AUX_ANAIF ADC conversion done event."]
    #[inline(always)]
    pub fn adc_done(&self) -> ADC_DONE_R {
        ADC_DONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
    #[inline(always)]
    pub fn smph_autotake_done(&self) -> SMPH_AUTOTAKE_DONE_R {
        SMPH_AUTOTAKE_DONE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER:T1TARGET for description."]
    #[inline(always)]
    pub fn timer1_ev(&self) -> TIMER1_EV_R {
        TIMER1_EV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
AUX_TIMER0_EV event, see AUX_TIMER:T0TARGET for description."]
    #[inline(always)]
    pub fn timer0_ev(&self) -> TIMER0_EV_R {
        TIMER0_EV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AUX_TDC:STAT.DONE"]
    #[inline(always)]
    pub fn tdc_done(&self) -> TDC_DONE_R {
        TDC_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Comparator B output"]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Comparator A output"]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
AON_RTC:EVFLAGS.CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(&self) -> AON_RTC_CH2_R {
        AON_RTC_CH2_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - 15:15\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio2(&mut self) -> AUXIO2_W {
        AUXIO2_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio1(&mut self) -> AUXIO1_W {
        AUXIO1_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio0(&mut self) -> AUXIO0_W {
        AUXIO0_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
AON_EVENT:AUXWUSEL.WU2_EV OR AON_EVENT:AUXWUSEL.WU1_EV OR AON_EVENT:AUXWUSEL.WU0_EV"]
    #[inline(always)]
    pub fn aon_prog_wu(&mut self) -> AON_PROG_WU_W {
        AON_PROG_WU_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
AON_WUC:AUXCTL.SWEV"]
    #[inline(always)]
    pub fn aon_sw(&mut self) -> AON_SW_W {
        AON_SW_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
    #[inline(always)]
    pub fn obsmux1(&mut self) -> OBSMUX1_W {
        OBSMUX1_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
    #[inline(always)]
    pub fn obsmux0(&mut self) -> OBSMUX0_W {
        OBSMUX0_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
    #[inline(always)]
    pub fn adc_fifo_almost_full(&mut self) -> ADC_FIFO_ALMOST_FULL_W {
        ADC_FIFO_ALMOST_FULL_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
AUX_ANAIF ADC conversion done event."]
    #[inline(always)]
    pub fn adc_done(&mut self) -> ADC_DONE_W {
        ADC_DONE_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
    #[inline(always)]
    pub fn smph_autotake_done(&mut self) -> SMPH_AUTOTAKE_DONE_W {
        SMPH_AUTOTAKE_DONE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER:T1TARGET for description."]
    #[inline(always)]
    pub fn timer1_ev(&mut self) -> TIMER1_EV_W {
        TIMER1_EV_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
AUX_TIMER0_EV event, see AUX_TIMER:T0TARGET for description."]
    #[inline(always)]
    pub fn timer0_ev(&mut self) -> TIMER0_EV_W {
        TIMER0_EV_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
AUX_TDC:STAT.DONE"]
    #[inline(always)]
    pub fn tdc_done(&mut self) -> TDC_DONE_W {
        TDC_DONE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Comparator B output"]
    #[inline(always)]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W {
        AUX_COMPB_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Comparator A output"]
    #[inline(always)]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W {
        AUX_COMPA_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
AON_RTC:EVFLAGS.CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(&mut self) -> AON_RTC_CH2_W {
        AON_RTC_CH2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Status 0 Register holds events 0 thru 15 of the 32-bit event bus that is synchronous to AUX clock. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat0](index.html) module"]
pub struct EVSTAT0_SPEC;
impl crate::RegisterSpec for EVSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evstat0::R](R) reader structure"]
impl crate::Readable for EVSTAT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evstat0::W](W) writer structure"]
impl crate::Writable for EVSTAT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVSTAT0 to value 0"]
impl crate::Resettable for EVSTAT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
