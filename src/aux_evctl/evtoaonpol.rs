#[doc = "Register `EVTOAONPOL` reader"]
pub struct R(crate::R<EVTOAONPOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVTOAONPOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVTOAONPOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVTOAONPOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVTOAONPOL` writer"]
pub struct W(crate::W<EVTOAONPOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVTOAONPOL_SPEC>;
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
impl From<crate::W<EVTOAONPOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVTOAONPOL_SPEC>) -> Self {
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
#[doc = "8:8\\]
Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV.\n\nValue on reset: 0"]
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
#[doc = "Field `TIMER1_EV` reader - 8:8\\]
Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV."]
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
#[doc = "Field `TIMER1_EV` writer - 8:8\\]
Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV."]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "7:7\\]
Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV.\n\nValue on reset: 0"]
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
#[doc = "Field `TIMER0_EV` reader - 7:7\\]
Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV."]
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
#[doc = "Field `TIMER0_EV` writer - 7:7\\]
Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV."]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "6:6\\]
Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE.\n\nValue on reset: 0"]
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
#[doc = "Field `TDC_DONE` reader - 6:6\\]
Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE."]
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
#[doc = "Field `TDC_DONE` writer - 6:6\\]
Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE."]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "5:5\\]
Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE.\n\nValue on reset: 0"]
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
#[doc = "Field `ADC_DONE` reader - 5:5\\]
Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE."]
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
#[doc = "Field `ADC_DONE` writer - 5:5\\]
Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE."]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "4:4\\]
Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_COMPB_A {
    #[doc = "1: Falling edge"]
    LOW = 1,
    #[doc = "0: Rising edge"]
    HIGH = 0,
}
impl From<AUX_COMPB_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_COMPB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_COMPB` reader - 4:4\\]
Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
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
#[doc = "Field `AUX_COMPB` writer - 4:4\\]
Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
pub struct AUX_COMPB_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_COMPB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_COMPB_A::LOW)
    }
    #[doc = "Rising edge"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "3:3\\]
Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_COMPA_A {
    #[doc = "1: Falling edge"]
    LOW = 1,
    #[doc = "0: Rising edge"]
    HIGH = 0,
}
impl From<AUX_COMPA_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_COMPA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_COMPA` reader - 3:3\\]
Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
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
#[doc = "Field `AUX_COMPA` writer - 3:3\\]
Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
pub struct AUX_COMPA_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_COMPA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_COMPA_A::LOW)
    }
    #[doc = "Rising edge"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RESERVED2` reader - 2:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_R(crate::FieldReader<u8, u8>);
impl RESERVED2_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED2` writer - 2:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
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
Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV."]
    #[inline(always)]
    pub fn timer1_ev(&self) -> TIMER1_EV_R {
        TIMER1_EV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV."]
    #[inline(always)]
    pub fn timer0_ev(&self) -> TIMER0_EV_R {
        TIMER0_EV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE."]
    #[inline(always)]
    pub fn tdc_done(&self) -> TDC_DONE_R {
        TDC_DONE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE."]
    #[inline(always)]
    pub fn adc_done(&self) -> ADC_DONE_R {
        ADC_DONE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits & 0x07) as u8)
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
Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV."]
    #[inline(always)]
    pub fn timer1_ev(&mut self) -> TIMER1_EV_W {
        TIMER1_EV_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV."]
    #[inline(always)]
    pub fn timer0_ev(&mut self) -> TIMER0_EV_W {
        TIMER0_EV_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE."]
    #[inline(always)]
    pub fn tdc_done(&mut self) -> TDC_DONE_W {
        TDC_DONE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE."]
    #[inline(always)]
    pub fn adc_done(&mut self) -> ADC_DONE_W {
        ADC_DONE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W {
        AUX_COMPB_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W {
        AUX_COMPA_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Events To AON Polarity Event source polarity configuration for EVTOAONFLAGS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtoaonpol](index.html) module"]
pub struct EVTOAONPOL_SPEC;
impl crate::RegisterSpec for EVTOAONPOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evtoaonpol::R](R) reader structure"]
impl crate::Readable for EVTOAONPOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evtoaonpol::W](W) writer structure"]
impl crate::Writable for EVTOAONPOL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVTOAONPOL to value 0"]
impl crate::Resettable for EVTOAONPOL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
