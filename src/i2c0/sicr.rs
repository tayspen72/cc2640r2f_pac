#[doc = "Register `SICR` reader"]
pub struct R(crate::R<SICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SICR` writer"]
pub struct W(crate::W<SICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SICR_SPEC>;
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
impl From<crate::W<SICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_R(crate::FieldReader<u32, u32>);
impl RESERVED3_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Field `STOPIC` reader - 2:2\\]
Stop condition interrupt clear Writing 1 to this bit clears SRIS.STOPRIS and SMIS.STOPMIS."]
pub struct STOPIC_R(crate::FieldReader<bool, bool>);
impl STOPIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOPIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOPIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOPIC` writer - 2:2\\]
Stop condition interrupt clear Writing 1 to this bit clears SRIS.STOPRIS and SMIS.STOPMIS."]
pub struct STOPIC_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPIC_W<'a> {
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
#[doc = "Field `STARTIC` reader - 1:1\\]
Start condition interrupt clear Writing 1 to this bit clears SRIS.STARTRIS SMIS.STARTMIS."]
pub struct STARTIC_R(crate::FieldReader<bool, bool>);
impl STARTIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        STARTIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTIC` writer - 1:1\\]
Start condition interrupt clear Writing 1 to this bit clears SRIS.STARTRIS SMIS.STARTMIS."]
pub struct STARTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTIC_W<'a> {
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
#[doc = "Field `DATAIC` reader - 0:0\\]
Data interrupt clear Writing 1 to this bit clears SRIS.DATARIS SMIS.DATAMIS."]
pub struct DATAIC_R(crate::FieldReader<bool, bool>);
impl DATAIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATAIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAIC` writer - 0:0\\]
Data interrupt clear Writing 1 to this bit clears SRIS.DATARIS SMIS.DATAMIS."]
pub struct DATAIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAIC_W<'a> {
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
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition interrupt clear Writing 1 to this bit clears SRIS.STOPRIS and SMIS.STOPMIS."]
    #[inline(always)]
    pub fn stopic(&self) -> STOPIC_R {
        STOPIC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition interrupt clear Writing 1 to this bit clears SRIS.STARTRIS SMIS.STARTMIS."]
    #[inline(always)]
    pub fn startic(&self) -> STARTIC_R {
        STARTIC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Data interrupt clear Writing 1 to this bit clears SRIS.DATARIS SMIS.DATAMIS."]
    #[inline(always)]
    pub fn dataic(&self) -> DATAIC_R {
        DATAIC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition interrupt clear Writing 1 to this bit clears SRIS.STOPRIS and SMIS.STOPMIS."]
    #[inline(always)]
    pub fn stopic(&mut self) -> STOPIC_W {
        STOPIC_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition interrupt clear Writing 1 to this bit clears SRIS.STARTRIS SMIS.STARTMIS."]
    #[inline(always)]
    pub fn startic(&mut self) -> STARTIC_W {
        STARTIC_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Data interrupt clear Writing 1 to this bit clears SRIS.DATARIS SMIS.DATAMIS."]
    #[inline(always)]
    pub fn dataic(&mut self) -> DATAIC_W {
        DATAIC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Interrupt Clear This register clears the raw interrupt SRIS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sicr](index.html) module"]
pub struct SICR_SPEC;
impl crate::RegisterSpec for SICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sicr::R](R) reader structure"]
impl crate::Readable for SICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sicr::W](W) writer structure"]
impl crate::Writable for SICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SICR to value 0"]
impl crate::Resettable for SICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
