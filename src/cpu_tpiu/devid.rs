#[doc = "Register `DEVID` reader"]
pub struct R(crate::R<DEVID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVID` writer"]
pub struct W(crate::W<DEVID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVID_SPEC>;
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
impl From<crate::W<DEVID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEVID` reader - 31:0\\]
This field returns: 0xCA1 if there is an ETM present. 0xCA0 if there is no ETM present."]
pub struct DEVID_R(crate::FieldReader<u32, u32>);
impl DEVID_R {
    pub(crate) fn new(bits: u32) -> Self {
        DEVID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVID` writer - 31:0\\]
This field returns: 0xCA1 if there is an ETM present. 0xCA0 if there is no ETM present."]
pub struct DEVID_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This field returns: 0xCA1 if there is an ETM present. 0xCA0 if there is no ETM present."]
    #[inline(always)]
    pub fn devid(&self) -> DEVID_R {
        DEVID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This field returns: 0xCA1 if there is an ETM present. 0xCA0 if there is no ETM present."]
    #[inline(always)]
    pub fn devid(&mut self) -> DEVID_W {
        DEVID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devid](index.html) module"]
pub struct DEVID_SPEC;
impl crate::RegisterSpec for DEVID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devid::R](R) reader structure"]
impl crate::Readable for DEVID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devid::W](W) writer structure"]
impl crate::Writable for DEVID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVID to value 0x0ca0"]
impl crate::Resettable for DEVID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0ca0
    }
}
