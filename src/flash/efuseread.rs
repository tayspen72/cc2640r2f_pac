#[doc = "Register `EFUSEREAD` reader"]
pub struct R(crate::R<EFUSEREAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSEREAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSEREAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSEREAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSEREAD` writer"]
pub struct W(crate::W<EFUSEREAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSEREAD_SPEC>;
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
impl From<crate::W<EFUSEREAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSEREAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED10` reader - 31:10\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED10_R(crate::FieldReader<u32, u32>);
impl RESERVED10_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED10_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED10` writer - 31:10\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED10_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | ((value as u32 & 0x003f_ffff) << 10);
        self.w
    }
}
#[doc = "Field `DATABIT` reader - 9:8\\]
Internal. Only to be used through TI provided API."]
pub struct DATABIT_R(crate::FieldReader<u8, u8>);
impl DATABIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATABIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATABIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATABIT` writer - 9:8\\]
Internal. Only to be used through TI provided API."]
pub struct DATABIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATABIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `READCLOCK` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub struct READCLOCK_R(crate::FieldReader<u8, u8>);
impl READCLOCK_R {
    pub(crate) fn new(bits: u8) -> Self {
        READCLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READCLOCK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READCLOCK` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub struct READCLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> READCLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `DEBUG` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub struct DEBUG_R(crate::FieldReader<bool, bool>);
impl DEBUG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEBUG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBUG` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub struct DEBUG_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SPARE` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub struct SPARE_R(crate::FieldReader<bool, bool>);
impl SPARE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub struct SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `MARGIN` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct MARGIN_R(crate::FieldReader<u8, u8>);
impl MARGIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        MARGIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MARGIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MARGIN` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct MARGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MARGIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - 31:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn databit(&self) -> DATABIT_R {
        DATABIT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn readclock(&self) -> READCLOCK_R {
        READCLOCK_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn debug(&self) -> DEBUG_R {
        DEBUG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn margin(&self) -> MARGIN_R {
        MARGIN_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 10:31 - 31:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved10(&mut self) -> RESERVED10_W {
        RESERVED10_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn databit(&mut self) -> DATABIT_W {
        DATABIT_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn readclock(&mut self) -> READCLOCK_W {
        READCLOCK_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn debug(&mut self) -> DEBUG_W {
        DEBUG_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn margin(&mut self) -> MARGIN_W {
        MARGIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuseread](index.html) module"]
pub struct EFUSEREAD_SPEC;
impl crate::RegisterSpec for EFUSEREAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuseread::R](R) reader structure"]
impl crate::Readable for EFUSEREAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuseread::W](W) writer structure"]
impl crate::Writable for EFUSEREAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EFUSEREAD to value 0"]
impl crate::Resettable for EFUSEREAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
