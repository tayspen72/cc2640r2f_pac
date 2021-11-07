#[doc = "Register `AIFWCLKSRC` reader"]
pub struct R(crate::R<AIFWCLKSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIFWCLKSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIFWCLKSRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIFWCLKSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIFWCLKSRC` writer"]
pub struct W(crate::W<AIFWCLKSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIFWCLKSRC_SPEC>;
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
impl From<crate::W<AIFWCLKSRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIFWCLKSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_R(crate::FieldReader<u32, u32>);
impl RESERVED3_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Field `WCLK_INV` reader - 2:2\\]
Inverts WCLK source (pad or internal) when set. 0: Not inverted 1: Inverted"]
pub struct WCLK_INV_R(crate::FieldReader<bool, bool>);
impl WCLK_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        WCLK_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WCLK_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WCLK_INV` writer - 2:2\\]
Inverts WCLK source (pad or internal) when set. 0: Not inverted 1: Inverted"]
pub struct WCLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> WCLK_INV_W<'a> {
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
#[doc = "1:0\\]
Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WCLK_SRC_A {
    #[doc = "2: Internal WCLK generator, from module PRCM"]
    INT = 2,
    #[doc = "1: External WCLK generator, from pad"]
    EXT = 1,
    #[doc = "0: None ('0')"]
    NONE = 0,
}
impl From<WCLK_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: WCLK_SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WCLK_SRC` reader - 1:0\\]
Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC"]
pub struct WCLK_SRC_R(crate::FieldReader<u8, WCLK_SRC_A>);
impl WCLK_SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        WCLK_SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WCLK_SRC_A {
        match self.bits {
            2 => WCLK_SRC_A::INT,
            1 => WCLK_SRC_A::EXT,
            0 => WCLK_SRC_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        **self == WCLK_SRC_A::INT
    }
    #[doc = "Checks if the value of the field is `EXT`"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        **self == WCLK_SRC_A::EXT
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == WCLK_SRC_A::NONE
    }
}
impl core::ops::Deref for WCLK_SRC_R {
    type Target = crate::FieldReader<u8, WCLK_SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WCLK_SRC` writer - 1:0\\]
Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC"]
pub struct WCLK_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> WCLK_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WCLK_SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal WCLK generator, from module PRCM"]
    #[inline(always)]
    pub fn int(self) -> &'a mut W {
        self.variant(WCLK_SRC_A::INT)
    }
    #[doc = "External WCLK generator, from pad"]
    #[inline(always)]
    pub fn ext(self) -> &'a mut W {
        self.variant(WCLK_SRC_A::EXT)
    }
    #[doc = "None ('0')"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WCLK_SRC_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
Inverts WCLK source (pad or internal) when set. 0: Not inverted 1: Inverted"]
    #[inline(always)]
    pub fn wclk_inv(&self) -> WCLK_INV_R {
        WCLK_INV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC"]
    #[inline(always)]
    pub fn wclk_src(&self) -> WCLK_SRC_R {
        WCLK_SRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Inverts WCLK source (pad or internal) when set. 0: Not inverted 1: Inverted"]
    #[inline(always)]
    pub fn wclk_inv(&mut self) -> WCLK_INV_W {
        WCLK_INV_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC"]
    #[inline(always)]
    pub fn wclk_src(&mut self) -> WCLK_SRC_W {
        WCLK_SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WCLK Source Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifwclksrc](index.html) module"]
pub struct AIFWCLKSRC_SPEC;
impl crate::RegisterSpec for AIFWCLKSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aifwclksrc::R](R) reader structure"]
impl crate::Readable for AIFWCLKSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aifwclksrc::W](W) writer structure"]
impl crate::Writable for AIFWCLKSRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AIFWCLKSRC to value 0"]
impl crate::Resettable for AIFWCLKSRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
