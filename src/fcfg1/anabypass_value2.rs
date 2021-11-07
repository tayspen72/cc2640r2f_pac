#[doc = "Register `ANABYPASS_VALUE2` reader"]
pub struct R(crate::R<ANABYPASS_VALUE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANABYPASS_VALUE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANABYPASS_VALUE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANABYPASS_VALUE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANABYPASS_VALUE2` writer"]
pub struct W(crate::W<ANABYPASS_VALUE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANABYPASS_VALUE2_SPEC>;
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
impl From<crate::W<ANABYPASS_VALUE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANABYPASS_VALUE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOSC_HF_IBIASTHERM` reader - 13:0\\]
Internal. Only to be used through TI provided API."]
pub struct XOSC_HF_IBIASTHERM_R(crate::FieldReader<u16, u16>);
impl XOSC_HF_IBIASTHERM_R {
    pub(crate) fn new(bits: u16) -> Self {
        XOSC_HF_IBIASTHERM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC_HF_IBIASTHERM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC_HF_IBIASTHERM` writer - 13:0\\]
Internal. Only to be used through TI provided API."]
pub struct XOSC_HF_IBIASTHERM_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_IBIASTHERM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_ibiastherm(&self) -> XOSC_HF_IBIASTHERM_R {
        XOSC_HF_IBIASTHERM_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_ibiastherm(&mut self) -> XOSC_HF_IBIASTHERM_W {
        XOSC_HF_IBIASTHERM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anabypass_value2](index.html) module"]
pub struct ANABYPASS_VALUE2_SPEC;
impl crate::RegisterSpec for ANABYPASS_VALUE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [anabypass_value2::R](R) reader structure"]
impl crate::Readable for ANABYPASS_VALUE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [anabypass_value2::W](W) writer structure"]
impl crate::Writable for ANABYPASS_VALUE2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANABYPASS_VALUE2 to value 0xffff_c3ff"]
impl crate::Resettable for ANABYPASS_VALUE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_c3ff
    }
}
