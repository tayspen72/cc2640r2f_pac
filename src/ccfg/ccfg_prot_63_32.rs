#[doc = "Register `CCFG_PROT_63_32` reader"]
pub struct R(crate::R<CCFG_PROT_63_32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_PROT_63_32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_PROT_63_32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_PROT_63_32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_PROT_63_32` writer"]
pub struct W(crate::W<CCFG_PROT_63_32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_PROT_63_32_SPEC>;
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
impl From<crate::W<CCFG_PROT_63_32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_PROT_63_32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRT_PROT_SEC_63` reader - 31:31\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_63_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_63_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_63_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_63_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_63` writer - 31:31\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_63_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_63_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_62` reader - 30:30\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_62_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_62_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_62_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_62_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_62` writer - 30:30\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_62_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_62_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_61` reader - 29:29\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_61_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_61_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_61_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_61_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_61` writer - 29:29\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_61_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_61_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_60` reader - 28:28\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_60_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_60_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_60_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_60_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_60` writer - 28:28\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_60_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_60_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_59` reader - 27:27\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_59_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_59_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_59_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_59_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_59` writer - 27:27\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_59_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_59_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_58` reader - 26:26\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_58_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_58_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_58_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_58_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_58` writer - 26:26\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_58_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_58_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_57` reader - 25:25\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_57_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_57_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_57_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_57_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_57` writer - 25:25\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_57_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_57_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_56` reader - 24:24\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_56_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_56_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_56_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_56_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_56` writer - 24:24\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_56_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_56_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_55` reader - 23:23\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_55_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_55_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_55_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_55_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_55` writer - 23:23\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_55_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_55_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_54` reader - 22:22\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_54_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_54_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_54_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_54_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_54` writer - 22:22\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_54_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_54_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_53` reader - 21:21\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_53_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_53_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_53_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_53_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_53` writer - 21:21\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_53_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_53_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_52` reader - 20:20\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_52_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_52_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_52_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_52_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_52` writer - 20:20\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_52_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_52_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_51` reader - 19:19\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_51_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_51_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_51_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_51_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_51` writer - 19:19\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_51_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_51_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_50` reader - 18:18\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_50_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_50_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_50_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_50_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_50` writer - 18:18\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_50_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_50_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_49` reader - 17:17\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_49_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_49_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_49_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_49_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_49` writer - 17:17\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_49_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_49_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_48` reader - 16:16\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_48_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_48_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_48_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_48_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_48` writer - 16:16\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_48_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_48_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_47` reader - 15:15\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_47_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_47_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_47_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_47_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_47` writer - 15:15\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_47_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_47_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_46` reader - 14:14\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_46_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_46_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_46_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_46_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_46` writer - 14:14\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_46_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_46_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_45` reader - 13:13\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_45_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_45_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_45_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_45_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_45` writer - 13:13\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_45_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_45_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_44` reader - 12:12\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_44_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_44_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_44_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_44_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_44` writer - 12:12\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_44_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_44_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_43` reader - 11:11\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_43_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_43_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_43_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_43_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_43` writer - 11:11\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_43_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_43_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_42` reader - 10:10\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_42_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_42_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_42_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_42_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_42` writer - 10:10\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_42_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_42_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_41` reader - 9:9\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_41_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_41_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_41_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_41_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_41` writer - 9:9\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_41_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_41_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_40` reader - 8:8\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_40_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_40_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_40_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_40_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_40` writer - 8:8\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_40_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_40_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_39` reader - 7:7\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_39_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_39_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_39_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_39_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_39` writer - 7:7\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_39_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_39_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_38` reader - 6:6\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_38_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_38_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_38_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_38_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_38` writer - 6:6\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_38_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_38_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_37` reader - 5:5\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_37_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_37_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_37_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_37_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_37` writer - 5:5\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_37_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_37_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_36` reader - 4:4\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_36_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_36_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_36_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_36_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_36` writer - 4:4\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_36_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_36_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_35` reader - 3:3\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_35_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_35_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_35_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_35` writer - 3:3\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_35_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_35_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_34` reader - 2:2\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_34_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_34_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_34_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_34` writer - 2:2\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_34_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_34_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_33` reader - 1:1\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_33_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_33_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_33_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_33` writer - 1:1\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_33_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_33_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_32` reader - 0:0\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_32_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_32_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_32` writer - 0:0\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_32_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_32_W<'a> {
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
    pub fn wrt_prot_sec_63(&self) -> WRT_PROT_SEC_63_R {
        WRT_PROT_SEC_63_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_62(&self) -> WRT_PROT_SEC_62_R {
        WRT_PROT_SEC_62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_61(&self) -> WRT_PROT_SEC_61_R {
        WRT_PROT_SEC_61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_60(&self) -> WRT_PROT_SEC_60_R {
        WRT_PROT_SEC_60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_59(&self) -> WRT_PROT_SEC_59_R {
        WRT_PROT_SEC_59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_58(&self) -> WRT_PROT_SEC_58_R {
        WRT_PROT_SEC_58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_57(&self) -> WRT_PROT_SEC_57_R {
        WRT_PROT_SEC_57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_56(&self) -> WRT_PROT_SEC_56_R {
        WRT_PROT_SEC_56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_55(&self) -> WRT_PROT_SEC_55_R {
        WRT_PROT_SEC_55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_54(&self) -> WRT_PROT_SEC_54_R {
        WRT_PROT_SEC_54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_53(&self) -> WRT_PROT_SEC_53_R {
        WRT_PROT_SEC_53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_52(&self) -> WRT_PROT_SEC_52_R {
        WRT_PROT_SEC_52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_51(&self) -> WRT_PROT_SEC_51_R {
        WRT_PROT_SEC_51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_50(&self) -> WRT_PROT_SEC_50_R {
        WRT_PROT_SEC_50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_49(&self) -> WRT_PROT_SEC_49_R {
        WRT_PROT_SEC_49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_48(&self) -> WRT_PROT_SEC_48_R {
        WRT_PROT_SEC_48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_47(&self) -> WRT_PROT_SEC_47_R {
        WRT_PROT_SEC_47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_46(&self) -> WRT_PROT_SEC_46_R {
        WRT_PROT_SEC_46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_45(&self) -> WRT_PROT_SEC_45_R {
        WRT_PROT_SEC_45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_44(&self) -> WRT_PROT_SEC_44_R {
        WRT_PROT_SEC_44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_43(&self) -> WRT_PROT_SEC_43_R {
        WRT_PROT_SEC_43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_42(&self) -> WRT_PROT_SEC_42_R {
        WRT_PROT_SEC_42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_41(&self) -> WRT_PROT_SEC_41_R {
        WRT_PROT_SEC_41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_40(&self) -> WRT_PROT_SEC_40_R {
        WRT_PROT_SEC_40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_39(&self) -> WRT_PROT_SEC_39_R {
        WRT_PROT_SEC_39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_38(&self) -> WRT_PROT_SEC_38_R {
        WRT_PROT_SEC_38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_37(&self) -> WRT_PROT_SEC_37_R {
        WRT_PROT_SEC_37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_36(&self) -> WRT_PROT_SEC_36_R {
        WRT_PROT_SEC_36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_35(&self) -> WRT_PROT_SEC_35_R {
        WRT_PROT_SEC_35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_34(&self) -> WRT_PROT_SEC_34_R {
        WRT_PROT_SEC_34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_33(&self) -> WRT_PROT_SEC_33_R {
        WRT_PROT_SEC_33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_32(&self) -> WRT_PROT_SEC_32_R {
        WRT_PROT_SEC_32_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_63(&mut self) -> WRT_PROT_SEC_63_W {
        WRT_PROT_SEC_63_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_62(&mut self) -> WRT_PROT_SEC_62_W {
        WRT_PROT_SEC_62_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_61(&mut self) -> WRT_PROT_SEC_61_W {
        WRT_PROT_SEC_61_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_60(&mut self) -> WRT_PROT_SEC_60_W {
        WRT_PROT_SEC_60_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_59(&mut self) -> WRT_PROT_SEC_59_W {
        WRT_PROT_SEC_59_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_58(&mut self) -> WRT_PROT_SEC_58_W {
        WRT_PROT_SEC_58_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_57(&mut self) -> WRT_PROT_SEC_57_W {
        WRT_PROT_SEC_57_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_56(&mut self) -> WRT_PROT_SEC_56_W {
        WRT_PROT_SEC_56_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_55(&mut self) -> WRT_PROT_SEC_55_W {
        WRT_PROT_SEC_55_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_54(&mut self) -> WRT_PROT_SEC_54_W {
        WRT_PROT_SEC_54_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_53(&mut self) -> WRT_PROT_SEC_53_W {
        WRT_PROT_SEC_53_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_52(&mut self) -> WRT_PROT_SEC_52_W {
        WRT_PROT_SEC_52_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_51(&mut self) -> WRT_PROT_SEC_51_W {
        WRT_PROT_SEC_51_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_50(&mut self) -> WRT_PROT_SEC_50_W {
        WRT_PROT_SEC_50_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_49(&mut self) -> WRT_PROT_SEC_49_W {
        WRT_PROT_SEC_49_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_48(&mut self) -> WRT_PROT_SEC_48_W {
        WRT_PROT_SEC_48_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_47(&mut self) -> WRT_PROT_SEC_47_W {
        WRT_PROT_SEC_47_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_46(&mut self) -> WRT_PROT_SEC_46_W {
        WRT_PROT_SEC_46_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_45(&mut self) -> WRT_PROT_SEC_45_W {
        WRT_PROT_SEC_45_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_44(&mut self) -> WRT_PROT_SEC_44_W {
        WRT_PROT_SEC_44_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_43(&mut self) -> WRT_PROT_SEC_43_W {
        WRT_PROT_SEC_43_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_42(&mut self) -> WRT_PROT_SEC_42_W {
        WRT_PROT_SEC_42_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_41(&mut self) -> WRT_PROT_SEC_41_W {
        WRT_PROT_SEC_41_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_40(&mut self) -> WRT_PROT_SEC_40_W {
        WRT_PROT_SEC_40_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_39(&mut self) -> WRT_PROT_SEC_39_W {
        WRT_PROT_SEC_39_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_38(&mut self) -> WRT_PROT_SEC_38_W {
        WRT_PROT_SEC_38_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_37(&mut self) -> WRT_PROT_SEC_37_W {
        WRT_PROT_SEC_37_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_36(&mut self) -> WRT_PROT_SEC_36_W {
        WRT_PROT_SEC_36_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_35(&mut self) -> WRT_PROT_SEC_35_W {
        WRT_PROT_SEC_35_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_34(&mut self) -> WRT_PROT_SEC_34_W {
        WRT_PROT_SEC_34_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_33(&mut self) -> WRT_PROT_SEC_33_W {
        WRT_PROT_SEC_33_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_32(&mut self) -> WRT_PROT_SEC_32_W {
        WRT_PROT_SEC_32_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protect Sectors 32-63 Each bit write protects one 4KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_prot_63_32](index.html) module"]
pub struct CCFG_PROT_63_32_SPEC;
impl crate::RegisterSpec for CCFG_PROT_63_32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_prot_63_32::R](R) reader structure"]
impl crate::Readable for CCFG_PROT_63_32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_prot_63_32::W](W) writer structure"]
impl crate::Writable for CCFG_PROT_63_32_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCFG_PROT_63_32 to value 0xffff_ffff"]
impl crate::Resettable for CCFG_PROT_63_32_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
