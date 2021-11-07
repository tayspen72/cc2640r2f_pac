#[doc = "Register `VIMSCLKG` reader"]
pub struct R(crate::R<VIMSCLKG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VIMSCLKG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VIMSCLKG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VIMSCLKG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VIMSCLKG` writer"]
pub struct W(crate::W<VIMSCLKG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VIMSCLKG_SPEC>;
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
impl From<crate::W<VIMSCLKG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VIMSCLKG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_R(crate::FieldReader<u32, u32>);
impl RESERVED2_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "Field `CLK_EN` reader - 1:0\\]
00: Disable clock 01: Disable clock when system CPU is in DeepSleep 11: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub struct CLK_EN_R(crate::FieldReader<u8, u8>);
impl CLK_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EN` writer - 1:0\\]
00: Disable clock 01: Disable clock when system CPU is in DeepSleep 11: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bits 0:1 - 1:0\\]
00: Disable clock 01: Disable clock when system CPU is in DeepSleep 11: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
00: Disable clock 01: Disable clock when system CPU is in DeepSleep 11: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
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
#[doc = "VIMS Clock Gate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vimsclkg](index.html) module"]
pub struct VIMSCLKG_SPEC;
impl crate::RegisterSpec for VIMSCLKG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vimsclkg::R](R) reader structure"]
impl crate::Readable for VIMSCLKG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vimsclkg::W](W) writer structure"]
impl crate::Writable for VIMSCLKG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VIMSCLKG to value 0x03"]
impl crate::Resettable for VIMSCLKG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
