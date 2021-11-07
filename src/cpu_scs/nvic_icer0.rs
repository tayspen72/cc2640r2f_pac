#[doc = "Register `NVIC_ICER0` reader"]
pub struct R(crate::R<NVIC_ICER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_ICER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_ICER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_ICER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_ICER0` writer"]
pub struct W(crate::W<NVIC_ICER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_ICER0_SPEC>;
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
impl From<crate::W<NVIC_ICER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_ICER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRENA31` reader - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA31_R(crate::FieldReader<bool, bool>);
impl CLRENA31_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA31` writer - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA31_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA31_W<'a> {
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
#[doc = "Field `CLRENA30` reader - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA30_R(crate::FieldReader<bool, bool>);
impl CLRENA30_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA30` writer - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA30_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA30_W<'a> {
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
#[doc = "Field `CLRENA29` reader - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA29_R(crate::FieldReader<bool, bool>);
impl CLRENA29_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA29` writer - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA29_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA29_W<'a> {
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
#[doc = "Field `CLRENA28` reader - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA28_R(crate::FieldReader<bool, bool>);
impl CLRENA28_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA28` writer - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA28_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA28_W<'a> {
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
#[doc = "Field `CLRENA27` reader - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA27_R(crate::FieldReader<bool, bool>);
impl CLRENA27_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA27` writer - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA27_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA27_W<'a> {
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
#[doc = "Field `CLRENA26` reader - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA26_R(crate::FieldReader<bool, bool>);
impl CLRENA26_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA26` writer - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA26_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA26_W<'a> {
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
#[doc = "Field `CLRENA25` reader - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA25_R(crate::FieldReader<bool, bool>);
impl CLRENA25_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA25` writer - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA25_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA25_W<'a> {
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
#[doc = "Field `CLRENA24` reader - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA24_R(crate::FieldReader<bool, bool>);
impl CLRENA24_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA24` writer - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA24_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA24_W<'a> {
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
#[doc = "Field `CLRENA23` reader - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA23_R(crate::FieldReader<bool, bool>);
impl CLRENA23_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA23` writer - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA23_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA23_W<'a> {
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
#[doc = "Field `CLRENA22` reader - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA22_R(crate::FieldReader<bool, bool>);
impl CLRENA22_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA22` writer - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA22_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA22_W<'a> {
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
#[doc = "Field `CLRENA21` reader - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA21_R(crate::FieldReader<bool, bool>);
impl CLRENA21_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA21` writer - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA21_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA21_W<'a> {
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
#[doc = "Field `CLRENA20` reader - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA20_R(crate::FieldReader<bool, bool>);
impl CLRENA20_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA20` writer - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA20_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA20_W<'a> {
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
#[doc = "Field `CLRENA19` reader - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA19_R(crate::FieldReader<bool, bool>);
impl CLRENA19_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA19` writer - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA19_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA19_W<'a> {
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
#[doc = "Field `CLRENA18` reader - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA18_R(crate::FieldReader<bool, bool>);
impl CLRENA18_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA18` writer - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA18_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA18_W<'a> {
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
#[doc = "Field `CLRENA17` reader - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA17_R(crate::FieldReader<bool, bool>);
impl CLRENA17_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA17` writer - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA17_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA17_W<'a> {
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
#[doc = "Field `CLRENA16` reader - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA16_R(crate::FieldReader<bool, bool>);
impl CLRENA16_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA16` writer - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA16_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA16_W<'a> {
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
#[doc = "Field `CLRENA15` reader - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA15_R(crate::FieldReader<bool, bool>);
impl CLRENA15_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA15` writer - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA15_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA15_W<'a> {
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
#[doc = "Field `CLRENA14` reader - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA14_R(crate::FieldReader<bool, bool>);
impl CLRENA14_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA14` writer - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA14_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA14_W<'a> {
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
#[doc = "Field `CLRENA13` reader - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA13_R(crate::FieldReader<bool, bool>);
impl CLRENA13_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA13` writer - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA13_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA13_W<'a> {
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
#[doc = "Field `CLRENA12` reader - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA12_R(crate::FieldReader<bool, bool>);
impl CLRENA12_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA12` writer - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA12_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA12_W<'a> {
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
#[doc = "Field `CLRENA11` reader - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA11_R(crate::FieldReader<bool, bool>);
impl CLRENA11_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA11` writer - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA11_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA11_W<'a> {
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
#[doc = "Field `CLRENA10` reader - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA10_R(crate::FieldReader<bool, bool>);
impl CLRENA10_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA10` writer - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA10_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA10_W<'a> {
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
#[doc = "Field `CLRENA9` reader - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA9_R(crate::FieldReader<bool, bool>);
impl CLRENA9_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA9` writer - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA9_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA9_W<'a> {
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
#[doc = "Field `CLRENA8` reader - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA8_R(crate::FieldReader<bool, bool>);
impl CLRENA8_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA8` writer - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA8_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA8_W<'a> {
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
#[doc = "Field `CLRENA7` reader - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA7_R(crate::FieldReader<bool, bool>);
impl CLRENA7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA7` writer - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA7_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA7_W<'a> {
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
#[doc = "Field `CLRENA6` reader - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA6_R(crate::FieldReader<bool, bool>);
impl CLRENA6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA6` writer - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA6_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA6_W<'a> {
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
#[doc = "Field `CLRENA5` reader - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA5_R(crate::FieldReader<bool, bool>);
impl CLRENA5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA5` writer - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA5_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA5_W<'a> {
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
#[doc = "Field `CLRENA4` reader - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA4_R(crate::FieldReader<bool, bool>);
impl CLRENA4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA4` writer - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA4_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA4_W<'a> {
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
#[doc = "Field `CLRENA3` reader - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA3_R(crate::FieldReader<bool, bool>);
impl CLRENA3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA3` writer - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA3_W<'a> {
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
#[doc = "Field `CLRENA2` reader - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA2_R(crate::FieldReader<bool, bool>);
impl CLRENA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA2` writer - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA2_W<'a> {
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
#[doc = "Field `CLRENA1` reader - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA1_R(crate::FieldReader<bool, bool>);
impl CLRENA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA1` writer - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA1_W<'a> {
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
#[doc = "Field `CLRENA0` reader - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA0_R(crate::FieldReader<bool, bool>);
impl CLRENA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRENA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA0` writer - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
pub struct CLRENA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA0_W<'a> {
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
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena31(&self) -> CLRENA31_R {
        CLRENA31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena30(&self) -> CLRENA30_R {
        CLRENA30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena29(&self) -> CLRENA29_R {
        CLRENA29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena28(&self) -> CLRENA28_R {
        CLRENA28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena27(&self) -> CLRENA27_R {
        CLRENA27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena26(&self) -> CLRENA26_R {
        CLRENA26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena25(&self) -> CLRENA25_R {
        CLRENA25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena24(&self) -> CLRENA24_R {
        CLRENA24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena23(&self) -> CLRENA23_R {
        CLRENA23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena22(&self) -> CLRENA22_R {
        CLRENA22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena21(&self) -> CLRENA21_R {
        CLRENA21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena20(&self) -> CLRENA20_R {
        CLRENA20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena19(&self) -> CLRENA19_R {
        CLRENA19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena18(&self) -> CLRENA18_R {
        CLRENA18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena17(&self) -> CLRENA17_R {
        CLRENA17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena16(&self) -> CLRENA16_R {
        CLRENA16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena15(&self) -> CLRENA15_R {
        CLRENA15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena14(&self) -> CLRENA14_R {
        CLRENA14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena13(&self) -> CLRENA13_R {
        CLRENA13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena12(&self) -> CLRENA12_R {
        CLRENA12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena11(&self) -> CLRENA11_R {
        CLRENA11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena10(&self) -> CLRENA10_R {
        CLRENA10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena9(&self) -> CLRENA9_R {
        CLRENA9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena8(&self) -> CLRENA8_R {
        CLRENA8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena7(&self) -> CLRENA7_R {
        CLRENA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena6(&self) -> CLRENA6_R {
        CLRENA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena5(&self) -> CLRENA5_R {
        CLRENA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena4(&self) -> CLRENA4_R {
        CLRENA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena3(&self) -> CLRENA3_R {
        CLRENA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena2(&self) -> CLRENA2_R {
        CLRENA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena1(&self) -> CLRENA1_R {
        CLRENA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena0(&self) -> CLRENA0_R {
        CLRENA0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena31(&mut self) -> CLRENA31_W {
        CLRENA31_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena30(&mut self) -> CLRENA30_W {
        CLRENA30_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena29(&mut self) -> CLRENA29_W {
        CLRENA29_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena28(&mut self) -> CLRENA28_W {
        CLRENA28_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena27(&mut self) -> CLRENA27_W {
        CLRENA27_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena26(&mut self) -> CLRENA26_W {
        CLRENA26_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena25(&mut self) -> CLRENA25_W {
        CLRENA25_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena24(&mut self) -> CLRENA24_W {
        CLRENA24_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena23(&mut self) -> CLRENA23_W {
        CLRENA23_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena22(&mut self) -> CLRENA22_W {
        CLRENA22_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena21(&mut self) -> CLRENA21_W {
        CLRENA21_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena20(&mut self) -> CLRENA20_W {
        CLRENA20_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena19(&mut self) -> CLRENA19_W {
        CLRENA19_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena18(&mut self) -> CLRENA18_W {
        CLRENA18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena17(&mut self) -> CLRENA17_W {
        CLRENA17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena16(&mut self) -> CLRENA16_W {
        CLRENA16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena15(&mut self) -> CLRENA15_W {
        CLRENA15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena14(&mut self) -> CLRENA14_W {
        CLRENA14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena13(&mut self) -> CLRENA13_W {
        CLRENA13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena12(&mut self) -> CLRENA12_W {
        CLRENA12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena11(&mut self) -> CLRENA11_W {
        CLRENA11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena10(&mut self) -> CLRENA10_W {
        CLRENA10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena9(&mut self) -> CLRENA9_W {
        CLRENA9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena8(&mut self) -> CLRENA8_W {
        CLRENA8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena7(&mut self) -> CLRENA7_W {
        CLRENA7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena6(&mut self) -> CLRENA6_W {
        CLRENA6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena5(&mut self) -> CLRENA5_W {
        CLRENA5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena4(&mut self) -> CLRENA4_W {
        CLRENA4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena3(&mut self) -> CLRENA3_W {
        CLRENA3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena2(&mut self) -> CLRENA2_W {
        CLRENA2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena1(&mut self) -> CLRENA1_W {
        CLRENA1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena0(&mut self) -> CLRENA0_W {
        CLRENA0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 0 to 31 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_icer0](index.html) module"]
pub struct NVIC_ICER0_SPEC;
impl crate::RegisterSpec for NVIC_ICER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_icer0::R](R) reader structure"]
impl crate::Readable for NVIC_ICER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_icer0::W](W) writer structure"]
impl crate::Writable for NVIC_ICER0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVIC_ICER0 to value 0"]
impl crate::Resettable for NVIC_ICER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
