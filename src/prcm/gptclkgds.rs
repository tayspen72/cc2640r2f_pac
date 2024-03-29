#[doc = "Register `GPTCLKGDS` reader"]
pub struct R(crate::R<GPTCLKGDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTCLKGDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTCLKGDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTCLKGDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTCLKGDS` writer"]
pub struct W(crate::W<GPTCLKGDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTCLKGDS_SPEC>;
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
impl From<crate::W<GPTCLKGDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTCLKGDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_R(crate::FieldReader<u32, u32>);
impl RESERVED4_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | ((value as u32 & 0x0fff_ffff) << 4);
        self.w
    }
}
#[doc = "3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_EN_A {
    #[doc = "8: Enable clock for GPT3"]
    GPT3 = 8,
    #[doc = "4: Enable clock for GPT2"]
    GPT2 = 4,
    #[doc = "2: Enable clock for GPT1"]
    GPT1 = 2,
    #[doc = "1: Enable clock for GPT0"]
    GPT0 = 1,
}
impl From<CLK_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLK_EN` reader - 3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub struct CLK_EN_R(crate::FieldReader<u8, CLK_EN_A>);
impl CLK_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_EN_A> {
        match self.bits {
            8 => Some(CLK_EN_A::GPT3),
            4 => Some(CLK_EN_A::GPT2),
            2 => Some(CLK_EN_A::GPT1),
            1 => Some(CLK_EN_A::GPT0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPT3`"]
    #[inline(always)]
    pub fn is_gpt3(&self) -> bool {
        **self == CLK_EN_A::GPT3
    }
    #[doc = "Checks if the value of the field is `GPT2`"]
    #[inline(always)]
    pub fn is_gpt2(&self) -> bool {
        **self == CLK_EN_A::GPT2
    }
    #[doc = "Checks if the value of the field is `GPT1`"]
    #[inline(always)]
    pub fn is_gpt1(&self) -> bool {
        **self == CLK_EN_A::GPT1
    }
    #[doc = "Checks if the value of the field is `GPT0`"]
    #[inline(always)]
    pub fn is_gpt0(&self) -> bool {
        **self == CLK_EN_A::GPT0
    }
}
impl core::ops::Deref for CLK_EN_R {
    type Target = crate::FieldReader<u8, CLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EN` writer - 3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Enable clock for GPT3"]
    #[inline(always)]
    pub fn gpt3(self) -> &'a mut W {
        self.variant(CLK_EN_A::GPT3)
    }
    #[doc = "Enable clock for GPT2"]
    #[inline(always)]
    pub fn gpt2(self) -> &'a mut W {
        self.variant(CLK_EN_A::GPT2)
    }
    #[doc = "Enable clock for GPT1"]
    #[inline(always)]
    pub fn gpt1(self) -> &'a mut W {
        self.variant(CLK_EN_A::GPT1)
    }
    #[doc = "Enable clock for GPT0"]
    #[inline(always)]
    pub fn gpt0(self) -> &'a mut W {
        self.variant(CLK_EN_A::GPT0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Each bit below has the following meaning: 0: Disable clock 1: Enable clock ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
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
#[doc = "GPT Clock Gate For Deep Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptclkgds](index.html) module"]
pub struct GPTCLKGDS_SPEC;
impl crate::RegisterSpec for GPTCLKGDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gptclkgds::R](R) reader structure"]
impl crate::Readable for GPTCLKGDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptclkgds::W](W) writer structure"]
impl crate::Writable for GPTCLKGDS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPTCLKGDS to value 0"]
impl crate::Resettable for GPTCLKGDS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
