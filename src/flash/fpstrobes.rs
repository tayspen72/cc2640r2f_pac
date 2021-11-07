#[doc = "Register `FPSTROBES` reader"]
pub struct R(crate::R<FPSTROBES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPSTROBES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPSTROBES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPSTROBES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPSTROBES` writer"]
pub struct W(crate::W<FPSTROBES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPSTROBES_SPEC>;
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
impl From<crate::W<FPSTROBES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPSTROBES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED9_R(crate::FieldReader<u32, u32>);
impl RESERVED9_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED9_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED9` writer - 31:9\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | ((value as u32 & 0x007f_ffff) << 9);
        self.w
    }
}
#[doc = "Field `EXECUTEZ` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct EXECUTEZ_R(crate::FieldReader<bool, bool>);
impl EXECUTEZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXECUTEZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXECUTEZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXECUTEZ` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct EXECUTEZ_W<'a> {
    w: &'a mut W,
}
impl<'a> EXECUTEZ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `RESERVED2` reader - 7:2\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED2_R(crate::FieldReader<u8, u8>);
impl RESERVED2_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED2` writer - 7:2\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | ((value as u32 & 0x3f) << 2);
        self.w
    }
}
#[doc = "Field `V3PWRDNZ` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub struct V3PWRDNZ_R(crate::FieldReader<bool, bool>);
impl V3PWRDNZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        V3PWRDNZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for V3PWRDNZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `V3PWRDNZ` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub struct V3PWRDNZ_W<'a> {
    w: &'a mut W,
}
impl<'a> V3PWRDNZ_W<'a> {
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
#[doc = "Field `V5PWRDNZ` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct V5PWRDNZ_R(crate::FieldReader<bool, bool>);
impl V5PWRDNZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        V5PWRDNZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for V5PWRDNZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `V5PWRDNZ` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct V5PWRDNZ_W<'a> {
    w: &'a mut W,
}
impl<'a> V5PWRDNZ_W<'a> {
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
    #[doc = "Bits 9:31 - 31:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn executez(&self) -> EXECUTEZ_R {
        EXECUTEZ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn v3pwrdnz(&self) -> V3PWRDNZ_R {
        V3PWRDNZ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn v5pwrdnz(&self) -> V5PWRDNZ_R {
        V5PWRDNZ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 9:31 - 31:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn executez(&mut self) -> EXECUTEZ_W {
        EXECUTEZ_W { w: self }
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn v3pwrdnz(&mut self) -> V3PWRDNZ_W {
        V3PWRDNZ_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn v5pwrdnz(&mut self) -> V5PWRDNZ_W {
        V5PWRDNZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpstrobes](index.html) module"]
pub struct FPSTROBES_SPEC;
impl crate::RegisterSpec for FPSTROBES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpstrobes::R](R) reader structure"]
impl crate::Readable for FPSTROBES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpstrobes::W](W) writer structure"]
impl crate::Writable for FPSTROBES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPSTROBES to value 0x0103"]
impl crate::Resettable for FPSTROBES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0103
    }
}
