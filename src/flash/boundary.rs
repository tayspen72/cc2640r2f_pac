#[doc = "Register `BOUNDARY` reader"]
pub struct R(crate::R<BOUNDARY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOUNDARY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOUNDARY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOUNDARY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOUNDARY` writer"]
pub struct W(crate::W<BOUNDARY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOUNDARY_SPEC>;
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
impl From<crate::W<BOUNDARY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOUNDARY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED24` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED24_R(crate::FieldReader<u8, u8>);
impl RESERVED24_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED24_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED24` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `DISROW0` reader - 23:23\\]
Internal. Only to be used through TI provided API."]
pub struct DISROW0_R(crate::FieldReader<bool, bool>);
impl DISROW0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISROW0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISROW0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISROW0` writer - 23:23\\]
Internal. Only to be used through TI provided API."]
pub struct DISROW0_W<'a> {
    w: &'a mut W,
}
impl<'a> DISROW0_W<'a> {
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
#[doc = "Field `SPARE` reader - 22:22\\]
Internal. Only to be used through TI provided API."]
pub struct SPARE_R(crate::FieldReader<bool, bool>);
impl SPARE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE` writer - 22:22\\]
Internal. Only to be used through TI provided API."]
pub struct SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `EFC_SELF_TEST_ERROR` reader - 21:21\\]
Internal. Only to be used through TI provided API."]
pub struct EFC_SELF_TEST_ERROR_R(crate::FieldReader<bool, bool>);
impl EFC_SELF_TEST_ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFC_SELF_TEST_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFC_SELF_TEST_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFC_SELF_TEST_ERROR` writer - 21:21\\]
Internal. Only to be used through TI provided API."]
pub struct EFC_SELF_TEST_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFC_SELF_TEST_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `EFC_INSTRUCTION_INFO` reader - 20:20\\]
Internal. Only to be used through TI provided API."]
pub struct EFC_INSTRUCTION_INFO_R(crate::FieldReader<bool, bool>);
impl EFC_INSTRUCTION_INFO_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFC_INSTRUCTION_INFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFC_INSTRUCTION_INFO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFC_INSTRUCTION_INFO` writer - 20:20\\]
Internal. Only to be used through TI provided API."]
pub struct EFC_INSTRUCTION_INFO_W<'a> {
    w: &'a mut W,
}
impl<'a> EFC_INSTRUCTION_INFO_W<'a> {
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
#[doc = "Field `EFC_INSTRUCTION_ERROR` reader - 19:19\\]
Internal. Only to be used through TI provided API."]
pub struct EFC_INSTRUCTION_ERROR_R(crate::FieldReader<bool, bool>);
impl EFC_INSTRUCTION_ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFC_INSTRUCTION_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFC_INSTRUCTION_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFC_INSTRUCTION_ERROR` writer - 19:19\\]
Internal. Only to be used through TI provided API."]
pub struct EFC_INSTRUCTION_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFC_INSTRUCTION_ERROR_W<'a> {
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
#[doc = "Field `EFC_AUTOLOAD_ERROR` reader - 18:18\\]
Internal. Only to be used through TI provided API."]
pub struct EFC_AUTOLOAD_ERROR_R(crate::FieldReader<bool, bool>);
impl EFC_AUTOLOAD_ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFC_AUTOLOAD_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFC_AUTOLOAD_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFC_AUTOLOAD_ERROR` writer - 18:18\\]
Internal. Only to be used through TI provided API."]
pub struct EFC_AUTOLOAD_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFC_AUTOLOAD_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `OUTPUTENABLE` reader - 17:14\\]
Internal. Only to be used through TI provided API."]
pub struct OUTPUTENABLE_R(crate::FieldReader<u8, u8>);
impl OUTPUTENABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        OUTPUTENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTPUTENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTPUTENABLE` writer - 17:14\\]
Internal. Only to be used through TI provided API."]
pub struct OUTPUTENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUTENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | ((value as u32 & 0x0f) << 14);
        self.w
    }
}
#[doc = "Field `SYS_ECC_SELF_TEST_EN` reader - 13:13\\]
Internal. Only to be used through TI provided API."]
pub struct SYS_ECC_SELF_TEST_EN_R(crate::FieldReader<bool, bool>);
impl SYS_ECC_SELF_TEST_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYS_ECC_SELF_TEST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_ECC_SELF_TEST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS_ECC_SELF_TEST_EN` writer - 13:13\\]
Internal. Only to be used through TI provided API."]
pub struct SYS_ECC_SELF_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_ECC_SELF_TEST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `SYS_ECC_OVERRIDE_EN` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub struct SYS_ECC_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl SYS_ECC_OVERRIDE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYS_ECC_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_ECC_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS_ECC_OVERRIDE_EN` writer - 12:12\\]
Internal. Only to be used through TI provided API."]
pub struct SYS_ECC_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_ECC_OVERRIDE_EN_W<'a> {
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
#[doc = "Field `EFC_FDI` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub struct EFC_FDI_R(crate::FieldReader<bool, bool>);
impl EFC_FDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFC_FDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFC_FDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFC_FDI` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub struct EFC_FDI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFC_FDI_W<'a> {
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
#[doc = "Field `SYS_DIEID_AUTOLOAD_EN` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub struct SYS_DIEID_AUTOLOAD_EN_R(crate::FieldReader<bool, bool>);
impl SYS_DIEID_AUTOLOAD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYS_DIEID_AUTOLOAD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_DIEID_AUTOLOAD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS_DIEID_AUTOLOAD_EN` writer - 10:10\\]
Internal. Only to be used through TI provided API."]
pub struct SYS_DIEID_AUTOLOAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_DIEID_AUTOLOAD_EN_W<'a> {
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
#[doc = "Field `SYS_REPAIR_EN` reader - 9:8\\]
Internal. Only to be used through TI provided API."]
pub struct SYS_REPAIR_EN_R(crate::FieldReader<u8, u8>);
impl SYS_REPAIR_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYS_REPAIR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_REPAIR_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS_REPAIR_EN` writer - 9:8\\]
Internal. Only to be used through TI provided API."]
pub struct SYS_REPAIR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_REPAIR_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `SYS_WS_READ_STATES` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub struct SYS_WS_READ_STATES_R(crate::FieldReader<u8, u8>);
impl SYS_WS_READ_STATES_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYS_WS_READ_STATES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_WS_READ_STATES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS_WS_READ_STATES` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub struct SYS_WS_READ_STATES_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_WS_READ_STATES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `INPUTENABLE` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct INPUTENABLE_R(crate::FieldReader<u8, u8>);
impl INPUTENABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        INPUTENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPUTENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPUTENABLE` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct INPUTENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUTENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn disrow0(&self) -> DISROW0_R {
        DISROW0_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_self_test_error(&self) -> EFC_SELF_TEST_ERROR_R {
        EFC_SELF_TEST_ERROR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_instruction_info(&self) -> EFC_INSTRUCTION_INFO_R {
        EFC_INSTRUCTION_INFO_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_instruction_error(&self) -> EFC_INSTRUCTION_ERROR_R {
        EFC_INSTRUCTION_ERROR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_autoload_error(&self) -> EFC_AUTOLOAD_ERROR_R {
        EFC_AUTOLOAD_ERROR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn outputenable(&self) -> OUTPUTENABLE_R {
        OUTPUTENABLE_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ecc_self_test_en(&self) -> SYS_ECC_SELF_TEST_EN_R {
        SYS_ECC_SELF_TEST_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ecc_override_en(&self) -> SYS_ECC_OVERRIDE_EN_R {
        SYS_ECC_OVERRIDE_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_fdi(&self) -> EFC_FDI_R {
        EFC_FDI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_dieid_autoload_en(&self) -> SYS_DIEID_AUTOLOAD_EN_R {
        SYS_DIEID_AUTOLOAD_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_repair_en(&self) -> SYS_REPAIR_EN_R {
        SYS_REPAIR_EN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ws_read_states(&self) -> SYS_WS_READ_STATES_R {
        SYS_WS_READ_STATES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn inputenable(&self) -> INPUTENABLE_R {
        INPUTENABLE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn disrow0(&mut self) -> DISROW0_W {
        DISROW0_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_self_test_error(&mut self) -> EFC_SELF_TEST_ERROR_W {
        EFC_SELF_TEST_ERROR_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_instruction_info(&mut self) -> EFC_INSTRUCTION_INFO_W {
        EFC_INSTRUCTION_INFO_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_instruction_error(&mut self) -> EFC_INSTRUCTION_ERROR_W {
        EFC_INSTRUCTION_ERROR_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_autoload_error(&mut self) -> EFC_AUTOLOAD_ERROR_W {
        EFC_AUTOLOAD_ERROR_W { w: self }
    }
    #[doc = "Bits 14:17 - 17:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn outputenable(&mut self) -> OUTPUTENABLE_W {
        OUTPUTENABLE_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ecc_self_test_en(&mut self) -> SYS_ECC_SELF_TEST_EN_W {
        SYS_ECC_SELF_TEST_EN_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ecc_override_en(&mut self) -> SYS_ECC_OVERRIDE_EN_W {
        SYS_ECC_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efc_fdi(&mut self) -> EFC_FDI_W {
        EFC_FDI_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_dieid_autoload_en(&mut self) -> SYS_DIEID_AUTOLOAD_EN_W {
        SYS_DIEID_AUTOLOAD_EN_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_repair_en(&mut self) -> SYS_REPAIR_EN_W {
        SYS_REPAIR_EN_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sys_ws_read_states(&mut self) -> SYS_WS_READ_STATES_W {
        SYS_WS_READ_STATES_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn inputenable(&mut self) -> INPUTENABLE_W {
        INPUTENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boundary](index.html) module"]
pub struct BOUNDARY_SPEC;
impl crate::RegisterSpec for BOUNDARY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [boundary::R](R) reader structure"]
impl crate::Readable for BOUNDARY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [boundary::W](W) writer structure"]
impl crate::Writable for BOUNDARY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOUNDARY to value 0"]
impl crate::Resettable for BOUNDARY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
