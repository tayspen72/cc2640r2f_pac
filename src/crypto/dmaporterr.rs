#[doc = "Register `DMAPORTERR` reader"]
pub struct R(crate::R<DMAPORTERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAPORTERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAPORTERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAPORTERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAPORTERR` writer"]
pub struct W(crate::W<DMAPORTERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAPORTERR_SPEC>;
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
impl From<crate::W<DMAPORTERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAPORTERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED13` reader - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED13_R(crate::FieldReader<u32, u32>);
impl RESERVED13_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED13_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED13` writer - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED13_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0007_ffff << 13)) | ((value as u32 & 0x0007_ffff) << 13);
        self.w
    }
}
#[doc = "Field `AHB_ERR` reader - 12:12\\]
A 1 indicates that the Crypto peripheral has detected an AHB bus error"]
pub struct AHB_ERR_R(crate::FieldReader<bool, bool>);
impl AHB_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHB_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_ERR` writer - 12:12\\]
A 1 indicates that the Crypto peripheral has detected an AHB bus error"]
pub struct AHB_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `RESERVED10` reader - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED10_R(crate::FieldReader<u8, u8>);
impl RESERVED10_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED10` writer - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED10_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `LAST_CH` reader - 9:9\\]
Indicates which channel was serviced last (channel 0 or channel 1) by the AHB master port."]
pub struct LAST_CH_R(crate::FieldReader<bool, bool>);
impl LAST_CH_R {
    pub(crate) fn new(bits: bool) -> Self {
        LAST_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LAST_CH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LAST_CH` writer - 9:9\\]
Indicates which channel was serviced last (channel 0 or channel 1) by the AHB master port."]
pub struct LAST_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_CH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 8:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_R(crate::FieldReader<u16, u16>);
impl RESERVED0_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - 8:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new(((self.bits >> 13) & 0x0007_ffff) as u32)
    }
    #[doc = "Bit 12 - 12:12\\]
A 1 indicates that the Crypto peripheral has detected an AHB bus error"]
    #[inline(always)]
    pub fn ahb_err(&self) -> AHB_ERR_R {
        AHB_ERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Indicates which channel was serviced last (channel 0 or channel 1) by the AHB master port."]
    #[inline(always)]
    pub fn last_ch(&self) -> LAST_CH_R {
        LAST_CH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 0:8 - 8:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&mut self) -> RESERVED13_W {
        RESERVED13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
A 1 indicates that the Crypto peripheral has detected an AHB bus error"]
    #[inline(always)]
    pub fn ahb_err(&mut self) -> AHB_ERR_W {
        AHB_ERR_W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&mut self) -> RESERVED10_W {
        RESERVED10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Indicates which channel was serviced last (channel 0 or channel 1) by the AHB master port."]
    #[inline(always)]
    pub fn last_ch(&mut self) -> LAST_CH_W {
        LAST_CH_W { w: self }
    }
    #[doc = "Bits 0:8 - 8:0\\]
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
#[doc = "DMA Controller Port Error\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaporterr](index.html) module"]
pub struct DMAPORTERR_SPEC;
impl crate::RegisterSpec for DMAPORTERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaporterr::R](R) reader structure"]
impl crate::Readable for DMAPORTERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaporterr::W](W) writer structure"]
impl crate::Writable for DMAPORTERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAPORTERR to value 0"]
impl crate::Resettable for DMAPORTERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
