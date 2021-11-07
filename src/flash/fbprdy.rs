#[doc = "Register `FBPRDY` reader"]
pub struct R(crate::R<FBPRDY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBPRDY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBPRDY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBPRDY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBPRDY` writer"]
pub struct W(crate::W<FBPRDY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBPRDY_SPEC>;
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
impl From<crate::W<FBPRDY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBPRDY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED17` reader - 31:17\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED17_R(crate::FieldReader<u16, u16>);
impl RESERVED17_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED17_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED17` writer - 31:17\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED17_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 17)) | ((value as u32 & 0x7fff) << 17);
        self.w
    }
}
#[doc = "Field `BANKBUSY` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub struct BANKBUSY_R(crate::FieldReader<bool, bool>);
impl BANKBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BANKBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BANKBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BANKBUSY` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub struct BANKBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKBUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PUMPRDY` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub struct PUMPRDY_R(crate::FieldReader<bool, bool>);
impl PUMPRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PUMPRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUMPRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUMPRDY` writer - 15:15\\]
Internal. Only to be used through TI provided API."]
pub struct PUMPRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMPRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `RESERVED1` reader - 14:1\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED1_R(crate::FieldReader<u16, u16>);
impl RESERVED1_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED1` writer - 14:1\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 1)) | ((value as u32 & 0x3fff) << 1);
        self.w
    }
}
#[doc = "Field `BANKRDY` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct BANKRDY_R(crate::FieldReader<bool, bool>);
impl BANKRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BANKRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BANKRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BANKRDY` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct BANKRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 17:31 - 31:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankbusy(&self) -> BANKBUSY_R {
        BANKBUSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pumprdy(&self) -> PUMPRDY_R {
        PUMPRDY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 1:14 - 14:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x3fff) as u16)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankrdy(&self) -> BANKRDY_R {
        BANKRDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 17:31 - 31:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved17(&mut self) -> RESERVED17_W {
        RESERVED17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankbusy(&mut self) -> BANKBUSY_W {
        BANKBUSY_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pumprdy(&mut self) -> PUMPRDY_W {
        PUMPRDY_W { w: self }
    }
    #[doc = "Bits 1:14 - 14:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankrdy(&mut self) -> BANKRDY_W {
        BANKRDY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbprdy](index.html) module"]
pub struct FBPRDY_SPEC;
impl crate::RegisterSpec for FBPRDY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbprdy::R](R) reader structure"]
impl crate::Readable for FBPRDY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbprdy::W](W) writer structure"]
impl crate::Writable for FBPRDY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FBPRDY to value 0x00ff_00fe"]
impl crate::Resettable for FBPRDY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_00fe
    }
}
