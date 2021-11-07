#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED23` reader - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED23_R(crate::FieldReader<u16, u16>);
impl RESERVED23_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED23_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED23` writer - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED23_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 23)) | ((value as u32 & 0x01ff) << 23);
        self.w
    }
}
#[doc = "Field `RCOSCHFCTRIMFRACT` reader - 22:18\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSCHFCTRIMFRACT_R(crate::FieldReader<u8, u8>);
impl RCOSCHFCTRIMFRACT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCOSCHFCTRIMFRACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCOSCHFCTRIMFRACT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCOSCHFCTRIMFRACT` writer - 22:18\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSCHFCTRIMFRACT_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHFCTRIMFRACT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | ((value as u32 & 0x1f) << 18);
        self.w
    }
}
#[doc = "Field `RCOSCHFCTRIMFRACT_EN` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSCHFCTRIMFRACT_EN_R(crate::FieldReader<bool, bool>);
impl RCOSCHFCTRIMFRACT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCOSCHFCTRIMFRACT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCOSCHFCTRIMFRACT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCOSCHFCTRIMFRACT_EN` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSCHFCTRIMFRACT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHFCTRIMFRACT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `SPARE2` reader - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE2_R(crate::FieldReader<u16, u16>);
impl SPARE2_R {
    pub(crate) fn new(bits: u16) -> Self {
        SPARE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE2` writer - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 2)) | ((value as u32 & 0x7fff) << 2);
        self.w
    }
}
#[doc = "Field `XOSC_HF_FAST_START` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct XOSC_HF_FAST_START_R(crate::FieldReader<u8, u8>);
impl XOSC_HF_FAST_START_R {
    pub(crate) fn new(bits: u8) -> Self {
        XOSC_HF_FAST_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC_HF_FAST_START_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC_HF_FAST_START` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct XOSC_HF_FAST_START_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_FAST_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 23:31 - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&self) -> RESERVED23_R {
        RESERVED23_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:22 - 22:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract(&self) -> RCOSCHFCTRIMFRACT_R {
        RCOSCHFCTRIMFRACT_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract_en(&self) -> RCOSCHFCTRIMFRACT_EN_R {
        RCOSCHFCTRIMFRACT_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 2:16 - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare2(&self) -> SPARE2_R {
        SPARE2_R::new(((self.bits >> 2) & 0x7fff) as u16)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_fast_start(&self) -> XOSC_HF_FAST_START_R {
        XOSC_HF_FAST_START_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 23:31 - 31:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&mut self) -> RESERVED23_W {
        RESERVED23_W { w: self }
    }
    #[doc = "Bits 18:22 - 22:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract(&mut self) -> RCOSCHFCTRIMFRACT_W {
        RCOSCHFCTRIMFRACT_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschfctrimfract_en(&mut self) -> RCOSCHFCTRIMFRACT_EN_W {
        RCOSCHFCTRIMFRACT_EN_W { w: self }
    }
    #[doc = "Bits 2:16 - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare2(&mut self) -> SPARE2_W {
        SPARE2_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_fast_start(&mut self) -> XOSC_HF_FAST_START_W {
        XOSC_HF_FAST_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 1 This register contains OSC_DIG configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
