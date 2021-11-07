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
#[doc = "Field `DMA_BUS_ERR` reader - 31:31\\]
If 1 is written to this bit, IRQSTAT.DMA_BUS_ERR is cleared."]
pub struct DMA_BUS_ERR_R(crate::FieldReader<bool, bool>);
impl DMA_BUS_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_BUS_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_BUS_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_BUS_ERR` writer - 31:31\\]
If 1 is written to this bit, IRQSTAT.DMA_BUS_ERR is cleared."]
pub struct DMA_BUS_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BUS_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `KEY_ST_WR_ERR` reader - 30:30\\]
If 1 is written to this bit, IRQSTAT.KEY_ST_WR_ERR is cleared."]
pub struct KEY_ST_WR_ERR_R(crate::FieldReader<bool, bool>);
impl KEY_ST_WR_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        KEY_ST_WR_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_ST_WR_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_ST_WR_ERR` writer - 30:30\\]
If 1 is written to this bit, IRQSTAT.KEY_ST_WR_ERR is cleared."]
pub struct KEY_ST_WR_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_ST_WR_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `KEY_ST_RD_ERR` reader - 29:29\\]
If 1 is written to this bit, IRQSTAT.KEY_ST_RD_ERR is cleared."]
pub struct KEY_ST_RD_ERR_R(crate::FieldReader<bool, bool>);
impl KEY_ST_RD_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        KEY_ST_RD_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_ST_RD_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_ST_RD_ERR` writer - 29:29\\]
If 1 is written to this bit, IRQSTAT.KEY_ST_RD_ERR is cleared."]
pub struct KEY_ST_RD_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_ST_RD_ERR_W<'a> {
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
#[doc = "Field `RESERVED2` reader - 28:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_R(crate::FieldReader<u32, u32>);
impl RESERVED2_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED2` writer - 28:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 2)) | ((value as u32 & 0x07ff_ffff) << 2);
        self.w
    }
}
#[doc = "Field `DMA_IN_DONE` reader - 1:1\\]
If 1 is written to this bit, IRQSTAT.DMA_IN_DONE is cleared."]
pub struct DMA_IN_DONE_R(crate::FieldReader<bool, bool>);
impl DMA_IN_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_IN_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_IN_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_IN_DONE` writer - 1:1\\]
If 1 is written to this bit, IRQSTAT.DMA_IN_DONE is cleared."]
pub struct DMA_IN_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_DONE_W<'a> {
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
#[doc = "Field `RESULT_AVAIL` reader - 0:0\\]
If 1 is written to this bit, IRQSTAT.RESULT_AVAIL is cleared."]
pub struct RESULT_AVAIL_R(crate::FieldReader<bool, bool>);
impl RESULT_AVAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESULT_AVAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESULT_AVAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESULT_AVAIL` writer - 0:0\\]
If 1 is written to this bit, IRQSTAT.RESULT_AVAIL is cleared."]
pub struct RESULT_AVAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> RESULT_AVAIL_W<'a> {
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
    #[doc = "Bit 31 - 31:31\\]
If 1 is written to this bit, IRQSTAT.DMA_BUS_ERR is cleared."]
    #[inline(always)]
    pub fn dma_bus_err(&self) -> DMA_BUS_ERR_R {
        DMA_BUS_ERR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
If 1 is written to this bit, IRQSTAT.KEY_ST_WR_ERR is cleared."]
    #[inline(always)]
    pub fn key_st_wr_err(&self) -> KEY_ST_WR_ERR_R {
        KEY_ST_WR_ERR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
If 1 is written to this bit, IRQSTAT.KEY_ST_RD_ERR is cleared."]
    #[inline(always)]
    pub fn key_st_rd_err(&self) -> KEY_ST_RD_ERR_R {
        KEY_ST_RD_ERR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 2:28 - 28:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x07ff_ffff) as u32)
    }
    #[doc = "Bit 1 - 1:1\\]
If 1 is written to this bit, IRQSTAT.DMA_IN_DONE is cleared."]
    #[inline(always)]
    pub fn dma_in_done(&self) -> DMA_IN_DONE_R {
        DMA_IN_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
If 1 is written to this bit, IRQSTAT.RESULT_AVAIL is cleared."]
    #[inline(always)]
    pub fn result_avail(&self) -> RESULT_AVAIL_R {
        RESULT_AVAIL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
If 1 is written to this bit, IRQSTAT.DMA_BUS_ERR is cleared."]
    #[inline(always)]
    pub fn dma_bus_err(&mut self) -> DMA_BUS_ERR_W {
        DMA_BUS_ERR_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
If 1 is written to this bit, IRQSTAT.KEY_ST_WR_ERR is cleared."]
    #[inline(always)]
    pub fn key_st_wr_err(&mut self) -> KEY_ST_WR_ERR_W {
        KEY_ST_WR_ERR_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
If 1 is written to this bit, IRQSTAT.KEY_ST_RD_ERR is cleared."]
    #[inline(always)]
    pub fn key_st_rd_err(&mut self) -> KEY_ST_RD_ERR_W {
        KEY_ST_RD_ERR_W { w: self }
    }
    #[doc = "Bits 2:28 - 28:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
If 1 is written to this bit, IRQSTAT.DMA_IN_DONE is cleared."]
    #[inline(always)]
    pub fn dma_in_done(&mut self) -> DMA_IN_DONE_W {
        DMA_IN_DONE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
If 1 is written to this bit, IRQSTAT.RESULT_AVAIL is cleared."]
    #[inline(always)]
    pub fn result_avail(&mut self) -> RESULT_AVAIL_W {
        RESULT_AVAIL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqclr](index.html) module"]
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
