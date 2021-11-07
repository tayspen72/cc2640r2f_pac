#[doc = "Register `FLASH_OTP_DATA4` reader"]
pub struct R(crate::R<FLASH_OTP_DATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_OTP_DATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_OTP_DATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_OTP_DATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_OTP_DATA4` writer"]
pub struct W(crate::W<FLASH_OTP_DATA4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_OTP_DATA4_SPEC>;
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
impl From<crate::W<FLASH_OTP_DATA4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_OTP_DATA4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STANDBY_MODE_SEL_INT_WRT` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub struct STANDBY_MODE_SEL_INT_WRT_R(crate::FieldReader<bool, bool>);
impl STANDBY_MODE_SEL_INT_WRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        STANDBY_MODE_SEL_INT_WRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STANDBY_MODE_SEL_INT_WRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STANDBY_MODE_SEL_INT_WRT` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub struct STANDBY_MODE_SEL_INT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_MODE_SEL_INT_WRT_W<'a> {
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
#[doc = "Field `STANDBY_PW_SEL_INT_WRT` reader - 30:29\\]
Internal. Only to be used through TI provided API."]
pub struct STANDBY_PW_SEL_INT_WRT_R(crate::FieldReader<u8, u8>);
impl STANDBY_PW_SEL_INT_WRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        STANDBY_PW_SEL_INT_WRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STANDBY_PW_SEL_INT_WRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STANDBY_PW_SEL_INT_WRT` writer - 30:29\\]
Internal. Only to be used through TI provided API."]
pub struct STANDBY_PW_SEL_INT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_PW_SEL_INT_WRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
#[doc = "Field `DIS_STANDBY_INT_WRT` reader - 28:28\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_STANDBY_INT_WRT_R(crate::FieldReader<bool, bool>);
impl DIS_STANDBY_INT_WRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_STANDBY_INT_WRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_STANDBY_INT_WRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_STANDBY_INT_WRT` writer - 28:28\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_STANDBY_INT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_STANDBY_INT_WRT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `DIS_IDLE_INT_WRT` reader - 27:27\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_IDLE_INT_WRT_R(crate::FieldReader<bool, bool>);
impl DIS_IDLE_INT_WRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_IDLE_INT_WRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_IDLE_INT_WRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_IDLE_INT_WRT` writer - 27:27\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_IDLE_INT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_IDLE_INT_WRT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `VIN_AT_X_INT_WRT` reader - 26:24\\]
Internal. Only to be used through TI provided API."]
pub struct VIN_AT_X_INT_WRT_R(crate::FieldReader<u8, u8>);
impl VIN_AT_X_INT_WRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        VIN_AT_X_INT_WRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VIN_AT_X_INT_WRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VIN_AT_X_INT_WRT` writer - 26:24\\]
Internal. Only to be used through TI provided API."]
pub struct VIN_AT_X_INT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> VIN_AT_X_INT_WRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `STANDBY_MODE_SEL_EXT_WRT` reader - 23:23\\]
Internal. Only to be used through TI provided API."]
pub struct STANDBY_MODE_SEL_EXT_WRT_R(crate::FieldReader<bool, bool>);
impl STANDBY_MODE_SEL_EXT_WRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        STANDBY_MODE_SEL_EXT_WRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STANDBY_MODE_SEL_EXT_WRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STANDBY_MODE_SEL_EXT_WRT` writer - 23:23\\]
Internal. Only to be used through TI provided API."]
pub struct STANDBY_MODE_SEL_EXT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_MODE_SEL_EXT_WRT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `STANDBY_PW_SEL_EXT_WRT` reader - 22:21\\]
Internal. Only to be used through TI provided API."]
pub struct STANDBY_PW_SEL_EXT_WRT_R(crate::FieldReader<u8, u8>);
impl STANDBY_PW_SEL_EXT_WRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        STANDBY_PW_SEL_EXT_WRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STANDBY_PW_SEL_EXT_WRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STANDBY_PW_SEL_EXT_WRT` writer - 22:21\\]
Internal. Only to be used through TI provided API."]
pub struct STANDBY_PW_SEL_EXT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_PW_SEL_EXT_WRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `DIS_STANDBY_EXT_WRT` reader - 20:20\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_STANDBY_EXT_WRT_R(crate::FieldReader<bool, bool>);
impl DIS_STANDBY_EXT_WRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_STANDBY_EXT_WRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_STANDBY_EXT_WRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_STANDBY_EXT_WRT` writer - 20:20\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_STANDBY_EXT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_STANDBY_EXT_WRT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `DIS_IDLE_EXT_WRT` reader - 19:19\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_IDLE_EXT_WRT_R(crate::FieldReader<bool, bool>);
impl DIS_IDLE_EXT_WRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_IDLE_EXT_WRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_IDLE_EXT_WRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_IDLE_EXT_WRT` writer - 19:19\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_IDLE_EXT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_IDLE_EXT_WRT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `VIN_AT_X_EXT_WRT` reader - 18:16\\]
Internal. Only to be used through TI provided API."]
pub struct VIN_AT_X_EXT_WRT_R(crate::FieldReader<u8, u8>);
impl VIN_AT_X_EXT_WRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        VIN_AT_X_EXT_WRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VIN_AT_X_EXT_WRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VIN_AT_X_EXT_WRT` writer - 18:16\\]
Internal. Only to be used through TI provided API."]
pub struct VIN_AT_X_EXT_WRT_W<'a> {
    w: &'a mut W,
}
impl<'a> VIN_AT_X_EXT_WRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `STANDBY_MODE_SEL_INT_RD` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub struct STANDBY_MODE_SEL_INT_RD_R(crate::FieldReader<bool, bool>);
impl STANDBY_MODE_SEL_INT_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        STANDBY_MODE_SEL_INT_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STANDBY_MODE_SEL_INT_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STANDBY_MODE_SEL_INT_RD` writer - 15:15\\]
Internal. Only to be used through TI provided API."]
pub struct STANDBY_MODE_SEL_INT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_MODE_SEL_INT_RD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `STANDBY_PW_SEL_INT_RD` reader - 14:13\\]
Internal. Only to be used through TI provided API."]
pub struct STANDBY_PW_SEL_INT_RD_R(crate::FieldReader<u8, u8>);
impl STANDBY_PW_SEL_INT_RD_R {
    pub(crate) fn new(bits: u8) -> Self {
        STANDBY_PW_SEL_INT_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STANDBY_PW_SEL_INT_RD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STANDBY_PW_SEL_INT_RD` writer - 14:13\\]
Internal. Only to be used through TI provided API."]
pub struct STANDBY_PW_SEL_INT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_PW_SEL_INT_RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Field `DIS_STANDBY_INT_RD` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_STANDBY_INT_RD_R(crate::FieldReader<bool, bool>);
impl DIS_STANDBY_INT_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_STANDBY_INT_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_STANDBY_INT_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_STANDBY_INT_RD` writer - 12:12\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_STANDBY_INT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_STANDBY_INT_RD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `DIS_IDLE_INT_RD` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_IDLE_INT_RD_R(crate::FieldReader<bool, bool>);
impl DIS_IDLE_INT_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_IDLE_INT_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_IDLE_INT_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_IDLE_INT_RD` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_IDLE_INT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_IDLE_INT_RD_W<'a> {
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
#[doc = "Field `VIN_AT_X_INT_RD` reader - 10:8\\]
Internal. Only to be used through TI provided API."]
pub struct VIN_AT_X_INT_RD_R(crate::FieldReader<u8, u8>);
impl VIN_AT_X_INT_RD_R {
    pub(crate) fn new(bits: u8) -> Self {
        VIN_AT_X_INT_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VIN_AT_X_INT_RD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VIN_AT_X_INT_RD` writer - 10:8\\]
Internal. Only to be used through TI provided API."]
pub struct VIN_AT_X_INT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> VIN_AT_X_INT_RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `STANDBY_MODE_SEL_EXT_RD` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub struct STANDBY_MODE_SEL_EXT_RD_R(crate::FieldReader<bool, bool>);
impl STANDBY_MODE_SEL_EXT_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        STANDBY_MODE_SEL_EXT_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STANDBY_MODE_SEL_EXT_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STANDBY_MODE_SEL_EXT_RD` writer - 7:7\\]
Internal. Only to be used through TI provided API."]
pub struct STANDBY_MODE_SEL_EXT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_MODE_SEL_EXT_RD_W<'a> {
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
#[doc = "Field `STANDBY_PW_SEL_EXT_RD` reader - 6:5\\]
Internal. Only to be used through TI provided API."]
pub struct STANDBY_PW_SEL_EXT_RD_R(crate::FieldReader<u8, u8>);
impl STANDBY_PW_SEL_EXT_RD_R {
    pub(crate) fn new(bits: u8) -> Self {
        STANDBY_PW_SEL_EXT_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STANDBY_PW_SEL_EXT_RD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STANDBY_PW_SEL_EXT_RD` writer - 6:5\\]
Internal. Only to be used through TI provided API."]
pub struct STANDBY_PW_SEL_EXT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_PW_SEL_EXT_RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `DIS_STANDBY_EXT_RD` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_STANDBY_EXT_RD_R(crate::FieldReader<bool, bool>);
impl DIS_STANDBY_EXT_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_STANDBY_EXT_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_STANDBY_EXT_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_STANDBY_EXT_RD` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_STANDBY_EXT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_STANDBY_EXT_RD_W<'a> {
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
#[doc = "Field `DIS_IDLE_EXT_RD` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_IDLE_EXT_RD_R(crate::FieldReader<bool, bool>);
impl DIS_IDLE_EXT_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_IDLE_EXT_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_IDLE_EXT_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_IDLE_EXT_RD` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_IDLE_EXT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_IDLE_EXT_RD_W<'a> {
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
#[doc = "Field `VIN_AT_X_EXT_RD` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub struct VIN_AT_X_EXT_RD_R(crate::FieldReader<u8, u8>);
impl VIN_AT_X_EXT_RD_R {
    pub(crate) fn new(bits: u8) -> Self {
        VIN_AT_X_EXT_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VIN_AT_X_EXT_RD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VIN_AT_X_EXT_RD` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub struct VIN_AT_X_EXT_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> VIN_AT_X_EXT_RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_int_wrt(&self) -> STANDBY_MODE_SEL_INT_WRT_R {
        STANDBY_MODE_SEL_INT_WRT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - 30:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_int_wrt(&self) -> STANDBY_PW_SEL_INT_WRT_R {
        STANDBY_PW_SEL_INT_WRT_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_int_wrt(&self) -> DIS_STANDBY_INT_WRT_R {
        DIS_STANDBY_INT_WRT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_int_wrt(&self) -> DIS_IDLE_INT_WRT_R {
        DIS_IDLE_INT_WRT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_int_wrt(&self) -> VIN_AT_X_INT_WRT_R {
        VIN_AT_X_INT_WRT_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_ext_wrt(&self) -> STANDBY_MODE_SEL_EXT_WRT_R {
        STANDBY_MODE_SEL_EXT_WRT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_ext_wrt(&self) -> STANDBY_PW_SEL_EXT_WRT_R {
        STANDBY_PW_SEL_EXT_WRT_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_ext_wrt(&self) -> DIS_STANDBY_EXT_WRT_R {
        DIS_STANDBY_EXT_WRT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_ext_wrt(&self) -> DIS_IDLE_EXT_WRT_R {
        DIS_IDLE_EXT_WRT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_ext_wrt(&self) -> VIN_AT_X_EXT_WRT_R {
        VIN_AT_X_EXT_WRT_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_int_rd(&self) -> STANDBY_MODE_SEL_INT_RD_R {
        STANDBY_MODE_SEL_INT_RD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_int_rd(&self) -> STANDBY_PW_SEL_INT_RD_R {
        STANDBY_PW_SEL_INT_RD_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_int_rd(&self) -> DIS_STANDBY_INT_RD_R {
        DIS_STANDBY_INT_RD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_int_rd(&self) -> DIS_IDLE_INT_RD_R {
        DIS_IDLE_INT_RD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_int_rd(&self) -> VIN_AT_X_INT_RD_R {
        VIN_AT_X_INT_RD_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_ext_rd(&self) -> STANDBY_MODE_SEL_EXT_RD_R {
        STANDBY_MODE_SEL_EXT_RD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_ext_rd(&self) -> STANDBY_PW_SEL_EXT_RD_R {
        STANDBY_PW_SEL_EXT_RD_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_ext_rd(&self) -> DIS_STANDBY_EXT_RD_R {
        DIS_STANDBY_EXT_RD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_ext_rd(&self) -> DIS_IDLE_EXT_RD_R {
        DIS_IDLE_EXT_RD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_ext_rd(&self) -> VIN_AT_X_EXT_RD_R {
        VIN_AT_X_EXT_RD_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_int_wrt(&mut self) -> STANDBY_MODE_SEL_INT_WRT_W {
        STANDBY_MODE_SEL_INT_WRT_W { w: self }
    }
    #[doc = "Bits 29:30 - 30:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_int_wrt(&mut self) -> STANDBY_PW_SEL_INT_WRT_W {
        STANDBY_PW_SEL_INT_WRT_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_int_wrt(&mut self) -> DIS_STANDBY_INT_WRT_W {
        DIS_STANDBY_INT_WRT_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_int_wrt(&mut self) -> DIS_IDLE_INT_WRT_W {
        DIS_IDLE_INT_WRT_W { w: self }
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_int_wrt(&mut self) -> VIN_AT_X_INT_WRT_W {
        VIN_AT_X_INT_WRT_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_ext_wrt(&mut self) -> STANDBY_MODE_SEL_EXT_WRT_W {
        STANDBY_MODE_SEL_EXT_WRT_W { w: self }
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_ext_wrt(&mut self) -> STANDBY_PW_SEL_EXT_WRT_W {
        STANDBY_PW_SEL_EXT_WRT_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_ext_wrt(&mut self) -> DIS_STANDBY_EXT_WRT_W {
        DIS_STANDBY_EXT_WRT_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_ext_wrt(&mut self) -> DIS_IDLE_EXT_WRT_W {
        DIS_IDLE_EXT_WRT_W { w: self }
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_ext_wrt(&mut self) -> VIN_AT_X_EXT_WRT_W {
        VIN_AT_X_EXT_WRT_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_int_rd(&mut self) -> STANDBY_MODE_SEL_INT_RD_W {
        STANDBY_MODE_SEL_INT_RD_W { w: self }
    }
    #[doc = "Bits 13:14 - 14:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_int_rd(&mut self) -> STANDBY_PW_SEL_INT_RD_W {
        STANDBY_PW_SEL_INT_RD_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_int_rd(&mut self) -> DIS_STANDBY_INT_RD_W {
        DIS_STANDBY_INT_RD_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_int_rd(&mut self) -> DIS_IDLE_INT_RD_W {
        DIS_IDLE_INT_RD_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_int_rd(&mut self) -> VIN_AT_X_INT_RD_W {
        VIN_AT_X_INT_RD_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_mode_sel_ext_rd(&mut self) -> STANDBY_MODE_SEL_EXT_RD_W {
        STANDBY_MODE_SEL_EXT_RD_W { w: self }
    }
    #[doc = "Bits 5:6 - 6:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn standby_pw_sel_ext_rd(&mut self) -> STANDBY_PW_SEL_EXT_RD_W {
        STANDBY_PW_SEL_EXT_RD_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_standby_ext_rd(&mut self) -> DIS_STANDBY_EXT_RD_W {
        DIS_STANDBY_EXT_RD_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_idle_ext_rd(&mut self) -> DIS_IDLE_EXT_RD_W {
        DIS_IDLE_EXT_RD_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x_ext_rd(&mut self) -> VIN_AT_X_EXT_RD_W {
        VIN_AT_X_EXT_RD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_otp_data4](index.html) module"]
pub struct FLASH_OTP_DATA4_SPEC;
impl crate::RegisterSpec for FLASH_OTP_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_otp_data4::R](R) reader structure"]
impl crate::Readable for FLASH_OTP_DATA4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_otp_data4::W](W) writer structure"]
impl crate::Writable for FLASH_OTP_DATA4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_OTP_DATA4 to value 0x9898_9f9f"]
impl crate::Resettable for FLASH_OTP_DATA4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x9898_9f9f
    }
}
