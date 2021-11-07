#[doc = "Register `ICEPICK_DEVICE_ID` reader"]
pub struct R(crate::R<ICEPICK_DEVICE_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICEPICK_DEVICE_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICEPICK_DEVICE_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICEPICK_DEVICE_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICEPICK_DEVICE_ID` writer"]
pub struct W(crate::W<ICEPICK_DEVICE_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICEPICK_DEVICE_ID_SPEC>;
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
impl From<crate::W<ICEPICK_DEVICE_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICEPICK_DEVICE_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PG_REV` reader - 31:28\\]
Field used to distinguish revisions of the device."]
pub struct PG_REV_R(crate::FieldReader<u8, u8>);
impl PG_REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PG_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG_REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG_REV` writer - 31:28\\]
Field used to distinguish revisions of the device."]
pub struct PG_REV_W<'a> {
    w: &'a mut W,
}
impl<'a> PG_REV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `WAFER_ID` reader - 27:12\\]
Field used to identify silicon die."]
pub struct WAFER_ID_R(crate::FieldReader<u16, u16>);
impl WAFER_ID_R {
    pub(crate) fn new(bits: u16) -> Self {
        WAFER_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAFER_ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAFER_ID` writer - 27:12\\]
Field used to identify silicon die."]
pub struct WAFER_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> WAFER_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 12)) | ((value as u32 & 0xffff) << 12);
        self.w
    }
}
#[doc = "Field `MANUFACTURER_ID` reader - 11:0\\]
Manufacturer code. 0x02F: Texas Instruments"]
pub struct MANUFACTURER_ID_R(crate::FieldReader<u16, u16>);
impl MANUFACTURER_ID_R {
    pub(crate) fn new(bits: u16) -> Self {
        MANUFACTURER_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MANUFACTURER_ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MANUFACTURER_ID` writer - 11:0\\]
Manufacturer code. 0x02F: Texas Instruments"]
pub struct MANUFACTURER_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> MANUFACTURER_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Field used to distinguish revisions of the device."]
    #[inline(always)]
    pub fn pg_rev(&self) -> PG_REV_R {
        PG_REV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Field used to identify silicon die."]
    #[inline(always)]
    pub fn wafer_id(&self) -> WAFER_ID_R {
        WAFER_ID_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 0:11 - 11:0\\]
Manufacturer code. 0x02F: Texas Instruments"]
    #[inline(always)]
    pub fn manufacturer_id(&self) -> MANUFACTURER_ID_R {
        MANUFACTURER_ID_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Field used to distinguish revisions of the device."]
    #[inline(always)]
    pub fn pg_rev(&mut self) -> PG_REV_W {
        PG_REV_W { w: self }
    }
    #[doc = "Bits 12:27 - 27:12\\]
Field used to identify silicon die."]
    #[inline(always)]
    pub fn wafer_id(&mut self) -> WAFER_ID_W {
        WAFER_ID_W { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\]
Manufacturer code. 0x02F: Texas Instruments"]
    #[inline(always)]
    pub fn manufacturer_id(&mut self) -> MANUFACTURER_ID_W {
        MANUFACTURER_ID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IcePick Device Identification Reading this register and the USER_ID register is the only support way of identifying a device.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icepick_device_id](index.html) module"]
pub struct ICEPICK_DEVICE_ID_SPEC;
impl crate::RegisterSpec for ICEPICK_DEVICE_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icepick_device_id::R](R) reader structure"]
impl crate::Readable for ICEPICK_DEVICE_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icepick_device_id::W](W) writer structure"]
impl crate::Writable for ICEPICK_DEVICE_ID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICEPICK_DEVICE_ID to value 0xbb99_a02f"]
impl crate::Resettable for ICEPICK_DEVICE_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xbb99_a02f
    }
}
