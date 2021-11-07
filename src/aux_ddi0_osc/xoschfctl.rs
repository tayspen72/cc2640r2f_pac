#[doc = "Register `XOSCHFCTL` reader"]
pub struct R(crate::R<XOSCHFCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOSCHFCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOSCHFCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOSCHFCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XOSCHFCTL` writer"]
pub struct W(crate::W<XOSCHFCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XOSCHFCTL_SPEC>;
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
impl From<crate::W<XOSCHFCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XOSCHFCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED10` reader - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED10_R(crate::FieldReader<u32, u32>);
impl RESERVED10_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED10_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED10` writer - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED10_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | ((value as u32 & 0x003f_ffff) << 10);
        self.w
    }
}
#[doc = "Field `PEAK_DET_ITRIM` reader - 9:8\\]
Internal. Only to be used through TI provided API."]
pub struct PEAK_DET_ITRIM_R(crate::FieldReader<u8, u8>);
impl PEAK_DET_ITRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PEAK_DET_ITRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEAK_DET_ITRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEAK_DET_ITRIM` writer - 9:8\\]
Internal. Only to be used through TI provided API."]
pub struct PEAK_DET_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAK_DET_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED7_R(crate::FieldReader<bool, bool>);
impl RESERVED7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `BYPASS` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub struct BYPASS_R(crate::FieldReader<bool, bool>);
impl BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RESERVED5` reader - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED5_R(crate::FieldReader<bool, bool>);
impl RESERVED5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED5` writer - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
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
#[doc = "Field `HP_BUF_ITRIM` reader - 4:2\\]
Internal. Only to be used through TI provided API."]
pub struct HP_BUF_ITRIM_R(crate::FieldReader<u8, u8>);
impl HP_BUF_ITRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        HP_BUF_ITRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HP_BUF_ITRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HP_BUF_ITRIM` writer - 4:2\\]
Internal. Only to be used through TI provided API."]
pub struct HP_BUF_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HP_BUF_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `LP_BUF_ITRIM` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct LP_BUF_ITRIM_R(crate::FieldReader<u8, u8>);
impl LP_BUF_ITRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        LP_BUF_ITRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LP_BUF_ITRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LP_BUF_ITRIM` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct LP_BUF_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_BUF_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn peak_det_itrim(&self) -> PEAK_DET_ITRIM_R {
        PEAK_DET_ITRIM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hp_buf_itrim(&self) -> HP_BUF_ITRIM_R {
        HP_BUF_ITRIM_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lp_buf_itrim(&self) -> LP_BUF_ITRIM_R {
        LP_BUF_ITRIM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&mut self) -> RESERVED10_W {
        RESERVED10_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn peak_det_itrim(&mut self) -> PEAK_DET_ITRIM_W {
        PEAK_DET_ITRIM_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bits 2:4 - 4:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hp_buf_itrim(&mut self) -> HP_BUF_ITRIM_W {
        HP_BUF_ITRIM_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lp_buf_itrim(&mut self) -> LP_BUF_ITRIM_W {
        LP_BUF_ITRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XOSCHF Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xoschfctl](index.html) module"]
pub struct XOSCHFCTL_SPEC;
impl crate::RegisterSpec for XOSCHFCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xoschfctl::R](R) reader structure"]
impl crate::Readable for XOSCHFCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xoschfctl::W](W) writer structure"]
impl crate::Writable for XOSCHFCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XOSCHFCTL to value 0"]
impl crate::Resettable for XOSCHFCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
