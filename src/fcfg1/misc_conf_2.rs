#[doc = "Register `MISC_CONF_2` reader"]
pub struct R(crate::R<MISC_CONF_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_CONF_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_CONF_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_CONF_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_CONF_2` writer"]
pub struct W(crate::W<MISC_CONF_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_CONF_2_SPEC>;
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
impl From<crate::W<MISC_CONF_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_CONF_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPOSC_COMP_P3` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct HPOSC_COMP_P3_R(crate::FieldReader<u8, u8>);
impl HPOSC_COMP_P3_R {
    pub(crate) fn new(bits: u8) -> Self {
        HPOSC_COMP_P3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPOSC_COMP_P3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPOSC_COMP_P3` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct HPOSC_COMP_P3_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOSC_COMP_P3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_comp_p3(&self) -> HPOSC_COMP_P3_R {
        HPOSC_COMP_P3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_comp_p3(&mut self) -> HPOSC_COMP_P3_W {
        HPOSC_COMP_P3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_conf_2](index.html) module"]
pub struct MISC_CONF_2_SPEC;
impl crate::RegisterSpec for MISC_CONF_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_conf_2::R](R) reader structure"]
impl crate::Readable for MISC_CONF_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_conf_2::W](W) writer structure"]
impl crate::Writable for MISC_CONF_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC_CONF_2 to value 0xffff_ff00"]
impl crate::Resettable for MISC_CONF_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ff00
    }
}
