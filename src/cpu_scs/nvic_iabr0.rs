#[doc = "Register `NVIC_IABR0` reader"]
pub struct R(crate::R<NVIC_IABR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IABR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IABR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IABR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IABR0` writer"]
pub struct W(crate::W<NVIC_IABR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IABR0_SPEC>;
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
impl From<crate::W<NVIC_IABR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IABR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTIVE31` reader - 31:31\\]
Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details)."]
pub struct ACTIVE31_R(crate::FieldReader<bool, bool>);
impl ACTIVE31_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE31` writer - 31:31\\]
Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details)."]
pub struct ACTIVE31_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE31_W<'a> {
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
#[doc = "Field `ACTIVE30` reader - 30:30\\]
Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details)."]
pub struct ACTIVE30_R(crate::FieldReader<bool, bool>);
impl ACTIVE30_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE30` writer - 30:30\\]
Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details)."]
pub struct ACTIVE30_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE30_W<'a> {
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
#[doc = "Field `ACTIVE29` reader - 29:29\\]
Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details)."]
pub struct ACTIVE29_R(crate::FieldReader<bool, bool>);
impl ACTIVE29_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE29` writer - 29:29\\]
Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details)."]
pub struct ACTIVE29_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE29_W<'a> {
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
#[doc = "Field `ACTIVE28` reader - 28:28\\]
Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details)."]
pub struct ACTIVE28_R(crate::FieldReader<bool, bool>);
impl ACTIVE28_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE28` writer - 28:28\\]
Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details)."]
pub struct ACTIVE28_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE28_W<'a> {
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
#[doc = "Field `ACTIVE27` reader - 27:27\\]
Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details)."]
pub struct ACTIVE27_R(crate::FieldReader<bool, bool>);
impl ACTIVE27_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE27` writer - 27:27\\]
Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details)."]
pub struct ACTIVE27_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE27_W<'a> {
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
#[doc = "Field `ACTIVE26` reader - 26:26\\]
Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details)."]
pub struct ACTIVE26_R(crate::FieldReader<bool, bool>);
impl ACTIVE26_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE26` writer - 26:26\\]
Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details)."]
pub struct ACTIVE26_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE26_W<'a> {
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
#[doc = "Field `ACTIVE25` reader - 25:25\\]
Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details)."]
pub struct ACTIVE25_R(crate::FieldReader<bool, bool>);
impl ACTIVE25_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE25` writer - 25:25\\]
Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details)."]
pub struct ACTIVE25_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE25_W<'a> {
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
#[doc = "Field `ACTIVE24` reader - 24:24\\]
Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details)."]
pub struct ACTIVE24_R(crate::FieldReader<bool, bool>);
impl ACTIVE24_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE24` writer - 24:24\\]
Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details)."]
pub struct ACTIVE24_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE24_W<'a> {
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
#[doc = "Field `ACTIVE23` reader - 23:23\\]
Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details)."]
pub struct ACTIVE23_R(crate::FieldReader<bool, bool>);
impl ACTIVE23_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE23` writer - 23:23\\]
Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details)."]
pub struct ACTIVE23_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE23_W<'a> {
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
#[doc = "Field `ACTIVE22` reader - 22:22\\]
Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details)."]
pub struct ACTIVE22_R(crate::FieldReader<bool, bool>);
impl ACTIVE22_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE22` writer - 22:22\\]
Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details)."]
pub struct ACTIVE22_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE22_W<'a> {
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
#[doc = "Field `ACTIVE21` reader - 21:21\\]
Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details)."]
pub struct ACTIVE21_R(crate::FieldReader<bool, bool>);
impl ACTIVE21_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE21` writer - 21:21\\]
Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details)."]
pub struct ACTIVE21_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE21_W<'a> {
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
#[doc = "Field `ACTIVE20` reader - 20:20\\]
Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details)."]
pub struct ACTIVE20_R(crate::FieldReader<bool, bool>);
impl ACTIVE20_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE20` writer - 20:20\\]
Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details)."]
pub struct ACTIVE20_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE20_W<'a> {
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
#[doc = "Field `ACTIVE19` reader - 19:19\\]
Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details)."]
pub struct ACTIVE19_R(crate::FieldReader<bool, bool>);
impl ACTIVE19_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE19` writer - 19:19\\]
Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details)."]
pub struct ACTIVE19_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE19_W<'a> {
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
#[doc = "Field `ACTIVE18` reader - 18:18\\]
Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details)."]
pub struct ACTIVE18_R(crate::FieldReader<bool, bool>);
impl ACTIVE18_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE18` writer - 18:18\\]
Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details)."]
pub struct ACTIVE18_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE18_W<'a> {
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
#[doc = "Field `ACTIVE17` reader - 17:17\\]
Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details)."]
pub struct ACTIVE17_R(crate::FieldReader<bool, bool>);
impl ACTIVE17_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE17` writer - 17:17\\]
Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details)."]
pub struct ACTIVE17_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE17_W<'a> {
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
#[doc = "Field `ACTIVE16` reader - 16:16\\]
Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details)."]
pub struct ACTIVE16_R(crate::FieldReader<bool, bool>);
impl ACTIVE16_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE16` writer - 16:16\\]
Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details)."]
pub struct ACTIVE16_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE16_W<'a> {
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
#[doc = "Field `ACTIVE15` reader - 15:15\\]
Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details)."]
pub struct ACTIVE15_R(crate::FieldReader<bool, bool>);
impl ACTIVE15_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE15` writer - 15:15\\]
Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details)."]
pub struct ACTIVE15_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE15_W<'a> {
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
#[doc = "Field `ACTIVE14` reader - 14:14\\]
Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details)."]
pub struct ACTIVE14_R(crate::FieldReader<bool, bool>);
impl ACTIVE14_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE14` writer - 14:14\\]
Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details)."]
pub struct ACTIVE14_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE14_W<'a> {
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
#[doc = "Field `ACTIVE13` reader - 13:13\\]
Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details)."]
pub struct ACTIVE13_R(crate::FieldReader<bool, bool>);
impl ACTIVE13_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE13` writer - 13:13\\]
Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details)."]
pub struct ACTIVE13_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE13_W<'a> {
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
#[doc = "Field `ACTIVE12` reader - 12:12\\]
Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details)."]
pub struct ACTIVE12_R(crate::FieldReader<bool, bool>);
impl ACTIVE12_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE12` writer - 12:12\\]
Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details)."]
pub struct ACTIVE12_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE12_W<'a> {
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
#[doc = "Field `ACTIVE11` reader - 11:11\\]
Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details)."]
pub struct ACTIVE11_R(crate::FieldReader<bool, bool>);
impl ACTIVE11_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE11` writer - 11:11\\]
Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details)."]
pub struct ACTIVE11_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE11_W<'a> {
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
#[doc = "Field `ACTIVE10` reader - 10:10\\]
Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details)."]
pub struct ACTIVE10_R(crate::FieldReader<bool, bool>);
impl ACTIVE10_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE10` writer - 10:10\\]
Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details)."]
pub struct ACTIVE10_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE10_W<'a> {
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
#[doc = "Field `ACTIVE9` reader - 9:9\\]
Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details)."]
pub struct ACTIVE9_R(crate::FieldReader<bool, bool>);
impl ACTIVE9_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE9` writer - 9:9\\]
Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details)."]
pub struct ACTIVE9_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE9_W<'a> {
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
#[doc = "Field `ACTIVE8` reader - 8:8\\]
Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details)."]
pub struct ACTIVE8_R(crate::FieldReader<bool, bool>);
impl ACTIVE8_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE8` writer - 8:8\\]
Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details)."]
pub struct ACTIVE8_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE8_W<'a> {
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
#[doc = "Field `ACTIVE7` reader - 7:7\\]
Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details)."]
pub struct ACTIVE7_R(crate::FieldReader<bool, bool>);
impl ACTIVE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE7` writer - 7:7\\]
Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details)."]
pub struct ACTIVE7_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE7_W<'a> {
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
#[doc = "Field `ACTIVE6` reader - 6:6\\]
Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details)."]
pub struct ACTIVE6_R(crate::FieldReader<bool, bool>);
impl ACTIVE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE6` writer - 6:6\\]
Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details)."]
pub struct ACTIVE6_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE6_W<'a> {
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
#[doc = "Field `ACTIVE5` reader - 5:5\\]
Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details)."]
pub struct ACTIVE5_R(crate::FieldReader<bool, bool>);
impl ACTIVE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE5` writer - 5:5\\]
Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details)."]
pub struct ACTIVE5_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE5_W<'a> {
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
#[doc = "Field `ACTIVE4` reader - 4:4\\]
Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details)."]
pub struct ACTIVE4_R(crate::FieldReader<bool, bool>);
impl ACTIVE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE4` writer - 4:4\\]
Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details)."]
pub struct ACTIVE4_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE4_W<'a> {
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
#[doc = "Field `ACTIVE3` reader - 3:3\\]
Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details)."]
pub struct ACTIVE3_R(crate::FieldReader<bool, bool>);
impl ACTIVE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE3` writer - 3:3\\]
Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details)."]
pub struct ACTIVE3_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE3_W<'a> {
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
#[doc = "Field `ACTIVE2` reader - 2:2\\]
Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details)."]
pub struct ACTIVE2_R(crate::FieldReader<bool, bool>);
impl ACTIVE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE2` writer - 2:2\\]
Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details)."]
pub struct ACTIVE2_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE2_W<'a> {
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
#[doc = "Field `ACTIVE1` reader - 1:1\\]
Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details)."]
pub struct ACTIVE1_R(crate::FieldReader<bool, bool>);
impl ACTIVE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE1` writer - 1:1\\]
Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details)."]
pub struct ACTIVE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE1_W<'a> {
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
#[doc = "Field `ACTIVE0` reader - 0:0\\]
Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details)."]
pub struct ACTIVE0_R(crate::FieldReader<bool, bool>);
impl ACTIVE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTIVE0` writer - 0:0\\]
Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details)."]
pub struct ACTIVE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE0_W<'a> {
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
Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details)."]
    #[inline(always)]
    pub fn active31(&self) -> ACTIVE31_R {
        ACTIVE31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details)."]
    #[inline(always)]
    pub fn active30(&self) -> ACTIVE30_R {
        ACTIVE30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details)."]
    #[inline(always)]
    pub fn active29(&self) -> ACTIVE29_R {
        ACTIVE29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details)."]
    #[inline(always)]
    pub fn active28(&self) -> ACTIVE28_R {
        ACTIVE28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details)."]
    #[inline(always)]
    pub fn active27(&self) -> ACTIVE27_R {
        ACTIVE27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details)."]
    #[inline(always)]
    pub fn active26(&self) -> ACTIVE26_R {
        ACTIVE26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details)."]
    #[inline(always)]
    pub fn active25(&self) -> ACTIVE25_R {
        ACTIVE25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details)."]
    #[inline(always)]
    pub fn active24(&self) -> ACTIVE24_R {
        ACTIVE24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline(always)]
    pub fn active23(&self) -> ACTIVE23_R {
        ACTIVE23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline(always)]
    pub fn active22(&self) -> ACTIVE22_R {
        ACTIVE22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline(always)]
    pub fn active21(&self) -> ACTIVE21_R {
        ACTIVE21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline(always)]
    pub fn active20(&self) -> ACTIVE20_R {
        ACTIVE20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline(always)]
    pub fn active19(&self) -> ACTIVE19_R {
        ACTIVE19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline(always)]
    pub fn active18(&self) -> ACTIVE18_R {
        ACTIVE18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline(always)]
    pub fn active17(&self) -> ACTIVE17_R {
        ACTIVE17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline(always)]
    pub fn active16(&self) -> ACTIVE16_R {
        ACTIVE16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline(always)]
    pub fn active15(&self) -> ACTIVE15_R {
        ACTIVE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline(always)]
    pub fn active14(&self) -> ACTIVE14_R {
        ACTIVE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline(always)]
    pub fn active13(&self) -> ACTIVE13_R {
        ACTIVE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline(always)]
    pub fn active12(&self) -> ACTIVE12_R {
        ACTIVE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline(always)]
    pub fn active11(&self) -> ACTIVE11_R {
        ACTIVE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline(always)]
    pub fn active10(&self) -> ACTIVE10_R {
        ACTIVE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline(always)]
    pub fn active9(&self) -> ACTIVE9_R {
        ACTIVE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline(always)]
    pub fn active8(&self) -> ACTIVE8_R {
        ACTIVE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details)."]
    #[inline(always)]
    pub fn active7(&self) -> ACTIVE7_R {
        ACTIVE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details)."]
    #[inline(always)]
    pub fn active6(&self) -> ACTIVE6_R {
        ACTIVE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details)."]
    #[inline(always)]
    pub fn active5(&self) -> ACTIVE5_R {
        ACTIVE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details)."]
    #[inline(always)]
    pub fn active4(&self) -> ACTIVE4_R {
        ACTIVE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details)."]
    #[inline(always)]
    pub fn active3(&self) -> ACTIVE3_R {
        ACTIVE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details)."]
    #[inline(always)]
    pub fn active2(&self) -> ACTIVE2_R {
        ACTIVE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details)."]
    #[inline(always)]
    pub fn active1(&self) -> ACTIVE1_R {
        ACTIVE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details)."]
    #[inline(always)]
    pub fn active0(&self) -> ACTIVE0_R {
        ACTIVE0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details)."]
    #[inline(always)]
    pub fn active31(&mut self) -> ACTIVE31_W {
        ACTIVE31_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details)."]
    #[inline(always)]
    pub fn active30(&mut self) -> ACTIVE30_W {
        ACTIVE30_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details)."]
    #[inline(always)]
    pub fn active29(&mut self) -> ACTIVE29_W {
        ACTIVE29_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details)."]
    #[inline(always)]
    pub fn active28(&mut self) -> ACTIVE28_W {
        ACTIVE28_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details)."]
    #[inline(always)]
    pub fn active27(&mut self) -> ACTIVE27_W {
        ACTIVE27_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details)."]
    #[inline(always)]
    pub fn active26(&mut self) -> ACTIVE26_W {
        ACTIVE26_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details)."]
    #[inline(always)]
    pub fn active25(&mut self) -> ACTIVE25_W {
        ACTIVE25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details)."]
    #[inline(always)]
    pub fn active24(&mut self) -> ACTIVE24_W {
        ACTIVE24_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline(always)]
    pub fn active23(&mut self) -> ACTIVE23_W {
        ACTIVE23_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline(always)]
    pub fn active22(&mut self) -> ACTIVE22_W {
        ACTIVE22_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline(always)]
    pub fn active21(&mut self) -> ACTIVE21_W {
        ACTIVE21_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline(always)]
    pub fn active20(&mut self) -> ACTIVE20_W {
        ACTIVE20_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline(always)]
    pub fn active19(&mut self) -> ACTIVE19_W {
        ACTIVE19_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline(always)]
    pub fn active18(&mut self) -> ACTIVE18_W {
        ACTIVE18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline(always)]
    pub fn active17(&mut self) -> ACTIVE17_W {
        ACTIVE17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline(always)]
    pub fn active16(&mut self) -> ACTIVE16_W {
        ACTIVE16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline(always)]
    pub fn active15(&mut self) -> ACTIVE15_W {
        ACTIVE15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline(always)]
    pub fn active14(&mut self) -> ACTIVE14_W {
        ACTIVE14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline(always)]
    pub fn active13(&mut self) -> ACTIVE13_W {
        ACTIVE13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline(always)]
    pub fn active12(&mut self) -> ACTIVE12_W {
        ACTIVE12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline(always)]
    pub fn active11(&mut self) -> ACTIVE11_W {
        ACTIVE11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline(always)]
    pub fn active10(&mut self) -> ACTIVE10_W {
        ACTIVE10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline(always)]
    pub fn active9(&mut self) -> ACTIVE9_W {
        ACTIVE9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline(always)]
    pub fn active8(&mut self) -> ACTIVE8_W {
        ACTIVE8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details)."]
    #[inline(always)]
    pub fn active7(&mut self) -> ACTIVE7_W {
        ACTIVE7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details)."]
    #[inline(always)]
    pub fn active6(&mut self) -> ACTIVE6_W {
        ACTIVE6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details)."]
    #[inline(always)]
    pub fn active5(&mut self) -> ACTIVE5_W {
        ACTIVE5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details)."]
    #[inline(always)]
    pub fn active4(&mut self) -> ACTIVE4_W {
        ACTIVE4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details)."]
    #[inline(always)]
    pub fn active3(&mut self) -> ACTIVE3_W {
        ACTIVE3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details)."]
    #[inline(always)]
    pub fn active2(&mut self) -> ACTIVE2_W {
        ACTIVE2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details)."]
    #[inline(always)]
    pub fn active1(&mut self) -> ACTIVE1_W {
        ACTIVE1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details)."]
    #[inline(always)]
    pub fn active0(&mut self) -> ACTIVE0_W {
        ACTIVE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 0 to 31 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_iabr0](index.html) module"]
pub struct NVIC_IABR0_SPEC;
impl crate::RegisterSpec for NVIC_IABR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_iabr0::R](R) reader structure"]
impl crate::Readable for NVIC_IABR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_iabr0::W](W) writer structure"]
impl crate::Writable for NVIC_IABR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVIC_IABR0 to value 0"]
impl crate::Resettable for NVIC_IABR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
