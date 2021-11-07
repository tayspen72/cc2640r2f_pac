#[doc = "Register `EVTOMCUPOL` reader"]
pub struct R(crate::R<EVTOMCUPOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVTOMCUPOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVTOMCUPOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVTOMCUPOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVTOMCUPOL` writer"]
pub struct W(crate::W<EVTOMCUPOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVTOMCUPOL_SPEC>;
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
impl From<crate::W<EVTOMCUPOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVTOMCUPOL_SPEC>) -> Self {
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
#[doc = "10:10\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_IRQ.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_IRQ_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<ADC_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_IRQ` reader - 10:10\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_IRQ."]
pub struct ADC_IRQ_R(crate::FieldReader<bool, ADC_IRQ_A>);
impl ADC_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_IRQ_A {
        match self.bits {
            true => ADC_IRQ_A::LOW,
            false => ADC_IRQ_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == ADC_IRQ_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == ADC_IRQ_A::HIGH
    }
}
impl core::ops::Deref for ADC_IRQ_R {
    type Target = crate::FieldReader<bool, ADC_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_IRQ` writer - 10:10\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_IRQ."]
pub struct ADC_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ADC_IRQ_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ADC_IRQ_A::HIGH)
    }
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
#[doc = "9:9\\]
Select the event source level that sets EVTOMCUFLAGS.OBSMUX0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OBSMUX0_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<OBSMUX0_A> for bool {
    #[inline(always)]
    fn from(variant: OBSMUX0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBSMUX0` reader - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.OBSMUX0."]
pub struct OBSMUX0_R(crate::FieldReader<bool, OBSMUX0_A>);
impl OBSMUX0_R {
    pub(crate) fn new(bits: bool) -> Self {
        OBSMUX0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBSMUX0_A {
        match self.bits {
            true => OBSMUX0_A::LOW,
            false => OBSMUX0_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == OBSMUX0_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == OBSMUX0_A::HIGH
    }
}
impl core::ops::Deref for OBSMUX0_R {
    type Target = crate::FieldReader<bool, OBSMUX0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OBSMUX0` writer - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.OBSMUX0."]
pub struct OBSMUX0_W<'a> {
    w: &'a mut W,
}
impl<'a> OBSMUX0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OBSMUX0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OBSMUX0_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OBSMUX0_A::HIGH)
    }
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
#[doc = "8:8\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_FIFO_ALMOST_FULL_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<ADC_FIFO_ALMOST_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_FIFO_ALMOST_FULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_FIFO_ALMOST_FULL` reader - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
pub struct ADC_FIFO_ALMOST_FULL_R(crate::FieldReader<bool, ADC_FIFO_ALMOST_FULL_A>);
impl ADC_FIFO_ALMOST_FULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_FIFO_ALMOST_FULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_FIFO_ALMOST_FULL_A {
        match self.bits {
            true => ADC_FIFO_ALMOST_FULL_A::LOW,
            false => ADC_FIFO_ALMOST_FULL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == ADC_FIFO_ALMOST_FULL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == ADC_FIFO_ALMOST_FULL_A::HIGH
    }
}
impl core::ops::Deref for ADC_FIFO_ALMOST_FULL_R {
    type Target = crate::FieldReader<bool, ADC_FIFO_ALMOST_FULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_FIFO_ALMOST_FULL` writer - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
pub struct ADC_FIFO_ALMOST_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_FIFO_ALMOST_FULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_FIFO_ALMOST_FULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ADC_FIFO_ALMOST_FULL_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ADC_FIFO_ALMOST_FULL_A::HIGH)
    }
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
#[doc = "7:7\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_DONE_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<ADC_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_DONE` reader - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_DONE."]
pub struct ADC_DONE_R(crate::FieldReader<bool, ADC_DONE_A>);
impl ADC_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_DONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DONE_A {
        match self.bits {
            true => ADC_DONE_A::LOW,
            false => ADC_DONE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == ADC_DONE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == ADC_DONE_A::HIGH
    }
}
impl core::ops::Deref for ADC_DONE_R {
    type Target = crate::FieldReader<bool, ADC_DONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_DONE` writer - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_DONE."]
pub struct ADC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_DONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ADC_DONE_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ADC_DONE_A::HIGH)
    }
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
#[doc = "6:6\\]
Select the event source level that sets EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPH_AUTOTAKE_DONE_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<SMPH_AUTOTAKE_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: SMPH_AUTOTAKE_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPH_AUTOTAKE_DONE` reader - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE."]
pub struct SMPH_AUTOTAKE_DONE_R(crate::FieldReader<bool, SMPH_AUTOTAKE_DONE_A>);
impl SMPH_AUTOTAKE_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMPH_AUTOTAKE_DONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPH_AUTOTAKE_DONE_A {
        match self.bits {
            true => SMPH_AUTOTAKE_DONE_A::LOW,
            false => SMPH_AUTOTAKE_DONE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SMPH_AUTOTAKE_DONE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SMPH_AUTOTAKE_DONE_A::HIGH
    }
}
impl core::ops::Deref for SMPH_AUTOTAKE_DONE_R {
    type Target = crate::FieldReader<bool, SMPH_AUTOTAKE_DONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPH_AUTOTAKE_DONE` writer - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE."]
pub struct SMPH_AUTOTAKE_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPH_AUTOTAKE_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPH_AUTOTAKE_DONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SMPH_AUTOTAKE_DONE_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SMPH_AUTOTAKE_DONE_A::HIGH)
    }
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
#[doc = "5:5\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER1_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER1_EV_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<TIMER1_EV_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER1_EV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER1_EV` reader - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER1_EV."]
pub struct TIMER1_EV_R(crate::FieldReader<bool, TIMER1_EV_A>);
impl TIMER1_EV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_EV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER1_EV_A {
        match self.bits {
            true => TIMER1_EV_A::LOW,
            false => TIMER1_EV_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == TIMER1_EV_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == TIMER1_EV_A::HIGH
    }
}
impl core::ops::Deref for TIMER1_EV_R {
    type Target = crate::FieldReader<bool, TIMER1_EV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_EV` writer - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER1_EV."]
pub struct TIMER1_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER1_EV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(TIMER1_EV_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(TIMER1_EV_A::HIGH)
    }
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
#[doc = "4:4\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER0_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER0_EV_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<TIMER0_EV_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER0_EV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER0_EV` reader - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER0_EV."]
pub struct TIMER0_EV_R(crate::FieldReader<bool, TIMER0_EV_A>);
impl TIMER0_EV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_EV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER0_EV_A {
        match self.bits {
            true => TIMER0_EV_A::LOW,
            false => TIMER0_EV_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == TIMER0_EV_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == TIMER0_EV_A::HIGH
    }
}
impl core::ops::Deref for TIMER0_EV_R {
    type Target = crate::FieldReader<bool, TIMER0_EV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_EV` writer - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER0_EV."]
pub struct TIMER0_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER0_EV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(TIMER0_EV_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(TIMER0_EV_A::HIGH)
    }
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
#[doc = "3:3\\]
Select the event source level that sets EVTOMCUFLAGS.TDC_DONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDC_DONE_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<TDC_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: TDC_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDC_DONE` reader - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.TDC_DONE."]
pub struct TDC_DONE_R(crate::FieldReader<bool, TDC_DONE_A>);
impl TDC_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDC_DONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDC_DONE_A {
        match self.bits {
            true => TDC_DONE_A::LOW,
            false => TDC_DONE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == TDC_DONE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == TDC_DONE_A::HIGH
    }
}
impl core::ops::Deref for TDC_DONE_R {
    type Target = crate::FieldReader<bool, TDC_DONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDC_DONE` writer - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.TDC_DONE."]
pub struct TDC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDC_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDC_DONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(TDC_DONE_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(TDC_DONE_A::HIGH)
    }
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
#[doc = "2:2\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPB.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_COMPB_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_COMPB_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_COMPB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_COMPB` reader - 2:2\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPB."]
pub struct AUX_COMPB_R(crate::FieldReader<bool, AUX_COMPB_A>);
impl AUX_COMPB_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUX_COMPB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_COMPB_A {
        match self.bits {
            true => AUX_COMPB_A::LOW,
            false => AUX_COMPB_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == AUX_COMPB_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == AUX_COMPB_A::HIGH
    }
}
impl core::ops::Deref for AUX_COMPB_R {
    type Target = crate::FieldReader<bool, AUX_COMPB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUX_COMPB` writer - 2:2\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPB."]
pub struct AUX_COMPB_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_COMPB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_COMPB_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_COMPB_A::HIGH)
    }
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
#[doc = "1:1\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_COMPA_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AUX_COMPA_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_COMPA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_COMPA` reader - 1:1\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPA."]
pub struct AUX_COMPA_R(crate::FieldReader<bool, AUX_COMPA_A>);
impl AUX_COMPA_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUX_COMPA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_COMPA_A {
        match self.bits {
            true => AUX_COMPA_A::LOW,
            false => AUX_COMPA_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == AUX_COMPA_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == AUX_COMPA_A::HIGH
    }
}
impl core::ops::Deref for AUX_COMPA_R {
    type Target = crate::FieldReader<bool, AUX_COMPA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUX_COMPA` writer - 1:1\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPA."]
pub struct AUX_COMPA_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_COMPA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_COMPA_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_COMPA_A::HIGH)
    }
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
#[doc = "0:0\\]
Select the event source level that sets EVTOMCUFLAGS.AON_WU_EV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AON_WU_EV_A {
    #[doc = "1: Low level"]
    LOW = 1,
    #[doc = "0: High level"]
    HIGH = 0,
}
impl From<AON_WU_EV_A> for bool {
    #[inline(always)]
    fn from(variant: AON_WU_EV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AON_WU_EV` reader - 0:0\\]
Select the event source level that sets EVTOMCUFLAGS.AON_WU_EV."]
pub struct AON_WU_EV_R(crate::FieldReader<bool, AON_WU_EV_A>);
impl AON_WU_EV_R {
    pub(crate) fn new(bits: bool) -> Self {
        AON_WU_EV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AON_WU_EV_A {
        match self.bits {
            true => AON_WU_EV_A::LOW,
            false => AON_WU_EV_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == AON_WU_EV_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == AON_WU_EV_A::HIGH
    }
}
impl core::ops::Deref for AON_WU_EV_R {
    type Target = crate::FieldReader<bool, AON_WU_EV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AON_WU_EV` writer - 0:0\\]
Select the event source level that sets EVTOMCUFLAGS.AON_WU_EV."]
pub struct AON_WU_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_WU_EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AON_WU_EV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AON_WU_EV_A::LOW)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(AON_WU_EV_A::HIGH)
    }
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
Select the event source level that sets EVTOMCUFLAGS.ADC_IRQ."]
    #[inline(always)]
    pub fn adc_irq(&self) -> ADC_IRQ_R {
        ADC_IRQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.OBSMUX0."]
    #[inline(always)]
    pub fn obsmux0(&self) -> OBSMUX0_R {
        OBSMUX0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    #[inline(always)]
    pub fn adc_fifo_almost_full(&self) -> ADC_FIFO_ALMOST_FULL_R {
        ADC_FIFO_ALMOST_FULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_DONE."]
    #[inline(always)]
    pub fn adc_done(&self) -> ADC_DONE_R {
        ADC_DONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE."]
    #[inline(always)]
    pub fn smph_autotake_done(&self) -> SMPH_AUTOTAKE_DONE_R {
        SMPH_AUTOTAKE_DONE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER1_EV."]
    #[inline(always)]
    pub fn timer1_ev(&self) -> TIMER1_EV_R {
        TIMER1_EV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER0_EV."]
    #[inline(always)]
    pub fn timer0_ev(&self) -> TIMER0_EV_R {
        TIMER0_EV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.TDC_DONE."]
    #[inline(always)]
    pub fn tdc_done(&self) -> TDC_DONE_R {
        TDC_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Select the event source level that sets EVTOMCUFLAGS.AON_WU_EV."]
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
Select the event source level that sets EVTOMCUFLAGS.ADC_IRQ."]
    #[inline(always)]
    pub fn adc_irq(&mut self) -> ADC_IRQ_W {
        ADC_IRQ_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Select the event source level that sets EVTOMCUFLAGS.OBSMUX0."]
    #[inline(always)]
    pub fn obsmux0(&mut self) -> OBSMUX0_W {
        OBSMUX0_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    #[inline(always)]
    pub fn adc_fifo_almost_full(&mut self) -> ADC_FIFO_ALMOST_FULL_W {
        ADC_FIFO_ALMOST_FULL_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Select the event source level that sets EVTOMCUFLAGS.ADC_DONE."]
    #[inline(always)]
    pub fn adc_done(&mut self) -> ADC_DONE_W {
        ADC_DONE_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Select the event source level that sets EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE."]
    #[inline(always)]
    pub fn smph_autotake_done(&mut self) -> SMPH_AUTOTAKE_DONE_W {
        SMPH_AUTOTAKE_DONE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER1_EV."]
    #[inline(always)]
    pub fn timer1_ev(&mut self) -> TIMER1_EV_W {
        TIMER1_EV_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Select the event source level that sets EVTOMCUFLAGS.TIMER0_EV."]
    #[inline(always)]
    pub fn timer0_ev(&mut self) -> TIMER0_EV_W {
        TIMER0_EV_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Select the event source level that sets EVTOMCUFLAGS.TDC_DONE."]
    #[inline(always)]
    pub fn tdc_done(&mut self) -> TDC_DONE_W {
        TDC_DONE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W {
        AUX_COMPB_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Select the event source level that sets EVTOMCUFLAGS.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W {
        AUX_COMPA_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Select the event source level that sets EVTOMCUFLAGS.AON_WU_EV."]
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
#[doc = "Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtomcupol](index.html) module"]
pub struct EVTOMCUPOL_SPEC;
impl crate::RegisterSpec for EVTOMCUPOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evtomcupol::R](R) reader structure"]
impl crate::Readable for EVTOMCUPOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evtomcupol::W](W) writer structure"]
impl crate::Writable for EVTOMCUPOL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVTOMCUPOL to value 0"]
impl crate::Resettable for EVTOMCUPOL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
