#[doc = "Register `CCFG_PROT_31_0` reader"]
pub struct R(crate::R<CCFG_PROT_31_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_PROT_31_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_PROT_31_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_PROT_31_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_PROT_31_0` writer"]
pub struct W(crate::W<CCFG_PROT_31_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_PROT_31_0_SPEC>;
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
impl From<crate::W<CCFG_PROT_31_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_PROT_31_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRT_PROT_SEC_31` reader - 31:31\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_31_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_31_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_31` writer - 31:31\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_31_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_31_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_30` reader - 30:30\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_30_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_30_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_30` writer - 30:30\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_30_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `WRT_PROT_SEC_29` reader - 29:29\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_29_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_29_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_29` writer - 29:29\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_29_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `WRT_PROT_SEC_28` reader - 28:28\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_28_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_28_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_28` writer - 28:28\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_28_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_28_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_27` reader - 27:27\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_27_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_27_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_27` writer - 27:27\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_27_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_27_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_26` reader - 26:26\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_26_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_26_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_26` writer - 26:26\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_26_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `WRT_PROT_SEC_25` reader - 25:25\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_25_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_25_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_25` writer - 25:25\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_25_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `WRT_PROT_SEC_24` reader - 24:24\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_24_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_24_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_24` writer - 24:24\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_24_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_24_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_23` reader - 23:23\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_23_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_23_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_23` writer - 23:23\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_23_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_23_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_22` reader - 22:22\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_22_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_22_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_22` writer - 22:22\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_22_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_22_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_21` reader - 21:21\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_21_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_21_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_21` writer - 21:21\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_21_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_21_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_20` reader - 20:20\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_20_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_20_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_20` writer - 20:20\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_20_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_20_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_19` reader - 19:19\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_19_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_19_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_19` writer - 19:19\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_19_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_19_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_18` reader - 18:18\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_18_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_18_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_18` writer - 18:18\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_18_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_18_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_17` reader - 17:17\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_17_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_17_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_17` writer - 17:17\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_17_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_17_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_16` reader - 16:16\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_16_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_16_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_16` writer - 16:16\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_16_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_16_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_15` reader - 15:15\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_15_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_15_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_15` writer - 15:15\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_15_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_15_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_14` reader - 14:14\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_14_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_14_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_14` writer - 14:14\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_14_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_14_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_13` reader - 13:13\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_13_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_13_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_13` writer - 13:13\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_13_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_13_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_12` reader - 12:12\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_12_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_12_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_12` writer - 12:12\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_12_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_12_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_11` reader - 11:11\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_11_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_11_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_11` writer - 11:11\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_11_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_11_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_10` reader - 10:10\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_10_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_10_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_10` writer - 10:10\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_10_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_10_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_9` reader - 9:9\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_9_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_9_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_9` writer - 9:9\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_9_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_9_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_8` reader - 8:8\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_8_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_8_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_8` writer - 8:8\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_8_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_8_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_7` reader - 7:7\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_7_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_7` writer - 7:7\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_7_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_7_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_6` reader - 6:6\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_6_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_6` writer - 6:6\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_6_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_6_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_5` reader - 5:5\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_5_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_5` writer - 5:5\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_5_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_5_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_4` reader - 4:4\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_4_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_4` writer - 4:4\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_4_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_4_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_3` reader - 3:3\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_3_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_3` writer - 3:3\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_3_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_3_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_2` reader - 2:2\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_2_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_2` writer - 2:2\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_2_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_2_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_1` reader - 1:1\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_1_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_1` writer - 1:1\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_1_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_0` reader - 0:0\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_0_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_0` writer - 0:0\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_0_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_0_W<'a> {
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
    #[doc = "Bit 31 - 31:31\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_31(&self) -> WRT_PROT_SEC_31_R {
        WRT_PROT_SEC_31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_30(&self) -> WRT_PROT_SEC_30_R {
        WRT_PROT_SEC_30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_29(&self) -> WRT_PROT_SEC_29_R {
        WRT_PROT_SEC_29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_28(&self) -> WRT_PROT_SEC_28_R {
        WRT_PROT_SEC_28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_27(&self) -> WRT_PROT_SEC_27_R {
        WRT_PROT_SEC_27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_26(&self) -> WRT_PROT_SEC_26_R {
        WRT_PROT_SEC_26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_25(&self) -> WRT_PROT_SEC_25_R {
        WRT_PROT_SEC_25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_24(&self) -> WRT_PROT_SEC_24_R {
        WRT_PROT_SEC_24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_23(&self) -> WRT_PROT_SEC_23_R {
        WRT_PROT_SEC_23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_22(&self) -> WRT_PROT_SEC_22_R {
        WRT_PROT_SEC_22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_21(&self) -> WRT_PROT_SEC_21_R {
        WRT_PROT_SEC_21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_20(&self) -> WRT_PROT_SEC_20_R {
        WRT_PROT_SEC_20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_19(&self) -> WRT_PROT_SEC_19_R {
        WRT_PROT_SEC_19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_18(&self) -> WRT_PROT_SEC_18_R {
        WRT_PROT_SEC_18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_17(&self) -> WRT_PROT_SEC_17_R {
        WRT_PROT_SEC_17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_16(&self) -> WRT_PROT_SEC_16_R {
        WRT_PROT_SEC_16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_15(&self) -> WRT_PROT_SEC_15_R {
        WRT_PROT_SEC_15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_14(&self) -> WRT_PROT_SEC_14_R {
        WRT_PROT_SEC_14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_13(&self) -> WRT_PROT_SEC_13_R {
        WRT_PROT_SEC_13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_12(&self) -> WRT_PROT_SEC_12_R {
        WRT_PROT_SEC_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_11(&self) -> WRT_PROT_SEC_11_R {
        WRT_PROT_SEC_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_10(&self) -> WRT_PROT_SEC_10_R {
        WRT_PROT_SEC_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_9(&self) -> WRT_PROT_SEC_9_R {
        WRT_PROT_SEC_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_8(&self) -> WRT_PROT_SEC_8_R {
        WRT_PROT_SEC_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_7(&self) -> WRT_PROT_SEC_7_R {
        WRT_PROT_SEC_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_6(&self) -> WRT_PROT_SEC_6_R {
        WRT_PROT_SEC_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_5(&self) -> WRT_PROT_SEC_5_R {
        WRT_PROT_SEC_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_4(&self) -> WRT_PROT_SEC_4_R {
        WRT_PROT_SEC_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_3(&self) -> WRT_PROT_SEC_3_R {
        WRT_PROT_SEC_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_2(&self) -> WRT_PROT_SEC_2_R {
        WRT_PROT_SEC_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_1(&self) -> WRT_PROT_SEC_1_R {
        WRT_PROT_SEC_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_0(&self) -> WRT_PROT_SEC_0_R {
        WRT_PROT_SEC_0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_31(&mut self) -> WRT_PROT_SEC_31_W {
        WRT_PROT_SEC_31_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_30(&mut self) -> WRT_PROT_SEC_30_W {
        WRT_PROT_SEC_30_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_29(&mut self) -> WRT_PROT_SEC_29_W {
        WRT_PROT_SEC_29_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_28(&mut self) -> WRT_PROT_SEC_28_W {
        WRT_PROT_SEC_28_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_27(&mut self) -> WRT_PROT_SEC_27_W {
        WRT_PROT_SEC_27_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_26(&mut self) -> WRT_PROT_SEC_26_W {
        WRT_PROT_SEC_26_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_25(&mut self) -> WRT_PROT_SEC_25_W {
        WRT_PROT_SEC_25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_24(&mut self) -> WRT_PROT_SEC_24_W {
        WRT_PROT_SEC_24_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_23(&mut self) -> WRT_PROT_SEC_23_W {
        WRT_PROT_SEC_23_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_22(&mut self) -> WRT_PROT_SEC_22_W {
        WRT_PROT_SEC_22_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_21(&mut self) -> WRT_PROT_SEC_21_W {
        WRT_PROT_SEC_21_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_20(&mut self) -> WRT_PROT_SEC_20_W {
        WRT_PROT_SEC_20_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_19(&mut self) -> WRT_PROT_SEC_19_W {
        WRT_PROT_SEC_19_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_18(&mut self) -> WRT_PROT_SEC_18_W {
        WRT_PROT_SEC_18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_17(&mut self) -> WRT_PROT_SEC_17_W {
        WRT_PROT_SEC_17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_16(&mut self) -> WRT_PROT_SEC_16_W {
        WRT_PROT_SEC_16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_15(&mut self) -> WRT_PROT_SEC_15_W {
        WRT_PROT_SEC_15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_14(&mut self) -> WRT_PROT_SEC_14_W {
        WRT_PROT_SEC_14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_13(&mut self) -> WRT_PROT_SEC_13_W {
        WRT_PROT_SEC_13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_12(&mut self) -> WRT_PROT_SEC_12_W {
        WRT_PROT_SEC_12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_11(&mut self) -> WRT_PROT_SEC_11_W {
        WRT_PROT_SEC_11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_10(&mut self) -> WRT_PROT_SEC_10_W {
        WRT_PROT_SEC_10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_9(&mut self) -> WRT_PROT_SEC_9_W {
        WRT_PROT_SEC_9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_8(&mut self) -> WRT_PROT_SEC_8_W {
        WRT_PROT_SEC_8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_7(&mut self) -> WRT_PROT_SEC_7_W {
        WRT_PROT_SEC_7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_6(&mut self) -> WRT_PROT_SEC_6_W {
        WRT_PROT_SEC_6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_5(&mut self) -> WRT_PROT_SEC_5_W {
        WRT_PROT_SEC_5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_4(&mut self) -> WRT_PROT_SEC_4_W {
        WRT_PROT_SEC_4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_3(&mut self) -> WRT_PROT_SEC_3_W {
        WRT_PROT_SEC_3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_2(&mut self) -> WRT_PROT_SEC_2_W {
        WRT_PROT_SEC_2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_1(&mut self) -> WRT_PROT_SEC_1_W {
        WRT_PROT_SEC_1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_0(&mut self) -> WRT_PROT_SEC_0_W {
        WRT_PROT_SEC_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protect Sectors 0-31 Each bit write protects one 4KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_prot_31_0](index.html) module"]
pub struct CCFG_PROT_31_0_SPEC;
impl crate::RegisterSpec for CCFG_PROT_31_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_prot_31_0::R](R) reader structure"]
impl crate::Readable for CCFG_PROT_31_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_prot_31_0::W](W) writer structure"]
impl crate::Writable for CCFG_PROT_31_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCFG_PROT_31_0 to value 0xffff_ffff"]
impl crate::Resettable for CCFG_PROT_31_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
