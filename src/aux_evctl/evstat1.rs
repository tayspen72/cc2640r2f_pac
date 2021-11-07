#[doc = "Register `EVSTAT1` reader"]
pub struct R(crate::R<EVSTAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVSTAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVSTAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVSTAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVSTAT1` writer"]
pub struct W(crate::W<EVSTAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVSTAT1_SPEC>;
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
impl From<crate::W<EVSTAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVSTAT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_R(crate::FieldReader<u16, u16>);
impl RESERVED16_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED16_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `ADC_IRQ` reader - 15:15\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
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
#[doc = "Field `ADC_IRQ` writer - 15:15\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `MCU_EV` reader - 14:14\\]
Event from EVENT configured by EVENT:AUXSEL0."]
pub struct MCU_EV_R(crate::FieldReader<bool, bool>);
impl MCU_EV_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCU_EV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_EV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_EV` writer - 14:14\\]
Event from EVENT configured by EVENT:AUXSEL0."]
pub struct MCU_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_EV_W<'a> {
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
#[doc = "Field `ACLK_REF` reader - 13:13\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_WUC:REFCLKCTL.REQ."]
pub struct ACLK_REF_R(crate::FieldReader<bool, bool>);
impl ACLK_REF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACLK_REF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACLK_REF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACLK_REF` writer - 13:13\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_WUC:REFCLKCTL.REQ."]
pub struct ACLK_REF_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_REF_W<'a> {
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
#[doc = "Field `AUXIO15` reader - 12:12\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
pub struct AUXIO15_R(crate::FieldReader<bool, bool>);
impl AUXIO15_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXIO15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXIO15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXIO15` writer - 12:12\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
pub struct AUXIO15_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO15_W<'a> {
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
#[doc = "Field `AUXIO14` reader - 11:11\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
pub struct AUXIO14_R(crate::FieldReader<bool, bool>);
impl AUXIO14_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXIO14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXIO14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXIO14` writer - 11:11\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
pub struct AUXIO14_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO14_W<'a> {
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
#[doc = "Field `AUXIO13` reader - 10:10\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
pub struct AUXIO13_R(crate::FieldReader<bool, bool>);
impl AUXIO13_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXIO13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXIO13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXIO13` writer - 10:10\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
pub struct AUXIO13_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO13_W<'a> {
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
#[doc = "Field `AUXIO12` reader - 9:9\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
pub struct AUXIO12_R(crate::FieldReader<bool, bool>);
impl AUXIO12_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXIO12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXIO12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXIO12` writer - 9:9\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
pub struct AUXIO12_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO12_W<'a> {
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
#[doc = "Field `AUXIO11` reader - 8:8\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
pub struct AUXIO11_R(crate::FieldReader<bool, bool>);
impl AUXIO11_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXIO11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXIO11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXIO11` writer - 8:8\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
pub struct AUXIO11_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO11_W<'a> {
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
#[doc = "Field `AUXIO10` reader - 7:7\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
pub struct AUXIO10_R(crate::FieldReader<bool, bool>);
impl AUXIO10_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXIO10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXIO10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXIO10` writer - 7:7\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
pub struct AUXIO10_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO10_W<'a> {
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
#[doc = "Field `AUXIO9` reader - 6:6\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
pub struct AUXIO9_R(crate::FieldReader<bool, bool>);
impl AUXIO9_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXIO9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXIO9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXIO9` writer - 6:6\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
pub struct AUXIO9_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO9_W<'a> {
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
#[doc = "Field `AUXIO8` reader - 5:5\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
pub struct AUXIO8_R(crate::FieldReader<bool, bool>);
impl AUXIO8_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXIO8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXIO8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXIO8` writer - 5:5\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
pub struct AUXIO8_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO8_W<'a> {
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
#[doc = "Field `AUXIO7` reader - 4:4\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
pub struct AUXIO7_R(crate::FieldReader<bool, bool>);
impl AUXIO7_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXIO7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXIO7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXIO7` writer - 4:4\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
pub struct AUXIO7_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO7_W<'a> {
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
#[doc = "Field `AUXIO6` reader - 3:3\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
pub struct AUXIO6_R(crate::FieldReader<bool, bool>);
impl AUXIO6_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXIO6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXIO6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXIO6` writer - 3:3\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
pub struct AUXIO6_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO6_W<'a> {
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
#[doc = "Field `AUXIO5` reader - 2:2\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
pub struct AUXIO5_R(crate::FieldReader<bool, bool>);
impl AUXIO5_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXIO5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXIO5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXIO5` writer - 2:2\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
pub struct AUXIO5_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO5_W<'a> {
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
#[doc = "Field `AUXIO4` reader - 1:1\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
pub struct AUXIO4_R(crate::FieldReader<bool, bool>);
impl AUXIO4_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXIO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXIO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXIO4` writer - 1:1\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
pub struct AUXIO4_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO4_W<'a> {
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
#[doc = "Field `AUXIO3` reader - 0:0\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
pub struct AUXIO3_R(crate::FieldReader<bool, bool>);
impl AUXIO3_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXIO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXIO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXIO3` writer - 0:0\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
pub struct AUXIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO3_W<'a> {
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
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[inline(always)]
    pub fn adc_irq(&self) -> ADC_IRQ_R {
        ADC_IRQ_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Event from EVENT configured by EVENT:AUXSEL0."]
    #[inline(always)]
    pub fn mcu_ev(&self) -> MCU_EV_R {
        MCU_EV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_WUC:REFCLKCTL.REQ."]
    #[inline(always)]
    pub fn aclk_ref(&self) -> ACLK_REF_R {
        ACLK_REF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio15(&self) -> AUXIO15_R {
        AUXIO15_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio14(&self) -> AUXIO14_R {
        AUXIO14_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio13(&self) -> AUXIO13_R {
        AUXIO13_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio12(&self) -> AUXIO12_R {
        AUXIO12_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio11(&self) -> AUXIO11_R {
        AUXIO11_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio10(&self) -> AUXIO10_R {
        AUXIO10_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio9(&self) -> AUXIO9_R {
        AUXIO9_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio8(&self) -> AUXIO8_R {
        AUXIO8_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio7(&self) -> AUXIO7_R {
        AUXIO7_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio6(&self) -> AUXIO6_R {
        AUXIO6_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio5(&self) -> AUXIO5_R {
        AUXIO5_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio4(&self) -> AUXIO4_R {
        AUXIO4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio3(&self) -> AUXIO3_R {
        AUXIO3_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[inline(always)]
    pub fn adc_irq(&mut self) -> ADC_IRQ_W {
        ADC_IRQ_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Event from EVENT configured by EVENT:AUXSEL0."]
    #[inline(always)]
    pub fn mcu_ev(&mut self) -> MCU_EV_W {
        MCU_EV_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_WUC:REFCLKCTL.REQ."]
    #[inline(always)]
    pub fn aclk_ref(&mut self) -> ACLK_REF_W {
        ACLK_REF_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio15(&mut self) -> AUXIO15_W {
        AUXIO15_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio14(&mut self) -> AUXIO14_W {
        AUXIO14_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio13(&mut self) -> AUXIO13_W {
        AUXIO13_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio12(&mut self) -> AUXIO12_W {
        AUXIO12_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio11(&mut self) -> AUXIO11_W {
        AUXIO11_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio10(&mut self) -> AUXIO10_W {
        AUXIO10_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio9(&mut self) -> AUXIO9_W {
        AUXIO9_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio8(&mut self) -> AUXIO8_W {
        AUXIO8_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio7(&mut self) -> AUXIO7_W {
        AUXIO7_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio6(&mut self) -> AUXIO6_W {
        AUXIO6_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio5(&mut self) -> AUXIO5_W {
        AUXIO5_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio4(&mut self) -> AUXIO4_W {
        AUXIO4_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio3(&mut self) -> AUXIO3_W {
        AUXIO3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Status 1 Current event source levels, 31:16\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat1](index.html) module"]
pub struct EVSTAT1_SPEC;
impl crate::RegisterSpec for EVSTAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evstat1::R](R) reader structure"]
impl crate::Readable for EVSTAT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evstat1::W](W) writer structure"]
impl crate::Writable for EVSTAT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVSTAT1 to value 0"]
impl crate::Resettable for EVSTAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
