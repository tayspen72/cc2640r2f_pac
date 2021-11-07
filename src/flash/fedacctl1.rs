#[doc = "Register `FEDACCTL1` reader"]
pub struct R(crate::R<FEDACCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEDACCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEDACCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEDACCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEDACCTL1` writer"]
pub struct W(crate::W<FEDACCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEDACCTL1_SPEC>;
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
impl From<crate::W<FEDACCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEDACCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED25` reader - 31:25\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED25_R(crate::FieldReader<u8, u8>);
impl RESERVED25_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED25_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED25` writer - 31:25\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED25_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
#[doc = "Field `SUSP_IGNR` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub struct SUSP_IGNR_R(crate::FieldReader<bool, bool>);
impl SUSP_IGNR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSP_IGNR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP_IGNR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP_IGNR` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub struct SUSP_IGNR_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_IGNR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `EDACEN` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub struct EDACEN_R(crate::FieldReader<u32, u32>);
impl EDACEN_R {
    pub(crate) fn new(bits: u32) -> Self {
        EDACEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDACEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDACEN` writer - 23:0\\]
Internal. Only to be used through TI provided API."]
pub struct EDACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EDACEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn susp_ignr(&self) -> SUSP_IGNR_R {
        SUSP_IGNR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn edacen(&self) -> EDACEN_R {
        EDACEN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&mut self) -> RESERVED25_W {
        RESERVED25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn susp_ignr(&mut self) -> SUSP_IGNR_W {
        SUSP_IGNR_W { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn edacen(&mut self) -> EDACEN_W {
        EDACEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fedacctl1](index.html) module"]
pub struct FEDACCTL1_SPEC;
impl crate::RegisterSpec for FEDACCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fedacctl1::R](R) reader structure"]
impl crate::Readable for FEDACCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fedacctl1::W](W) writer structure"]
impl crate::Writable for FEDACCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FEDACCTL1 to value 0"]
impl crate::Resettable for FEDACCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
