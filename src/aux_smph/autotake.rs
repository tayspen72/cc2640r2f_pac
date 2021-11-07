#[doc = "Register `AUTOTAKE` reader"]
pub struct R(crate::R<AUTOTAKE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOTAKE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOTAKE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOTAKE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOTAKE` writer"]
pub struct W(crate::W<AUTOTAKE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOTAKE_SPEC>;
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
impl From<crate::W<AUTOTAKE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOTAKE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_R(crate::FieldReader<u32, u32>);
impl RESERVED3_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Field `SMPH_ID` reader - 2:0\\]
Write the semaphore ID,0x0-0x7, to SMPH_ID to request this semaphore until it is granted. When semaphore SMPH_ID is granted, event AUX_EVCTL:EVSTAT0.AUX_SMPH_AUTOTAKE_DONE becomes 1. The event becomes 0 when software releases the semaphore or writes a new value to SMPH_ID. To avoid corrupted semaphores: - Usage of this functionality must be restricted to one CPU core. - Software must wait until AUX_EVCTL:EVSTAT0.AUX_SMPH_AUTOTAKE_DONE is 1 before it writes a new value to SMPH_ID."]
pub struct SMPH_ID_R(crate::FieldReader<u8, u8>);
impl SMPH_ID_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMPH_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPH_ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPH_ID` writer - 2:0\\]
Write the semaphore ID,0x0-0x7, to SMPH_ID to request this semaphore until it is granted. When semaphore SMPH_ID is granted, event AUX_EVCTL:EVSTAT0.AUX_SMPH_AUTOTAKE_DONE becomes 1. The event becomes 0 when software releases the semaphore or writes a new value to SMPH_ID. To avoid corrupted semaphores: - Usage of this functionality must be restricted to one CPU core. - Software must wait until AUX_EVCTL:EVSTAT0.AUX_SMPH_AUTOTAKE_DONE is 1 before it writes a new value to SMPH_ID."]
pub struct SMPH_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPH_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Write the semaphore ID,0x0-0x7, to SMPH_ID to request this semaphore until it is granted. When semaphore SMPH_ID is granted, event AUX_EVCTL:EVSTAT0.AUX_SMPH_AUTOTAKE_DONE becomes 1. The event becomes 0 when software releases the semaphore or writes a new value to SMPH_ID. To avoid corrupted semaphores: - Usage of this functionality must be restricted to one CPU core. - Software must wait until AUX_EVCTL:EVSTAT0.AUX_SMPH_AUTOTAKE_DONE is 1 before it writes a new value to SMPH_ID."]
    #[inline(always)]
    pub fn smph_id(&self) -> SMPH_ID_R {
        SMPH_ID_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Write the semaphore ID,0x0-0x7, to SMPH_ID to request this semaphore until it is granted. When semaphore SMPH_ID is granted, event AUX_EVCTL:EVSTAT0.AUX_SMPH_AUTOTAKE_DONE becomes 1. The event becomes 0 when software releases the semaphore or writes a new value to SMPH_ID. To avoid corrupted semaphores: - Usage of this functionality must be restricted to one CPU core. - Software must wait until AUX_EVCTL:EVSTAT0.AUX_SMPH_AUTOTAKE_DONE is 1 before it writes a new value to SMPH_ID."]
    #[inline(always)]
    pub fn smph_id(&mut self) -> SMPH_ID_W {
        SMPH_ID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auto Take Sticky Request for Single Semaphore.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autotake](index.html) module"]
pub struct AUTOTAKE_SPEC;
impl crate::RegisterSpec for AUTOTAKE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autotake::R](R) reader structure"]
impl crate::Readable for AUTOTAKE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autotake::W](W) writer structure"]
impl crate::Writable for AUTOTAKE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUTOTAKE to value 0"]
impl crate::Resettable for AUTOTAKE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
