#[doc = "Register `FSM_ST_MACHINE` reader"]
pub struct R(crate::R<FSM_ST_MACHINE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_ST_MACHINE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_ST_MACHINE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_ST_MACHINE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_ST_MACHINE` writer"]
pub struct W(crate::W<FSM_ST_MACHINE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_ST_MACHINE_SPEC>;
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
impl From<crate::W<FSM_ST_MACHINE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_ST_MACHINE_SPEC>) -> Self {
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
#[doc = "Field `DO_PRECOND` reader - 23:23\\]
Internal. Only to be used through TI provided API."]
pub struct DO_PRECOND_R(crate::FieldReader<bool, bool>);
impl DO_PRECOND_R {
    pub(crate) fn new(bits: bool) -> Self {
        DO_PRECOND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DO_PRECOND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DO_PRECOND` writer - 23:23\\]
Internal. Only to be used through TI provided API."]
pub struct DO_PRECOND_W<'a> {
    w: &'a mut W,
}
impl<'a> DO_PRECOND_W<'a> {
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
#[doc = "Field `FSM_INT_EN` reader - 22:22\\]
Internal. Only to be used through TI provided API."]
pub struct FSM_INT_EN_R(crate::FieldReader<bool, bool>);
impl FSM_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSM_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSM_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSM_INT_EN` writer - 22:22\\]
Internal. Only to be used through TI provided API."]
pub struct FSM_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_INT_EN_W<'a> {
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
#[doc = "Field `ALL_BANKS` reader - 21:21\\]
Internal. Only to be used through TI provided API."]
pub struct ALL_BANKS_R(crate::FieldReader<bool, bool>);
impl ALL_BANKS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALL_BANKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALL_BANKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALL_BANKS` writer - 21:21\\]
Internal. Only to be used through TI provided API."]
pub struct ALL_BANKS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALL_BANKS_W<'a> {
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
#[doc = "Field `CMPV_ALLOWED` reader - 20:20\\]
Internal. Only to be used through TI provided API."]
pub struct CMPV_ALLOWED_R(crate::FieldReader<bool, bool>);
impl CMPV_ALLOWED_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPV_ALLOWED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPV_ALLOWED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPV_ALLOWED` writer - 20:20\\]
Internal. Only to be used through TI provided API."]
pub struct CMPV_ALLOWED_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPV_ALLOWED_W<'a> {
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
#[doc = "Field `RANDOM` reader - 19:19\\]
Internal. Only to be used through TI provided API."]
pub struct RANDOM_R(crate::FieldReader<bool, bool>);
impl RANDOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RANDOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANDOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANDOM` writer - 19:19\\]
Internal. Only to be used through TI provided API."]
pub struct RANDOM_W<'a> {
    w: &'a mut W,
}
impl<'a> RANDOM_W<'a> {
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
#[doc = "Field `RV_SEC_EN` reader - 18:18\\]
Internal. Only to be used through TI provided API."]
pub struct RV_SEC_EN_R(crate::FieldReader<bool, bool>);
impl RV_SEC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RV_SEC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RV_SEC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RV_SEC_EN` writer - 18:18\\]
Internal. Only to be used through TI provided API."]
pub struct RV_SEC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RV_SEC_EN_W<'a> {
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
#[doc = "Field `RV_RES` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub struct RV_RES_R(crate::FieldReader<bool, bool>);
impl RV_RES_R {
    pub(crate) fn new(bits: bool) -> Self {
        RV_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RV_RES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RV_RES` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub struct RV_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> RV_RES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `RV_INT_EN` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub struct RV_INT_EN_R(crate::FieldReader<bool, bool>);
impl RV_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RV_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RV_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RV_INT_EN` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub struct RV_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RV_INT_EN_W<'a> {
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
#[doc = "Field `RESERVED15` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED15_R(crate::FieldReader<bool, bool>);
impl RESERVED15_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED15` writer - 15:15\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED15_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED15_W<'a> {
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
#[doc = "Field `ONE_TIME_GOOD` reader - 14:14\\]
Internal. Only to be used through TI provided API."]
pub struct ONE_TIME_GOOD_R(crate::FieldReader<bool, bool>);
impl ONE_TIME_GOOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONE_TIME_GOOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONE_TIME_GOOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONE_TIME_GOOD` writer - 14:14\\]
Internal. Only to be used through TI provided API."]
pub struct ONE_TIME_GOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ONE_TIME_GOOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `RESERVED12` reader - 13:12\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED12_R(crate::FieldReader<u8, u8>);
impl RESERVED12_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED12` writer - 13:12\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `DO_REDU_COL` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub struct DO_REDU_COL_R(crate::FieldReader<bool, bool>);
impl DO_REDU_COL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DO_REDU_COL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DO_REDU_COL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DO_REDU_COL` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub struct DO_REDU_COL_W<'a> {
    w: &'a mut W,
}
impl<'a> DO_REDU_COL_W<'a> {
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
#[doc = "Field `DBG_SHORT_ROW` reader - 10:7\\]
Internal. Only to be used through TI provided API."]
pub struct DBG_SHORT_ROW_R(crate::FieldReader<u8, u8>);
impl DBG_SHORT_ROW_R {
    pub(crate) fn new(bits: u8) -> Self {
        DBG_SHORT_ROW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_SHORT_ROW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_SHORT_ROW` writer - 10:7\\]
Internal. Only to be used through TI provided API."]
pub struct DBG_SHORT_ROW_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_SHORT_ROW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | ((value as u32 & 0x0f) << 7);
        self.w
    }
}
#[doc = "Field `RESERVED6` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED6_R(crate::FieldReader<bool, bool>);
impl RESERVED6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED6` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
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
#[doc = "Field `PGM_SEC_COF_EN` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub struct PGM_SEC_COF_EN_R(crate::FieldReader<bool, bool>);
impl PGM_SEC_COF_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PGM_SEC_COF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGM_SEC_COF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGM_SEC_COF_EN` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub struct PGM_SEC_COF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_SEC_COF_EN_W<'a> {
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
#[doc = "Field `PREC_STOP_EN` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub struct PREC_STOP_EN_R(crate::FieldReader<bool, bool>);
impl PREC_STOP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PREC_STOP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREC_STOP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREC_STOP_EN` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub struct PREC_STOP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREC_STOP_EN_W<'a> {
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
#[doc = "Field `DIS_TST_EN` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_TST_EN_R(crate::FieldReader<bool, bool>);
impl DIS_TST_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_TST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_TST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_TST_EN` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_TST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_TST_EN_W<'a> {
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
#[doc = "Field `CMD_EN` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub struct CMD_EN_R(crate::FieldReader<bool, bool>);
impl CMD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_EN` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub struct CMD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_EN_W<'a> {
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
#[doc = "Field `INV_DATA` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub struct INV_DATA_R(crate::FieldReader<bool, bool>);
impl INV_DATA_R {
    pub(crate) fn new(bits: bool) -> Self {
        INV_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INV_DATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV_DATA` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub struct INV_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_DATA_W<'a> {
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
#[doc = "Field `OVERRIDE` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct OVERRIDE_R(crate::FieldReader<bool, bool>);
impl OVERRIDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRIDE` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE_W<'a> {
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
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn do_precond(&self) -> DO_PRECOND_R {
        DO_PRECOND_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_int_en(&self) -> FSM_INT_EN_R {
        FSM_INT_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn all_banks(&self) -> ALL_BANKS_R {
        ALL_BANKS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cmpv_allowed(&self) -> CMPV_ALLOWED_R {
        CMPV_ALLOWED_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn random(&self) -> RANDOM_R {
        RANDOM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rv_sec_en(&self) -> RV_SEC_EN_R {
        RV_SEC_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rv_res(&self) -> RV_RES_R {
        RV_RES_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rv_int_en(&self) -> RV_INT_EN_R {
        RV_INT_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn one_time_good(&self) -> ONE_TIME_GOOD_R {
        ONE_TIME_GOOD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn do_redu_col(&self) -> DO_REDU_COL_R {
        DO_REDU_COL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 7:10 - 10:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dbg_short_row(&self) -> DBG_SHORT_ROW_R {
        DBG_SHORT_ROW_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_sec_cof_en(&self) -> PGM_SEC_COF_EN_R {
        PGM_SEC_COF_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn prec_stop_en(&self) -> PREC_STOP_EN_R {
        PREC_STOP_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_tst_en(&self) -> DIS_TST_EN_R {
        DIS_TST_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cmd_en(&self) -> CMD_EN_R {
        CMD_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn inv_data(&self) -> INV_DATA_R {
        INV_DATA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn override_(&self) -> OVERRIDE_R {
        OVERRIDE_R::new((self.bits & 0x01) != 0)
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
    pub fn do_precond(&mut self) -> DO_PRECOND_W {
        DO_PRECOND_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_int_en(&mut self) -> FSM_INT_EN_W {
        FSM_INT_EN_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn all_banks(&mut self) -> ALL_BANKS_W {
        ALL_BANKS_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cmpv_allowed(&mut self) -> CMPV_ALLOWED_W {
        CMPV_ALLOWED_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn random(&mut self) -> RANDOM_W {
        RANDOM_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rv_sec_en(&mut self) -> RV_SEC_EN_W {
        RV_SEC_EN_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rv_res(&mut self) -> RV_RES_W {
        RV_RES_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rv_int_en(&mut self) -> RV_INT_EN_W {
        RV_INT_EN_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved15(&mut self) -> RESERVED15_W {
        RESERVED15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn one_time_good(&mut self) -> ONE_TIME_GOOD_W {
        ONE_TIME_GOOD_W { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn do_redu_col(&mut self) -> DO_REDU_COL_W {
        DO_REDU_COL_W { w: self }
    }
    #[doc = "Bits 7:10 - 10:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dbg_short_row(&mut self) -> DBG_SHORT_ROW_W {
        DBG_SHORT_ROW_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_sec_cof_en(&mut self) -> PGM_SEC_COF_EN_W {
        PGM_SEC_COF_EN_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn prec_stop_en(&mut self) -> PREC_STOP_EN_W {
        PREC_STOP_EN_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_tst_en(&mut self) -> DIS_TST_EN_W {
        DIS_TST_EN_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cmd_en(&mut self) -> CMD_EN_W {
        CMD_EN_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn inv_data(&mut self) -> INV_DATA_W {
        INV_DATA_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn override_(&mut self) -> OVERRIDE_W {
        OVERRIDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_st_machine](index.html) module"]
pub struct FSM_ST_MACHINE_SPEC;
impl crate::RegisterSpec for FSM_ST_MACHINE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_st_machine::R](R) reader structure"]
impl crate::Readable for FSM_ST_MACHINE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_st_machine::W](W) writer structure"]
impl crate::Writable for FSM_ST_MACHINE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_ST_MACHINE to value 0x0080_0500"]
impl crate::Resettable for FSM_ST_MACHINE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0500
    }
}
