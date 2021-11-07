#[doc = "Register `ISRC` reader"]
pub struct R(crate::R<ISRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISRC` writer"]
pub struct W(crate::W<ISRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISRC_SPEC>;
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
impl From<crate::W<ISRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "7:2\\]
Adjust current from current source. Output currents may be combined to get desired total current.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIM_A {
    #[doc = "32: 11.75 uA"]
    _11P75U = 32,
    #[doc = "16: 4.5 uA"]
    _4P5U = 16,
    #[doc = "8: 2.0 uA"]
    _2P0U = 8,
    #[doc = "4: 1.0 uA"]
    _1P0U = 4,
    #[doc = "2: 0.5 uA"]
    _0P5U = 2,
    #[doc = "1: 0.25 uA"]
    _0P25U = 1,
    #[doc = "0: No current connected"]
    NC = 0,
}
impl From<TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRIM` reader - 7:2\\]
Adjust current from current source. Output currents may be combined to get desired total current."]
pub struct TRIM_R(crate::FieldReader<u8, TRIM_A>);
impl TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIM_A> {
        match self.bits {
            32 => Some(TRIM_A::_11P75U),
            16 => Some(TRIM_A::_4P5U),
            8 => Some(TRIM_A::_2P0U),
            4 => Some(TRIM_A::_1P0U),
            2 => Some(TRIM_A::_0P5U),
            1 => Some(TRIM_A::_0P25U),
            0 => Some(TRIM_A::NC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_11P75U`"]
    #[inline(always)]
    pub fn is_11p75u(&self) -> bool {
        **self == TRIM_A::_11P75U
    }
    #[doc = "Checks if the value of the field is `_4P5U`"]
    #[inline(always)]
    pub fn is_4p5u(&self) -> bool {
        **self == TRIM_A::_4P5U
    }
    #[doc = "Checks if the value of the field is `_2P0U`"]
    #[inline(always)]
    pub fn is_2p0u(&self) -> bool {
        **self == TRIM_A::_2P0U
    }
    #[doc = "Checks if the value of the field is `_1P0U`"]
    #[inline(always)]
    pub fn is_1p0u(&self) -> bool {
        **self == TRIM_A::_1P0U
    }
    #[doc = "Checks if the value of the field is `_0P5U`"]
    #[inline(always)]
    pub fn is_0p5u(&self) -> bool {
        **self == TRIM_A::_0P5U
    }
    #[doc = "Checks if the value of the field is `_0P25U`"]
    #[inline(always)]
    pub fn is_0p25u(&self) -> bool {
        **self == TRIM_A::_0P25U
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        **self == TRIM_A::NC
    }
}
impl core::ops::Deref for TRIM_R {
    type Target = crate::FieldReader<u8, TRIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM` writer - 7:2\\]
Adjust current from current source. Output currents may be combined to get desired total current."]
pub struct TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "11.75 uA"]
    #[inline(always)]
    pub fn _11p75u(self) -> &'a mut W {
        self.variant(TRIM_A::_11P75U)
    }
    #[doc = "4.5 uA"]
    #[inline(always)]
    pub fn _4p5u(self) -> &'a mut W {
        self.variant(TRIM_A::_4P5U)
    }
    #[doc = "2.0 uA"]
    #[inline(always)]
    pub fn _2p0u(self) -> &'a mut W {
        self.variant(TRIM_A::_2P0U)
    }
    #[doc = "1.0 uA"]
    #[inline(always)]
    pub fn _1p0u(self) -> &'a mut W {
        self.variant(TRIM_A::_1P0U)
    }
    #[doc = "0.5 uA"]
    #[inline(always)]
    pub fn _0p5u(self) -> &'a mut W {
        self.variant(TRIM_A::_0P5U)
    }
    #[doc = "0.25 uA"]
    #[inline(always)]
    pub fn _0p25u(self) -> &'a mut W {
        self.variant(TRIM_A::_0P25U)
    }
    #[doc = "No current connected"]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(TRIM_A::NC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | ((value as u8 & 0x3f) << 2);
        self.w
    }
}
#[doc = "Field `RESERVED1` reader - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1_R(crate::FieldReader<bool, bool>);
impl RESERVED1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED1` writer - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `EN` reader - 0:0\\]
Current source enable"]
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
Current source enable"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:7 - 7:2\\]
Adjust current from current source. Output currents may be combined to get desired total current."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Current source enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:7 - 7:2\\]
Adjust current from current source. Output currents may be combined to get desired total current."]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W {
        TRIM_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Current source enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current Source Strength and trim control for current source. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isrc](index.html) module"]
pub struct ISRC_SPEC;
impl crate::RegisterSpec for ISRC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [isrc::R](R) reader structure"]
impl crate::Readable for ISRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isrc::W](W) writer structure"]
impl crate::Writable for ISRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISRC to value 0"]
impl crate::Resettable for ISRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
