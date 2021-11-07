#[doc = "Register `FCFG1_REVISION` reader"]
pub struct R(crate::R<FCFG1_REVISION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFG1_REVISION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFG1_REVISION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFG1_REVISION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFG1_REVISION` writer"]
pub struct W(crate::W<FCFG1_REVISION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFG1_REVISION_SPEC>;
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
impl From<crate::W<FCFG1_REVISION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFG1_REVISION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REV` reader - 31:0\\]
The revision number of the FCFG1 layout. This value will be read by application SW in order to determine which FCFG1 parameters that have valid values. This revision number must be incremented by 1 before any devices are to be produced if the FCFG1 layout has changed since the previous production of devices. Value migth change without warning."]
pub struct REV_R(crate::FieldReader<u32, u32>);
impl REV_R {
    pub(crate) fn new(bits: u32) -> Self {
        REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV` writer - 31:0\\]
The revision number of the FCFG1 layout. This value will be read by application SW in order to determine which FCFG1 parameters that have valid values. This revision number must be incremented by 1 before any devices are to be produced if the FCFG1 layout has changed since the previous production of devices. Value migth change without warning."]
pub struct REV_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The revision number of the FCFG1 layout. This value will be read by application SW in order to determine which FCFG1 parameters that have valid values. This revision number must be incremented by 1 before any devices are to be produced if the FCFG1 layout has changed since the previous production of devices. Value migth change without warning."]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The revision number of the FCFG1 layout. This value will be read by application SW in order to determine which FCFG1 parameters that have valid values. This revision number must be incremented by 1 before any devices are to be produced if the FCFG1 layout has changed since the previous production of devices. Value migth change without warning."]
    #[inline(always)]
    pub fn rev(&mut self) -> REV_W {
        REV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Factory Configuration (FCFG1) Revision\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg1_revision](index.html) module"]
pub struct FCFG1_REVISION_SPEC;
impl crate::RegisterSpec for FCFG1_REVISION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfg1_revision::R](R) reader structure"]
impl crate::Readable for FCFG1_REVISION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcfg1_revision::W](W) writer structure"]
impl crate::Writable for FCFG1_REVISION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCFG1_REVISION to value 0x25"]
impl crate::Resettable for FCFG1_REVISION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x25
    }
}
