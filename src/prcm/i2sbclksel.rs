#[doc = "Register `I2SBCLKSEL` reader"]
pub struct R(crate::R<I2SBCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SBCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SBCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SBCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SBCLKSEL` writer"]
pub struct W(crate::W<I2SBCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SBCLKSEL_SPEC>;
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
impl From<crate::W<I2SBCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SBCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPARE` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE_R(crate::FieldReader<u32, u32>);
impl SPARE_R {
    pub(crate) fn new(bits: u32) -> Self {
        SPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | ((value as u32 & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Field `SRC` reader - 0:0\\]
BCLK source selector 0: Use external BCLK 1: Use internally generated clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub struct SRC_R(crate::FieldReader<bool, bool>);
impl SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC` writer - 0:0\\]
BCLK source selector 0: Use external BCLK 1: Use internally generated clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
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
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\]
BCLK source selector 0: Use external BCLK 1: Use internally generated clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
BCLK source selector 0: Use external BCLK 1: Use internally generated clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sbclksel](index.html) module"]
pub struct I2SBCLKSEL_SPEC;
impl crate::RegisterSpec for I2SBCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sbclksel::R](R) reader structure"]
impl crate::Readable for I2SBCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2sbclksel::W](W) writer structure"]
impl crate::Writable for I2SBCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2SBCLKSEL to value 0"]
impl crate::Resettable for I2SBCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
