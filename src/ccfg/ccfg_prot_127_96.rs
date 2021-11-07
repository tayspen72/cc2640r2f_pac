#[doc = "Register `CCFG_PROT_127_96` reader"]
pub struct R(crate::R<CCFG_PROT_127_96_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_PROT_127_96_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_PROT_127_96_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_PROT_127_96_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_PROT_127_96` writer"]
pub struct W(crate::W<CCFG_PROT_127_96_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_PROT_127_96_SPEC>;
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
impl From<crate::W<CCFG_PROT_127_96_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_PROT_127_96_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRT_PROT_SEC_127` reader - 31:31\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_127_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_127_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_127_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_127_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_127` writer - 31:31\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_127_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_127_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_126` reader - 30:30\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_126_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_126_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_126_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_126_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_126` writer - 30:30\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_126_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_126_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_125` reader - 29:29\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_125_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_125_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_125_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_125_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_125` writer - 29:29\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_125_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_125_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_124` reader - 28:28\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_124_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_124_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_124_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_124_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_124` writer - 28:28\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_124_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_124_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_123` reader - 27:27\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_123_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_123_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_123_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_123_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_123` writer - 27:27\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_123_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_123_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_122` reader - 26:26\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_122_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_122_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_122_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_122_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_122` writer - 26:26\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_122_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_122_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_121` reader - 25:25\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_121_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_121_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_121_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_121_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_121` writer - 25:25\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_121_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_121_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_120` reader - 24:24\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_120_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_120_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_120_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_120_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_120` writer - 24:24\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_120_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_120_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_119` reader - 23:23\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_119_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_119_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_119_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_119_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_119` writer - 23:23\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_119_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_119_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_118` reader - 22:22\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_118_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_118_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_118_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_118_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_118` writer - 22:22\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_118_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_118_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_117` reader - 21:21\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_117_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_117_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_117_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_117_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_117` writer - 21:21\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_117_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_117_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_116` reader - 20:20\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_116_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_116_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_116_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_116_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_116` writer - 20:20\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_116_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_116_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_115` reader - 19:19\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_115_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_115_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_115_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_115_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_115` writer - 19:19\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_115_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_115_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_114` reader - 18:18\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_114_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_114_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_114_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_114_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_114` writer - 18:18\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_114_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_114_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_113` reader - 17:17\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_113_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_113_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_113_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_113_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_113` writer - 17:17\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_113_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_113_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_112` reader - 16:16\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_112_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_112_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_112_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_112_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_112` writer - 16:16\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_112_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_112_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_111` reader - 15:15\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_111_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_111_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_111_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_111_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_111` writer - 15:15\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_111_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_111_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_110` reader - 14:14\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_110_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_110_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_110_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_110_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_110` writer - 14:14\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_110_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_110_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_109` reader - 13:13\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_109_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_109_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_109_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_109_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_109` writer - 13:13\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_109_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_109_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_108` reader - 12:12\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_108_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_108_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_108_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_108_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_108` writer - 12:12\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_108_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_108_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_107` reader - 11:11\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_107_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_107_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_107_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_107_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_107` writer - 11:11\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_107_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_107_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_106` reader - 10:10\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_106_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_106_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_106_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_106_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_106` writer - 10:10\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_106_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_106_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_105` reader - 9:9\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_105_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_105_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_105_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_105_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_105` writer - 9:9\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_105_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_105_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_104` reader - 8:8\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_104_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_104_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_104_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_104_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_104` writer - 8:8\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_104_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_104_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_103` reader - 7:7\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_103_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_103_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_103_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_103_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_103` writer - 7:7\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_103_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_103_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_102` reader - 6:6\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_102_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_102_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_102_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_102_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_102` writer - 6:6\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_102_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_102_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_101` reader - 5:5\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_101_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_101_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_101_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_101_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_101` writer - 5:5\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_101_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_101_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_100` reader - 4:4\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_100_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_100_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_100_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_100_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_100` writer - 4:4\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_100_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_100_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_99` reader - 3:3\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_99_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_99_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_99_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_99_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_99` writer - 3:3\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_99_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_99_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_98` reader - 2:2\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_98_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_98_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_98_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_98_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_98` writer - 2:2\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_98_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_98_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_97` reader - 1:1\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_97_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_97_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_97_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_97_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_97` writer - 1:1\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_97_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_97_W<'a> {
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
#[doc = "Field `WRT_PROT_SEC_96` reader - 0:0\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_96_R(crate::FieldReader<bool, bool>);
impl WRT_PROT_SEC_96_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRT_PROT_SEC_96_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRT_PROT_SEC_96_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRT_PROT_SEC_96` writer - 0:0\\]
0: Sector protected"]
pub struct WRT_PROT_SEC_96_W<'a> {
    w: &'a mut W,
}
impl<'a> WRT_PROT_SEC_96_W<'a> {
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
    pub fn wrt_prot_sec_127(&self) -> WRT_PROT_SEC_127_R {
        WRT_PROT_SEC_127_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_126(&self) -> WRT_PROT_SEC_126_R {
        WRT_PROT_SEC_126_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_125(&self) -> WRT_PROT_SEC_125_R {
        WRT_PROT_SEC_125_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_124(&self) -> WRT_PROT_SEC_124_R {
        WRT_PROT_SEC_124_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_123(&self) -> WRT_PROT_SEC_123_R {
        WRT_PROT_SEC_123_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_122(&self) -> WRT_PROT_SEC_122_R {
        WRT_PROT_SEC_122_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_121(&self) -> WRT_PROT_SEC_121_R {
        WRT_PROT_SEC_121_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_120(&self) -> WRT_PROT_SEC_120_R {
        WRT_PROT_SEC_120_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_119(&self) -> WRT_PROT_SEC_119_R {
        WRT_PROT_SEC_119_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_118(&self) -> WRT_PROT_SEC_118_R {
        WRT_PROT_SEC_118_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_117(&self) -> WRT_PROT_SEC_117_R {
        WRT_PROT_SEC_117_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_116(&self) -> WRT_PROT_SEC_116_R {
        WRT_PROT_SEC_116_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_115(&self) -> WRT_PROT_SEC_115_R {
        WRT_PROT_SEC_115_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_114(&self) -> WRT_PROT_SEC_114_R {
        WRT_PROT_SEC_114_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_113(&self) -> WRT_PROT_SEC_113_R {
        WRT_PROT_SEC_113_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_112(&self) -> WRT_PROT_SEC_112_R {
        WRT_PROT_SEC_112_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_111(&self) -> WRT_PROT_SEC_111_R {
        WRT_PROT_SEC_111_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_110(&self) -> WRT_PROT_SEC_110_R {
        WRT_PROT_SEC_110_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_109(&self) -> WRT_PROT_SEC_109_R {
        WRT_PROT_SEC_109_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_108(&self) -> WRT_PROT_SEC_108_R {
        WRT_PROT_SEC_108_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_107(&self) -> WRT_PROT_SEC_107_R {
        WRT_PROT_SEC_107_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_106(&self) -> WRT_PROT_SEC_106_R {
        WRT_PROT_SEC_106_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_105(&self) -> WRT_PROT_SEC_105_R {
        WRT_PROT_SEC_105_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_104(&self) -> WRT_PROT_SEC_104_R {
        WRT_PROT_SEC_104_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_103(&self) -> WRT_PROT_SEC_103_R {
        WRT_PROT_SEC_103_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_102(&self) -> WRT_PROT_SEC_102_R {
        WRT_PROT_SEC_102_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_101(&self) -> WRT_PROT_SEC_101_R {
        WRT_PROT_SEC_101_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_100(&self) -> WRT_PROT_SEC_100_R {
        WRT_PROT_SEC_100_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_99(&self) -> WRT_PROT_SEC_99_R {
        WRT_PROT_SEC_99_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_98(&self) -> WRT_PROT_SEC_98_R {
        WRT_PROT_SEC_98_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_97(&self) -> WRT_PROT_SEC_97_R {
        WRT_PROT_SEC_97_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_96(&self) -> WRT_PROT_SEC_96_R {
        WRT_PROT_SEC_96_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_127(&mut self) -> WRT_PROT_SEC_127_W {
        WRT_PROT_SEC_127_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_126(&mut self) -> WRT_PROT_SEC_126_W {
        WRT_PROT_SEC_126_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_125(&mut self) -> WRT_PROT_SEC_125_W {
        WRT_PROT_SEC_125_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_124(&mut self) -> WRT_PROT_SEC_124_W {
        WRT_PROT_SEC_124_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_123(&mut self) -> WRT_PROT_SEC_123_W {
        WRT_PROT_SEC_123_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_122(&mut self) -> WRT_PROT_SEC_122_W {
        WRT_PROT_SEC_122_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_121(&mut self) -> WRT_PROT_SEC_121_W {
        WRT_PROT_SEC_121_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_120(&mut self) -> WRT_PROT_SEC_120_W {
        WRT_PROT_SEC_120_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_119(&mut self) -> WRT_PROT_SEC_119_W {
        WRT_PROT_SEC_119_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_118(&mut self) -> WRT_PROT_SEC_118_W {
        WRT_PROT_SEC_118_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_117(&mut self) -> WRT_PROT_SEC_117_W {
        WRT_PROT_SEC_117_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_116(&mut self) -> WRT_PROT_SEC_116_W {
        WRT_PROT_SEC_116_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_115(&mut self) -> WRT_PROT_SEC_115_W {
        WRT_PROT_SEC_115_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_114(&mut self) -> WRT_PROT_SEC_114_W {
        WRT_PROT_SEC_114_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_113(&mut self) -> WRT_PROT_SEC_113_W {
        WRT_PROT_SEC_113_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_112(&mut self) -> WRT_PROT_SEC_112_W {
        WRT_PROT_SEC_112_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_111(&mut self) -> WRT_PROT_SEC_111_W {
        WRT_PROT_SEC_111_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_110(&mut self) -> WRT_PROT_SEC_110_W {
        WRT_PROT_SEC_110_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_109(&mut self) -> WRT_PROT_SEC_109_W {
        WRT_PROT_SEC_109_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_108(&mut self) -> WRT_PROT_SEC_108_W {
        WRT_PROT_SEC_108_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_107(&mut self) -> WRT_PROT_SEC_107_W {
        WRT_PROT_SEC_107_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_106(&mut self) -> WRT_PROT_SEC_106_W {
        WRT_PROT_SEC_106_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_105(&mut self) -> WRT_PROT_SEC_105_W {
        WRT_PROT_SEC_105_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_104(&mut self) -> WRT_PROT_SEC_104_W {
        WRT_PROT_SEC_104_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_103(&mut self) -> WRT_PROT_SEC_103_W {
        WRT_PROT_SEC_103_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_102(&mut self) -> WRT_PROT_SEC_102_W {
        WRT_PROT_SEC_102_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_101(&mut self) -> WRT_PROT_SEC_101_W {
        WRT_PROT_SEC_101_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_100(&mut self) -> WRT_PROT_SEC_100_W {
        WRT_PROT_SEC_100_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_99(&mut self) -> WRT_PROT_SEC_99_W {
        WRT_PROT_SEC_99_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_98(&mut self) -> WRT_PROT_SEC_98_W {
        WRT_PROT_SEC_98_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_97(&mut self) -> WRT_PROT_SEC_97_W {
        WRT_PROT_SEC_97_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_96(&mut self) -> WRT_PROT_SEC_96_W {
        WRT_PROT_SEC_96_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_prot_127_96](index.html) module"]
pub struct CCFG_PROT_127_96_SPEC;
impl crate::RegisterSpec for CCFG_PROT_127_96_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_prot_127_96::R](R) reader structure"]
impl crate::Readable for CCFG_PROT_127_96_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_prot_127_96::W](W) writer structure"]
impl crate::Writable for CCFG_PROT_127_96_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCFG_PROT_127_96 to value 0xffff_ffff"]
impl crate::Resettable for CCFG_PROT_127_96_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
