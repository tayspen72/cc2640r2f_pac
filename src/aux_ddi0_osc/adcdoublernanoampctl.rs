#[doc = "Register `ADCDOUBLERNANOAMPCTL` reader"]
pub struct R(crate::R<ADCDOUBLERNANOAMPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCDOUBLERNANOAMPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCDOUBLERNANOAMPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCDOUBLERNANOAMPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCDOUBLERNANOAMPCTL` writer"]
pub struct W(crate::W<ADCDOUBLERNANOAMPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCDOUBLERNANOAMPCTL_SPEC>;
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
impl From<crate::W<ADCDOUBLERNANOAMPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCDOUBLERNANOAMPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
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
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
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
#[doc = "Field `NANOAMP_BIAS_ENABLE` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub struct NANOAMP_BIAS_ENABLE_R(crate::FieldReader<bool, bool>);
impl NANOAMP_BIAS_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NANOAMP_BIAS_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NANOAMP_BIAS_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NANOAMP_BIAS_ENABLE` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub struct NANOAMP_BIAS_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> NANOAMP_BIAS_ENABLE_W<'a> {
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
#[doc = "Field `SPARE23` reader - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub struct SPARE23_R(crate::FieldReader<bool, bool>);
impl SPARE23_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPARE23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE23` writer - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub struct SPARE23_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `RESERVED6` reader - 22:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED6_R(crate::FieldReader<u32, u32>);
impl RESERVED6_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED6_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED6` writer - 22:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 6)) | ((value as u32 & 0x0001_ffff) << 6);
        self.w
    }
}
#[doc = "Field `ADC_SH_MODE_EN` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub struct ADC_SH_MODE_EN_R(crate::FieldReader<bool, bool>);
impl ADC_SH_MODE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_SH_MODE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_SH_MODE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_SH_MODE_EN` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub struct ADC_SH_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SH_MODE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `ADC_SH_VBUF_EN` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub struct ADC_SH_VBUF_EN_R(crate::FieldReader<bool, bool>);
impl ADC_SH_VBUF_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_SH_VBUF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_SH_VBUF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_SH_VBUF_EN` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub struct ADC_SH_VBUF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SH_VBUF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RESERVED2` reader - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
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
#[doc = "Field `RESERVED2` writer - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `ADC_IREF_CTRL` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct ADC_IREF_CTRL_R(crate::FieldReader<u8, u8>);
impl ADC_IREF_CTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_IREF_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_IREF_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_IREF_CTRL` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct ADC_IREF_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_IREF_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nanoamp_bias_enable(&self) -> NANOAMP_BIAS_ENABLE_R {
        NANOAMP_BIAS_ENABLE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn spare23(&self) -> SPARE23_R {
        SPARE23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 6:22 - 22:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_sh_mode_en(&self) -> ADC_SH_MODE_EN_R {
        ADC_SH_MODE_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_sh_vbuf_en(&self) -> ADC_SH_VBUF_EN_R {
        ADC_SH_VBUF_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_iref_ctrl(&self) -> ADC_IREF_CTRL_R {
        ADC_IREF_CTRL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&mut self) -> RESERVED25_W {
        RESERVED25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nanoamp_bias_enable(&mut self) -> NANOAMP_BIAS_ENABLE_W {
        NANOAMP_BIAS_ENABLE_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn spare23(&mut self) -> SPARE23_W {
        SPARE23_W { w: self }
    }
    #[doc = "Bits 6:22 - 22:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_sh_mode_en(&mut self) -> ADC_SH_MODE_EN_W {
        ADC_SH_MODE_EN_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_sh_vbuf_en(&mut self) -> ADC_SH_VBUF_EN_W {
        ADC_SH_VBUF_EN_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_iref_ctrl(&mut self) -> ADC_IREF_CTRL_W {
        ADC_IREF_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Doubler Nanoamp Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcdoublernanoampctl](index.html) module"]
pub struct ADCDOUBLERNANOAMPCTL_SPEC;
impl crate::RegisterSpec for ADCDOUBLERNANOAMPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcdoublernanoampctl::R](R) reader structure"]
impl crate::Readable for ADCDOUBLERNANOAMPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcdoublernanoampctl::W](W) writer structure"]
impl crate::Writable for ADCDOUBLERNANOAMPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCDOUBLERNANOAMPCTL to value 0"]
impl crate::Resettable for ADCDOUBLERNANOAMPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
