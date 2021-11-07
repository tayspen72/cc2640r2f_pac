#[doc = "Register `INFRCLKDIVR` reader"]
pub struct R(crate::R<INFRCLKDIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INFRCLKDIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INFRCLKDIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INFRCLKDIVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INFRCLKDIVR` writer"]
pub struct W(crate::W<INFRCLKDIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INFRCLKDIVR_SPEC>;
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
impl From<crate::W<INFRCLKDIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INFRCLKDIVR_SPEC>) -> Self {
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
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in run mode. Division ratio affects both infrastructure clock and perbusull clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RATIO_A {
    #[doc = "3: Divide by 32"]
    DIV32 = 3,
    #[doc = "2: Divide by 8"]
    DIV8 = 2,
    #[doc = "1: Divide by 2"]
    DIV2 = 1,
    #[doc = "0: Divide by 1"]
    DIV1 = 0,
}
impl From<RATIO_A> for u8 {
    #[inline(always)]
    fn from(variant: RATIO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RATIO` reader - 1:0\\]
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in run mode. Division ratio affects both infrastructure clock and perbusull clock."]
pub struct RATIO_R(crate::FieldReader<u8, RATIO_A>);
impl RATIO_R {
    pub(crate) fn new(bits: u8) -> Self {
        RATIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RATIO_A {
        match self.bits {
            3 => RATIO_A::DIV32,
            2 => RATIO_A::DIV8,
            1 => RATIO_A::DIV2,
            0 => RATIO_A::DIV1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == RATIO_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == RATIO_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == RATIO_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == RATIO_A::DIV1
    }
}
impl core::ops::Deref for RATIO_R {
    type Target = crate::FieldReader<u8, RATIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RATIO` writer - 1:0\\]
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in run mode. Division ratio affects both infrastructure clock and perbusull clock."]
pub struct RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RATIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RATIO_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(RATIO_A::DIV32)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(RATIO_A::DIV8)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(RATIO_A::DIV2)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(RATIO_A::DIV1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
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
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in run mode. Division ratio affects both infrastructure clock and perbusull clock."]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 0x03) as u8)
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
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in run mode. Division ratio affects both infrastructure clock and perbusull clock."]
    #[inline(always)]
    pub fn ratio(&mut self) -> RATIO_W {
        RATIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Infrastructure Clock Division Factor For Run Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [infrclkdivr](index.html) module"]
pub struct INFRCLKDIVR_SPEC;
impl crate::RegisterSpec for INFRCLKDIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [infrclkdivr::R](R) reader structure"]
impl crate::Readable for INFRCLKDIVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [infrclkdivr::W](W) writer structure"]
impl crate::Writable for INFRCLKDIVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INFRCLKDIVR to value 0"]
impl crate::Resettable for INFRCLKDIVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
