#[doc = "Register `TER` reader"]
pub struct R(crate::R<TER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TER` writer"]
pub struct W(crate::W<TER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TER_SPEC>;
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
impl From<crate::W<TER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STIMENA31` reader - 31:31\\]
Bit mask to enable tracing on ITM stimulus port 31."]
pub struct STIMENA31_R(crate::FieldReader<bool, bool>);
impl STIMENA31_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA31` writer - 31:31\\]
Bit mask to enable tracing on ITM stimulus port 31."]
pub struct STIMENA31_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA31_W<'a> {
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
#[doc = "Field `STIMENA30` reader - 30:30\\]
Bit mask to enable tracing on ITM stimulus port 30."]
pub struct STIMENA30_R(crate::FieldReader<bool, bool>);
impl STIMENA30_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA30` writer - 30:30\\]
Bit mask to enable tracing on ITM stimulus port 30."]
pub struct STIMENA30_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA30_W<'a> {
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
#[doc = "Field `STIMENA29` reader - 29:29\\]
Bit mask to enable tracing on ITM stimulus port 29."]
pub struct STIMENA29_R(crate::FieldReader<bool, bool>);
impl STIMENA29_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA29` writer - 29:29\\]
Bit mask to enable tracing on ITM stimulus port 29."]
pub struct STIMENA29_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA29_W<'a> {
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
#[doc = "Field `STIMENA28` reader - 28:28\\]
Bit mask to enable tracing on ITM stimulus port 28."]
pub struct STIMENA28_R(crate::FieldReader<bool, bool>);
impl STIMENA28_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA28` writer - 28:28\\]
Bit mask to enable tracing on ITM stimulus port 28."]
pub struct STIMENA28_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA28_W<'a> {
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
#[doc = "Field `STIMENA27` reader - 27:27\\]
Bit mask to enable tracing on ITM stimulus port 27."]
pub struct STIMENA27_R(crate::FieldReader<bool, bool>);
impl STIMENA27_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA27` writer - 27:27\\]
Bit mask to enable tracing on ITM stimulus port 27."]
pub struct STIMENA27_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA27_W<'a> {
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
#[doc = "Field `STIMENA26` reader - 26:26\\]
Bit mask to enable tracing on ITM stimulus port 26."]
pub struct STIMENA26_R(crate::FieldReader<bool, bool>);
impl STIMENA26_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA26` writer - 26:26\\]
Bit mask to enable tracing on ITM stimulus port 26."]
pub struct STIMENA26_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA26_W<'a> {
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
#[doc = "Field `STIMENA25` reader - 25:25\\]
Bit mask to enable tracing on ITM stimulus port 25."]
pub struct STIMENA25_R(crate::FieldReader<bool, bool>);
impl STIMENA25_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA25` writer - 25:25\\]
Bit mask to enable tracing on ITM stimulus port 25."]
pub struct STIMENA25_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA25_W<'a> {
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
#[doc = "Field `STIMENA24` reader - 24:24\\]
Bit mask to enable tracing on ITM stimulus port 24."]
pub struct STIMENA24_R(crate::FieldReader<bool, bool>);
impl STIMENA24_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA24` writer - 24:24\\]
Bit mask to enable tracing on ITM stimulus port 24."]
pub struct STIMENA24_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA24_W<'a> {
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
#[doc = "Field `STIMENA23` reader - 23:23\\]
Bit mask to enable tracing on ITM stimulus port 23."]
pub struct STIMENA23_R(crate::FieldReader<bool, bool>);
impl STIMENA23_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA23` writer - 23:23\\]
Bit mask to enable tracing on ITM stimulus port 23."]
pub struct STIMENA23_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA23_W<'a> {
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
#[doc = "Field `STIMENA22` reader - 22:22\\]
Bit mask to enable tracing on ITM stimulus port 22."]
pub struct STIMENA22_R(crate::FieldReader<bool, bool>);
impl STIMENA22_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA22` writer - 22:22\\]
Bit mask to enable tracing on ITM stimulus port 22."]
pub struct STIMENA22_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA22_W<'a> {
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
#[doc = "Field `STIMENA21` reader - 21:21\\]
Bit mask to enable tracing on ITM stimulus port 21."]
pub struct STIMENA21_R(crate::FieldReader<bool, bool>);
impl STIMENA21_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA21` writer - 21:21\\]
Bit mask to enable tracing on ITM stimulus port 21."]
pub struct STIMENA21_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA21_W<'a> {
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
#[doc = "Field `STIMENA20` reader - 20:20\\]
Bit mask to enable tracing on ITM stimulus port 20."]
pub struct STIMENA20_R(crate::FieldReader<bool, bool>);
impl STIMENA20_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA20` writer - 20:20\\]
Bit mask to enable tracing on ITM stimulus port 20."]
pub struct STIMENA20_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA20_W<'a> {
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
#[doc = "Field `STIMENA19` reader - 19:19\\]
Bit mask to enable tracing on ITM stimulus port 19."]
pub struct STIMENA19_R(crate::FieldReader<bool, bool>);
impl STIMENA19_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA19` writer - 19:19\\]
Bit mask to enable tracing on ITM stimulus port 19."]
pub struct STIMENA19_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA19_W<'a> {
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
#[doc = "Field `STIMENA18` reader - 18:18\\]
Bit mask to enable tracing on ITM stimulus port 18."]
pub struct STIMENA18_R(crate::FieldReader<bool, bool>);
impl STIMENA18_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA18` writer - 18:18\\]
Bit mask to enable tracing on ITM stimulus port 18."]
pub struct STIMENA18_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA18_W<'a> {
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
#[doc = "Field `STIMENA17` reader - 17:17\\]
Bit mask to enable tracing on ITM stimulus port 17."]
pub struct STIMENA17_R(crate::FieldReader<bool, bool>);
impl STIMENA17_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA17` writer - 17:17\\]
Bit mask to enable tracing on ITM stimulus port 17."]
pub struct STIMENA17_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA17_W<'a> {
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
#[doc = "Field `STIMENA16` reader - 16:16\\]
Bit mask to enable tracing on ITM stimulus port 16."]
pub struct STIMENA16_R(crate::FieldReader<bool, bool>);
impl STIMENA16_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA16` writer - 16:16\\]
Bit mask to enable tracing on ITM stimulus port 16."]
pub struct STIMENA16_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA16_W<'a> {
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
#[doc = "Field `STIMENA15` reader - 15:15\\]
Bit mask to enable tracing on ITM stimulus port 15."]
pub struct STIMENA15_R(crate::FieldReader<bool, bool>);
impl STIMENA15_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA15` writer - 15:15\\]
Bit mask to enable tracing on ITM stimulus port 15."]
pub struct STIMENA15_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA15_W<'a> {
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
#[doc = "Field `STIMENA14` reader - 14:14\\]
Bit mask to enable tracing on ITM stimulus port 14."]
pub struct STIMENA14_R(crate::FieldReader<bool, bool>);
impl STIMENA14_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA14` writer - 14:14\\]
Bit mask to enable tracing on ITM stimulus port 14."]
pub struct STIMENA14_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA14_W<'a> {
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
#[doc = "Field `STIMENA13` reader - 13:13\\]
Bit mask to enable tracing on ITM stimulus port 13."]
pub struct STIMENA13_R(crate::FieldReader<bool, bool>);
impl STIMENA13_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA13` writer - 13:13\\]
Bit mask to enable tracing on ITM stimulus port 13."]
pub struct STIMENA13_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA13_W<'a> {
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
#[doc = "Field `STIMENA12` reader - 12:12\\]
Bit mask to enable tracing on ITM stimulus port 12."]
pub struct STIMENA12_R(crate::FieldReader<bool, bool>);
impl STIMENA12_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA12` writer - 12:12\\]
Bit mask to enable tracing on ITM stimulus port 12."]
pub struct STIMENA12_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA12_W<'a> {
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
#[doc = "Field `STIMENA11` reader - 11:11\\]
Bit mask to enable tracing on ITM stimulus port 11."]
pub struct STIMENA11_R(crate::FieldReader<bool, bool>);
impl STIMENA11_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA11` writer - 11:11\\]
Bit mask to enable tracing on ITM stimulus port 11."]
pub struct STIMENA11_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA11_W<'a> {
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
#[doc = "Field `STIMENA10` reader - 10:10\\]
Bit mask to enable tracing on ITM stimulus port 10."]
pub struct STIMENA10_R(crate::FieldReader<bool, bool>);
impl STIMENA10_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA10` writer - 10:10\\]
Bit mask to enable tracing on ITM stimulus port 10."]
pub struct STIMENA10_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA10_W<'a> {
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
#[doc = "Field `STIMENA9` reader - 9:9\\]
Bit mask to enable tracing on ITM stimulus port 9."]
pub struct STIMENA9_R(crate::FieldReader<bool, bool>);
impl STIMENA9_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA9` writer - 9:9\\]
Bit mask to enable tracing on ITM stimulus port 9."]
pub struct STIMENA9_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA9_W<'a> {
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
#[doc = "Field `STIMENA8` reader - 8:8\\]
Bit mask to enable tracing on ITM stimulus port 8."]
pub struct STIMENA8_R(crate::FieldReader<bool, bool>);
impl STIMENA8_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA8` writer - 8:8\\]
Bit mask to enable tracing on ITM stimulus port 8."]
pub struct STIMENA8_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA8_W<'a> {
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
#[doc = "Field `STIMENA7` reader - 7:7\\]
Bit mask to enable tracing on ITM stimulus port 7."]
pub struct STIMENA7_R(crate::FieldReader<bool, bool>);
impl STIMENA7_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA7` writer - 7:7\\]
Bit mask to enable tracing on ITM stimulus port 7."]
pub struct STIMENA7_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA7_W<'a> {
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
#[doc = "Field `STIMENA6` reader - 6:6\\]
Bit mask to enable tracing on ITM stimulus port 6."]
pub struct STIMENA6_R(crate::FieldReader<bool, bool>);
impl STIMENA6_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA6` writer - 6:6\\]
Bit mask to enable tracing on ITM stimulus port 6."]
pub struct STIMENA6_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA6_W<'a> {
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
#[doc = "Field `STIMENA5` reader - 5:5\\]
Bit mask to enable tracing on ITM stimulus port 5."]
pub struct STIMENA5_R(crate::FieldReader<bool, bool>);
impl STIMENA5_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA5` writer - 5:5\\]
Bit mask to enable tracing on ITM stimulus port 5."]
pub struct STIMENA5_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA5_W<'a> {
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
#[doc = "Field `STIMENA4` reader - 4:4\\]
Bit mask to enable tracing on ITM stimulus port 4."]
pub struct STIMENA4_R(crate::FieldReader<bool, bool>);
impl STIMENA4_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA4` writer - 4:4\\]
Bit mask to enable tracing on ITM stimulus port 4."]
pub struct STIMENA4_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA4_W<'a> {
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
#[doc = "Field `STIMENA3` reader - 3:3\\]
Bit mask to enable tracing on ITM stimulus port 3."]
pub struct STIMENA3_R(crate::FieldReader<bool, bool>);
impl STIMENA3_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA3` writer - 3:3\\]
Bit mask to enable tracing on ITM stimulus port 3."]
pub struct STIMENA3_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA3_W<'a> {
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
#[doc = "Field `STIMENA2` reader - 2:2\\]
Bit mask to enable tracing on ITM stimulus port 2."]
pub struct STIMENA2_R(crate::FieldReader<bool, bool>);
impl STIMENA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA2` writer - 2:2\\]
Bit mask to enable tracing on ITM stimulus port 2."]
pub struct STIMENA2_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA2_W<'a> {
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
#[doc = "Field `STIMENA1` reader - 1:1\\]
Bit mask to enable tracing on ITM stimulus port 1."]
pub struct STIMENA1_R(crate::FieldReader<bool, bool>);
impl STIMENA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA1` writer - 1:1\\]
Bit mask to enable tracing on ITM stimulus port 1."]
pub struct STIMENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA1_W<'a> {
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
#[doc = "Field `STIMENA0` reader - 0:0\\]
Bit mask to enable tracing on ITM stimulus port 0."]
pub struct STIMENA0_R(crate::FieldReader<bool, bool>);
impl STIMENA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        STIMENA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIMENA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIMENA0` writer - 0:0\\]
Bit mask to enable tracing on ITM stimulus port 0."]
pub struct STIMENA0_W<'a> {
    w: &'a mut W,
}
impl<'a> STIMENA0_W<'a> {
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
Bit mask to enable tracing on ITM stimulus port 31."]
    #[inline(always)]
    pub fn stimena31(&self) -> STIMENA31_R {
        STIMENA31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Bit mask to enable tracing on ITM stimulus port 30."]
    #[inline(always)]
    pub fn stimena30(&self) -> STIMENA30_R {
        STIMENA30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Bit mask to enable tracing on ITM stimulus port 29."]
    #[inline(always)]
    pub fn stimena29(&self) -> STIMENA29_R {
        STIMENA29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Bit mask to enable tracing on ITM stimulus port 28."]
    #[inline(always)]
    pub fn stimena28(&self) -> STIMENA28_R {
        STIMENA28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Bit mask to enable tracing on ITM stimulus port 27."]
    #[inline(always)]
    pub fn stimena27(&self) -> STIMENA27_R {
        STIMENA27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Bit mask to enable tracing on ITM stimulus port 26."]
    #[inline(always)]
    pub fn stimena26(&self) -> STIMENA26_R {
        STIMENA26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Bit mask to enable tracing on ITM stimulus port 25."]
    #[inline(always)]
    pub fn stimena25(&self) -> STIMENA25_R {
        STIMENA25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Bit mask to enable tracing on ITM stimulus port 24."]
    #[inline(always)]
    pub fn stimena24(&self) -> STIMENA24_R {
        STIMENA24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Bit mask to enable tracing on ITM stimulus port 23."]
    #[inline(always)]
    pub fn stimena23(&self) -> STIMENA23_R {
        STIMENA23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Bit mask to enable tracing on ITM stimulus port 22."]
    #[inline(always)]
    pub fn stimena22(&self) -> STIMENA22_R {
        STIMENA22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Bit mask to enable tracing on ITM stimulus port 21."]
    #[inline(always)]
    pub fn stimena21(&self) -> STIMENA21_R {
        STIMENA21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Bit mask to enable tracing on ITM stimulus port 20."]
    #[inline(always)]
    pub fn stimena20(&self) -> STIMENA20_R {
        STIMENA20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Bit mask to enable tracing on ITM stimulus port 19."]
    #[inline(always)]
    pub fn stimena19(&self) -> STIMENA19_R {
        STIMENA19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Bit mask to enable tracing on ITM stimulus port 18."]
    #[inline(always)]
    pub fn stimena18(&self) -> STIMENA18_R {
        STIMENA18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Bit mask to enable tracing on ITM stimulus port 17."]
    #[inline(always)]
    pub fn stimena17(&self) -> STIMENA17_R {
        STIMENA17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Bit mask to enable tracing on ITM stimulus port 16."]
    #[inline(always)]
    pub fn stimena16(&self) -> STIMENA16_R {
        STIMENA16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Bit mask to enable tracing on ITM stimulus port 15."]
    #[inline(always)]
    pub fn stimena15(&self) -> STIMENA15_R {
        STIMENA15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Bit mask to enable tracing on ITM stimulus port 14."]
    #[inline(always)]
    pub fn stimena14(&self) -> STIMENA14_R {
        STIMENA14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Bit mask to enable tracing on ITM stimulus port 13."]
    #[inline(always)]
    pub fn stimena13(&self) -> STIMENA13_R {
        STIMENA13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Bit mask to enable tracing on ITM stimulus port 12."]
    #[inline(always)]
    pub fn stimena12(&self) -> STIMENA12_R {
        STIMENA12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Bit mask to enable tracing on ITM stimulus port 11."]
    #[inline(always)]
    pub fn stimena11(&self) -> STIMENA11_R {
        STIMENA11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Bit mask to enable tracing on ITM stimulus port 10."]
    #[inline(always)]
    pub fn stimena10(&self) -> STIMENA10_R {
        STIMENA10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Bit mask to enable tracing on ITM stimulus port 9."]
    #[inline(always)]
    pub fn stimena9(&self) -> STIMENA9_R {
        STIMENA9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Bit mask to enable tracing on ITM stimulus port 8."]
    #[inline(always)]
    pub fn stimena8(&self) -> STIMENA8_R {
        STIMENA8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Bit mask to enable tracing on ITM stimulus port 7."]
    #[inline(always)]
    pub fn stimena7(&self) -> STIMENA7_R {
        STIMENA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Bit mask to enable tracing on ITM stimulus port 6."]
    #[inline(always)]
    pub fn stimena6(&self) -> STIMENA6_R {
        STIMENA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Bit mask to enable tracing on ITM stimulus port 5."]
    #[inline(always)]
    pub fn stimena5(&self) -> STIMENA5_R {
        STIMENA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Bit mask to enable tracing on ITM stimulus port 4."]
    #[inline(always)]
    pub fn stimena4(&self) -> STIMENA4_R {
        STIMENA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Bit mask to enable tracing on ITM stimulus port 3."]
    #[inline(always)]
    pub fn stimena3(&self) -> STIMENA3_R {
        STIMENA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Bit mask to enable tracing on ITM stimulus port 2."]
    #[inline(always)]
    pub fn stimena2(&self) -> STIMENA2_R {
        STIMENA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Bit mask to enable tracing on ITM stimulus port 1."]
    #[inline(always)]
    pub fn stimena1(&self) -> STIMENA1_R {
        STIMENA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Bit mask to enable tracing on ITM stimulus port 0."]
    #[inline(always)]
    pub fn stimena0(&self) -> STIMENA0_R {
        STIMENA0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Bit mask to enable tracing on ITM stimulus port 31."]
    #[inline(always)]
    pub fn stimena31(&mut self) -> STIMENA31_W {
        STIMENA31_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Bit mask to enable tracing on ITM stimulus port 30."]
    #[inline(always)]
    pub fn stimena30(&mut self) -> STIMENA30_W {
        STIMENA30_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Bit mask to enable tracing on ITM stimulus port 29."]
    #[inline(always)]
    pub fn stimena29(&mut self) -> STIMENA29_W {
        STIMENA29_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Bit mask to enable tracing on ITM stimulus port 28."]
    #[inline(always)]
    pub fn stimena28(&mut self) -> STIMENA28_W {
        STIMENA28_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Bit mask to enable tracing on ITM stimulus port 27."]
    #[inline(always)]
    pub fn stimena27(&mut self) -> STIMENA27_W {
        STIMENA27_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
Bit mask to enable tracing on ITM stimulus port 26."]
    #[inline(always)]
    pub fn stimena26(&mut self) -> STIMENA26_W {
        STIMENA26_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Bit mask to enable tracing on ITM stimulus port 25."]
    #[inline(always)]
    pub fn stimena25(&mut self) -> STIMENA25_W {
        STIMENA25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Bit mask to enable tracing on ITM stimulus port 24."]
    #[inline(always)]
    pub fn stimena24(&mut self) -> STIMENA24_W {
        STIMENA24_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Bit mask to enable tracing on ITM stimulus port 23."]
    #[inline(always)]
    pub fn stimena23(&mut self) -> STIMENA23_W {
        STIMENA23_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Bit mask to enable tracing on ITM stimulus port 22."]
    #[inline(always)]
    pub fn stimena22(&mut self) -> STIMENA22_W {
        STIMENA22_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Bit mask to enable tracing on ITM stimulus port 21."]
    #[inline(always)]
    pub fn stimena21(&mut self) -> STIMENA21_W {
        STIMENA21_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Bit mask to enable tracing on ITM stimulus port 20."]
    #[inline(always)]
    pub fn stimena20(&mut self) -> STIMENA20_W {
        STIMENA20_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Bit mask to enable tracing on ITM stimulus port 19."]
    #[inline(always)]
    pub fn stimena19(&mut self) -> STIMENA19_W {
        STIMENA19_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Bit mask to enable tracing on ITM stimulus port 18."]
    #[inline(always)]
    pub fn stimena18(&mut self) -> STIMENA18_W {
        STIMENA18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Bit mask to enable tracing on ITM stimulus port 17."]
    #[inline(always)]
    pub fn stimena17(&mut self) -> STIMENA17_W {
        STIMENA17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Bit mask to enable tracing on ITM stimulus port 16."]
    #[inline(always)]
    pub fn stimena16(&mut self) -> STIMENA16_W {
        STIMENA16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Bit mask to enable tracing on ITM stimulus port 15."]
    #[inline(always)]
    pub fn stimena15(&mut self) -> STIMENA15_W {
        STIMENA15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Bit mask to enable tracing on ITM stimulus port 14."]
    #[inline(always)]
    pub fn stimena14(&mut self) -> STIMENA14_W {
        STIMENA14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Bit mask to enable tracing on ITM stimulus port 13."]
    #[inline(always)]
    pub fn stimena13(&mut self) -> STIMENA13_W {
        STIMENA13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Bit mask to enable tracing on ITM stimulus port 12."]
    #[inline(always)]
    pub fn stimena12(&mut self) -> STIMENA12_W {
        STIMENA12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Bit mask to enable tracing on ITM stimulus port 11."]
    #[inline(always)]
    pub fn stimena11(&mut self) -> STIMENA11_W {
        STIMENA11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Bit mask to enable tracing on ITM stimulus port 10."]
    #[inline(always)]
    pub fn stimena10(&mut self) -> STIMENA10_W {
        STIMENA10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Bit mask to enable tracing on ITM stimulus port 9."]
    #[inline(always)]
    pub fn stimena9(&mut self) -> STIMENA9_W {
        STIMENA9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Bit mask to enable tracing on ITM stimulus port 8."]
    #[inline(always)]
    pub fn stimena8(&mut self) -> STIMENA8_W {
        STIMENA8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Bit mask to enable tracing on ITM stimulus port 7."]
    #[inline(always)]
    pub fn stimena7(&mut self) -> STIMENA7_W {
        STIMENA7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Bit mask to enable tracing on ITM stimulus port 6."]
    #[inline(always)]
    pub fn stimena6(&mut self) -> STIMENA6_W {
        STIMENA6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Bit mask to enable tracing on ITM stimulus port 5."]
    #[inline(always)]
    pub fn stimena5(&mut self) -> STIMENA5_W {
        STIMENA5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Bit mask to enable tracing on ITM stimulus port 4."]
    #[inline(always)]
    pub fn stimena4(&mut self) -> STIMENA4_W {
        STIMENA4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Bit mask to enable tracing on ITM stimulus port 3."]
    #[inline(always)]
    pub fn stimena3(&mut self) -> STIMENA3_W {
        STIMENA3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Bit mask to enable tracing on ITM stimulus port 2."]
    #[inline(always)]
    pub fn stimena2(&mut self) -> STIMENA2_W {
        STIMENA2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Bit mask to enable tracing on ITM stimulus port 1."]
    #[inline(always)]
    pub fn stimena1(&mut self) -> STIMENA1_W {
        STIMENA1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Bit mask to enable tracing on ITM stimulus port 0."]
    #[inline(always)]
    pub fn stimena0(&mut self) -> STIMENA0_W {
        STIMENA0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trace Enable Use the Trace Enable Register to generate trace data by writing to the corresponding stimulus port. Note: Privileged writes are accepted to this register if TCR.ITMENA is set. User writes are accepted to this register if TCR.ITMENA is set and the appropriate privilege mask is cleared. Privileged access to the stimulus ports enables an RTOS kernel to guarantee instrumentation slots or bandwidth as required.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ter](index.html) module"]
pub struct TER_SPEC;
impl crate::RegisterSpec for TER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ter::R](R) reader structure"]
impl crate::Readable for TER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ter::W](W) writer structure"]
impl crate::Writable for TER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TER to value 0"]
impl crate::Resettable for TER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
