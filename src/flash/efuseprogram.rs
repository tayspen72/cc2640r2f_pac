#[doc = "Register `EFUSEPROGRAM` reader"]
pub struct R(crate::R<EFUSEPROGRAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSEPROGRAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSEPROGRAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSEPROGRAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSEPROGRAM` writer"]
pub struct W(crate::W<EFUSEPROGRAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSEPROGRAM_SPEC>;
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
impl From<crate::W<EFUSEPROGRAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSEPROGRAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED31` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED31_R(crate::FieldReader<bool, bool>);
impl RESERVED31_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED31` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED31_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `COMPAREDISABLE` reader - 30:30\\]
Internal. Only to be used through TI provided API."]
pub struct COMPAREDISABLE_R(crate::FieldReader<bool, bool>);
impl COMPAREDISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPAREDISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPAREDISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPAREDISABLE` writer - 30:30\\]
Internal. Only to be used through TI provided API."]
pub struct COMPAREDISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREDISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `CLOCKSTALL` reader - 29:14\\]
Internal. Only to be used through TI provided API."]
pub struct CLOCKSTALL_R(crate::FieldReader<u16, u16>);
impl CLOCKSTALL_R {
    pub(crate) fn new(bits: u16) -> Self {
        CLOCKSTALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOCKSTALL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOCKSTALL` writer - 29:14\\]
Internal. Only to be used through TI provided API."]
pub struct CLOCKSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKSTALL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 14)) | ((value as u32 & 0xffff) << 14);
        self.w
    }
}
#[doc = "Field `VPPTOVDD` reader - 13:13\\]
Internal. Only to be used through TI provided API."]
pub struct VPPTOVDD_R(crate::FieldReader<bool, bool>);
impl VPPTOVDD_R {
    pub(crate) fn new(bits: bool) -> Self {
        VPPTOVDD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VPPTOVDD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VPPTOVDD` writer - 13:13\\]
Internal. Only to be used through TI provided API."]
pub struct VPPTOVDD_W<'a> {
    w: &'a mut W,
}
impl<'a> VPPTOVDD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `ITERATIONS` reader - 12:9\\]
Internal. Only to be used through TI provided API."]
pub struct ITERATIONS_R(crate::FieldReader<u8, u8>);
impl ITERATIONS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ITERATIONS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITERATIONS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITERATIONS` writer - 12:9\\]
Internal. Only to be used through TI provided API."]
pub struct ITERATIONS_W<'a> {
    w: &'a mut W,
}
impl<'a> ITERATIONS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | ((value as u32 & 0x0f) << 9);
        self.w
    }
}
#[doc = "Field `WRITECLOCK` reader - 8:0\\]
Internal. Only to be used through TI provided API."]
pub struct WRITECLOCK_R(crate::FieldReader<u16, u16>);
impl WRITECLOCK_R {
    pub(crate) fn new(bits: u16) -> Self {
        WRITECLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRITECLOCK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRITECLOCK` writer - 8:0\\]
Internal. Only to be used through TI provided API."]
pub struct WRITECLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITECLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved31(&self) -> RESERVED31_R {
        RESERVED31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn comparedisable(&self) -> COMPAREDISABLE_R {
        COMPAREDISABLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 14:29 - 29:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn clockstall(&self) -> CLOCKSTALL_R {
        CLOCKSTALL_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vpptovdd(&self) -> VPPTOVDD_R {
        VPPTOVDD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 9:12 - 12:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iterations(&self) -> ITERATIONS_R {
        ITERATIONS_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 0:8 - 8:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn writeclock(&self) -> WRITECLOCK_R {
        WRITECLOCK_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved31(&mut self) -> RESERVED31_W {
        RESERVED31_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn comparedisable(&mut self) -> COMPAREDISABLE_W {
        COMPAREDISABLE_W { w: self }
    }
    #[doc = "Bits 14:29 - 29:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn clockstall(&mut self) -> CLOCKSTALL_W {
        CLOCKSTALL_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vpptovdd(&mut self) -> VPPTOVDD_W {
        VPPTOVDD_W { w: self }
    }
    #[doc = "Bits 9:12 - 12:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iterations(&mut self) -> ITERATIONS_W {
        ITERATIONS_W { w: self }
    }
    #[doc = "Bits 0:8 - 8:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn writeclock(&mut self) -> WRITECLOCK_W {
        WRITECLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuseprogram](index.html) module"]
pub struct EFUSEPROGRAM_SPEC;
impl crate::RegisterSpec for EFUSEPROGRAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuseprogram::R](R) reader structure"]
impl crate::Readable for EFUSEPROGRAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuseprogram::W](W) writer structure"]
impl crate::Writable for EFUSEPROGRAM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EFUSEPROGRAM to value 0"]
impl crate::Resettable for EFUSEPROGRAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
