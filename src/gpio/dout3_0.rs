#[doc = "Register `DOUT3_0` reader"]
pub struct R(crate::R<DOUT3_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUT3_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUT3_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUT3_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUT3_0` writer"]
pub struct W(crate::W<DOUT3_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUT3_0_SPEC>;
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
impl From<crate::W<DOUT3_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUT3_0_SPEC>) -> Self {
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
#[doc = "Field `DIO3` reader - 24:24\\]
Sets the state of the pin that is configured as DIO#3, if the corresponding DOE31_0 bitfield is set."]
pub struct DIO3_R(crate::FieldReader<bool, bool>);
impl DIO3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO3` writer - 24:24\\]
Sets the state of the pin that is configured as DIO#3, if the corresponding DOE31_0 bitfield is set."]
pub struct DIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO3_W<'a> {
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
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED17_R(crate::FieldReader<u8, u8>);
impl RESERVED17_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED17_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED17` writer - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED17_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 17)) | ((value as u32 & 0x7f) << 17);
        self.w
    }
}
#[doc = "Field `DIO2` reader - 16:16\\]
Sets the state of the pin that is configured as DIO#2, if the corresponding DOE31_0 bitfield is set."]
pub struct DIO2_R(crate::FieldReader<bool, bool>);
impl DIO2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO2` writer - 16:16\\]
Sets the state of the pin that is configured as DIO#2, if the corresponding DOE31_0 bitfield is set."]
pub struct DIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED9_R(crate::FieldReader<u8, u8>);
impl RESERVED9_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED9` writer - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | ((value as u32 & 0x7f) << 9);
        self.w
    }
}
#[doc = "Field `DIO1` reader - 8:8\\]
Sets the state of the pin that is configured as DIO#1, if the corresponding DOE31_0 bitfield is set."]
pub struct DIO1_R(crate::FieldReader<bool, bool>);
impl DIO1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO1` writer - 8:8\\]
Sets the state of the pin that is configured as DIO#1, if the corresponding DOE31_0 bitfield is set."]
pub struct DIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO1_W<'a> {
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
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1_R(crate::FieldReader<u8, u8>);
impl RESERVED1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | ((value as u32 & 0x7f) << 1);
        self.w
    }
}
#[doc = "Field `DIO0` reader - 0:0\\]
Sets the state of the pin that is configured as DIO#0, if the corresponding DOE31_0 bitfield is set."]
pub struct DIO0_R(crate::FieldReader<bool, bool>);
impl DIO0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO0` writer - 0:0\\]
Sets the state of the pin that is configured as DIO#0, if the corresponding DOE31_0 bitfield is set."]
pub struct DIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO0_W<'a> {
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
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#3, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub fn dio3(&self) -> DIO3_R {
        DIO3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#2, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub fn dio2(&self) -> DIO2_R {
        DIO2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#1, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub fn dio1(&self) -> DIO1_R {
        DIO1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#0, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub fn dio0(&self) -> DIO0_R {
        DIO0_R::new((self.bits & 0x01) != 0)
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
Sets the state of the pin that is configured as DIO#3, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub fn dio3(&mut self) -> DIO3_W {
        DIO3_W { w: self }
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&mut self) -> RESERVED17_W {
        RESERVED17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#2, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub fn dio2(&mut self) -> DIO2_W {
        DIO2_W { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#1, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub fn dio1(&mut self) -> DIO1_W {
        DIO1_W { w: self }
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#0, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub fn dio0(&mut self) -> DIO0_W {
        DIO0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Out 0 to 3 Alias register for byte access to each bit in DOUT31_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout3_0](index.html) module"]
pub struct DOUT3_0_SPEC;
impl crate::RegisterSpec for DOUT3_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dout3_0::R](R) reader structure"]
impl crate::Readable for DOUT3_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dout3_0::W](W) writer structure"]
impl crate::Writable for DOUT3_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOUT3_0 to value 0"]
impl crate::Resettable for DOUT3_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
