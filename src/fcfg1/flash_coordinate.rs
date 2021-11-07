#[doc = "Register `FLASH_COORDINATE` reader"]
pub struct R(crate::R<FLASH_COORDINATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_COORDINATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_COORDINATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_COORDINATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_COORDINATE` writer"]
pub struct W(crate::W<FLASH_COORDINATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_COORDINATE_SPEC>;
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
impl From<crate::W<FLASH_COORDINATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_COORDINATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XCOORDINATE` reader - 31:16\\]
X coordinate of this unit on the wafer."]
pub struct XCOORDINATE_R(crate::FieldReader<u16, u16>);
impl XCOORDINATE_R {
    pub(crate) fn new(bits: u16) -> Self {
        XCOORDINATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XCOORDINATE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XCOORDINATE` writer - 31:16\\]
X coordinate of this unit on the wafer."]
pub struct XCOORDINATE_W<'a> {
    w: &'a mut W,
}
impl<'a> XCOORDINATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `YCOORDINATE` reader - 15:0\\]
Y coordinate of this unit on the wafer."]
pub struct YCOORDINATE_R(crate::FieldReader<u16, u16>);
impl YCOORDINATE_R {
    pub(crate) fn new(bits: u16) -> Self {
        YCOORDINATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YCOORDINATE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `YCOORDINATE` writer - 15:0\\]
Y coordinate of this unit on the wafer."]
pub struct YCOORDINATE_W<'a> {
    w: &'a mut W,
}
impl<'a> YCOORDINATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
X coordinate of this unit on the wafer."]
    #[inline(always)]
    pub fn xcoordinate(&self) -> XCOORDINATE_R {
        XCOORDINATE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Y coordinate of this unit on the wafer."]
    #[inline(always)]
    pub fn ycoordinate(&self) -> YCOORDINATE_R {
        YCOORDINATE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
X coordinate of this unit on the wafer."]
    #[inline(always)]
    pub fn xcoordinate(&mut self) -> XCOORDINATE_W {
        XCOORDINATE_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Y coordinate of this unit on the wafer."]
    #[inline(always)]
    pub fn ycoordinate(&mut self) -> YCOORDINATE_W {
        YCOORDINATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH_COORDINATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_coordinate](index.html) module"]
pub struct FLASH_COORDINATE_SPEC;
impl crate::RegisterSpec for FLASH_COORDINATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_coordinate::R](R) reader structure"]
impl crate::Readable for FLASH_COORDINATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_coordinate::W](W) writer structure"]
impl crate::Writable for FLASH_COORDINATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_COORDINATE to value 0"]
impl crate::Resettable for FLASH_COORDINATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
