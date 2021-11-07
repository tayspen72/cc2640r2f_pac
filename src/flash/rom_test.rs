#[doc = "Register `ROM_TEST` reader"]
pub struct R(crate::R<ROM_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_TEST` writer"]
pub struct W(crate::W<ROM_TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_TEST_SPEC>;
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
impl From<crate::W<ROM_TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_KEY` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct ROM_KEY_R(crate::FieldReader<u32, u32>);
impl ROM_KEY_R {
    pub(crate) fn new(bits: u32) -> Self {
        ROM_KEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_KEY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_KEY` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct ROM_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_KEY_W<'a> {
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
    pub fn rom_key(&self) -> ROM_KEY_R {
        ROM_KEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rom_key(&mut self) -> ROM_KEY_W {
        ROM_KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_test](index.html) module"]
pub struct ROM_TEST_SPEC;
impl crate::RegisterSpec for ROM_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_test::R](R) reader structure"]
impl crate::Readable for ROM_TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_test::W](W) writer structure"]
impl crate::Writable for ROM_TEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_TEST to value 0"]
impl crate::Resettable for ROM_TEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
