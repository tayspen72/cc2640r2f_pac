#[doc = "Register `TBPMR` reader"]
pub struct R(crate::R<TBPMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBPMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBPMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBPMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBPMR` writer"]
pub struct W(crate::W<TBPMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBPMR_SPEC>;
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
impl From<crate::W<TBPMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBPMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED8_R(crate::FieldReader<u32, u32>);
impl RESERVED8_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED8_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Field `TBPSMR` reader - 7:0\\]
GPT Timer B Pre-scale Match Register. In 16 bit mode this field holds bits 23 to 16."]
pub struct TBPSMR_R(crate::FieldReader<u8, u8>);
impl TBPSMR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TBPSMR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBPSMR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBPSMR` writer - 7:0\\]
GPT Timer B Pre-scale Match Register. In 16 bit mode this field holds bits 23 to 16."]
pub struct TBPSMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPSMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\]
GPT Timer B Pre-scale Match Register. In 16 bit mode this field holds bits 23 to 16."]
    #[inline(always)]
    pub fn tbpsmr(&self) -> TBPSMR_R {
        TBPSMR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
GPT Timer B Pre-scale Match Register. In 16 bit mode this field holds bits 23 to 16."]
    #[inline(always)]
    pub fn tbpsmr(&mut self) -> TBPSMR_W {
        TBPSMR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer B Pre-scale Match This register allows software to extend the range of the TBMATCHR when used individually.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbpmr](index.html) module"]
pub struct TBPMR_SPEC;
impl crate::RegisterSpec for TBPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbpmr::R](R) reader structure"]
impl crate::Readable for TBPMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbpmr::W](W) writer structure"]
impl crate::Writable for TBPMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBPMR to value 0"]
impl crate::Resettable for TBPMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
