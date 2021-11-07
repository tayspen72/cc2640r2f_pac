#[doc = "Register `FMSTAT` reader"]
pub struct R(crate::R<FMSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMSTAT` writer"]
pub struct W(crate::W<FMSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMSTAT_SPEC>;
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
impl From<crate::W<FMSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED18` reader - 31:18\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED18_R(crate::FieldReader<u16, u16>);
impl RESERVED18_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED18_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED18` writer - 31:18\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED18_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 18)) | ((value as u32 & 0x3fff) << 18);
        self.w
    }
}
#[doc = "Field `RVSUSP` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub struct RVSUSP_R(crate::FieldReader<bool, bool>);
impl RVSUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RVSUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RVSUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RVSUSP` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub struct RVSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> RVSUSP_W<'a> {
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
#[doc = "Field `RDVER` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub struct RDVER_R(crate::FieldReader<bool, bool>);
impl RDVER_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDVER` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub struct RDVER_W<'a> {
    w: &'a mut W,
}
impl<'a> RDVER_W<'a> {
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
#[doc = "Field `RVF` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub struct RVF_R(crate::FieldReader<bool, bool>);
impl RVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RVF` writer - 15:15\\]
Internal. Only to be used through TI provided API."]
pub struct RVF_W<'a> {
    w: &'a mut W,
}
impl<'a> RVF_W<'a> {
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
#[doc = "Field `ILA` reader - 14:14\\]
Internal. Only to be used through TI provided API."]
pub struct ILA_R(crate::FieldReader<bool, bool>);
impl ILA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ILA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ILA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ILA` writer - 14:14\\]
Internal. Only to be used through TI provided API."]
pub struct ILA_W<'a> {
    w: &'a mut W,
}
impl<'a> ILA_W<'a> {
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
#[doc = "Field `DBF` reader - 13:13\\]
Internal. Only to be used through TI provided API."]
pub struct DBF_R(crate::FieldReader<bool, bool>);
impl DBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBF` writer - 13:13\\]
Internal. Only to be used through TI provided API."]
pub struct DBF_W<'a> {
    w: &'a mut W,
}
impl<'a> DBF_W<'a> {
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
#[doc = "Field `PGV` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub struct PGV_R(crate::FieldReader<bool, bool>);
impl PGV_R {
    pub(crate) fn new(bits: bool) -> Self {
        PGV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGV` writer - 12:12\\]
Internal. Only to be used through TI provided API."]
pub struct PGV_W<'a> {
    w: &'a mut W,
}
impl<'a> PGV_W<'a> {
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
#[doc = "Field `PCV` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub struct PCV_R(crate::FieldReader<bool, bool>);
impl PCV_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCV` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub struct PCV_W<'a> {
    w: &'a mut W,
}
impl<'a> PCV_W<'a> {
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
#[doc = "Field `EV` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub struct EV_R(crate::FieldReader<bool, bool>);
impl EV_R {
    pub(crate) fn new(bits: bool) -> Self {
        EV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EV` writer - 10:10\\]
Internal. Only to be used through TI provided API."]
pub struct EV_W<'a> {
    w: &'a mut W,
}
impl<'a> EV_W<'a> {
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
#[doc = "Field `CV` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub struct CV_R(crate::FieldReader<bool, bool>);
impl CV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CV` writer - 9:9\\]
Internal. Only to be used through TI provided API."]
pub struct CV_W<'a> {
    w: &'a mut W,
}
impl<'a> CV_W<'a> {
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
#[doc = "Field `BUSY` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
#[doc = "Field `ERS` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub struct ERS_R(crate::FieldReader<bool, bool>);
impl ERS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERS` writer - 7:7\\]
Internal. Only to be used through TI provided API."]
pub struct ERS_W<'a> {
    w: &'a mut W,
}
impl<'a> ERS_W<'a> {
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
#[doc = "Field `PGM` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub struct PGM_R(crate::FieldReader<bool, bool>);
impl PGM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PGM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGM` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub struct PGM_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_W<'a> {
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
#[doc = "Field `INVDAT` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub struct INVDAT_R(crate::FieldReader<bool, bool>);
impl INVDAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INVDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVDAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVDAT` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub struct INVDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> INVDAT_W<'a> {
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
#[doc = "Field `CSTAT` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub struct CSTAT_R(crate::FieldReader<bool, bool>);
impl CSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSTAT` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub struct CSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTAT_W<'a> {
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
#[doc = "Field `VOLSTAT` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub struct VOLSTAT_R(crate::FieldReader<bool, bool>);
impl VOLSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        VOLSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VOLSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VOLSTAT` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub struct VOLSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> VOLSTAT_W<'a> {
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
#[doc = "Field `ESUSP` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub struct ESUSP_R(crate::FieldReader<bool, bool>);
impl ESUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESUSP` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub struct ESUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> ESUSP_W<'a> {
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
#[doc = "Field `PSUSP` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub struct PSUSP_R(crate::FieldReader<bool, bool>);
impl PSUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSUSP` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub struct PSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> PSUSP_W<'a> {
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
#[doc = "Field `SLOCK` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct SLOCK_R(crate::FieldReader<bool, bool>);
impl SLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOCK` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct SLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOCK_W<'a> {
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
    #[doc = "Bits 18:31 - 31:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvsusp(&self) -> RVSUSP_R {
        RVSUSP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rdver(&self) -> RDVER_R {
        RDVER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvf(&self) -> RVF_R {
        RVF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ila(&self) -> ILA_R {
        ILA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgv(&self) -> PGV_R {
        PGV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pcv(&self) -> PCV_R {
        PCV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ers(&self) -> ERS_R {
        ERS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm(&self) -> PGM_R {
        PGM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn invdat(&self) -> INVDAT_R {
        INVDAT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cstat(&self) -> CSTAT_R {
        CSTAT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn volstat(&self) -> VOLSTAT_R {
        VOLSTAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn esusp(&self) -> ESUSP_R {
        ESUSP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn psusp(&self) -> PSUSP_R {
        PSUSP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn slock(&self) -> SLOCK_R {
        SLOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 18:31 - 31:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved18(&mut self) -> RESERVED18_W {
        RESERVED18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvsusp(&mut self) -> RVSUSP_W {
        RVSUSP_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rdver(&mut self) -> RDVER_W {
        RDVER_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvf(&mut self) -> RVF_W {
        RVF_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ila(&mut self) -> ILA_W {
        ILA_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dbf(&mut self) -> DBF_W {
        DBF_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgv(&mut self) -> PGV_W {
        PGV_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pcv(&mut self) -> PCV_W {
        PCV_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ev(&mut self) -> EV_W {
        EV_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cv(&mut self) -> CV_W {
        CV_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ers(&mut self) -> ERS_W {
        ERS_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm(&mut self) -> PGM_W {
        PGM_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn invdat(&mut self) -> INVDAT_W {
        INVDAT_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cstat(&mut self) -> CSTAT_W {
        CSTAT_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn volstat(&mut self) -> VOLSTAT_W {
        VOLSTAT_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn esusp(&mut self) -> ESUSP_W {
        ESUSP_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn psusp(&mut self) -> PSUSP_W {
        PSUSP_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn slock(&mut self) -> SLOCK_W {
        SLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmstat](index.html) module"]
pub struct FMSTAT_SPEC;
impl crate::RegisterSpec for FMSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmstat::R](R) reader structure"]
impl crate::Readable for FMSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmstat::W](W) writer structure"]
impl crate::Writable for FMSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMSTAT to value 0"]
impl crate::Resettable for FMSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
