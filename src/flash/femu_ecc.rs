#[doc = "Register `FEMU_ECC` reader"]
pub struct R(crate::R<FEMU_ECC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEMU_ECC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEMU_ECC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEMU_ECC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEMU_ECC` writer"]
pub struct W(crate::W<FEMU_ECC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEMU_ECC_SPEC>;
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
impl From<crate::W<FEMU_ECC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEMU_ECC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMU_ECC` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct EMU_ECC_R(crate::FieldReader<u32, u32>);
impl EMU_ECC_R {
    pub(crate) fn new(bits: u32) -> Self {
        EMU_ECC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMU_ECC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMU_ECC` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct EMU_ECC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMU_ECC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn emu_ecc(&self) -> EMU_ECC_R {
        EMU_ECC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn emu_ecc(&mut self) -> EMU_ECC_W {
        EMU_ECC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [femu_ecc](index.html) module"]
pub struct FEMU_ECC_SPEC;
impl crate::RegisterSpec for FEMU_ECC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [femu_ecc::R](R) reader structure"]
impl crate::Readable for FEMU_ECC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [femu_ecc::W](W) writer structure"]
impl crate::Writable for FEMU_ECC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FEMU_ECC to value 0"]
impl crate::Resettable for FEMU_ECC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
