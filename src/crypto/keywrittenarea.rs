#[doc = "Register `KEYWRITTENAREA` reader"]
pub struct R(crate::R<KEYWRITTENAREA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYWRITTENAREA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYWRITTENAREA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYWRITTENAREA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYWRITTENAREA` writer"]
pub struct W(crate::W<KEYWRITTENAREA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYWRITTENAREA_SPEC>;
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
impl From<crate::W<KEYWRITTENAREA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYWRITTENAREA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED8_R(crate::FieldReader<u32, u32>);
impl RESERVED8_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED8_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "7:7\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA_WRITTEN7_A {
    #[doc = "1: This RAM area is written with valid key information"]
    WRITTEN = 1,
    #[doc = "0: This RAM area is not written with valid key information"]
    NOT_WRITTEN = 0,
}
impl From<RAM_AREA_WRITTEN7_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA_WRITTEN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN7` reader - 7:7\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN7_R(crate::FieldReader<bool, RAM_AREA_WRITTEN7_A>);
impl RAM_AREA_WRITTEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA_WRITTEN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA_WRITTEN7_A {
        match self.bits {
            true => RAM_AREA_WRITTEN7_A::WRITTEN,
            false => RAM_AREA_WRITTEN7_A::NOT_WRITTEN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITTEN`"]
    #[inline(always)]
    pub fn is_written(&self) -> bool {
        **self == RAM_AREA_WRITTEN7_A::WRITTEN
    }
    #[doc = "Checks if the value of the field is `NOT_WRITTEN`"]
    #[inline(always)]
    pub fn is_not_written(&self) -> bool {
        **self == RAM_AREA_WRITTEN7_A::NOT_WRITTEN
    }
}
impl core::ops::Deref for RAM_AREA_WRITTEN7_R {
    type Target = crate::FieldReader<bool, RAM_AREA_WRITTEN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN7` writer - 7:7\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA_WRITTEN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN7_A::WRITTEN)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn not_written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN7_A::NOT_WRITTEN)
    }
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
#[doc = "6:6\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA_WRITTEN6_A {
    #[doc = "1: This RAM area is written with valid key information"]
    WRITTEN = 1,
    #[doc = "0: This RAM area is not written with valid key information"]
    NOT_WRITTEN = 0,
}
impl From<RAM_AREA_WRITTEN6_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA_WRITTEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN6` reader - 6:6\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN6_R(crate::FieldReader<bool, RAM_AREA_WRITTEN6_A>);
impl RAM_AREA_WRITTEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA_WRITTEN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA_WRITTEN6_A {
        match self.bits {
            true => RAM_AREA_WRITTEN6_A::WRITTEN,
            false => RAM_AREA_WRITTEN6_A::NOT_WRITTEN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITTEN`"]
    #[inline(always)]
    pub fn is_written(&self) -> bool {
        **self == RAM_AREA_WRITTEN6_A::WRITTEN
    }
    #[doc = "Checks if the value of the field is `NOT_WRITTEN`"]
    #[inline(always)]
    pub fn is_not_written(&self) -> bool {
        **self == RAM_AREA_WRITTEN6_A::NOT_WRITTEN
    }
}
impl core::ops::Deref for RAM_AREA_WRITTEN6_R {
    type Target = crate::FieldReader<bool, RAM_AREA_WRITTEN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN6` writer - 6:6\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA_WRITTEN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN6_A::WRITTEN)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn not_written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN6_A::NOT_WRITTEN)
    }
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
#[doc = "5:5\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA_WRITTEN5_A {
    #[doc = "1: This RAM area is written with valid key information"]
    WRITTEN = 1,
    #[doc = "0: This RAM area is not written with valid key information"]
    NOT_WRITTEN = 0,
}
impl From<RAM_AREA_WRITTEN5_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA_WRITTEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN5` reader - 5:5\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN5_R(crate::FieldReader<bool, RAM_AREA_WRITTEN5_A>);
impl RAM_AREA_WRITTEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA_WRITTEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA_WRITTEN5_A {
        match self.bits {
            true => RAM_AREA_WRITTEN5_A::WRITTEN,
            false => RAM_AREA_WRITTEN5_A::NOT_WRITTEN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITTEN`"]
    #[inline(always)]
    pub fn is_written(&self) -> bool {
        **self == RAM_AREA_WRITTEN5_A::WRITTEN
    }
    #[doc = "Checks if the value of the field is `NOT_WRITTEN`"]
    #[inline(always)]
    pub fn is_not_written(&self) -> bool {
        **self == RAM_AREA_WRITTEN5_A::NOT_WRITTEN
    }
}
impl core::ops::Deref for RAM_AREA_WRITTEN5_R {
    type Target = crate::FieldReader<bool, RAM_AREA_WRITTEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN5` writer - 5:5\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA_WRITTEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN5_A::WRITTEN)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn not_written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN5_A::NOT_WRITTEN)
    }
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
#[doc = "4:4\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA_WRITTEN4_A {
    #[doc = "1: This RAM area is written with valid key information"]
    WRITTEN = 1,
    #[doc = "0: This RAM area is not written with valid key information"]
    NOT_WRITTEN = 0,
}
impl From<RAM_AREA_WRITTEN4_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA_WRITTEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN4` reader - 4:4\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN4_R(crate::FieldReader<bool, RAM_AREA_WRITTEN4_A>);
impl RAM_AREA_WRITTEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA_WRITTEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA_WRITTEN4_A {
        match self.bits {
            true => RAM_AREA_WRITTEN4_A::WRITTEN,
            false => RAM_AREA_WRITTEN4_A::NOT_WRITTEN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITTEN`"]
    #[inline(always)]
    pub fn is_written(&self) -> bool {
        **self == RAM_AREA_WRITTEN4_A::WRITTEN
    }
    #[doc = "Checks if the value of the field is `NOT_WRITTEN`"]
    #[inline(always)]
    pub fn is_not_written(&self) -> bool {
        **self == RAM_AREA_WRITTEN4_A::NOT_WRITTEN
    }
}
impl core::ops::Deref for RAM_AREA_WRITTEN4_R {
    type Target = crate::FieldReader<bool, RAM_AREA_WRITTEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN4` writer - 4:4\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA_WRITTEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN4_A::WRITTEN)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn not_written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN4_A::NOT_WRITTEN)
    }
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
#[doc = "3:3\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA_WRITTEN3_A {
    #[doc = "1: This RAM area is written with valid key information"]
    WRITTEN = 1,
    #[doc = "0: This RAM area is not written with valid key information"]
    NOT_WRITTEN = 0,
}
impl From<RAM_AREA_WRITTEN3_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA_WRITTEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN3` reader - 3:3\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN3_R(crate::FieldReader<bool, RAM_AREA_WRITTEN3_A>);
impl RAM_AREA_WRITTEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA_WRITTEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA_WRITTEN3_A {
        match self.bits {
            true => RAM_AREA_WRITTEN3_A::WRITTEN,
            false => RAM_AREA_WRITTEN3_A::NOT_WRITTEN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITTEN`"]
    #[inline(always)]
    pub fn is_written(&self) -> bool {
        **self == RAM_AREA_WRITTEN3_A::WRITTEN
    }
    #[doc = "Checks if the value of the field is `NOT_WRITTEN`"]
    #[inline(always)]
    pub fn is_not_written(&self) -> bool {
        **self == RAM_AREA_WRITTEN3_A::NOT_WRITTEN
    }
}
impl core::ops::Deref for RAM_AREA_WRITTEN3_R {
    type Target = crate::FieldReader<bool, RAM_AREA_WRITTEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN3` writer - 3:3\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA_WRITTEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN3_A::WRITTEN)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn not_written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN3_A::NOT_WRITTEN)
    }
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
#[doc = "2:2\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA_WRITTEN2_A {
    #[doc = "1: This RAM area is written with valid key information"]
    WRITTEN = 1,
    #[doc = "0: This RAM area is not written with valid key information"]
    NOT_WRITTEN = 0,
}
impl From<RAM_AREA_WRITTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA_WRITTEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN2` reader - 2:2\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN2_R(crate::FieldReader<bool, RAM_AREA_WRITTEN2_A>);
impl RAM_AREA_WRITTEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA_WRITTEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA_WRITTEN2_A {
        match self.bits {
            true => RAM_AREA_WRITTEN2_A::WRITTEN,
            false => RAM_AREA_WRITTEN2_A::NOT_WRITTEN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITTEN`"]
    #[inline(always)]
    pub fn is_written(&self) -> bool {
        **self == RAM_AREA_WRITTEN2_A::WRITTEN
    }
    #[doc = "Checks if the value of the field is `NOT_WRITTEN`"]
    #[inline(always)]
    pub fn is_not_written(&self) -> bool {
        **self == RAM_AREA_WRITTEN2_A::NOT_WRITTEN
    }
}
impl core::ops::Deref for RAM_AREA_WRITTEN2_R {
    type Target = crate::FieldReader<bool, RAM_AREA_WRITTEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN2` writer - 2:2\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA_WRITTEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN2_A::WRITTEN)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn not_written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN2_A::NOT_WRITTEN)
    }
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
#[doc = "1:1\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA_WRITTEN1_A {
    #[doc = "1: This RAM area is written with valid key information"]
    WRITTEN = 1,
    #[doc = "0: This RAM area is not written with valid key information"]
    NOT_WRITTEN = 0,
}
impl From<RAM_AREA_WRITTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA_WRITTEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN1` reader - 1:1\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN1_R(crate::FieldReader<bool, RAM_AREA_WRITTEN1_A>);
impl RAM_AREA_WRITTEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA_WRITTEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA_WRITTEN1_A {
        match self.bits {
            true => RAM_AREA_WRITTEN1_A::WRITTEN,
            false => RAM_AREA_WRITTEN1_A::NOT_WRITTEN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITTEN`"]
    #[inline(always)]
    pub fn is_written(&self) -> bool {
        **self == RAM_AREA_WRITTEN1_A::WRITTEN
    }
    #[doc = "Checks if the value of the field is `NOT_WRITTEN`"]
    #[inline(always)]
    pub fn is_not_written(&self) -> bool {
        **self == RAM_AREA_WRITTEN1_A::NOT_WRITTEN
    }
}
impl core::ops::Deref for RAM_AREA_WRITTEN1_R {
    type Target = crate::FieldReader<bool, RAM_AREA_WRITTEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN1` writer - 1:1\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA_WRITTEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN1_A::WRITTEN)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn not_written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN1_A::NOT_WRITTEN)
    }
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
#[doc = "0:0\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA_WRITTEN0_A {
    #[doc = "1: This RAM area is written with valid key information"]
    WRITTEN = 1,
    #[doc = "0: This RAM area is not written with valid key information"]
    NOT_WRITTEN = 0,
}
impl From<RAM_AREA_WRITTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA_WRITTEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN0` reader - 0:0\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN0_R(crate::FieldReader<bool, RAM_AREA_WRITTEN0_A>);
impl RAM_AREA_WRITTEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA_WRITTEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA_WRITTEN0_A {
        match self.bits {
            true => RAM_AREA_WRITTEN0_A::WRITTEN,
            false => RAM_AREA_WRITTEN0_A::NOT_WRITTEN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITTEN`"]
    #[inline(always)]
    pub fn is_written(&self) -> bool {
        **self == RAM_AREA_WRITTEN0_A::WRITTEN
    }
    #[doc = "Checks if the value of the field is `NOT_WRITTEN`"]
    #[inline(always)]
    pub fn is_not_written(&self) -> bool {
        **self == RAM_AREA_WRITTEN0_A::NOT_WRITTEN
    }
}
impl core::ops::Deref for RAM_AREA_WRITTEN0_R {
    type Target = crate::FieldReader<bool, RAM_AREA_WRITTEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN0` writer - 0:0\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA_WRITTEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline(always)]
    pub fn written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN0_A::WRITTEN)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline(always)]
    pub fn not_written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN0_A::NOT_WRITTEN)
    }
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
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 7 - 7:7\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written7(&self) -> RAM_AREA_WRITTEN7_R {
        RAM_AREA_WRITTEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written6(&self) -> RAM_AREA_WRITTEN6_R {
        RAM_AREA_WRITTEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written5(&self) -> RAM_AREA_WRITTEN5_R {
        RAM_AREA_WRITTEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written4(&self) -> RAM_AREA_WRITTEN4_R {
        RAM_AREA_WRITTEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written3(&self) -> RAM_AREA_WRITTEN3_R {
        RAM_AREA_WRITTEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written2(&self) -> RAM_AREA_WRITTEN2_R {
        RAM_AREA_WRITTEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written1(&self) -> RAM_AREA_WRITTEN1_R {
        RAM_AREA_WRITTEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written0(&self) -> RAM_AREA_WRITTEN0_R {
        RAM_AREA_WRITTEN0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written7(&mut self) -> RAM_AREA_WRITTEN7_W {
        RAM_AREA_WRITTEN7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written6(&mut self) -> RAM_AREA_WRITTEN6_W {
        RAM_AREA_WRITTEN6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written5(&mut self) -> RAM_AREA_WRITTEN5_W {
        RAM_AREA_WRITTEN5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written4(&mut self) -> RAM_AREA_WRITTEN4_W {
        RAM_AREA_WRITTEN4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written3(&mut self) -> RAM_AREA_WRITTEN3_W {
        RAM_AREA_WRITTEN3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written2(&mut self) -> RAM_AREA_WRITTEN2_W {
        RAM_AREA_WRITTEN2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written1(&mut self) -> RAM_AREA_WRITTEN1_W {
        RAM_AREA_WRITTEN1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written0(&mut self) -> RAM_AREA_WRITTEN0_W {
        RAM_AREA_WRITTEN0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Written Area Status This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and will result in an error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keywrittenarea](index.html) module"]
pub struct KEYWRITTENAREA_SPEC;
impl crate::RegisterSpec for KEYWRITTENAREA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keywrittenarea::R](R) reader structure"]
impl crate::Readable for KEYWRITTENAREA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keywrittenarea::W](W) writer structure"]
impl crate::Writable for KEYWRITTENAREA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYWRITTENAREA to value 0"]
impl crate::Resettable for KEYWRITTENAREA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
