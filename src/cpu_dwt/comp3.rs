#[doc = "Register `COMP3` reader"]
pub struct R(crate::R<COMP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP3` writer"]
pub struct W(crate::W<COMP3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP3_SPEC>;
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
impl From<crate::W<COMP3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP` reader - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION3."]
pub struct COMP_R(crate::FieldReader<u32, u32>);
impl COMP_R {
    pub(crate) fn new(bits: u32) -> Self {
        COMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP` writer - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION3."]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION3."]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION3."]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 3 This register is used to write the reference value for comparator 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp3](index.html) module"]
pub struct COMP3_SPEC;
impl crate::RegisterSpec for COMP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp3::R](R) reader structure"]
impl crate::Readable for COMP3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp3::W](W) writer structure"]
impl crate::Writable for COMP3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP3 to value 0"]
impl crate::Resettable for COMP3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
