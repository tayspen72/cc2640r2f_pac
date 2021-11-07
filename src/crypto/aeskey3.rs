#[doc = "Register `AESKEY3` reader"]
pub struct R(crate::R<AESKEY3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESKEY3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESKEY3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESKEY3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESKEY3` writer"]
pub struct W(crate::W<AESKEY3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESKEY3_SPEC>;
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
impl From<crate::W<AESKEY3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESKEY3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY3` reader - 31:0\\]
AESKEY3.* bits 31+x:0+x or AESKEY2.* bits 159+x:128+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register arrary. The interpretation of this field depends on the crypto operation mode."]
pub struct KEY3_R(crate::FieldReader<u32, u32>);
impl KEY3_R {
    pub(crate) fn new(bits: u32) -> Self {
        KEY3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY3` writer - 31:0\\]
AESKEY3.* bits 31+x:0+x or AESKEY2.* bits 159+x:128+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register arrary. The interpretation of this field depends on the crypto operation mode."]
pub struct KEY3_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
AESKEY3.* bits 31+x:0+x or AESKEY2.* bits 159+x:128+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register arrary. The interpretation of this field depends on the crypto operation mode."]
    #[inline(always)]
    pub fn key3(&self) -> KEY3_R {
        KEY3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
AESKEY3.* bits 31+x:0+x or AESKEY2.* bits 159+x:128+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register arrary. The interpretation of this field depends on the crypto operation mode."]
    #[inline(always)]
    pub fn key3(&mut self) -> KEY3_W {
        KEY3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear AES_KEY3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aeskey3](index.html) module"]
pub struct AESKEY3_SPEC;
impl crate::RegisterSpec for AESKEY3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aeskey3::R](R) reader structure"]
impl crate::Readable for AESKEY3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aeskey3::W](W) writer structure"]
impl crate::Writable for AESKEY3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESKEY3 to value 0"]
impl crate::Resettable for AESKEY3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
