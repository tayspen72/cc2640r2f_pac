#[doc = "Register `MODCLKEN0` reader"]
pub struct R(crate::R<MODCLKEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODCLKEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODCLKEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODCLKEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODCLKEN0` writer"]
pub struct W(crate::W<MODCLKEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODCLKEN0_SPEC>;
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
impl From<crate::W<MODCLKEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODCLKEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "7:7\\]
Enables (1) or disables (0) clock for AUX_ADI4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_ADI4_A {
    #[doc = "1: System CPU has requested clock for AUX_ADI4"]
    EN = 1,
    #[doc = "0: System CPU has not requested clock for AUX_ADI4"]
    DIS = 0,
}
impl From<AUX_ADI4_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_ADI4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_ADI4` reader - 7:7\\]
Enables (1) or disables (0) clock for AUX_ADI4."]
pub struct AUX_ADI4_R(crate::FieldReader<bool, AUX_ADI4_A>);
impl AUX_ADI4_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUX_ADI4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_ADI4_A {
        match self.bits {
            true => AUX_ADI4_A::EN,
            false => AUX_ADI4_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == AUX_ADI4_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == AUX_ADI4_A::DIS
    }
}
impl core::ops::Deref for AUX_ADI4_R {
    type Target = crate::FieldReader<bool, AUX_ADI4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUX_ADI4` writer - 7:7\\]
Enables (1) or disables (0) clock for AUX_ADI4."]
pub struct AUX_ADI4_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_ADI4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_ADI4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System CPU has requested clock for AUX_ADI4"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(AUX_ADI4_A::EN)
    }
    #[doc = "System CPU has not requested clock for AUX_ADI4"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(AUX_ADI4_A::DIS)
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
Enables (1) or disables (0) clock for AUX_DDI0_OSC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_DDI0_OSC_A {
    #[doc = "1: System CPU has requested clock for AUX_DDI0_OSC"]
    EN = 1,
    #[doc = "0: System CPU has not requested clock for AUX_DDI0_OSC"]
    DIS = 0,
}
impl From<AUX_DDI0_OSC_A> for bool {
    #[inline(always)]
    fn from(variant: AUX_DDI0_OSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_DDI0_OSC` reader - 6:6\\]
Enables (1) or disables (0) clock for AUX_DDI0_OSC."]
pub struct AUX_DDI0_OSC_R(crate::FieldReader<bool, AUX_DDI0_OSC_A>);
impl AUX_DDI0_OSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUX_DDI0_OSC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUX_DDI0_OSC_A {
        match self.bits {
            true => AUX_DDI0_OSC_A::EN,
            false => AUX_DDI0_OSC_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == AUX_DDI0_OSC_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == AUX_DDI0_OSC_A::DIS
    }
}
impl core::ops::Deref for AUX_DDI0_OSC_R {
    type Target = crate::FieldReader<bool, AUX_DDI0_OSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUX_DDI0_OSC` writer - 6:6\\]
Enables (1) or disables (0) clock for AUX_DDI0_OSC."]
pub struct AUX_DDI0_OSC_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_DDI0_OSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUX_DDI0_OSC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System CPU has requested clock for AUX_DDI0_OSC"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(AUX_DDI0_OSC_A::EN)
    }
    #[doc = "System CPU has not requested clock for AUX_DDI0_OSC"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(AUX_DDI0_OSC_A::DIS)
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
Enables (1) or disables (0) clock for AUX_TDCIF. Note that the TDC counter and reference clock sources must be requested separately using TDCCLKCTL and REFCLKCTL, respectively.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDC_A {
    #[doc = "1: System CPU has requested clock for TDC"]
    EN = 1,
    #[doc = "0: System CPU has not requested clock for TDC"]
    DIS = 0,
}
impl From<TDC_A> for bool {
    #[inline(always)]
    fn from(variant: TDC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDC` reader - 5:5\\]
Enables (1) or disables (0) clock for AUX_TDCIF. Note that the TDC counter and reference clock sources must be requested separately using TDCCLKCTL and REFCLKCTL, respectively."]
pub struct TDC_R(crate::FieldReader<bool, TDC_A>);
impl TDC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDC_A {
        match self.bits {
            true => TDC_A::EN,
            false => TDC_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TDC_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TDC_A::DIS
    }
}
impl core::ops::Deref for TDC_R {
    type Target = crate::FieldReader<bool, TDC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDC` writer - 5:5\\]
Enables (1) or disables (0) clock for AUX_TDCIF. Note that the TDC counter and reference clock sources must be requested separately using TDCCLKCTL and REFCLKCTL, respectively."]
pub struct TDC_W<'a> {
    w: &'a mut W,
}
impl<'a> TDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System CPU has requested clock for TDC"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TDC_A::EN)
    }
    #[doc = "System CPU has not requested clock for TDC"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TDC_A::DIS)
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
Enables (1) or disables (0) clock for AUX_ANAIF. Note that the ADC internal clock must be requested separately using ADCCLKCTL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANAIF_A {
    #[doc = "1: System CPU has requested clock for ANAIF"]
    EN = 1,
    #[doc = "0: System CPU has not requested clock for ANAIF"]
    DIS = 0,
}
impl From<ANAIF_A> for bool {
    #[inline(always)]
    fn from(variant: ANAIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANAIF` reader - 4:4\\]
Enables (1) or disables (0) clock for AUX_ANAIF. Note that the ADC internal clock must be requested separately using ADCCLKCTL."]
pub struct ANAIF_R(crate::FieldReader<bool, ANAIF_A>);
impl ANAIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANAIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANAIF_A {
        match self.bits {
            true => ANAIF_A::EN,
            false => ANAIF_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == ANAIF_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == ANAIF_A::DIS
    }
}
impl core::ops::Deref for ANAIF_R {
    type Target = crate::FieldReader<bool, ANAIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANAIF` writer - 4:4\\]
Enables (1) or disables (0) clock for AUX_ANAIF. Note that the ADC internal clock must be requested separately using ADCCLKCTL."]
pub struct ANAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ANAIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANAIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System CPU has requested clock for ANAIF"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ANAIF_A::EN)
    }
    #[doc = "System CPU has not requested clock for ANAIF"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ANAIF_A::DIS)
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
Enables (1) or disables (0) clock for AUX_TIMER.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_A {
    #[doc = "1: System CPU has requested clock for TIMER"]
    EN = 1,
    #[doc = "0: System CPU has not requested clock for TIMER"]
    DIS = 0,
}
impl From<TIMER_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER` reader - 3:3\\]
Enables (1) or disables (0) clock for AUX_TIMER."]
pub struct TIMER_R(crate::FieldReader<bool, TIMER_A>);
impl TIMER_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_A {
        match self.bits {
            true => TIMER_A::EN,
            false => TIMER_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TIMER_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TIMER_A::DIS
    }
}
impl core::ops::Deref for TIMER_R {
    type Target = crate::FieldReader<bool, TIMER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER` writer - 3:3\\]
Enables (1) or disables (0) clock for AUX_TIMER."]
pub struct TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System CPU has requested clock for TIMER"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TIMER_A::EN)
    }
    #[doc = "System CPU has not requested clock for TIMER"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TIMER_A::DIS)
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
Enables (1) or disables (0) clock for AUX_AIODIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIODIO1_A {
    #[doc = "1: System CPU has requested clock for AIODIO1"]
    EN = 1,
    #[doc = "0: System CPU has not requested clock for AIODIO1"]
    DIS = 0,
}
impl From<AIODIO1_A> for bool {
    #[inline(always)]
    fn from(variant: AIODIO1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIODIO1` reader - 2:2\\]
Enables (1) or disables (0) clock for AUX_AIODIO1."]
pub struct AIODIO1_R(crate::FieldReader<bool, AIODIO1_A>);
impl AIODIO1_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIODIO1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AIODIO1_A {
        match self.bits {
            true => AIODIO1_A::EN,
            false => AIODIO1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == AIODIO1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == AIODIO1_A::DIS
    }
}
impl core::ops::Deref for AIODIO1_R {
    type Target = crate::FieldReader<bool, AIODIO1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIODIO1` writer - 2:2\\]
Enables (1) or disables (0) clock for AUX_AIODIO1."]
pub struct AIODIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> AIODIO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AIODIO1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System CPU has requested clock for AIODIO1"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(AIODIO1_A::EN)
    }
    #[doc = "System CPU has not requested clock for AIODIO1"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(AIODIO1_A::DIS)
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
Enables (1) or disables (0) clock for AUX_AIODIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIODIO0_A {
    #[doc = "1: System CPU has requested clock for AIODIO0"]
    EN = 1,
    #[doc = "0: System CPU has not requested clock for AIODIO0"]
    DIS = 0,
}
impl From<AIODIO0_A> for bool {
    #[inline(always)]
    fn from(variant: AIODIO0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIODIO0` reader - 1:1\\]
Enables (1) or disables (0) clock for AUX_AIODIO0."]
pub struct AIODIO0_R(crate::FieldReader<bool, AIODIO0_A>);
impl AIODIO0_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIODIO0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AIODIO0_A {
        match self.bits {
            true => AIODIO0_A::EN,
            false => AIODIO0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == AIODIO0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == AIODIO0_A::DIS
    }
}
impl core::ops::Deref for AIODIO0_R {
    type Target = crate::FieldReader<bool, AIODIO0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIODIO0` writer - 1:1\\]
Enables (1) or disables (0) clock for AUX_AIODIO0."]
pub struct AIODIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> AIODIO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AIODIO0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System CPU has requested clock for AIODIO0"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(AIODIO0_A::EN)
    }
    #[doc = "System CPU has not requested clock for AIODIO0"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(AIODIO0_A::DIS)
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
Enables (1) or disables (0) clock for AUX_SMPH.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPH_A {
    #[doc = "1: System CPU has requested clock for SMPH"]
    EN = 1,
    #[doc = "0: System CPU has not requested clock for SMPH"]
    DIS = 0,
}
impl From<SMPH_A> for bool {
    #[inline(always)]
    fn from(variant: SMPH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPH` reader - 0:0\\]
Enables (1) or disables (0) clock for AUX_SMPH."]
pub struct SMPH_R(crate::FieldReader<bool, SMPH_A>);
impl SMPH_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMPH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPH_A {
        match self.bits {
            true => SMPH_A::EN,
            false => SMPH_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == SMPH_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == SMPH_A::DIS
    }
}
impl core::ops::Deref for SMPH_R {
    type Target = crate::FieldReader<bool, SMPH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPH` writer - 0:0\\]
Enables (1) or disables (0) clock for AUX_SMPH."]
pub struct SMPH_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System CPU has requested clock for SMPH"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SMPH_A::EN)
    }
    #[doc = "System CPU has not requested clock for SMPH"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SMPH_A::DIS)
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
    #[doc = "Bit 7 - 7:7\\]
Enables (1) or disables (0) clock for AUX_ADI4."]
    #[inline(always)]
    pub fn aux_adi4(&self) -> AUX_ADI4_R {
        AUX_ADI4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enables (1) or disables (0) clock for AUX_DDI0_OSC."]
    #[inline(always)]
    pub fn aux_ddi0_osc(&self) -> AUX_DDI0_OSC_R {
        AUX_DDI0_OSC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enables (1) or disables (0) clock for AUX_TDCIF. Note that the TDC counter and reference clock sources must be requested separately using TDCCLKCTL and REFCLKCTL, respectively."]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables (1) or disables (0) clock for AUX_ANAIF. Note that the ADC internal clock must be requested separately using ADCCLKCTL."]
    #[inline(always)]
    pub fn anaif(&self) -> ANAIF_R {
        ANAIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables (1) or disables (0) clock for AUX_TIMER."]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables (1) or disables (0) clock for AUX_AIODIO1."]
    #[inline(always)]
    pub fn aiodio1(&self) -> AIODIO1_R {
        AIODIO1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables (1) or disables (0) clock for AUX_AIODIO0."]
    #[inline(always)]
    pub fn aiodio0(&self) -> AIODIO0_R {
        AIODIO0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Enables (1) or disables (0) clock for AUX_SMPH."]
    #[inline(always)]
    pub fn smph(&self) -> SMPH_R {
        SMPH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - 7:7\\]
Enables (1) or disables (0) clock for AUX_ADI4."]
    #[inline(always)]
    pub fn aux_adi4(&mut self) -> AUX_ADI4_W {
        AUX_ADI4_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Enables (1) or disables (0) clock for AUX_DDI0_OSC."]
    #[inline(always)]
    pub fn aux_ddi0_osc(&mut self) -> AUX_DDI0_OSC_W {
        AUX_DDI0_OSC_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Enables (1) or disables (0) clock for AUX_TDCIF. Note that the TDC counter and reference clock sources must be requested separately using TDCCLKCTL and REFCLKCTL, respectively."]
    #[inline(always)]
    pub fn tdc(&mut self) -> TDC_W {
        TDC_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Enables (1) or disables (0) clock for AUX_ANAIF. Note that the ADC internal clock must be requested separately using ADCCLKCTL."]
    #[inline(always)]
    pub fn anaif(&mut self) -> ANAIF_W {
        ANAIF_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Enables (1) or disables (0) clock for AUX_TIMER."]
    #[inline(always)]
    pub fn timer(&mut self) -> TIMER_W {
        TIMER_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Enables (1) or disables (0) clock for AUX_AIODIO1."]
    #[inline(always)]
    pub fn aiodio1(&mut self) -> AIODIO1_W {
        AIODIO1_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Enables (1) or disables (0) clock for AUX_AIODIO0."]
    #[inline(always)]
    pub fn aiodio0(&mut self) -> AIODIO0_W {
        AIODIO0_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Enables (1) or disables (0) clock for AUX_SMPH."]
    #[inline(always)]
    pub fn smph(&mut self) -> SMPH_W {
        SMPH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Clock Enable Clock enable for each module in the AUX domain For use by the system CPU The settings in this register are OR'ed with the corresponding settings in MODCLKEN1. This allows the system CPU and AUX_SCE to request clocks independently. Settings take effect immediately.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modclken0](index.html) module"]
pub struct MODCLKEN0_SPEC;
impl crate::RegisterSpec for MODCLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modclken0::R](R) reader structure"]
impl crate::Readable for MODCLKEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modclken0::W](W) writer structure"]
impl crate::Writable for MODCLKEN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODCLKEN0 to value 0"]
impl crate::Resettable for MODCLKEN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
