#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
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
#[doc = "13:13\\]
Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMABIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<DMABIM_A> for bool {
    #[inline(always)]
    fn from(variant: DMABIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMABIM` reader - 13:13\\]
Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS"]
pub struct DMABIM_R(crate::FieldReader<bool, DMABIM_A>);
impl DMABIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMABIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMABIM_A {
        match self.bits {
            true => DMABIM_A::EN,
            false => DMABIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == DMABIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == DMABIM_A::DIS
    }
}
impl core::ops::Deref for DMABIM_R {
    type Target = crate::FieldReader<bool, DMABIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMABIM` writer - 13:13\\]
Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS"]
pub struct DMABIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMABIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMABIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DMABIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMABIM_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `RESERVED12` reader - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED12_R(crate::FieldReader<bool, bool>);
impl RESERVED12_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED12` writer - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
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
#[doc = "11:11\\]
Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBMIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<TBMIM_A> for bool {
    #[inline(always)]
    fn from(variant: TBMIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBMIM` reader - 11:11\\]
Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS"]
pub struct TBMIM_R(crate::FieldReader<bool, TBMIM_A>);
impl TBMIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBMIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBMIM_A {
        match self.bits {
            true => TBMIM_A::EN,
            false => TBMIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TBMIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TBMIM_A::DIS
    }
}
impl core::ops::Deref for TBMIM_R {
    type Target = crate::FieldReader<bool, TBMIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBMIM` writer - 11:11\\]
Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS"]
pub struct TBMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBMIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TBMIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBMIM_A::DIS)
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
Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBEIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<CBEIM_A> for bool {
    #[inline(always)]
    fn from(variant: CBEIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBEIM` reader - 10:10\\]
Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS"]
pub struct CBEIM_R(crate::FieldReader<bool, CBEIM_A>);
impl CBEIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CBEIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBEIM_A {
        match self.bits {
            true => CBEIM_A::EN,
            false => CBEIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CBEIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CBEIM_A::DIS
    }
}
impl core::ops::Deref for CBEIM_R {
    type Target = crate::FieldReader<bool, CBEIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBEIM` writer - 10:10\\]
Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS"]
pub struct CBEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CBEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBEIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CBEIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CBEIM_A::DIS)
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
Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBMIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<CBMIM_A> for bool {
    #[inline(always)]
    fn from(variant: CBMIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBMIM` reader - 9:9\\]
Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS"]
pub struct CBMIM_R(crate::FieldReader<bool, CBMIM_A>);
impl CBMIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CBMIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBMIM_A {
        match self.bits {
            true => CBMIM_A::EN,
            false => CBMIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CBMIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CBMIM_A::DIS
    }
}
impl core::ops::Deref for CBMIM_R {
    type Target = crate::FieldReader<bool, CBMIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBMIM` writer - 9:9\\]
Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS"]
pub struct CBMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CBMIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBMIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CBMIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CBMIM_A::DIS)
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
Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBTOIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<TBTOIM_A> for bool {
    #[inline(always)]
    fn from(variant: TBTOIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBTOIM` reader - 8:8\\]
Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS"]
pub struct TBTOIM_R(crate::FieldReader<bool, TBTOIM_A>);
impl TBTOIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBTOIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBTOIM_A {
        match self.bits {
            true => TBTOIM_A::EN,
            false => TBTOIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TBTOIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TBTOIM_A::DIS
    }
}
impl core::ops::Deref for TBTOIM_R {
    type Target = crate::FieldReader<bool, TBTOIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBTOIM` writer - 8:8\\]
Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS"]
pub struct TBTOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TBTOIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBTOIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TBTOIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBTOIM_A::DIS)
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
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED6_R(crate::FieldReader<u8, u8>);
impl RESERVED6_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "5:5\\]
Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAAIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<DMAAIM_A> for bool {
    #[inline(always)]
    fn from(variant: DMAAIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAAIM` reader - 5:5\\]
Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS"]
pub struct DMAAIM_R(crate::FieldReader<bool, DMAAIM_A>);
impl DMAAIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAAIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAAIM_A {
        match self.bits {
            true => DMAAIM_A::EN,
            false => DMAAIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == DMAAIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == DMAAIM_A::DIS
    }
}
impl core::ops::Deref for DMAAIM_R {
    type Target = crate::FieldReader<bool, DMAAIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAAIM` writer - 5:5\\]
Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS"]
pub struct DMAAIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAAIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAAIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DMAAIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMAAIM_A::DIS)
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
Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<TAMIM_A> for bool {
    #[inline(always)]
    fn from(variant: TAMIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMIM` reader - 4:4\\]
Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS"]
pub struct TAMIM_R(crate::FieldReader<bool, TAMIM_A>);
impl TAMIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMIM_A {
        match self.bits {
            true => TAMIM_A::EN,
            false => TAMIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TAMIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TAMIM_A::DIS
    }
}
impl core::ops::Deref for TAMIM_R {
    type Target = crate::FieldReader<bool, TAMIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMIM` writer - 4:4\\]
Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS"]
pub struct TAMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TAMIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TAMIM_A::DIS)
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
#[doc = "2:2\\]
Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAEIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<CAEIM_A> for bool {
    #[inline(always)]
    fn from(variant: CAEIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAEIM` reader - 2:2\\]
Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS"]
pub struct CAEIM_R(crate::FieldReader<bool, CAEIM_A>);
impl CAEIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAEIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAEIM_A {
        match self.bits {
            true => CAEIM_A::EN,
            false => CAEIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CAEIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CAEIM_A::DIS
    }
}
impl core::ops::Deref for CAEIM_R {
    type Target = crate::FieldReader<bool, CAEIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAEIM` writer - 2:2\\]
Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS"]
pub struct CAEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAEIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CAEIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CAEIM_A::DIS)
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
Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAMIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<CAMIM_A> for bool {
    #[inline(always)]
    fn from(variant: CAMIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAMIM` reader - 1:1\\]
Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS"]
pub struct CAMIM_R(crate::FieldReader<bool, CAMIM_A>);
impl CAMIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAMIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAMIM_A {
        match self.bits {
            true => CAMIM_A::EN,
            false => CAMIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CAMIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CAMIM_A::DIS
    }
}
impl core::ops::Deref for CAMIM_R {
    type Target = crate::FieldReader<bool, CAMIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAMIM` writer - 1:1\\]
Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS"]
pub struct CAMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAMIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CAMIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CAMIM_A::DIS)
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
Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TATOIM_A {
    #[doc = "1: Enable Interrupt"]
    EN = 1,
    #[doc = "0: Disable Interrupt"]
    DIS = 0,
}
impl From<TATOIM_A> for bool {
    #[inline(always)]
    fn from(variant: TATOIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TATOIM` reader - 0:0\\]
Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS"]
pub struct TATOIM_R(crate::FieldReader<bool, TATOIM_A>);
impl TATOIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TATOIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TATOIM_A {
        match self.bits {
            true => TATOIM_A::EN,
            false => TATOIM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TATOIM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TATOIM_A::DIS
    }
}
impl core::ops::Deref for TATOIM_R {
    type Target = crate::FieldReader<bool, TATOIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TATOIM` writer - 0:0\\]
Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS"]
pub struct TATOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TATOIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TATOIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TATOIM_A::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TATOIM_A::DIS)
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
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new(((self.bits >> 14) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 13 - 13:13\\]
Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS"]
    #[inline(always)]
    pub fn dmabim(&self) -> DMABIM_R {
        DMABIM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS"]
    #[inline(always)]
    pub fn tbmim(&self) -> TBMIM_R {
        TBMIM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS"]
    #[inline(always)]
    pub fn cbeim(&self) -> CBEIM_R {
        CBEIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS"]
    #[inline(always)]
    pub fn cbmim(&self) -> CBMIM_R {
        CBMIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS"]
    #[inline(always)]
    pub fn tbtoim(&self) -> TBTOIM_R {
        TBTOIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS"]
    #[inline(always)]
    pub fn dmaaim(&self) -> DMAAIM_R {
        DMAAIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS"]
    #[inline(always)]
    pub fn tamim(&self) -> TAMIM_R {
        TAMIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS"]
    #[inline(always)]
    pub fn caeim(&self) -> CAEIM_R {
        CAEIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS"]
    #[inline(always)]
    pub fn camim(&self) -> CAMIM_R {
        CAMIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS"]
    #[inline(always)]
    pub fn tatoim(&self) -> TATOIM_R {
        TATOIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&mut self) -> RESERVED14_W {
        RESERVED14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS"]
    #[inline(always)]
    pub fn dmabim(&mut self) -> DMABIM_W {
        DMABIM_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS"]
    #[inline(always)]
    pub fn tbmim(&mut self) -> TBMIM_W {
        TBMIM_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS"]
    #[inline(always)]
    pub fn cbeim(&mut self) -> CBEIM_W {
        CBEIM_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS"]
    #[inline(always)]
    pub fn cbmim(&mut self) -> CBMIM_W {
        CBMIM_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS"]
    #[inline(always)]
    pub fn tbtoim(&mut self) -> TBTOIM_W {
        TBTOIM_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS"]
    #[inline(always)]
    pub fn dmaaim(&mut self) -> DMAAIM_W {
        DMAAIM_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS"]
    #[inline(always)]
    pub fn tamim(&mut self) -> TAMIM_W {
        TAMIM_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS"]
    #[inline(always)]
    pub fn caeim(&mut self) -> CAEIM_W {
        CAEIM_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS"]
    #[inline(always)]
    pub fn camim(&mut self) -> CAMIM_W {
        CAMIM_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS"]
    #[inline(always)]
    pub fn tatoim(&mut self) -> TATOIM_W {
        TATOIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask This register is used to enable the interrupts. Associated registers: RIS, MIS, ICLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
