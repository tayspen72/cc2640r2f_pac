#[doc = "Register `INT_CAUS` reader"]
pub struct R(crate::R<INT_CAUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_CAUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_CAUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_CAUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_CAUS` writer"]
pub struct W(crate::W<INT_CAUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CAUS_SPEC>;
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
impl From<crate::W<INT_CAUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CAUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
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
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "Field `CAUSE_RESET` reader - 1:1\\]
Indicates that the cause of an interrupt was a reset generated but blocked due to TEST.TEST_EN (only possible when TEST.TEST_EN is set)."]
pub struct CAUSE_RESET_R(crate::FieldReader<bool, bool>);
impl CAUSE_RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAUSE_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAUSE_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAUSE_RESET` writer - 1:1\\]
Indicates that the cause of an interrupt was a reset generated but blocked due to TEST.TEST_EN (only possible when TEST.TEST_EN is set)."]
pub struct CAUSE_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAUSE_RESET_W<'a> {
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
#[doc = "Field `CAUSE_INTR` reader - 0:0\\]
Replica of RIS.WDTRIS"]
pub struct CAUSE_INTR_R(crate::FieldReader<bool, bool>);
impl CAUSE_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAUSE_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAUSE_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAUSE_INTR` writer - 0:0\\]
Replica of RIS.WDTRIS"]
pub struct CAUSE_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAUSE_INTR_W<'a> {
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
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates that the cause of an interrupt was a reset generated but blocked due to TEST.TEST_EN (only possible when TEST.TEST_EN is set)."]
    #[inline(always)]
    pub fn cause_reset(&self) -> CAUSE_RESET_R {
        CAUSE_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Replica of RIS.WDTRIS"]
    #[inline(always)]
    pub fn cause_intr(&self) -> CAUSE_INTR_R {
        CAUSE_INTR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates that the cause of an interrupt was a reset generated but blocked due to TEST.TEST_EN (only possible when TEST.TEST_EN is set)."]
    #[inline(always)]
    pub fn cause_reset(&mut self) -> CAUSE_RESET_W {
        CAUSE_RESET_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Replica of RIS.WDTRIS"]
    #[inline(always)]
    pub fn cause_intr(&mut self) -> CAUSE_INTR_W {
        CAUSE_INTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Cause Test Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_caus](index.html) module"]
pub struct INT_CAUS_SPEC;
impl crate::RegisterSpec for INT_CAUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_caus::R](R) reader structure"]
impl crate::Readable for INT_CAUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_caus::W](W) writer structure"]
impl crate::Writable for INT_CAUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CAUS to value 0"]
impl crate::Resettable for INT_CAUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
