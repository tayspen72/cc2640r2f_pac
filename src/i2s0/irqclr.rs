#[doc = "Register `IRQCLR` reader"]
pub struct R(crate::R<IRQCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQCLR` writer"]
pub struct W(crate::W<IRQCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQCLR_SPEC>;
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
impl From<crate::W<IRQCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
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
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | ((value as u32 & 0x03ff_ffff) << 6);
        self.w
    }
}
#[doc = "Field `AIF_DMA_IN` reader - 5:5\\]
1: Clears the interrupt of IRQFLAGS.AIF_DMA_IN (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub struct AIF_DMA_IN_R(crate::FieldReader<bool, bool>);
impl AIF_DMA_IN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIF_DMA_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIF_DMA_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIF_DMA_IN` writer - 5:5\\]
1: Clears the interrupt of IRQFLAGS.AIF_DMA_IN (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub struct AIF_DMA_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> AIF_DMA_IN_W<'a> {
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
#[doc = "Field `AIF_DMA_OUT` reader - 4:4\\]
1: Clears the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub struct AIF_DMA_OUT_R(crate::FieldReader<bool, bool>);
impl AIF_DMA_OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIF_DMA_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIF_DMA_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIF_DMA_OUT` writer - 4:4\\]
1: Clears the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub struct AIF_DMA_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> AIF_DMA_OUT_W<'a> {
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
#[doc = "Field `WCLK_TIMEOUT` reader - 3:3\\]
1: Clears the interrupt of IRQFLAGS.WCLK_TIMEOUT (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub struct WCLK_TIMEOUT_R(crate::FieldReader<bool, bool>);
impl WCLK_TIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WCLK_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WCLK_TIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WCLK_TIMEOUT` writer - 3:3\\]
1: Clears the interrupt of IRQFLAGS.WCLK_TIMEOUT (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub struct WCLK_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> WCLK_TIMEOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `BUS_ERR` reader - 2:2\\]
1: Clears the interrupt of IRQFLAGS.BUS_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub struct BUS_ERR_R(crate::FieldReader<bool, bool>);
impl BUS_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUS_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUS_ERR` writer - 2:2\\]
1: Clears the interrupt of IRQFLAGS.BUS_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub struct BUS_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `WCLK_ERR` reader - 1:1\\]
1: Clears the interrupt of IRQFLAGS.WCLK_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub struct WCLK_ERR_R(crate::FieldReader<bool, bool>);
impl WCLK_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        WCLK_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WCLK_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WCLK_ERR` writer - 1:1\\]
1: Clears the interrupt of IRQFLAGS.WCLK_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub struct WCLK_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> WCLK_ERR_W<'a> {
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
#[doc = "Field `PTR_ERR` reader - 0:0\\]
1: Clears the interrupt of IRQFLAGS.PTR_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub struct PTR_ERR_R(crate::FieldReader<bool, bool>);
impl PTR_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTR_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTR_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTR_ERR` writer - 0:0\\]
1: Clears the interrupt of IRQFLAGS.PTR_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
pub struct PTR_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PTR_ERR_W<'a> {
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
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
    #[doc = "Bit 5 - 5:5\\]
1: Clears the interrupt of IRQFLAGS.AIF_DMA_IN (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    pub fn aif_dma_in(&self) -> AIF_DMA_IN_R {
        AIF_DMA_IN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
1: Clears the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    pub fn aif_dma_out(&self) -> AIF_DMA_OUT_R {
        AIF_DMA_OUT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
1: Clears the interrupt of IRQFLAGS.WCLK_TIMEOUT (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    pub fn wclk_timeout(&self) -> WCLK_TIMEOUT_R {
        WCLK_TIMEOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
1: Clears the interrupt of IRQFLAGS.BUS_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    pub fn bus_err(&self) -> BUS_ERR_R {
        BUS_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Clears the interrupt of IRQFLAGS.WCLK_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    pub fn wclk_err(&self) -> WCLK_ERR_R {
        WCLK_ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
1: Clears the interrupt of IRQFLAGS.PTR_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    pub fn ptr_err(&self) -> PTR_ERR_R {
        PTR_ERR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
1: Clears the interrupt of IRQFLAGS.AIF_DMA_IN (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    pub fn aif_dma_in(&mut self) -> AIF_DMA_IN_W {
        AIF_DMA_IN_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
1: Clears the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    pub fn aif_dma_out(&mut self) -> AIF_DMA_OUT_W {
        AIF_DMA_OUT_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
1: Clears the interrupt of IRQFLAGS.WCLK_TIMEOUT (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    pub fn wclk_timeout(&mut self) -> WCLK_TIMEOUT_W {
        WCLK_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
1: Clears the interrupt of IRQFLAGS.BUS_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    pub fn bus_err(&mut self) -> BUS_ERR_W {
        BUS_ERR_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
1: Clears the interrupt of IRQFLAGS.WCLK_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    pub fn wclk_err(&mut self) -> WCLK_ERR_W {
        WCLK_ERR_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
1: Clears the interrupt of IRQFLAGS.PTR_ERR (unless a set criteria was given at the same time in which the clear will be ignored)"]
    #[inline(always)]
    pub fn ptr_err(&mut self) -> PTR_ERR_W {
        PTR_ERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqclr](index.html) module"]
pub struct IRQCLR_SPEC;
impl crate::RegisterSpec for IRQCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqclr::R](R) reader structure"]
impl crate::Readable for IRQCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqclr::W](W) writer structure"]
impl crate::Writable for IRQCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQCLR to value 0"]
impl crate::Resettable for IRQCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
