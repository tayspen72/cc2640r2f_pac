#[doc = "Register `DMABUSCFG` reader"]
pub struct R(crate::R<DMABUSCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMABUSCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMABUSCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMABUSCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMABUSCFG` writer"]
pub struct W(crate::W<DMABUSCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMABUSCFG_SPEC>;
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
impl From<crate::W<DMABUSCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMABUSCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_R(crate::FieldReader<u16, u16>);
impl RESERVED16_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED16_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "15:12\\]
Maximum burst size that can be performed on the AHB bus\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AHB_MST1_BURST_SIZE_A {
    #[doc = "6: 64 bytes"]
    _64_BYTE = 6,
    #[doc = "5: 32 bytes"]
    _32_BYTE = 5,
    #[doc = "4: 16 bytes"]
    _16_BYTE = 4,
    #[doc = "3: 8 bytes"]
    _8_BYTE = 3,
    #[doc = "2: 4 bytes"]
    _4_BYTE = 2,
}
impl From<AHB_MST1_BURST_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: AHB_MST1_BURST_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AHB_MST1_BURST_SIZE` reader - 15:12\\]
Maximum burst size that can be performed on the AHB bus"]
pub struct AHB_MST1_BURST_SIZE_R(crate::FieldReader<u8, AHB_MST1_BURST_SIZE_A>);
impl AHB_MST1_BURST_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        AHB_MST1_BURST_SIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AHB_MST1_BURST_SIZE_A> {
        match self.bits {
            6 => Some(AHB_MST1_BURST_SIZE_A::_64_BYTE),
            5 => Some(AHB_MST1_BURST_SIZE_A::_32_BYTE),
            4 => Some(AHB_MST1_BURST_SIZE_A::_16_BYTE),
            3 => Some(AHB_MST1_BURST_SIZE_A::_8_BYTE),
            2 => Some(AHB_MST1_BURST_SIZE_A::_4_BYTE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_64_BYTE`"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        **self == AHB_MST1_BURST_SIZE_A::_64_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        **self == AHB_MST1_BURST_SIZE_A::_32_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        **self == AHB_MST1_BURST_SIZE_A::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        **self == AHB_MST1_BURST_SIZE_A::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_4_BYTE`"]
    #[inline(always)]
    pub fn is_4_byte(&self) -> bool {
        **self == AHB_MST1_BURST_SIZE_A::_4_BYTE
    }
}
impl core::ops::Deref for AHB_MST1_BURST_SIZE_R {
    type Target = crate::FieldReader<u8, AHB_MST1_BURST_SIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_MST1_BURST_SIZE` writer - 15:12\\]
Maximum burst size that can be performed on the AHB bus"]
pub struct AHB_MST1_BURST_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_BURST_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_MST1_BURST_SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZE_A::_64_BYTE)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZE_A::_32_BYTE)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZE_A::_16_BYTE)
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZE_A::_8_BYTE)
    }
    #[doc = "4 bytes"]
    #[inline(always)]
    pub fn _4_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZE_A::_4_BYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "11:11\\]
Idle transfer insertion between consecutive burst transfers on AHB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_MST1_IDLE_EN_A {
    #[doc = "1: Idle transfer insertion enabled"]
    IDLE = 1,
    #[doc = "0: Do not insert idle transfers."]
    NO_IDLE = 0,
}
impl From<AHB_MST1_IDLE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_MST1_IDLE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB_MST1_IDLE_EN` reader - 11:11\\]
Idle transfer insertion between consecutive burst transfers on AHB"]
pub struct AHB_MST1_IDLE_EN_R(crate::FieldReader<bool, AHB_MST1_IDLE_EN_A>);
impl AHB_MST1_IDLE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHB_MST1_IDLE_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_MST1_IDLE_EN_A {
        match self.bits {
            true => AHB_MST1_IDLE_EN_A::IDLE,
            false => AHB_MST1_IDLE_EN_A::NO_IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == AHB_MST1_IDLE_EN_A::IDLE
    }
    #[doc = "Checks if the value of the field is `NO_IDLE`"]
    #[inline(always)]
    pub fn is_no_idle(&self) -> bool {
        **self == AHB_MST1_IDLE_EN_A::NO_IDLE
    }
}
impl core::ops::Deref for AHB_MST1_IDLE_EN_R {
    type Target = crate::FieldReader<bool, AHB_MST1_IDLE_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_MST1_IDLE_EN` writer - 11:11\\]
Idle transfer insertion between consecutive burst transfers on AHB"]
pub struct AHB_MST1_IDLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_IDLE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_MST1_IDLE_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Idle transfer insertion enabled"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(AHB_MST1_IDLE_EN_A::IDLE)
    }
    #[doc = "Do not insert idle transfers."]
    #[inline(always)]
    pub fn no_idle(self) -> &'a mut W {
        self.variant(AHB_MST1_IDLE_EN_A::NO_IDLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "10:10\\]
Burst length type of AHB transfer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_MST1_INCR_EN_A {
    #[doc = "1: Fixed length bursts or single transfers"]
    SPECIFIED = 1,
    #[doc = "0: Unspecified length burst transfers"]
    UNSPECIFIED = 0,
}
impl From<AHB_MST1_INCR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_MST1_INCR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB_MST1_INCR_EN` reader - 10:10\\]
Burst length type of AHB transfer"]
pub struct AHB_MST1_INCR_EN_R(crate::FieldReader<bool, AHB_MST1_INCR_EN_A>);
impl AHB_MST1_INCR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHB_MST1_INCR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_MST1_INCR_EN_A {
        match self.bits {
            true => AHB_MST1_INCR_EN_A::SPECIFIED,
            false => AHB_MST1_INCR_EN_A::UNSPECIFIED,
        }
    }
    #[doc = "Checks if the value of the field is `SPECIFIED`"]
    #[inline(always)]
    pub fn is_specified(&self) -> bool {
        **self == AHB_MST1_INCR_EN_A::SPECIFIED
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        **self == AHB_MST1_INCR_EN_A::UNSPECIFIED
    }
}
impl core::ops::Deref for AHB_MST1_INCR_EN_R {
    type Target = crate::FieldReader<bool, AHB_MST1_INCR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_MST1_INCR_EN` writer - 10:10\\]
Burst length type of AHB transfer"]
pub struct AHB_MST1_INCR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_INCR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_MST1_INCR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fixed length bursts or single transfers"]
    #[inline(always)]
    pub fn specified(self) -> &'a mut W {
        self.variant(AHB_MST1_INCR_EN_A::SPECIFIED)
    }
    #[doc = "Unspecified length burst transfers"]
    #[inline(always)]
    pub fn unspecified(self) -> &'a mut W {
        self.variant(AHB_MST1_INCR_EN_A::UNSPECIFIED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "9:9\\]
Locked transform on AHB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_MST1_LOCK_EN_A {
    #[doc = "1: Transfers are locked"]
    LOCKED = 1,
    #[doc = "0: Transfers are not locked"]
    NOT_LOCKED = 0,
}
impl From<AHB_MST1_LOCK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_MST1_LOCK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB_MST1_LOCK_EN` reader - 9:9\\]
Locked transform on AHB"]
pub struct AHB_MST1_LOCK_EN_R(crate::FieldReader<bool, AHB_MST1_LOCK_EN_A>);
impl AHB_MST1_LOCK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHB_MST1_LOCK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_MST1_LOCK_EN_A {
        match self.bits {
            true => AHB_MST1_LOCK_EN_A::LOCKED,
            false => AHB_MST1_LOCK_EN_A::NOT_LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == AHB_MST1_LOCK_EN_A::LOCKED
    }
    #[doc = "Checks if the value of the field is `NOT_LOCKED`"]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        **self == AHB_MST1_LOCK_EN_A::NOT_LOCKED
    }
}
impl core::ops::Deref for AHB_MST1_LOCK_EN_R {
    type Target = crate::FieldReader<bool, AHB_MST1_LOCK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_MST1_LOCK_EN` writer - 9:9\\]
Locked transform on AHB"]
pub struct AHB_MST1_LOCK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_LOCK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_MST1_LOCK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transfers are locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(AHB_MST1_LOCK_EN_A::LOCKED)
    }
    #[doc = "Transfers are not locked"]
    #[inline(always)]
    pub fn not_locked(self) -> &'a mut W {
        self.variant(AHB_MST1_LOCK_EN_A::NOT_LOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "8:8\\]
Endianess for the AHB master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_MST1_BIGEND_A {
    #[doc = "1: Big Endian"]
    BIG_ENDIAN = 1,
    #[doc = "0: Little Endian"]
    LITTLE_ENDIAN = 0,
}
impl From<AHB_MST1_BIGEND_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_MST1_BIGEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB_MST1_BIGEND` reader - 8:8\\]
Endianess for the AHB master"]
pub struct AHB_MST1_BIGEND_R(crate::FieldReader<bool, AHB_MST1_BIGEND_A>);
impl AHB_MST1_BIGEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHB_MST1_BIGEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_MST1_BIGEND_A {
        match self.bits {
            true => AHB_MST1_BIGEND_A::BIG_ENDIAN,
            false => AHB_MST1_BIGEND_A::LITTLE_ENDIAN,
        }
    }
    #[doc = "Checks if the value of the field is `BIG_ENDIAN`"]
    #[inline(always)]
    pub fn is_big_endian(&self) -> bool {
        **self == AHB_MST1_BIGEND_A::BIG_ENDIAN
    }
    #[doc = "Checks if the value of the field is `LITTLE_ENDIAN`"]
    #[inline(always)]
    pub fn is_little_endian(&self) -> bool {
        **self == AHB_MST1_BIGEND_A::LITTLE_ENDIAN
    }
}
impl core::ops::Deref for AHB_MST1_BIGEND_R {
    type Target = crate::FieldReader<bool, AHB_MST1_BIGEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_MST1_BIGEND` writer - 8:8\\]
Endianess for the AHB master"]
pub struct AHB_MST1_BIGEND_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_BIGEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_MST1_BIGEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Big Endian"]
    #[inline(always)]
    pub fn big_endian(self) -> &'a mut W {
        self.variant(AHB_MST1_BIGEND_A::BIG_ENDIAN)
    }
    #[doc = "Little Endian"]
    #[inline(always)]
    pub fn little_endian(self) -> &'a mut W {
        self.variant(AHB_MST1_BIGEND_A::LITTLE_ENDIAN)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_R(crate::FieldReader<u8, u8>);
impl RESERVED0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Maximum burst size that can be performed on the AHB bus"]
    #[inline(always)]
    pub fn ahb_mst1_burst_size(&self) -> AHB_MST1_BURST_SIZE_R {
        AHB_MST1_BURST_SIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Idle transfer insertion between consecutive burst transfers on AHB"]
    #[inline(always)]
    pub fn ahb_mst1_idle_en(&self) -> AHB_MST1_IDLE_EN_R {
        AHB_MST1_IDLE_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Burst length type of AHB transfer"]
    #[inline(always)]
    pub fn ahb_mst1_incr_en(&self) -> AHB_MST1_INCR_EN_R {
        AHB_MST1_INCR_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Locked transform on AHB"]
    #[inline(always)]
    pub fn ahb_mst1_lock_en(&self) -> AHB_MST1_LOCK_EN_R {
        AHB_MST1_LOCK_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Endianess for the AHB master"]
    #[inline(always)]
    pub fn ahb_mst1_bigend(&self) -> AHB_MST1_BIGEND_R {
        AHB_MST1_BIGEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Maximum burst size that can be performed on the AHB bus"]
    #[inline(always)]
    pub fn ahb_mst1_burst_size(&mut self) -> AHB_MST1_BURST_SIZE_W {
        AHB_MST1_BURST_SIZE_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Idle transfer insertion between consecutive burst transfers on AHB"]
    #[inline(always)]
    pub fn ahb_mst1_idle_en(&mut self) -> AHB_MST1_IDLE_EN_W {
        AHB_MST1_IDLE_EN_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Burst length type of AHB transfer"]
    #[inline(always)]
    pub fn ahb_mst1_incr_en(&mut self) -> AHB_MST1_INCR_EN_W {
        AHB_MST1_INCR_EN_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Locked transform on AHB"]
    #[inline(always)]
    pub fn ahb_mst1_lock_en(&mut self) -> AHB_MST1_LOCK_EN_W {
        AHB_MST1_LOCK_EN_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Endianess for the AHB master"]
    #[inline(always)]
    pub fn ahb_mst1_bigend(&mut self) -> AHB_MST1_BIGEND_W {
        AHB_MST1_BIGEND_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
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
#[doc = "DMA Controller Master Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmabuscfg](index.html) module"]
pub struct DMABUSCFG_SPEC;
impl crate::RegisterSpec for DMABUSCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmabuscfg::R](R) reader structure"]
impl crate::Readable for DMABUSCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmabuscfg::W](W) writer structure"]
impl crate::Writable for DMABUSCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMABUSCFG to value 0x2400"]
impl crate::Resettable for DMABUSCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2400
    }
}
