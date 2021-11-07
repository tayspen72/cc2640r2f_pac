#[doc = "Register `DMAEV` reader"]
pub struct R(crate::R<DMAEV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAEV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAEV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAEV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAEV` writer"]
pub struct W(crate::W<DMAEV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAEV_SPEC>;
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
impl From<crate::W<DMAEV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAEV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
pub struct RESERVED12_R(crate::FieldReader<u32, u32>);
impl RESERVED12_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED12_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED12` writer - 31:12\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | ((value as u32 & 0x000f_ffff) << 12);
        self.w
    }
}
#[doc = "Field `TBMDMAEN` reader - 11:11\\]
GPT Timer B Match DMA Trigger Enable"]
pub struct TBMDMAEN_R(crate::FieldReader<bool, bool>);
impl TBMDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBMDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBMDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBMDMAEN` writer - 11:11\\]
GPT Timer B Match DMA Trigger Enable"]
pub struct TBMDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMDMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `CBEDMAEN` reader - 10:10\\]
GPT Timer B Capture Event DMA Trigger Enable"]
pub struct CBEDMAEN_R(crate::FieldReader<bool, bool>);
impl CBEDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CBEDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBEDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBEDMAEN` writer - 10:10\\]
GPT Timer B Capture Event DMA Trigger Enable"]
pub struct CBEDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CBEDMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CBMDMAEN` reader - 9:9\\]
GPT Timer B Capture Match DMA Trigger Enable"]
pub struct CBMDMAEN_R(crate::FieldReader<bool, bool>);
impl CBMDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CBMDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBMDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBMDMAEN` writer - 9:9\\]
GPT Timer B Capture Match DMA Trigger Enable"]
pub struct CBMDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CBMDMAEN_W<'a> {
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
#[doc = "Field `TBTODMAEN` reader - 8:8\\]
GPT Timer B Time-Out DMA Trigger Enable"]
pub struct TBTODMAEN_R(crate::FieldReader<bool, bool>);
impl TBTODMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBTODMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBTODMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBTODMAEN` writer - 8:8\\]
GPT Timer B Time-Out DMA Trigger Enable"]
pub struct TBTODMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBTODMAEN_W<'a> {
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
#[doc = "Field `RESERVED5` reader - 7:5\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
pub struct RESERVED5_R(crate::FieldReader<u8, u8>);
impl RESERVED5_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED5` writer - 7:5\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `TAMDMAEN` reader - 4:4\\]
GPT Timer A Match DMA Trigger Enable"]
pub struct TAMDMAEN_R(crate::FieldReader<bool, bool>);
impl TAMDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMDMAEN` writer - 4:4\\]
GPT Timer A Match DMA Trigger Enable"]
pub struct TAMDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMDMAEN_W<'a> {
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
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_R(crate::FieldReader<bool, bool>);
impl RESERVED3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
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
#[doc = "Field `CAEDMAEN` reader - 2:2\\]
GPT Timer A Capture Event DMA Trigger Enable"]
pub struct CAEDMAEN_R(crate::FieldReader<bool, bool>);
impl CAEDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAEDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAEDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAEDMAEN` writer - 2:2\\]
GPT Timer A Capture Event DMA Trigger Enable"]
pub struct CAEDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEDMAEN_W<'a> {
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
#[doc = "Field `CAMDMAEN` reader - 1:1\\]
GPT Timer A Capture Match DMA Trigger Enable"]
pub struct CAMDMAEN_R(crate::FieldReader<bool, bool>);
impl CAMDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAMDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAMDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAMDMAEN` writer - 1:1\\]
GPT Timer A Capture Match DMA Trigger Enable"]
pub struct CAMDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMDMAEN_W<'a> {
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
#[doc = "Field `TATODMAEN` reader - 0:0\\]
GPT Timer A Time-Out DMA Trigger Enable"]
pub struct TATODMAEN_R(crate::FieldReader<bool, bool>);
impl TATODMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TATODMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TATODMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TATODMAEN` writer - 0:0\\]
GPT Timer A Time-Out DMA Trigger Enable"]
pub struct TATODMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TATODMAEN_W<'a> {
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
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 11 - 11:11\\]
GPT Timer B Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn tbmdmaen(&self) -> TBMDMAEN_R {
        TBMDMAEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
GPT Timer B Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn cbedmaen(&self) -> CBEDMAEN_R {
        CBEDMAEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Capture Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn cbmdmaen(&self) -> CBMDMAEN_R {
        CBMDMAEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Time-Out DMA Trigger Enable"]
    #[inline(always)]
    pub fn tbtodmaen(&self) -> TBTODMAEN_R {
        TBTODMAEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer A Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn tamdmaen(&self) -> TAMDMAEN_R {
        TAMDMAEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer A Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn caedmaen(&self) -> CAEDMAEN_R {
        CAEDMAEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Capture Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn camdmaen(&self) -> CAMDMAEN_R {
        CAMDMAEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
GPT Timer A Time-Out DMA Trigger Enable"]
    #[inline(always)]
    pub fn tatodmaen(&self) -> TATODMAEN_R {
        TATODMAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
GPT Timer B Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn tbmdmaen(&mut self) -> TBMDMAEN_W {
        TBMDMAEN_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
GPT Timer B Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn cbedmaen(&mut self) -> CBEDMAEN_W {
        CBEDMAEN_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Capture Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn cbmdmaen(&mut self) -> CBMDMAEN_W {
        CBMDMAEN_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Time-Out DMA Trigger Enable"]
    #[inline(always)]
    pub fn tbtodmaen(&mut self) -> TBTODMAEN_W {
        TBTODMAEN_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer A Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn tamdmaen(&mut self) -> TAMDMAEN_W {
        TAMDMAEN_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer A Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn caedmaen(&mut self) -> CAEDMAEN_W {
        CAEDMAEN_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Capture Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn camdmaen(&mut self) -> CAMDMAEN_W {
        CAMDMAEN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
GPT Timer A Time-Out DMA Trigger Enable"]
    #[inline(always)]
    pub fn tatodmaen(&mut self) -> TATODMAEN_W {
        TATODMAEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Event This register allows software to enable/disable GPT DMA trigger events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaev](index.html) module"]
pub struct DMAEV_SPEC;
impl crate::RegisterSpec for DMAEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaev::R](R) reader structure"]
impl crate::Readable for DMAEV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaev::W](W) writer structure"]
impl crate::Writable for DMAEV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAEV to value 0"]
impl crate::Resettable for DMAEV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
