#[doc = "Register `RFCBITS` reader"]
pub struct R(crate::R<RFCBITS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCBITS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCBITS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCBITS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFCBITS` writer"]
pub struct W(crate::W<RFCBITS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCBITS_SPEC>;
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
impl From<crate::W<RFCBITS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFCBITS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ` reader - 31:0\\]
Control bits for RFC. The RF core CPE processor will automatically check this register when it boots, and it can be used to immediately instruct CPE to perform some tasks at its start-up. The supported functionality is ROM-defined and may vary. See the technical reference manual for more details."]
pub struct READ_R(crate::FieldReader<u32, u32>);
impl READ_R {
    pub(crate) fn new(bits: u32) -> Self {
        READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ` writer - 31:0\\]
Control bits for RFC. The RF core CPE processor will automatically check this register when it boots, and it can be used to immediately instruct CPE to perform some tasks at its start-up. The supported functionality is ROM-defined and may vary. See the technical reference manual for more details."]
pub struct READ_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Control bits for RFC. The RF core CPE processor will automatically check this register when it boots, and it can be used to immediately instruct CPE to perform some tasks at its start-up. The supported functionality is ROM-defined and may vary. See the technical reference manual for more details."]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Control bits for RFC. The RF core CPE processor will automatically check this register when it boots, and it can be used to immediately instruct CPE to perform some tasks at its start-up. The supported functionality is ROM-defined and may vary. See the technical reference manual for more details."]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W {
        READ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control To RFC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcbits](index.html) module"]
pub struct RFCBITS_SPEC;
impl crate::RegisterSpec for RFCBITS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcbits::R](R) reader structure"]
impl crate::Readable for RFCBITS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcbits::W](W) writer structure"]
impl crate::Writable for RFCBITS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFCBITS to value 0"]
impl crate::Resettable for RFCBITS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
