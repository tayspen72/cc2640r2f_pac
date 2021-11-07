#[doc = "Register `FCOR_ERR_POS` reader"]
pub struct R(crate::R<FCOR_ERR_POS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCOR_ERR_POS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCOR_ERR_POS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCOR_ERR_POS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCOR_ERR_POS` writer"]
pub struct W(crate::W<FCOR_ERR_POS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCOR_ERR_POS_SPEC>;
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
impl From<crate::W<FCOR_ERR_POS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCOR_ERR_POS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SERR_POS` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct SERR_POS_R(crate::FieldReader<u32, u32>);
impl SERR_POS_R {
    pub(crate) fn new(bits: u32) -> Self {
        SERR_POS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERR_POS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERR_POS` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct SERR_POS_W<'a> {
    w: &'a mut W,
}
impl<'a> SERR_POS_W<'a> {
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
    pub fn serr_pos(&self) -> SERR_POS_R {
        SERR_POS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn serr_pos(&mut self) -> SERR_POS_W {
        SERR_POS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcor_err_pos](index.html) module"]
pub struct FCOR_ERR_POS_SPEC;
impl crate::RegisterSpec for FCOR_ERR_POS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcor_err_pos::R](R) reader structure"]
impl crate::Readable for FCOR_ERR_POS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcor_err_pos::W](W) writer structure"]
impl crate::Writable for FCOR_ERR_POS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCOR_ERR_POS to value 0"]
impl crate::Resettable for FCOR_ERR_POS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
