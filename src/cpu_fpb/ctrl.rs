#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED14_R(crate::FieldReader<u32, u32>);
impl RESERVED14_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED14_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED14_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 14)) | ((value as u32 & 0x0003_ffff) << 14);
        self.w
    }
}
#[doc = "Field `NUM_CODE2` reader - 13:12\\]
Number of full banks of code comparators, sixteen comparators per bank. Where less than sixteen code comparators are provided, the bank count is zero, and the number present indicated by NUM_CODE1. This read only field contains 3'b000 to indicate 0 banks for Cortex-M processor."]
pub struct NUM_CODE2_R(crate::FieldReader<u8, u8>);
impl NUM_CODE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUM_CODE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_CODE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUM_CODE2` writer - 13:12\\]
Number of full banks of code comparators, sixteen comparators per bank. Where less than sixteen code comparators are provided, the bank count is zero, and the number present indicated by NUM_CODE1. This read only field contains 3'b000 to indicate 0 banks for Cortex-M processor."]
pub struct NUM_CODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_CODE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `NUM_LIT` reader - 11:8\\]
Number of literal slots field. 0x0: No literal slots 0x2: Two literal slots"]
pub struct NUM_LIT_R(crate::FieldReader<u8, u8>);
impl NUM_LIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUM_LIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_LIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUM_LIT` writer - 11:8\\]
Number of literal slots field. 0x0: No literal slots 0x2: Two literal slots"]
pub struct NUM_LIT_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_LIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `NUM_CODE1` reader - 7:4\\]
Number of code slots field. 0x0: No code slots 0x2: Two code slots 0x6: Six code slots"]
pub struct NUM_CODE1_R(crate::FieldReader<u8, u8>);
impl NUM_CODE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUM_CODE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_CODE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUM_CODE1` writer - 7:4\\]
Number of code slots field. 0x0: No code slots 0x2: Two code slots 0x6: Six code slots"]
pub struct NUM_CODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_CODE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
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
#[doc = "Field `KEY` reader - 1:1\\]
Key field. In order to write to this register, this bit-field must be written to '1'. This bit always reads 0."]
pub struct KEY_R(crate::FieldReader<bool, bool>);
impl KEY_R {
    pub(crate) fn new(bits: bool) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY` writer - 1:1\\]
Key field. In order to write to this register, this bit-field must be written to '1'. This bit always reads 0."]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
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
#[doc = "Field `ENABLE` reader - 0:0\\]
Flash patch unit enable bit 0x0: Flash patch unit disabled 0x1: Flash patch unit enabled"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - 0:0\\]
Flash patch unit enable bit 0x0: Flash patch unit disabled 0x1: Flash patch unit enabled"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new(((self.bits >> 14) & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Number of full banks of code comparators, sixteen comparators per bank. Where less than sixteen code comparators are provided, the bank count is zero, and the number present indicated by NUM_CODE1. This read only field contains 3'b000 to indicate 0 banks for Cortex-M processor."]
    #[inline(always)]
    pub fn num_code2(&self) -> NUM_CODE2_R {
        NUM_CODE2_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of literal slots field. 0x0: No literal slots 0x2: Two literal slots"]
    #[inline(always)]
    pub fn num_lit(&self) -> NUM_LIT_R {
        NUM_LIT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of code slots field. 0x0: No code slots 0x2: Two code slots 0x6: Six code slots"]
    #[inline(always)]
    pub fn num_code1(&self) -> NUM_CODE1_R {
        NUM_CODE1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - 1:1\\]
Key field. In order to write to this register, this bit-field must be written to '1'. This bit always reads 0."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Flash patch unit enable bit 0x0: Flash patch unit disabled 0x1: Flash patch unit enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&mut self) -> RESERVED14_W {
        RESERVED14_W { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\]
Number of full banks of code comparators, sixteen comparators per bank. Where less than sixteen code comparators are provided, the bank count is zero, and the number present indicated by NUM_CODE1. This read only field contains 3'b000 to indicate 0 banks for Cortex-M processor."]
    #[inline(always)]
    pub fn num_code2(&mut self) -> NUM_CODE2_W {
        NUM_CODE2_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of literal slots field. 0x0: No literal slots 0x2: Two literal slots"]
    #[inline(always)]
    pub fn num_lit(&mut self) -> NUM_LIT_W {
        NUM_LIT_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of code slots field. 0x0: No code slots 0x2: Two code slots 0x6: Six code slots"]
    #[inline(always)]
    pub fn num_code1(&mut self) -> NUM_CODE1_W {
        NUM_CODE1_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Key field. In order to write to this register, this bit-field must be written to '1'. This bit always reads 0."]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Flash patch unit enable bit 0x0: Flash patch unit disabled 0x1: Flash patch unit enabled"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control This register is used to enable the flash patch block.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x0260"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0260
    }
}
