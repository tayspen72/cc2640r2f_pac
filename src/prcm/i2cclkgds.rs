#[doc = "Register `I2CCLKGDS` reader"]
pub struct R(crate::R<I2CCLKGDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2CCLKGDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2CCLKGDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2CCLKGDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2CCLKGDS` writer"]
pub struct W(crate::W<I2CCLKGDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2CCLKGDS_SPEC>;
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
impl From<crate::W<I2CCLKGDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2CCLKGDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1_R(crate::FieldReader<u32, u32>);
impl RESERVED1_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | ((value as u32 & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Field `CLK_EN` reader - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub struct CLK_EN_R(crate::FieldReader<bool, bool>);
impl CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EN` writer - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Clock Gate For Deep Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cclkgds](index.html) module"]
pub struct I2CCLKGDS_SPEC;
impl crate::RegisterSpec for I2CCLKGDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2cclkgds::R](R) reader structure"]
impl crate::Readable for I2CCLKGDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2cclkgds::W](W) writer structure"]
impl crate::Writable for I2CCLKGDS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2CCLKGDS to value 0"]
impl crate::Resettable for I2CCLKGDS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
