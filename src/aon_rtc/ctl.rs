#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED19` reader - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED19_R(crate::FieldReader<u16, u16>);
impl RESERVED19_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED19_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED19` writer - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED19_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 19)) | ((value as u32 & 0x1fff) << 19);
        self.w
    }
}
#[doc = "18:16\\]
Eventmask selecting which delayed events that form the combined event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMB_EV_MASK_A {
    #[doc = "4: Use Channel 2 delayed event in combined event"]
    CH2 = 4,
    #[doc = "2: Use Channel 1 delayed event in combined event"]
    CH1 = 2,
    #[doc = "1: Use Channel 0 delayed event in combined event"]
    CH0 = 1,
    #[doc = "0: No event is selected for combined event."]
    NONE = 0,
}
impl From<COMB_EV_MASK_A> for u8 {
    #[inline(always)]
    fn from(variant: COMB_EV_MASK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMB_EV_MASK` reader - 18:16\\]
Eventmask selecting which delayed events that form the combined event."]
pub struct COMB_EV_MASK_R(crate::FieldReader<u8, COMB_EV_MASK_A>);
impl COMB_EV_MASK_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMB_EV_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMB_EV_MASK_A> {
        match self.bits {
            4 => Some(COMB_EV_MASK_A::CH2),
            2 => Some(COMB_EV_MASK_A::CH1),
            1 => Some(COMB_EV_MASK_A::CH0),
            0 => Some(COMB_EV_MASK_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        **self == COMB_EV_MASK_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        **self == COMB_EV_MASK_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        **self == COMB_EV_MASK_A::CH0
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == COMB_EV_MASK_A::NONE
    }
}
impl core::ops::Deref for COMB_EV_MASK_R {
    type Target = crate::FieldReader<u8, COMB_EV_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMB_EV_MASK` writer - 18:16\\]
Eventmask selecting which delayed events that form the combined event."]
pub struct COMB_EV_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMB_EV_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMB_EV_MASK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Use Channel 2 delayed event in combined event"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(COMB_EV_MASK_A::CH2)
    }
    #[doc = "Use Channel 1 delayed event in combined event"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(COMB_EV_MASK_A::CH1)
    }
    #[doc = "Use Channel 0 delayed event in combined event"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(COMB_EV_MASK_A::CH0)
    }
    #[doc = "No event is selected for combined event."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(COMB_EV_MASK_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED12` reader - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED12_R(crate::FieldReader<u8, u8>);
impl RESERVED12_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED12` writer - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "11:8\\]
Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_DELAY_A {
    #[doc = "13: Delay by 144 clock cycles"]
    D144 = 13,
    #[doc = "12: Delay by 128 clock cycles"]
    D128 = 12,
    #[doc = "11: Delay by 112 clock cycles"]
    D112 = 11,
    #[doc = "10: Delay by 96 clock cycles"]
    D96 = 10,
    #[doc = "9: Delay by 80 clock cycles"]
    D80 = 9,
    #[doc = "8: Delay by 64 clock cycles"]
    D64 = 8,
    #[doc = "7: Delay by 48 clock cycles"]
    D48 = 7,
    #[doc = "6: Delay by 32 clock cycles"]
    D32 = 6,
    #[doc = "5: Delay by 16 clock cycles"]
    D16 = 5,
    #[doc = "4: Delay by 8 clock cycles"]
    D8 = 4,
    #[doc = "3: Delay by 4 clock cycles"]
    D4 = 3,
    #[doc = "2: Delay by 2 clock cycles"]
    D2 = 2,
    #[doc = "1: Delay by 1 clock cycles"]
    D1 = 1,
    #[doc = "0: No delay on delayed event"]
    D0 = 0,
}
impl From<EV_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: EV_DELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EV_DELAY` reader - 11:8\\]
Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed"]
pub struct EV_DELAY_R(crate::FieldReader<u8, EV_DELAY_A>);
impl EV_DELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        EV_DELAY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EV_DELAY_A> {
        match self.bits {
            13 => Some(EV_DELAY_A::D144),
            12 => Some(EV_DELAY_A::D128),
            11 => Some(EV_DELAY_A::D112),
            10 => Some(EV_DELAY_A::D96),
            9 => Some(EV_DELAY_A::D80),
            8 => Some(EV_DELAY_A::D64),
            7 => Some(EV_DELAY_A::D48),
            6 => Some(EV_DELAY_A::D32),
            5 => Some(EV_DELAY_A::D16),
            4 => Some(EV_DELAY_A::D8),
            3 => Some(EV_DELAY_A::D4),
            2 => Some(EV_DELAY_A::D2),
            1 => Some(EV_DELAY_A::D1),
            0 => Some(EV_DELAY_A::D0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `D144`"]
    #[inline(always)]
    pub fn is_d144(&self) -> bool {
        **self == EV_DELAY_A::D144
    }
    #[doc = "Checks if the value of the field is `D128`"]
    #[inline(always)]
    pub fn is_d128(&self) -> bool {
        **self == EV_DELAY_A::D128
    }
    #[doc = "Checks if the value of the field is `D112`"]
    #[inline(always)]
    pub fn is_d112(&self) -> bool {
        **self == EV_DELAY_A::D112
    }
    #[doc = "Checks if the value of the field is `D96`"]
    #[inline(always)]
    pub fn is_d96(&self) -> bool {
        **self == EV_DELAY_A::D96
    }
    #[doc = "Checks if the value of the field is `D80`"]
    #[inline(always)]
    pub fn is_d80(&self) -> bool {
        **self == EV_DELAY_A::D80
    }
    #[doc = "Checks if the value of the field is `D64`"]
    #[inline(always)]
    pub fn is_d64(&self) -> bool {
        **self == EV_DELAY_A::D64
    }
    #[doc = "Checks if the value of the field is `D48`"]
    #[inline(always)]
    pub fn is_d48(&self) -> bool {
        **self == EV_DELAY_A::D48
    }
    #[doc = "Checks if the value of the field is `D32`"]
    #[inline(always)]
    pub fn is_d32(&self) -> bool {
        **self == EV_DELAY_A::D32
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        **self == EV_DELAY_A::D16
    }
    #[doc = "Checks if the value of the field is `D8`"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        **self == EV_DELAY_A::D8
    }
    #[doc = "Checks if the value of the field is `D4`"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        **self == EV_DELAY_A::D4
    }
    #[doc = "Checks if the value of the field is `D2`"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        **self == EV_DELAY_A::D2
    }
    #[doc = "Checks if the value of the field is `D1`"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        **self == EV_DELAY_A::D1
    }
    #[doc = "Checks if the value of the field is `D0`"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        **self == EV_DELAY_A::D0
    }
}
impl core::ops::Deref for EV_DELAY_R {
    type Target = crate::FieldReader<u8, EV_DELAY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EV_DELAY` writer - 11:8\\]
Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed"]
pub struct EV_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> EV_DELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV_DELAY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Delay by 144 clock cycles"]
    #[inline(always)]
    pub fn d144(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D144)
    }
    #[doc = "Delay by 128 clock cycles"]
    #[inline(always)]
    pub fn d128(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D128)
    }
    #[doc = "Delay by 112 clock cycles"]
    #[inline(always)]
    pub fn d112(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D112)
    }
    #[doc = "Delay by 96 clock cycles"]
    #[inline(always)]
    pub fn d96(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D96)
    }
    #[doc = "Delay by 80 clock cycles"]
    #[inline(always)]
    pub fn d80(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D80)
    }
    #[doc = "Delay by 64 clock cycles"]
    #[inline(always)]
    pub fn d64(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D64)
    }
    #[doc = "Delay by 48 clock cycles"]
    #[inline(always)]
    pub fn d48(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D48)
    }
    #[doc = "Delay by 32 clock cycles"]
    #[inline(always)]
    pub fn d32(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D32)
    }
    #[doc = "Delay by 16 clock cycles"]
    #[inline(always)]
    pub fn d16(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D16)
    }
    #[doc = "Delay by 8 clock cycles"]
    #[inline(always)]
    pub fn d8(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D8)
    }
    #[doc = "Delay by 4 clock cycles"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D4)
    }
    #[doc = "Delay by 2 clock cycles"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D2)
    }
    #[doc = "Delay by 1 clock cycles"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D1)
    }
    #[doc = "No delay on delayed event"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut W {
        self.variant(EV_DELAY_A::D0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RESET` reader - 7:7\\]
RTC Counter reset. Writing 1 to this bit will reset the RTC counter. This bit is cleared when reset takes effect"]
pub struct RESET_R(crate::FieldReader<bool, bool>);
impl RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET` writer - 7:7\\]
RTC Counter reset. Writing 1 to this bit will reset the RTC counter. This bit is cleared when reset takes effect"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
#[doc = "Field `RESERVED3` reader - 6:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_R(crate::FieldReader<u8, u8>);
impl RESERVED3_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 6:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | ((value as u32 & 0x0f) << 3);
        self.w
    }
}
#[doc = "Field `RTC_4KHZ_EN` reader - 2:2\\]
RTC_4KHZ is a 4 KHz reference output, tapped from SUBSEC.VALUE bit 19 which is used by AUX timer. 0: RTC_4KHZ signal is forced to 0 1: RTC_4KHZ is enabled ( provied that RTC is enabled EN)"]
pub struct RTC_4KHZ_EN_R(crate::FieldReader<bool, bool>);
impl RTC_4KHZ_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_4KHZ_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_4KHZ_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_4KHZ_EN` writer - 2:2\\]
RTC_4KHZ is a 4 KHz reference output, tapped from SUBSEC.VALUE bit 19 which is used by AUX timer. 0: RTC_4KHZ signal is forced to 0 1: RTC_4KHZ is enabled ( provied that RTC is enabled EN)"]
pub struct RTC_4KHZ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_4KHZ_EN_W<'a> {
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
#[doc = "Field `RTC_UPD_EN` reader - 1:1\\]
RTC_UPD is a 16 KHz signal used to sync up the radio timer. The 16 Khz is SCLK_LF divided by 2 0: RTC_UPD signal is forced to 0 1: RTC_UPD signal is toggling @16 kHz"]
pub struct RTC_UPD_EN_R(crate::FieldReader<bool, bool>);
impl RTC_UPD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_UPD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_UPD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_UPD_EN` writer - 1:1\\]
RTC_UPD is a 16 KHz signal used to sync up the radio timer. The 16 Khz is SCLK_LF divided by 2 0: RTC_UPD signal is forced to 0 1: RTC_UPD signal is toggling @16 kHz"]
pub struct RTC_UPD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_UPD_EN_W<'a> {
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
#[doc = "Field `EN` reader - 0:0\\]
Enable RTC counter 0: Halted (frozen) 1: Running"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - 0:0\\]
Enable RTC counter 0: Halted (frozen) 1: Running"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> RESERVED19_R {
        RESERVED19_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Eventmask selecting which delayed events that form the combined event."]
    #[inline(always)]
    pub fn comb_ev_mask(&self) -> COMB_EV_MASK_R {
        COMB_EV_MASK_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed"]
    #[inline(always)]
    pub fn ev_delay(&self) -> EV_DELAY_R {
        EV_DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
RTC Counter reset. Writing 1 to this bit will reset the RTC counter. This bit is cleared when reset takes effect"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - 6:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
RTC_4KHZ is a 4 KHz reference output, tapped from SUBSEC.VALUE bit 19 which is used by AUX timer. 0: RTC_4KHZ signal is forced to 0 1: RTC_4KHZ is enabled ( provied that RTC is enabled EN)"]
    #[inline(always)]
    pub fn rtc_4khz_en(&self) -> RTC_4KHZ_EN_R {
        RTC_4KHZ_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RTC_UPD is a 16 KHz signal used to sync up the radio timer. The 16 Khz is SCLK_LF divided by 2 0: RTC_UPD signal is forced to 0 1: RTC_UPD signal is toggling @16 kHz"]
    #[inline(always)]
    pub fn rtc_upd_en(&self) -> RTC_UPD_EN_R {
        RTC_UPD_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Enable RTC counter 0: Halted (frozen) 1: Running"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&mut self) -> RESERVED19_W {
        RESERVED19_W { w: self }
    }
    #[doc = "Bits 16:18 - 18:16\\]
Eventmask selecting which delayed events that form the combined event."]
    #[inline(always)]
    pub fn comb_ev_mask(&mut self) -> COMB_EV_MASK_W {
        COMB_EV_MASK_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed"]
    #[inline(always)]
    pub fn ev_delay(&mut self) -> EV_DELAY_W {
        EV_DELAY_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
RTC Counter reset. Writing 1 to this bit will reset the RTC counter. This bit is cleared when reset takes effect"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bits 3:6 - 6:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
RTC_4KHZ is a 4 KHz reference output, tapped from SUBSEC.VALUE bit 19 which is used by AUX timer. 0: RTC_4KHZ signal is forced to 0 1: RTC_4KHZ is enabled ( provied that RTC is enabled EN)"]
    #[inline(always)]
    pub fn rtc_4khz_en(&mut self) -> RTC_4KHZ_EN_W {
        RTC_4KHZ_EN_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
RTC_UPD is a 16 KHz signal used to sync up the radio timer. The 16 Khz is SCLK_LF divided by 2 0: RTC_UPD signal is forced to 0 1: RTC_UPD signal is toggling @16 kHz"]
    #[inline(always)]
    pub fn rtc_upd_en(&mut self) -> RTC_UPD_EN_W {
        RTC_UPD_EN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Enable RTC counter 0: Halted (frozen) 1: Running"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control This register contains various bitfields for configuration of RTC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
