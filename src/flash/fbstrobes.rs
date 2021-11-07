#[doc = "Register `FBSTROBES` reader"]
pub struct R(crate::R<FBSTROBES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBSTROBES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBSTROBES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBSTROBES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBSTROBES` writer"]
pub struct W(crate::W<FBSTROBES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBSTROBES_SPEC>;
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
impl From<crate::W<FBSTROBES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBSTROBES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED25` reader - 31:25\\]
Internal. Only to be used through TI provided API."]
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
Internal. Only to be used through TI provided API."]
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
#[doc = "Field `ECBIT` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub struct ECBIT_R(crate::FieldReader<bool, bool>);
impl ECBIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECBIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECBIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECBIT` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub struct ECBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ECBIT_W<'a> {
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
#[doc = "Field `RESERVED19` reader - 23:19\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED19_R(crate::FieldReader<u8, u8>);
impl RESERVED19_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED19_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED19` writer - 23:19\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED19_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Field `RWAIT2_FLCLK` reader - 18:18\\]
Internal. Only to be used through TI provided API."]
pub struct RWAIT2_FLCLK_R(crate::FieldReader<bool, bool>);
impl RWAIT2_FLCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWAIT2_FLCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWAIT2_FLCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWAIT2_FLCLK` writer - 18:18\\]
Internal. Only to be used through TI provided API."]
pub struct RWAIT2_FLCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> RWAIT2_FLCLK_W<'a> {
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
#[doc = "Field `RWAIT_FLCLK` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub struct RWAIT_FLCLK_R(crate::FieldReader<bool, bool>);
impl RWAIT_FLCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWAIT_FLCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWAIT_FLCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWAIT_FLCLK` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub struct RWAIT_FLCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> RWAIT_FLCLK_W<'a> {
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
#[doc = "Field `FLCLKEN` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub struct FLCLKEN_R(crate::FieldReader<bool, bool>);
impl FLCLKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLCLKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLCLKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLCLKEN` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub struct FLCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLCLKEN_W<'a> {
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
Internal. Only to be used through TI provided API."]
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
Internal. Only to be used through TI provided API."]
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
#[doc = "Field `CTRLENZ` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct CTRLENZ_R(crate::FieldReader<bool, bool>);
impl CTRLENZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTRLENZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLENZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLENZ` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct CTRLENZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLENZ_W<'a> {
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
#[doc = "Field `RESERVED7` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED7_R(crate::FieldReader<bool, bool>);
impl RESERVED7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED7` writer - 7:7\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
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
#[doc = "Field `NOCOLRED` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub struct NOCOLRED_R(crate::FieldReader<bool, bool>);
impl NOCOLRED_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOCOLRED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOCOLRED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOCOLRED` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub struct NOCOLRED_W<'a> {
    w: &'a mut W,
}
impl<'a> NOCOLRED_W<'a> {
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
#[doc = "Field `PRECOL` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub struct PRECOL_R(crate::FieldReader<bool, bool>);
impl PRECOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRECOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRECOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRECOL` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub struct PRECOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRECOL_W<'a> {
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
#[doc = "Field `TI_OTP` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub struct TI_OTP_R(crate::FieldReader<bool, bool>);
impl TI_OTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TI_OTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TI_OTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI_OTP` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub struct TI_OTP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI_OTP_W<'a> {
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
#[doc = "Field `OTP` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub struct OTP_R(crate::FieldReader<bool, bool>);
impl OTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTP` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub struct OTP_W<'a> {
    w: &'a mut W,
}
impl<'a> OTP_W<'a> {
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
#[doc = "Field `TEZ` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub struct TEZ_R(crate::FieldReader<bool, bool>);
impl TEZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEZ` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub struct TEZ_W<'a> {
    w: &'a mut W,
}
impl<'a> TEZ_W<'a> {
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
#[doc = "Field `RESERVED0` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
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
#[doc = "Field `RESERVED0` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ecbit(&self) -> ECBIT_R {
        ECBIT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved19(&self) -> RESERVED19_R {
        RESERVED19_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rwait2_flclk(&self) -> RWAIT2_FLCLK_R {
        RWAIT2_FLCLK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rwait_flclk(&self) -> RWAIT_FLCLK_R {
        RWAIT_FLCLK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flclken(&self) -> FLCLKEN_R {
        FLCLKEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrlenz(&self) -> CTRLENZ_R {
        CTRLENZ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nocolred(&self) -> NOCOLRED_R {
        NOCOLRED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn precol(&self) -> PRECOL_R {
        PRECOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ti_otp(&self) -> TI_OTP_R {
        TI_OTP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn otp(&self) -> OTP_R {
        OTP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tez(&self) -> TEZ_R {
        TEZ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&mut self) -> RESERVED25_W {
        RESERVED25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ecbit(&mut self) -> ECBIT_W {
        ECBIT_W { w: self }
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved19(&mut self) -> RESERVED19_W {
        RESERVED19_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rwait2_flclk(&mut self) -> RWAIT2_FLCLK_W {
        RWAIT2_FLCLK_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rwait_flclk(&mut self) -> RWAIT_FLCLK_W {
        RWAIT_FLCLK_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flclken(&mut self) -> FLCLKEN_W {
        FLCLKEN_W { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrlenz(&mut self) -> CTRLENZ_W {
        CTRLENZ_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nocolred(&mut self) -> NOCOLRED_W {
        NOCOLRED_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn precol(&mut self) -> PRECOL_W {
        PRECOL_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ti_otp(&mut self) -> TI_OTP_W {
        TI_OTP_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn otp(&mut self) -> OTP_W {
        OTP_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tez(&mut self) -> TEZ_W {
        TEZ_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbstrobes](index.html) module"]
pub struct FBSTROBES_SPEC;
impl crate::RegisterSpec for FBSTROBES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbstrobes::R](R) reader structure"]
impl crate::Readable for FBSTROBES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbstrobes::W](W) writer structure"]
impl crate::Writable for FBSTROBES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FBSTROBES to value 0x0104"]
impl crate::Resettable for FBSTROBES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0104
    }
}
