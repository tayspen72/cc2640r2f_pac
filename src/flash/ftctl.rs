#[doc = "Register `FTCTL` reader"]
pub struct R(crate::R<FTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTCTL` writer"]
pub struct W(crate::W<FTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTCTL_SPEC>;
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
impl From<crate::W<FTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTCTL_SPEC>) -> Self {
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
#[doc = "Field `WDATA_BLK_CLR` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub struct WDATA_BLK_CLR_R(crate::FieldReader<bool, bool>);
impl WDATA_BLK_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDATA_BLK_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDATA_BLK_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDATA_BLK_CLR` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub struct WDATA_BLK_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WDATA_BLK_CLR_W<'a> {
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
#[doc = "Field `RESERVED2` reader - 15:2\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED2_R(crate::FieldReader<u16, u16>);
impl RESERVED2_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED2` writer - 15:2\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u32 & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Field `TEST_EN` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub struct TEST_EN_R(crate::FieldReader<bool, bool>);
impl TEST_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_EN` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub struct TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED0_R(crate::FieldReader<bool, bool>);
impl RESERVED0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
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
    pub fn wdata_blk_clr(&self) -> WDATA_BLK_CLR_R {
        WDATA_BLK_CLR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn test_en(&self) -> TEST_EN_R {
        TEST_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x01) != 0)
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
    pub fn wdata_blk_clr(&mut self) -> WDATA_BLK_CLR_W {
        WDATA_BLK_CLR_W { w: self }
    }
    #[doc = "Bits 2:15 - 15:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn test_en(&mut self) -> TEST_EN_W {
        TEST_EN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftctl](index.html) module"]
pub struct FTCTL_SPEC;
impl crate::RegisterSpec for FTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftctl::R](R) reader structure"]
impl crate::Readable for FTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftctl::W](W) writer structure"]
impl crate::Writable for FTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FTCTL to value 0"]
impl crate::Resettable for FTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
