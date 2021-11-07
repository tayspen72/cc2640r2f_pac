#[doc = "Register `ATESTCTL` reader"]
pub struct R(crate::R<ATESTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATESTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATESTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATESTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATESTCTL` writer"]
pub struct W(crate::W<ATESTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATESTCTL_SPEC>;
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
impl From<crate::W<ATESTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATESTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPARE30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE30_R(crate::FieldReader<u8, u8>);
impl SPARE30_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPARE30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE30_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE30` writer - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE30_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `SCLK_LF_AUX_EN` reader - 29:29\\]
Enable 32 kHz clock to AUX_COMPB."]
pub struct SCLK_LF_AUX_EN_R(crate::FieldReader<bool, bool>);
impl SCLK_LF_AUX_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCLK_LF_AUX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLK_LF_AUX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLK_LF_AUX_EN` writer - 29:29\\]
Enable 32 kHz clock to AUX_COMPB."]
pub struct SCLK_LF_AUX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_LF_AUX_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 28:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_R(crate::FieldReader<u32, u32>);
impl RESERVED0_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - 28:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | (value as u32 & 0x1fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare30(&self) -> SPARE30_R {
        SPARE30_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
Enable 32 kHz clock to AUX_COMPB."]
    #[inline(always)]
    pub fn sclk_lf_aux_en(&self) -> SCLK_LF_AUX_EN_R {
        SCLK_LF_AUX_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 0:28 - 28:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare30(&mut self) -> SPARE30_W {
        SPARE30_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Enable 32 kHz clock to AUX_COMPB."]
    #[inline(always)]
    pub fn sclk_lf_aux_en(&mut self) -> SCLK_LF_AUX_EN_W {
        SCLK_LF_AUX_EN_W { w: self }
    }
    #[doc = "Bits 0:28 - 28:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
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
#[doc = "Analog Test Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atestctl](index.html) module"]
pub struct ATESTCTL_SPEC;
impl crate::RegisterSpec for ATESTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atestctl::R](R) reader structure"]
impl crate::Readable for ATESTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atestctl::W](W) writer structure"]
impl crate::Writable for ATESTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ATESTCTL to value 0"]
impl crate::Resettable for ATESTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
