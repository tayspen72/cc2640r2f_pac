#[doc = "Register `SSICLKGR` reader"]
pub struct R(crate::R<SSICLKGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSICLKGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSICLKGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSICLKGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSICLKGR` writer"]
pub struct W(crate::W<SSICLKGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSICLKGR_SPEC>;
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
impl From<crate::W<SSICLKGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSICLKGR_SPEC>) -> Self {
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
#[doc = "1:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_EN_A {
    #[doc = "2: Enable clock for SSI1"]
    SSI1 = 2,
    #[doc = "1: Enable clock for SSI0"]
    SSI0 = 1,
}
impl From<CLK_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLK_EN` reader - 1:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub struct CLK_EN_R(crate::FieldReader<u8, CLK_EN_A>);
impl CLK_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_EN_A> {
        match self.bits {
            2 => Some(CLK_EN_A::SSI1),
            1 => Some(CLK_EN_A::SSI0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SSI1`"]
    #[inline(always)]
    pub fn is_ssi1(&self) -> bool {
        **self == CLK_EN_A::SSI1
    }
    #[doc = "Checks if the value of the field is `SSI0`"]
    #[inline(always)]
    pub fn is_ssi0(&self) -> bool {
        **self == CLK_EN_A::SSI0
    }
}
impl core::ops::Deref for CLK_EN_R {
    type Target = crate::FieldReader<u8, CLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EN` writer - 1:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Enable clock for SSI1"]
    #[inline(always)]
    pub fn ssi1(self) -> &'a mut W {
        self.variant(CLK_EN_A::SSI1)
    }
    #[doc = "Enable clock for SSI0"]
    #[inline(always)]
    pub fn ssi0(self) -> &'a mut W {
        self.variant(CLK_EN_A::SSI0)
    }
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
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
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
#[doc = "SSI Clock Gate For Run Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssiclkgr](index.html) module"]
pub struct SSICLKGR_SPEC;
impl crate::RegisterSpec for SSICLKGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssiclkgr::R](R) reader structure"]
impl crate::Readable for SSICLKGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssiclkgr::W](W) writer structure"]
impl crate::Writable for SSICLKGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSICLKGR to value 0"]
impl crate::Resettable for SSICLKGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
