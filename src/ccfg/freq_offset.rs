#[doc = "Register `FREQ_OFFSET` reader"]
pub struct R(crate::R<FREQ_OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQ_OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQ_OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQ_OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQ_OFFSET` writer"]
pub struct W(crate::W<FREQ_OFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQ_OFFSET_SPEC>;
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
impl From<crate::W<FREQ_OFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQ_OFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HF_COMP_P0` reader - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct HF_COMP_P0_R(crate::FieldReader<u16, u16>);
impl HF_COMP_P0_R {
    pub(crate) fn new(bits: u16) -> Self {
        HF_COMP_P0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HF_COMP_P0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HF_COMP_P0` writer - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct HF_COMP_P0_W<'a> {
    w: &'a mut W,
}
impl<'a> HF_COMP_P0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `HF_COMP_P1` reader - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct HF_COMP_P1_R(crate::FieldReader<u8, u8>);
impl HF_COMP_P1_R {
    pub(crate) fn new(bits: u8) -> Self {
        HF_COMP_P1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HF_COMP_P1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HF_COMP_P1` writer - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct HF_COMP_P1_W<'a> {
    w: &'a mut W,
}
impl<'a> HF_COMP_P1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `HF_COMP_P2` reader - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct HF_COMP_P2_R(crate::FieldReader<u8, u8>);
impl HF_COMP_P2_R {
    pub(crate) fn new(bits: u8) -> Self {
        HF_COMP_P2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HF_COMP_P2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HF_COMP_P2` writer - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct HF_COMP_P2_W<'a> {
    w: &'a mut W,
}
impl<'a> HF_COMP_P2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p0(&self) -> HF_COMP_P0_R {
        HF_COMP_P0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p1(&self) -> HF_COMP_P1_R {
        HF_COMP_P1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p2(&self) -> HF_COMP_P2_R {
        HF_COMP_P2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p0(&mut self) -> HF_COMP_P0_W {
        HF_COMP_P0_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p1(&mut self) -> HF_COMP_P1_W {
        HF_COMP_P1_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p2(&mut self) -> HF_COMP_P2_W {
        HF_COMP_P2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frequency Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freq_offset](index.html) module"]
pub struct FREQ_OFFSET_SPEC;
impl crate::RegisterSpec for FREQ_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [freq_offset::R](R) reader structure"]
impl crate::Readable for FREQ_OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [freq_offset::W](W) writer structure"]
impl crate::Writable for FREQ_OFFSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FREQ_OFFSET to value 0xffff_ffff"]
impl crate::Resettable for FREQ_OFFSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
