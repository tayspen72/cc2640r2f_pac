#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
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
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "31:31\\]
Set based on the accurate high frequency XTAL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_IS_24M_A {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    _24M = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    _48M = 0,
}
impl From<XTAL_IS_24M_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_IS_24M_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTAL_IS_24M` reader - 31:31\\]
Set based on the accurate high frequency XTAL."]
pub struct XTAL_IS_24M_R(crate::FieldReader<bool, XTAL_IS_24M_A>);
impl XTAL_IS_24M_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL_IS_24M_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTAL_IS_24M_A {
        match self.bits {
            true => XTAL_IS_24M_A::_24M,
            false => XTAL_IS_24M_A::_48M,
        }
    }
    #[doc = "Checks if the value of the field is `_24M`"]
    #[inline(always)]
    pub fn is_24m(&self) -> bool {
        **self == XTAL_IS_24M_A::_24M
    }
    #[doc = "Checks if the value of the field is `_48M`"]
    #[inline(always)]
    pub fn is_48m(&self) -> bool {
        **self == XTAL_IS_24M_A::_48M
    }
}
impl core::ops::Deref for XTAL_IS_24M_R {
    type Target = crate::FieldReader<bool, XTAL_IS_24M_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL_IS_24M` writer - 31:31\\]
Set based on the accurate high frequency XTAL."]
pub struct XTAL_IS_24M_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_IS_24M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL_IS_24M_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _24m(self) -> &'a mut W {
        self.variant(XTAL_IS_24M_A::_24M)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _48m(self) -> &'a mut W {
        self.variant(XTAL_IS_24M_A::_48M)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `RESERVED30` reader - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED30_R(crate::FieldReader<bool, bool>);
impl RESERVED30_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED30` writer - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED30_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `BYPASS_XOSC_LF_CLK_QUAL` reader - 29:29\\]
Internal. Only to be used through TI provided API."]
pub struct BYPASS_XOSC_LF_CLK_QUAL_R(crate::FieldReader<bool, bool>);
impl BYPASS_XOSC_LF_CLK_QUAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_XOSC_LF_CLK_QUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPASS_XOSC_LF_CLK_QUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS_XOSC_LF_CLK_QUAL` writer - 29:29\\]
Internal. Only to be used through TI provided API."]
pub struct BYPASS_XOSC_LF_CLK_QUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_XOSC_LF_CLK_QUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `BYPASS_RCOSC_LF_CLK_QUAL` reader - 28:28\\]
Internal. Only to be used through TI provided API."]
pub struct BYPASS_RCOSC_LF_CLK_QUAL_R(crate::FieldReader<bool, bool>);
impl BYPASS_RCOSC_LF_CLK_QUAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_RCOSC_LF_CLK_QUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPASS_RCOSC_LF_CLK_QUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS_RCOSC_LF_CLK_QUAL` writer - 28:28\\]
Internal. Only to be used through TI provided API."]
pub struct BYPASS_RCOSC_LF_CLK_QUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_RCOSC_LF_CLK_QUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `DOUBLER_START_DURATION` reader - 27:26\\]
Internal. Only to be used through TI provided API."]
pub struct DOUBLER_START_DURATION_R(crate::FieldReader<u8, u8>);
impl DOUBLER_START_DURATION_R {
    pub(crate) fn new(bits: u8) -> Self {
        DOUBLER_START_DURATION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUBLER_START_DURATION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUBLER_START_DURATION` writer - 27:26\\]
Internal. Only to be used through TI provided API."]
pub struct DOUBLER_START_DURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUBLER_START_DURATION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `DOUBLER_RESET_DURATION` reader - 25:25\\]
Internal. Only to be used through TI provided API."]
pub struct DOUBLER_RESET_DURATION_R(crate::FieldReader<bool, bool>);
impl DOUBLER_RESET_DURATION_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOUBLER_RESET_DURATION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUBLER_RESET_DURATION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUBLER_RESET_DURATION` writer - 25:25\\]
Internal. Only to be used through TI provided API."]
pub struct DOUBLER_RESET_DURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUBLER_RESET_DURATION_W<'a> {
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
#[doc = "Field `RESERVED23` reader - 24:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED23_R(crate::FieldReader<u8, u8>);
impl RESERVED23_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED23_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED23` writer - 24:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED23_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | ((value as u32 & 0x03) << 23);
        self.w
    }
}
#[doc = "Field `FORCE_KICKSTART_EN` reader - 22:22\\]
Internal. Only to be used through TI provided API."]
pub struct FORCE_KICKSTART_EN_R(crate::FieldReader<bool, bool>);
impl FORCE_KICKSTART_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_KICKSTART_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_KICKSTART_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_KICKSTART_EN` writer - 22:22\\]
Internal. Only to be used through TI provided API."]
pub struct FORCE_KICKSTART_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_KICKSTART_EN_W<'a> {
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
#[doc = "Field `RESERVED17` reader - 21:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED17_R(crate::FieldReader<u8, u8>);
impl RESERVED17_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED17_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED17` writer - 21:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED17_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | ((value as u32 & 0x1f) << 17);
        self.w
    }
}
#[doc = "Field `ALLOW_SCLK_HF_SWITCHING` reader - 16:16\\]
0: Default - Switching of HF clock source is disabled . 1: Allows switching of sclk_hf source. Provided to prevent switching of the SCLK_HF source when running from flash (a long period during switching could corrupt flash). When sclk_hf switching is disabled, a new source can be started when SCLK_HF_SRC_SEL is changed, but the switch will not occur until this bit is set. This bit should be set to enable clock switching after STAT0.PENDINGSCLKHFSWITCHING indicates the new HF clock is ready. When switching completes (also indicated by STAT0.PENDINGSCLKHFSWITCHING) sclk_hf switching should be disabled to prevent flash corruption. Switching should not be enabled when running from flash."]
pub struct ALLOW_SCLK_HF_SWITCHING_R(crate::FieldReader<bool, bool>);
impl ALLOW_SCLK_HF_SWITCHING_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALLOW_SCLK_HF_SWITCHING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALLOW_SCLK_HF_SWITCHING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALLOW_SCLK_HF_SWITCHING` writer - 16:16\\]
0: Default - Switching of HF clock source is disabled . 1: Allows switching of sclk_hf source. Provided to prevent switching of the SCLK_HF source when running from flash (a long period during switching could corrupt flash). When sclk_hf switching is disabled, a new source can be started when SCLK_HF_SRC_SEL is changed, but the switch will not occur until this bit is set. This bit should be set to enable clock switching after STAT0.PENDINGSCLKHFSWITCHING indicates the new HF clock is ready. When switching completes (also indicated by STAT0.PENDINGSCLKHFSWITCHING) sclk_hf switching should be disabled to prevent flash corruption. Switching should not be enabled when running from flash."]
pub struct ALLOW_SCLK_HF_SWITCHING_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLOW_SCLK_HF_SWITCHING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED15` reader - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
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
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
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
#[doc = "Field `HPOSC_MODE_EN` reader - 14:14\\]
Internal. Only to be used through TI provided API."]
pub struct HPOSC_MODE_EN_R(crate::FieldReader<bool, bool>);
impl HPOSC_MODE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPOSC_MODE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPOSC_MODE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPOSC_MODE_EN` writer - 14:14\\]
Internal. Only to be used through TI provided API."]
pub struct HPOSC_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOSC_MODE_EN_W<'a> {
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
#[doc = "Field `RESERVED13` reader - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED13_R(crate::FieldReader<bool, bool>);
impl RESERVED13_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED13` writer - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED13_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED13_W<'a> {
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
#[doc = "Field `RCOSC_LF_TRIMMED` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSC_LF_TRIMMED_R(crate::FieldReader<bool, bool>);
impl RCOSC_LF_TRIMMED_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCOSC_LF_TRIMMED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCOSC_LF_TRIMMED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCOSC_LF_TRIMMED` writer - 12:12\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSC_LF_TRIMMED_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_LF_TRIMMED_W<'a> {
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
#[doc = "Field `XOSC_HF_POWER_MODE` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub struct XOSC_HF_POWER_MODE_R(crate::FieldReader<bool, bool>);
impl XOSC_HF_POWER_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        XOSC_HF_POWER_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC_HF_POWER_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC_HF_POWER_MODE` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub struct XOSC_HF_POWER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_POWER_MODE_W<'a> {
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
#[doc = "Field `XOSC_LF_DIG_BYPASS` reader - 10:10\\]
Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
pub struct XOSC_LF_DIG_BYPASS_R(crate::FieldReader<bool, bool>);
impl XOSC_LF_DIG_BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        XOSC_LF_DIG_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC_LF_DIG_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC_LF_DIG_BYPASS` writer - 10:10\\]
Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
pub struct XOSC_LF_DIG_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_LF_DIG_BYPASS_W<'a> {
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
#[doc = "Field `CLK_LOSS_EN` reader - 9:9\\]
Enable clock loss detection and hence the indicators to system controller. Checks both SCLK_HF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
pub struct CLK_LOSS_EN_R(crate::FieldReader<bool, bool>);
impl CLK_LOSS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLK_LOSS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_LOSS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_LOSS_EN` writer - 9:9\\]
Enable clock loss detection and hence the indicators to system controller. Checks both SCLK_HF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
pub struct CLK_LOSS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_LOSS_EN_W<'a> {
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
#[doc = "Field `ACLK_TDC_SRC_SEL` reader - 8:7\\]
Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used"]
pub struct ACLK_TDC_SRC_SEL_R(crate::FieldReader<u8, u8>);
impl ACLK_TDC_SRC_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACLK_TDC_SRC_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACLK_TDC_SRC_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACLK_TDC_SRC_SEL` writer - 8:7\\]
Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used"]
pub struct ACLK_TDC_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_TDC_SRC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `ACLK_REF_SRC_SEL` reader - 6:5\\]
Source select for aclk_ref 00: RCOSC_HF derived (31.25kHz) 01: XOSC_HF derived (31.25kHz) 10: RCOSC_LF (32kHz) 11: XOSC_LF (32.768kHz)"]
pub struct ACLK_REF_SRC_SEL_R(crate::FieldReader<u8, u8>);
impl ACLK_REF_SRC_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACLK_REF_SRC_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACLK_REF_SRC_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACLK_REF_SRC_SEL` writer - 6:5\\]
Source select for aclk_ref 00: RCOSC_HF derived (31.25kHz) 01: XOSC_HF derived (31.25kHz) 10: RCOSC_LF (32kHz) 11: XOSC_LF (32.768kHz)"]
pub struct ACLK_REF_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_REF_SRC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `SPARE4` reader - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE4_R(crate::FieldReader<bool, bool>);
impl SPARE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPARE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE4` writer - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE4_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE4_W<'a> {
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
#[doc = "3:2\\]
Source select for sclk_lf\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCLK_LF_SRC_SEL_A {
    #[doc = "3: Low frequency XOSC"]
    XOSCLF = 3,
    #[doc = "2: Low frequency RCOSC"]
    RCOSCLF = 2,
    #[doc = "1: Low frequency clock derived from High Frequency XOSC"]
    XOSCHFDLF = 1,
    #[doc = "0: Low frequency clock derived from High Frequency RCOSC"]
    RCOSCHFDLF = 0,
}
impl From<SCLK_LF_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLK_LF_SRC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SCLK_LF_SRC_SEL` reader - 3:2\\]
Source select for sclk_lf"]
pub struct SCLK_LF_SRC_SEL_R(crate::FieldReader<u8, SCLK_LF_SRC_SEL_A>);
impl SCLK_LF_SRC_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCLK_LF_SRC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_LF_SRC_SEL_A {
        match self.bits {
            3 => SCLK_LF_SRC_SEL_A::XOSCLF,
            2 => SCLK_LF_SRC_SEL_A::RCOSCLF,
            1 => SCLK_LF_SRC_SEL_A::XOSCHFDLF,
            0 => SCLK_LF_SRC_SEL_A::RCOSCHFDLF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XOSCLF`"]
    #[inline(always)]
    pub fn is_xosclf(&self) -> bool {
        **self == SCLK_LF_SRC_SEL_A::XOSCLF
    }
    #[doc = "Checks if the value of the field is `RCOSCLF`"]
    #[inline(always)]
    pub fn is_rcosclf(&self) -> bool {
        **self == SCLK_LF_SRC_SEL_A::RCOSCLF
    }
    #[doc = "Checks if the value of the field is `XOSCHFDLF`"]
    #[inline(always)]
    pub fn is_xoschfdlf(&self) -> bool {
        **self == SCLK_LF_SRC_SEL_A::XOSCHFDLF
    }
    #[doc = "Checks if the value of the field is `RCOSCHFDLF`"]
    #[inline(always)]
    pub fn is_rcoschfdlf(&self) -> bool {
        **self == SCLK_LF_SRC_SEL_A::RCOSCHFDLF
    }
}
impl core::ops::Deref for SCLK_LF_SRC_SEL_R {
    type Target = crate::FieldReader<u8, SCLK_LF_SRC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_LF_SRC_SEL` writer - 3:2\\]
Source select for sclk_lf"]
pub struct SCLK_LF_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_LF_SRC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLK_LF_SRC_SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low frequency XOSC"]
    #[inline(always)]
    pub fn xosclf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_SEL_A::XOSCLF)
    }
    #[doc = "Low frequency RCOSC"]
    #[inline(always)]
    pub fn rcosclf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_SEL_A::RCOSCLF)
    }
    #[doc = "Low frequency clock derived from High Frequency XOSC"]
    #[inline(always)]
    pub fn xoschfdlf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_SEL_A::XOSCHFDLF)
    }
    #[doc = "Low frequency clock derived from High Frequency RCOSC"]
    #[inline(always)]
    pub fn rcoschfdlf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_SEL_A::RCOSCHFDLF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "1:1\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_MF_SRC_SEL_A {
    #[doc = "1: Medium frequency clock derived from high frequency XOSC."]
    XCOSCHFDMF = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    RCOSCHFDMF = 0,
}
impl From<SCLK_MF_SRC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SCLK_MF_SRC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLK_MF_SRC_SEL` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub struct SCLK_MF_SRC_SEL_R(crate::FieldReader<bool, SCLK_MF_SRC_SEL_A>);
impl SCLK_MF_SRC_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCLK_MF_SRC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_MF_SRC_SEL_A {
        match self.bits {
            true => SCLK_MF_SRC_SEL_A::XCOSCHFDMF,
            false => SCLK_MF_SRC_SEL_A::RCOSCHFDMF,
        }
    }
    #[doc = "Checks if the value of the field is `XCOSCHFDMF`"]
    #[inline(always)]
    pub fn is_xcoschfdmf(&self) -> bool {
        **self == SCLK_MF_SRC_SEL_A::XCOSCHFDMF
    }
    #[doc = "Checks if the value of the field is `RCOSCHFDMF`"]
    #[inline(always)]
    pub fn is_rcoschfdmf(&self) -> bool {
        **self == SCLK_MF_SRC_SEL_A::RCOSCHFDMF
    }
}
impl core::ops::Deref for SCLK_MF_SRC_SEL_R {
    type Target = crate::FieldReader<bool, SCLK_MF_SRC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_MF_SRC_SEL` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub struct SCLK_MF_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_MF_SRC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLK_MF_SRC_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Medium frequency clock derived from high frequency XOSC."]
    #[inline(always)]
    pub fn xcoschfdmf(self) -> &'a mut W {
        self.variant(SCLK_MF_SRC_SEL_A::XCOSCHFDMF)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfdmf(self) -> &'a mut W {
        self.variant(SCLK_MF_SRC_SEL_A::RCOSCHFDMF)
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
Source select for sclk_hf. XOSC option is supported for test and debug only and should be used when the XOSC_HF is running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_HF_SRC_SEL_A {
    #[doc = "1: High frequency XOSC clk"]
    XOSC = 1,
    #[doc = "0: High frequency RCOSC clock"]
    RCOSC = 0,
}
impl From<SCLK_HF_SRC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SCLK_HF_SRC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLK_HF_SRC_SEL` reader - 0:0\\]
Source select for sclk_hf. XOSC option is supported for test and debug only and should be used when the XOSC_HF is running."]
pub struct SCLK_HF_SRC_SEL_R(crate::FieldReader<bool, SCLK_HF_SRC_SEL_A>);
impl SCLK_HF_SRC_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCLK_HF_SRC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLK_HF_SRC_SEL_A {
        match self.bits {
            true => SCLK_HF_SRC_SEL_A::XOSC,
            false => SCLK_HF_SRC_SEL_A::RCOSC,
        }
    }
    #[doc = "Checks if the value of the field is `XOSC`"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        **self == SCLK_HF_SRC_SEL_A::XOSC
    }
    #[doc = "Checks if the value of the field is `RCOSC`"]
    #[inline(always)]
    pub fn is_rcosc(&self) -> bool {
        **self == SCLK_HF_SRC_SEL_A::RCOSC
    }
}
impl core::ops::Deref for SCLK_HF_SRC_SEL_R {
    type Target = crate::FieldReader<bool, SCLK_HF_SRC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_HF_SRC_SEL` writer - 0:0\\]
Source select for sclk_hf. XOSC option is supported for test and debug only and should be used when the XOSC_HF is running."]
pub struct SCLK_HF_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_HF_SRC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLK_HF_SRC_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "High frequency XOSC clk"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut W {
        self.variant(SCLK_HF_SRC_SEL_A::XOSC)
    }
    #[doc = "High frequency RCOSC clock"]
    #[inline(always)]
    pub fn rcosc(self) -> &'a mut W {
        self.variant(SCLK_HF_SRC_SEL_A::RCOSC)
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
    #[doc = "Bit 31 - 31:31\\]
Set based on the accurate high frequency XTAL."]
    #[inline(always)]
    pub fn xtal_is_24m(&self) -> XTAL_IS_24M_R {
        XTAL_IS_24M_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> RESERVED30_R {
        RESERVED30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass_xosc_lf_clk_qual(&self) -> BYPASS_XOSC_LF_CLK_QUAL_R {
        BYPASS_XOSC_LF_CLK_QUAL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass_rcosc_lf_clk_qual(&self) -> BYPASS_RCOSC_LF_CLK_QUAL_R {
        BYPASS_RCOSC_LF_CLK_QUAL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn doubler_start_duration(&self) -> DOUBLER_START_DURATION_R {
        DOUBLER_START_DURATION_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn doubler_reset_duration(&self) -> DOUBLER_RESET_DURATION_R {
        DOUBLER_RESET_DURATION_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 23:24 - 24:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&self) -> RESERVED23_R {
        RESERVED23_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn force_kickstart_en(&self) -> FORCE_KICKSTART_EN_R {
        FORCE_KICKSTART_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 17:21 - 21:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Default - Switching of HF clock source is disabled . 1: Allows switching of sclk_hf source. Provided to prevent switching of the SCLK_HF source when running from flash (a long period during switching could corrupt flash). When sclk_hf switching is disabled, a new source can be started when SCLK_HF_SRC_SEL is changed, but the switch will not occur until this bit is set. This bit should be set to enable clock switching after STAT0.PENDINGSCLKHFSWITCHING indicates the new HF clock is ready. When switching completes (also indicated by STAT0.PENDINGSCLKHFSWITCHING) sclk_hf switching should be disabled to prevent flash corruption. Switching should not be enabled when running from flash."]
    #[inline(always)]
    pub fn allow_sclk_hf_switching(&self) -> ALLOW_SCLK_HF_SWITCHING_R {
        ALLOW_SCLK_HF_SWITCHING_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_mode_en(&self) -> HPOSC_MODE_EN_R {
        HPOSC_MODE_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_lf_trimmed(&self) -> RCOSC_LF_TRIMMED_R {
        RCOSC_LF_TRIMMED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_power_mode(&self) -> XOSC_HF_POWER_MODE_R {
        XOSC_HF_POWER_MODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
    #[inline(always)]
    pub fn xosc_lf_dig_bypass(&self) -> XOSC_LF_DIG_BYPASS_R {
        XOSC_LF_DIG_BYPASS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable clock loss detection and hence the indicators to system controller. Checks both SCLK_HF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
    #[inline(always)]
    pub fn clk_loss_en(&self) -> CLK_LOSS_EN_R {
        CLK_LOSS_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used"]
    #[inline(always)]
    pub fn aclk_tdc_src_sel(&self) -> ACLK_TDC_SRC_SEL_R {
        ACLK_TDC_SRC_SEL_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Source select for aclk_ref 00: RCOSC_HF derived (31.25kHz) 01: XOSC_HF derived (31.25kHz) 10: RCOSC_LF (32kHz) 11: XOSC_LF (32.768kHz)"]
    #[inline(always)]
    pub fn aclk_ref_src_sel(&self) -> ACLK_REF_SRC_SEL_R {
        ACLK_REF_SRC_SEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare4(&self) -> SPARE4_R {
        SPARE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Source select for sclk_lf"]
    #[inline(always)]
    pub fn sclk_lf_src_sel(&self) -> SCLK_LF_SRC_SEL_R {
        SCLK_LF_SRC_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sclk_mf_src_sel(&self) -> SCLK_MF_SRC_SEL_R {
        SCLK_MF_SRC_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Source select for sclk_hf. XOSC option is supported for test and debug only and should be used when the XOSC_HF is running."]
    #[inline(always)]
    pub fn sclk_hf_src_sel(&self) -> SCLK_HF_SRC_SEL_R {
        SCLK_HF_SRC_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Set based on the accurate high frequency XTAL."]
    #[inline(always)]
    pub fn xtal_is_24m(&mut self) -> XTAL_IS_24M_W {
        XTAL_IS_24M_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&mut self) -> RESERVED30_W {
        RESERVED30_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass_xosc_lf_clk_qual(&mut self) -> BYPASS_XOSC_LF_CLK_QUAL_W {
        BYPASS_XOSC_LF_CLK_QUAL_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass_rcosc_lf_clk_qual(&mut self) -> BYPASS_RCOSC_LF_CLK_QUAL_W {
        BYPASS_RCOSC_LF_CLK_QUAL_W { w: self }
    }
    #[doc = "Bits 26:27 - 27:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn doubler_start_duration(&mut self) -> DOUBLER_START_DURATION_W {
        DOUBLER_START_DURATION_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn doubler_reset_duration(&mut self) -> DOUBLER_RESET_DURATION_W {
        DOUBLER_RESET_DURATION_W { w: self }
    }
    #[doc = "Bits 23:24 - 24:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&mut self) -> RESERVED23_W {
        RESERVED23_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn force_kickstart_en(&mut self) -> FORCE_KICKSTART_EN_W {
        FORCE_KICKSTART_EN_W { w: self }
    }
    #[doc = "Bits 17:21 - 21:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&mut self) -> RESERVED17_W {
        RESERVED17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
0: Default - Switching of HF clock source is disabled . 1: Allows switching of sclk_hf source. Provided to prevent switching of the SCLK_HF source when running from flash (a long period during switching could corrupt flash). When sclk_hf switching is disabled, a new source can be started when SCLK_HF_SRC_SEL is changed, but the switch will not occur until this bit is set. This bit should be set to enable clock switching after STAT0.PENDINGSCLKHFSWITCHING indicates the new HF clock is ready. When switching completes (also indicated by STAT0.PENDINGSCLKHFSWITCHING) sclk_hf switching should be disabled to prevent flash corruption. Switching should not be enabled when running from flash."]
    #[inline(always)]
    pub fn allow_sclk_hf_switching(&mut self) -> ALLOW_SCLK_HF_SWITCHING_W {
        ALLOW_SCLK_HF_SWITCHING_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&mut self) -> RESERVED15_W {
        RESERVED15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_mode_en(&mut self) -> HPOSC_MODE_EN_W {
        HPOSC_MODE_EN_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&mut self) -> RESERVED13_W {
        RESERVED13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_lf_trimmed(&mut self) -> RCOSC_LF_TRIMMED_W {
        RCOSC_LF_TRIMMED_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_power_mode(&mut self) -> XOSC_HF_POWER_MODE_W {
        XOSC_HF_POWER_MODE_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
    #[inline(always)]
    pub fn xosc_lf_dig_bypass(&mut self) -> XOSC_LF_DIG_BYPASS_W {
        XOSC_LF_DIG_BYPASS_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Enable clock loss detection and hence the indicators to system controller. Checks both SCLK_HF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
    #[inline(always)]
    pub fn clk_loss_en(&mut self) -> CLK_LOSS_EN_W {
        CLK_LOSS_EN_W { w: self }
    }
    #[doc = "Bits 7:8 - 8:7\\]
Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used"]
    #[inline(always)]
    pub fn aclk_tdc_src_sel(&mut self) -> ACLK_TDC_SRC_SEL_W {
        ACLK_TDC_SRC_SEL_W { w: self }
    }
    #[doc = "Bits 5:6 - 6:5\\]
Source select for aclk_ref 00: RCOSC_HF derived (31.25kHz) 01: XOSC_HF derived (31.25kHz) 10: RCOSC_LF (32kHz) 11: XOSC_LF (32.768kHz)"]
    #[inline(always)]
    pub fn aclk_ref_src_sel(&mut self) -> ACLK_REF_SRC_SEL_W {
        ACLK_REF_SRC_SEL_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare4(&mut self) -> SPARE4_W {
        SPARE4_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
Source select for sclk_lf"]
    #[inline(always)]
    pub fn sclk_lf_src_sel(&mut self) -> SCLK_LF_SRC_SEL_W {
        SCLK_LF_SRC_SEL_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sclk_mf_src_sel(&mut self) -> SCLK_MF_SRC_SEL_W {
        SCLK_MF_SRC_SEL_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Source select for sclk_hf. XOSC option is supported for test and debug only and should be used when the XOSC_HF is running."]
    #[inline(always)]
    pub fn sclk_hf_src_sel(&mut self) -> SCLK_HF_SRC_SEL_W {
        SCLK_HF_SRC_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 0 Controls clock source selects\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
