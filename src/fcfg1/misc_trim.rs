#[doc = "Register `MISC_TRIM` reader"]
pub struct R(crate::R<MISC_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_TRIM` writer"]
pub struct W(crate::W<MISC_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_TRIM_SPEC>;
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
impl From<crate::W<MISC_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEMPVSLOPE` reader - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
pub struct TEMPVSLOPE_R(crate::FieldReader<u8, u8>);
impl TEMPVSLOPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TEMPVSLOPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEMPVSLOPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEMPVSLOPE` writer - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
pub struct TEMPVSLOPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMPVSLOPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
    #[inline(always)]
    pub fn tempvslope(&self) -> TEMPVSLOPE_R {
        TEMPVSLOPE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
    #[inline(always)]
    pub fn tempvslope(&mut self) -> TEMPVSLOPE_W {
        TEMPVSLOPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Miscellaneous Trim Parameters\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_trim](index.html) module"]
pub struct MISC_TRIM_SPEC;
impl crate::RegisterSpec for MISC_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_trim::R](R) reader structure"]
impl crate::Readable for MISC_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_trim::W](W) writer structure"]
impl crate::Writable for MISC_TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC_TRIM to value 0xffff_ff33"]
impl crate::Resettable for MISC_TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ff33
    }
}
