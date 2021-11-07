#[doc = "Register `DIN31_0` reader"]
pub struct R(crate::R<DIN31_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIN31_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIN31_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIN31_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIN31_0` writer"]
pub struct W(crate::W<DIN31_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIN31_0_SPEC>;
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
impl From<crate::W<DIN31_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIN31_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIO31` reader - 31:31\\]
Data input from DIO 31"]
pub struct DIO31_R(crate::FieldReader<bool, bool>);
impl DIO31_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO31` writer - 31:31\\]
Data input from DIO 31"]
pub struct DIO31_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO31_W<'a> {
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
#[doc = "Field `DIO30` reader - 30:30\\]
Data input from DIO 30"]
pub struct DIO30_R(crate::FieldReader<bool, bool>);
impl DIO30_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO30` writer - 30:30\\]
Data input from DIO 30"]
pub struct DIO30_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO30_W<'a> {
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
#[doc = "Field `DIO29` reader - 29:29\\]
Data input from DIO 29"]
pub struct DIO29_R(crate::FieldReader<bool, bool>);
impl DIO29_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO29` writer - 29:29\\]
Data input from DIO 29"]
pub struct DIO29_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO29_W<'a> {
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
#[doc = "Field `DIO28` reader - 28:28\\]
Data input from DIO 28"]
pub struct DIO28_R(crate::FieldReader<bool, bool>);
impl DIO28_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO28` writer - 28:28\\]
Data input from DIO 28"]
pub struct DIO28_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO28_W<'a> {
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
#[doc = "Field `DIO27` reader - 27:27\\]
Data input from DIO 27"]
pub struct DIO27_R(crate::FieldReader<bool, bool>);
impl DIO27_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO27` writer - 27:27\\]
Data input from DIO 27"]
pub struct DIO27_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO27_W<'a> {
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
#[doc = "Field `DIO26` reader - 26:26\\]
Data input from DIO 26"]
pub struct DIO26_R(crate::FieldReader<bool, bool>);
impl DIO26_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO26` writer - 26:26\\]
Data input from DIO 26"]
pub struct DIO26_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO26_W<'a> {
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
#[doc = "Field `DIO25` reader - 25:25\\]
Data input from DIO 25"]
pub struct DIO25_R(crate::FieldReader<bool, bool>);
impl DIO25_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO25` writer - 25:25\\]
Data input from DIO 25"]
pub struct DIO25_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO25_W<'a> {
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
#[doc = "Field `DIO24` reader - 24:24\\]
Data input from DIO 24"]
pub struct DIO24_R(crate::FieldReader<bool, bool>);
impl DIO24_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO24` writer - 24:24\\]
Data input from DIO 24"]
pub struct DIO24_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO24_W<'a> {
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
#[doc = "Field `DIO23` reader - 23:23\\]
Data input from DIO 23"]
pub struct DIO23_R(crate::FieldReader<bool, bool>);
impl DIO23_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO23` writer - 23:23\\]
Data input from DIO 23"]
pub struct DIO23_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO23_W<'a> {
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
#[doc = "Field `DIO22` reader - 22:22\\]
Data input from DIO 22"]
pub struct DIO22_R(crate::FieldReader<bool, bool>);
impl DIO22_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO22` writer - 22:22\\]
Data input from DIO 22"]
pub struct DIO22_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO22_W<'a> {
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
#[doc = "Field `DIO21` reader - 21:21\\]
Data input from DIO 21"]
pub struct DIO21_R(crate::FieldReader<bool, bool>);
impl DIO21_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO21` writer - 21:21\\]
Data input from DIO 21"]
pub struct DIO21_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO21_W<'a> {
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
#[doc = "Field `DIO20` reader - 20:20\\]
Data input from DIO 20"]
pub struct DIO20_R(crate::FieldReader<bool, bool>);
impl DIO20_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO20` writer - 20:20\\]
Data input from DIO 20"]
pub struct DIO20_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO20_W<'a> {
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
#[doc = "Field `DIO19` reader - 19:19\\]
Data input from DIO 19"]
pub struct DIO19_R(crate::FieldReader<bool, bool>);
impl DIO19_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO19` writer - 19:19\\]
Data input from DIO 19"]
pub struct DIO19_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO19_W<'a> {
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
#[doc = "Field `DIO18` reader - 18:18\\]
Data input from DIO 18"]
pub struct DIO18_R(crate::FieldReader<bool, bool>);
impl DIO18_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO18` writer - 18:18\\]
Data input from DIO 18"]
pub struct DIO18_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO18_W<'a> {
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
#[doc = "Field `DIO17` reader - 17:17\\]
Data input from DIO 17"]
pub struct DIO17_R(crate::FieldReader<bool, bool>);
impl DIO17_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO17` writer - 17:17\\]
Data input from DIO 17"]
pub struct DIO17_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO17_W<'a> {
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
#[doc = "Field `DIO16` reader - 16:16\\]
Data input from DIO 16"]
pub struct DIO16_R(crate::FieldReader<bool, bool>);
impl DIO16_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO16` writer - 16:16\\]
Data input from DIO 16"]
pub struct DIO16_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO16_W<'a> {
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
#[doc = "Field `DIO15` reader - 15:15\\]
Data input from DIO 15"]
pub struct DIO15_R(crate::FieldReader<bool, bool>);
impl DIO15_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO15` writer - 15:15\\]
Data input from DIO 15"]
pub struct DIO15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO15_W<'a> {
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
#[doc = "Field `DIO14` reader - 14:14\\]
Data input from DIO 14"]
pub struct DIO14_R(crate::FieldReader<bool, bool>);
impl DIO14_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO14` writer - 14:14\\]
Data input from DIO 14"]
pub struct DIO14_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO14_W<'a> {
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
#[doc = "Field `DIO13` reader - 13:13\\]
Data input from DIO 13"]
pub struct DIO13_R(crate::FieldReader<bool, bool>);
impl DIO13_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO13` writer - 13:13\\]
Data input from DIO 13"]
pub struct DIO13_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO13_W<'a> {
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
#[doc = "Field `DIO12` reader - 12:12\\]
Data input from DIO 12"]
pub struct DIO12_R(crate::FieldReader<bool, bool>);
impl DIO12_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO12` writer - 12:12\\]
Data input from DIO 12"]
pub struct DIO12_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO12_W<'a> {
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
#[doc = "Field `DIO11` reader - 11:11\\]
Data input from DIO 11"]
pub struct DIO11_R(crate::FieldReader<bool, bool>);
impl DIO11_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO11` writer - 11:11\\]
Data input from DIO 11"]
pub struct DIO11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO11_W<'a> {
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
#[doc = "Field `DIO10` reader - 10:10\\]
Data input from DIO 10"]
pub struct DIO10_R(crate::FieldReader<bool, bool>);
impl DIO10_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO10` writer - 10:10\\]
Data input from DIO 10"]
pub struct DIO10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO10_W<'a> {
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
#[doc = "Field `DIO9` reader - 9:9\\]
Data input from DIO 9"]
pub struct DIO9_R(crate::FieldReader<bool, bool>);
impl DIO9_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO9` writer - 9:9\\]
Data input from DIO 9"]
pub struct DIO9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO9_W<'a> {
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
#[doc = "Field `DIO8` reader - 8:8\\]
Data input from DIO 8"]
pub struct DIO8_R(crate::FieldReader<bool, bool>);
impl DIO8_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO8` writer - 8:8\\]
Data input from DIO 8"]
pub struct DIO8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO8_W<'a> {
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
#[doc = "Field `DIO7` reader - 7:7\\]
Data input from DIO 7"]
pub struct DIO7_R(crate::FieldReader<bool, bool>);
impl DIO7_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO7` writer - 7:7\\]
Data input from DIO 7"]
pub struct DIO7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO7_W<'a> {
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
#[doc = "Field `DIO6` reader - 6:6\\]
Data input from DIO 6"]
pub struct DIO6_R(crate::FieldReader<bool, bool>);
impl DIO6_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO6` writer - 6:6\\]
Data input from DIO 6"]
pub struct DIO6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO6_W<'a> {
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
#[doc = "Field `DIO5` reader - 5:5\\]
Data input from DIO 5"]
pub struct DIO5_R(crate::FieldReader<bool, bool>);
impl DIO5_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO5` writer - 5:5\\]
Data input from DIO 5"]
pub struct DIO5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO5_W<'a> {
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
#[doc = "Field `DIO4` reader - 4:4\\]
Data input from DIO 4"]
pub struct DIO4_R(crate::FieldReader<bool, bool>);
impl DIO4_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO4` writer - 4:4\\]
Data input from DIO 4"]
pub struct DIO4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO4_W<'a> {
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
#[doc = "Field `DIO3` reader - 3:3\\]
Data input from DIO 3"]
pub struct DIO3_R(crate::FieldReader<bool, bool>);
impl DIO3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO3` writer - 3:3\\]
Data input from DIO 3"]
pub struct DIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO3_W<'a> {
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
#[doc = "Field `DIO2` reader - 2:2\\]
Data input from DIO 2"]
pub struct DIO2_R(crate::FieldReader<bool, bool>);
impl DIO2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO2` writer - 2:2\\]
Data input from DIO 2"]
pub struct DIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO2_W<'a> {
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
#[doc = "Field `DIO1` reader - 1:1\\]
Data input from DIO 1"]
pub struct DIO1_R(crate::FieldReader<bool, bool>);
impl DIO1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO1` writer - 1:1\\]
Data input from DIO 1"]
pub struct DIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO1_W<'a> {
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
#[doc = "Field `DIO0` reader - 0:0\\]
Data input from DIO 0"]
pub struct DIO0_R(crate::FieldReader<bool, bool>);
impl DIO0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIO0` writer - 0:0\\]
Data input from DIO 0"]
pub struct DIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO0_W<'a> {
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
Data input from DIO 31"]
    #[inline(always)]
    pub fn dio31(&self) -> DIO31_R {
        DIO31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Data input from DIO 30"]
    #[inline(always)]
    pub fn dio30(&self) -> DIO30_R {
        DIO30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Data input from DIO 29"]
    #[inline(always)]
    pub fn dio29(&self) -> DIO29_R {
        DIO29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Data input from DIO 28"]
    #[inline(always)]
    pub fn dio28(&self) -> DIO28_R {
        DIO28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Data input from DIO 27"]
    #[inline(always)]
    pub fn dio27(&self) -> DIO27_R {
        DIO27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Data input from DIO 26"]
    #[inline(always)]
    pub fn dio26(&self) -> DIO26_R {
        DIO26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Data input from DIO 25"]
    #[inline(always)]
    pub fn dio25(&self) -> DIO25_R {
        DIO25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Data input from DIO 24"]
    #[inline(always)]
    pub fn dio24(&self) -> DIO24_R {
        DIO24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Data input from DIO 23"]
    #[inline(always)]
    pub fn dio23(&self) -> DIO23_R {
        DIO23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Data input from DIO 22"]
    #[inline(always)]
    pub fn dio22(&self) -> DIO22_R {
        DIO22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Data input from DIO 21"]
    #[inline(always)]
    pub fn dio21(&self) -> DIO21_R {
        DIO21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Data input from DIO 20"]
    #[inline(always)]
    pub fn dio20(&self) -> DIO20_R {
        DIO20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Data input from DIO 19"]
    #[inline(always)]
    pub fn dio19(&self) -> DIO19_R {
        DIO19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Data input from DIO 18"]
    #[inline(always)]
    pub fn dio18(&self) -> DIO18_R {
        DIO18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Data input from DIO 17"]
    #[inline(always)]
    pub fn dio17(&self) -> DIO17_R {
        DIO17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Data input from DIO 16"]
    #[inline(always)]
    pub fn dio16(&self) -> DIO16_R {
        DIO16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Data input from DIO 15"]
    #[inline(always)]
    pub fn dio15(&self) -> DIO15_R {
        DIO15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Data input from DIO 14"]
    #[inline(always)]
    pub fn dio14(&self) -> DIO14_R {
        DIO14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Data input from DIO 13"]
    #[inline(always)]
    pub fn dio13(&self) -> DIO13_R {
        DIO13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Data input from DIO 12"]
    #[inline(always)]
    pub fn dio12(&self) -> DIO12_R {
        DIO12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Data input from DIO 11"]
    #[inline(always)]
    pub fn dio11(&self) -> DIO11_R {
        DIO11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Data input from DIO 10"]
    #[inline(always)]
    pub fn dio10(&self) -> DIO10_R {
        DIO10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Data input from DIO 9"]
    #[inline(always)]
    pub fn dio9(&self) -> DIO9_R {
        DIO9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Data input from DIO 8"]
    #[inline(always)]
    pub fn dio8(&self) -> DIO8_R {
        DIO8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Data input from DIO 7"]
    #[inline(always)]
    pub fn dio7(&self) -> DIO7_R {
        DIO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Data input from DIO 6"]
    #[inline(always)]
    pub fn dio6(&self) -> DIO6_R {
        DIO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Data input from DIO 5"]
    #[inline(always)]
    pub fn dio5(&self) -> DIO5_R {
        DIO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Data input from DIO 4"]
    #[inline(always)]
    pub fn dio4(&self) -> DIO4_R {
        DIO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Data input from DIO 3"]
    #[inline(always)]
    pub fn dio3(&self) -> DIO3_R {
        DIO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Data input from DIO 2"]
    #[inline(always)]
    pub fn dio2(&self) -> DIO2_R {
        DIO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Data input from DIO 1"]
    #[inline(always)]
    pub fn dio1(&self) -> DIO1_R {
        DIO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Data input from DIO 0"]
    #[inline(always)]
    pub fn dio0(&self) -> DIO0_R {
        DIO0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Data input from DIO 31"]
    #[inline(always)]
    pub fn dio31(&mut self) -> DIO31_W {
        DIO31_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Data input from DIO 30"]
    #[inline(always)]
    pub fn dio30(&mut self) -> DIO30_W {
        DIO30_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Data input from DIO 29"]
    #[inline(always)]
    pub fn dio29(&mut self) -> DIO29_W {
        DIO29_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Data input from DIO 28"]
    #[inline(always)]
    pub fn dio28(&mut self) -> DIO28_W {
        DIO28_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Data input from DIO 27"]
    #[inline(always)]
    pub fn dio27(&mut self) -> DIO27_W {
        DIO27_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
Data input from DIO 26"]
    #[inline(always)]
    pub fn dio26(&mut self) -> DIO26_W {
        DIO26_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Data input from DIO 25"]
    #[inline(always)]
    pub fn dio25(&mut self) -> DIO25_W {
        DIO25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Data input from DIO 24"]
    #[inline(always)]
    pub fn dio24(&mut self) -> DIO24_W {
        DIO24_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Data input from DIO 23"]
    #[inline(always)]
    pub fn dio23(&mut self) -> DIO23_W {
        DIO23_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Data input from DIO 22"]
    #[inline(always)]
    pub fn dio22(&mut self) -> DIO22_W {
        DIO22_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Data input from DIO 21"]
    #[inline(always)]
    pub fn dio21(&mut self) -> DIO21_W {
        DIO21_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Data input from DIO 20"]
    #[inline(always)]
    pub fn dio20(&mut self) -> DIO20_W {
        DIO20_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Data input from DIO 19"]
    #[inline(always)]
    pub fn dio19(&mut self) -> DIO19_W {
        DIO19_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Data input from DIO 18"]
    #[inline(always)]
    pub fn dio18(&mut self) -> DIO18_W {
        DIO18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Data input from DIO 17"]
    #[inline(always)]
    pub fn dio17(&mut self) -> DIO17_W {
        DIO17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Data input from DIO 16"]
    #[inline(always)]
    pub fn dio16(&mut self) -> DIO16_W {
        DIO16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Data input from DIO 15"]
    #[inline(always)]
    pub fn dio15(&mut self) -> DIO15_W {
        DIO15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Data input from DIO 14"]
    #[inline(always)]
    pub fn dio14(&mut self) -> DIO14_W {
        DIO14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Data input from DIO 13"]
    #[inline(always)]
    pub fn dio13(&mut self) -> DIO13_W {
        DIO13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Data input from DIO 12"]
    #[inline(always)]
    pub fn dio12(&mut self) -> DIO12_W {
        DIO12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Data input from DIO 11"]
    #[inline(always)]
    pub fn dio11(&mut self) -> DIO11_W {
        DIO11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Data input from DIO 10"]
    #[inline(always)]
    pub fn dio10(&mut self) -> DIO10_W {
        DIO10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Data input from DIO 9"]
    #[inline(always)]
    pub fn dio9(&mut self) -> DIO9_W {
        DIO9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Data input from DIO 8"]
    #[inline(always)]
    pub fn dio8(&mut self) -> DIO8_W {
        DIO8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Data input from DIO 7"]
    #[inline(always)]
    pub fn dio7(&mut self) -> DIO7_W {
        DIO7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Data input from DIO 6"]
    #[inline(always)]
    pub fn dio6(&mut self) -> DIO6_W {
        DIO6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Data input from DIO 5"]
    #[inline(always)]
    pub fn dio5(&mut self) -> DIO5_W {
        DIO5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Data input from DIO 4"]
    #[inline(always)]
    pub fn dio4(&mut self) -> DIO4_W {
        DIO4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Data input from DIO 3"]
    #[inline(always)]
    pub fn dio3(&mut self) -> DIO3_W {
        DIO3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Data input from DIO 2"]
    #[inline(always)]
    pub fn dio2(&mut self) -> DIO2_W {
        DIO2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Data input from DIO 1"]
    #[inline(always)]
    pub fn dio1(&mut self) -> DIO1_W {
        DIO1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Data input from DIO 0"]
    #[inline(always)]
    pub fn dio0(&mut self) -> DIO0_W {
        DIO0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Input from DIO 0 to 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [din31_0](index.html) module"]
pub struct DIN31_0_SPEC;
impl crate::RegisterSpec for DIN31_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [din31_0::R](R) reader structure"]
impl crate::Readable for DIN31_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [din31_0::W](W) writer structure"]
impl crate::Writable for DIN31_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIN31_0 to value 0"]
impl crate::Resettable for DIN31_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
