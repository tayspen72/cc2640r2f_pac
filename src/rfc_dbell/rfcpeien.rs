#[doc = "Register `RFCPEIEN` reader"]
pub struct R(crate::R<RFCPEIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCPEIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCPEIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCPEIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFCPEIEN` writer"]
pub struct W(crate::W<RFCPEIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCPEIEN_SPEC>;
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
impl From<crate::W<RFCPEIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFCPEIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERNAL_ERROR` reader - 31:31\\]
Interrupt enable for RFCPEIFG.INTERNAL_ERROR."]
pub struct INTERNAL_ERROR_R(crate::FieldReader<bool, bool>);
impl INTERNAL_ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTERNAL_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERNAL_ERROR` writer - 31:31\\]
Interrupt enable for RFCPEIFG.INTERNAL_ERROR."]
pub struct INTERNAL_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_ERROR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `BOOT_DONE` reader - 30:30\\]
Interrupt enable for RFCPEIFG.BOOT_DONE."]
pub struct BOOT_DONE_R(crate::FieldReader<bool, bool>);
impl BOOT_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT_DONE` writer - 30:30\\]
Interrupt enable for RFCPEIFG.BOOT_DONE."]
pub struct BOOT_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MODULES_UNLOCKED` reader - 29:29\\]
Interrupt enable for RFCPEIFG.MODULES_UNLOCKED."]
pub struct MODULES_UNLOCKED_R(crate::FieldReader<bool, bool>);
impl MODULES_UNLOCKED_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODULES_UNLOCKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODULES_UNLOCKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODULES_UNLOCKED` writer - 29:29\\]
Interrupt enable for RFCPEIFG.MODULES_UNLOCKED."]
pub struct MODULES_UNLOCKED_W<'a> {
    w: &'a mut W,
}
impl<'a> MODULES_UNLOCKED_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SYNTH_NO_LOCK` reader - 28:28\\]
Interrupt enable for RFCPEIFG.SYNTH_NO_LOCK."]
pub struct SYNTH_NO_LOCK_R(crate::FieldReader<bool, bool>);
impl SYNTH_NO_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNTH_NO_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNTH_NO_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNTH_NO_LOCK` writer - 28:28\\]
Interrupt enable for RFCPEIFG.SYNTH_NO_LOCK."]
pub struct SYNTH_NO_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNTH_NO_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IRQ27` reader - 27:27\\]
Interrupt enable for RFCPEIFG.IRQ27."]
pub struct IRQ27_R(crate::FieldReader<bool, bool>);
impl IRQ27_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQ27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ27` writer - 27:27\\]
Interrupt enable for RFCPEIFG.IRQ27."]
pub struct IRQ27_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RX_ABORTED` reader - 26:26\\]
Interrupt enable for RFCPEIFG.RX_ABORTED."]
pub struct RX_ABORTED_R(crate::FieldReader<bool, bool>);
impl RX_ABORTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_ABORTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ABORTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ABORTED` writer - 26:26\\]
Interrupt enable for RFCPEIFG.RX_ABORTED."]
pub struct RX_ABORTED_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ABORTED_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RX_N_DATA_WRITTEN` reader - 25:25\\]
Interrupt enable for RFCPEIFG.RX_N_DATA_WRITTEN."]
pub struct RX_N_DATA_WRITTEN_R(crate::FieldReader<bool, bool>);
impl RX_N_DATA_WRITTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_N_DATA_WRITTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_N_DATA_WRITTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_N_DATA_WRITTEN` writer - 25:25\\]
Interrupt enable for RFCPEIFG.RX_N_DATA_WRITTEN."]
pub struct RX_N_DATA_WRITTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_N_DATA_WRITTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RX_DATA_WRITTEN` reader - 24:24\\]
Interrupt enable for RFCPEIFG.RX_DATA_WRITTEN."]
pub struct RX_DATA_WRITTEN_R(crate::FieldReader<bool, bool>);
impl RX_DATA_WRITTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_DATA_WRITTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DATA_WRITTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DATA_WRITTEN` writer - 24:24\\]
Interrupt enable for RFCPEIFG.RX_DATA_WRITTEN."]
pub struct RX_DATA_WRITTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_WRITTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RX_ENTRY_DONE` reader - 23:23\\]
Interrupt enable for RFCPEIFG.RX_ENTRY_DONE."]
pub struct RX_ENTRY_DONE_R(crate::FieldReader<bool, bool>);
impl RX_ENTRY_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_ENTRY_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ENTRY_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ENTRY_DONE` writer - 23:23\\]
Interrupt enable for RFCPEIFG.RX_ENTRY_DONE."]
pub struct RX_ENTRY_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ENTRY_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RX_BUF_FULL` reader - 22:22\\]
Interrupt enable for RFCPEIFG.RX_BUF_FULL."]
pub struct RX_BUF_FULL_R(crate::FieldReader<bool, bool>);
impl RX_BUF_FULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_BUF_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BUF_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BUF_FULL` writer - 22:22\\]
Interrupt enable for RFCPEIFG.RX_BUF_FULL."]
pub struct RX_BUF_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BUF_FULL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RX_CTRL_ACK` reader - 21:21\\]
Interrupt enable for RFCPEIFG.RX_CTRL_ACK."]
pub struct RX_CTRL_ACK_R(crate::FieldReader<bool, bool>);
impl RX_CTRL_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_CTRL_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_CTRL_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CTRL_ACK` writer - 21:21\\]
Interrupt enable for RFCPEIFG.RX_CTRL_ACK."]
pub struct RX_CTRL_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CTRL_ACK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RX_CTRL` reader - 20:20\\]
Interrupt enable for RFCPEIFG.RX_CTRL."]
pub struct RX_CTRL_R(crate::FieldReader<bool, bool>);
impl RX_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CTRL` writer - 20:20\\]
Interrupt enable for RFCPEIFG.RX_CTRL."]
pub struct RX_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CTRL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RX_EMPTY` reader - 19:19\\]
Interrupt enable for RFCPEIFG.RX_EMPTY."]
pub struct RX_EMPTY_R(crate::FieldReader<bool, bool>);
impl RX_EMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_EMPTY` writer - 19:19\\]
Interrupt enable for RFCPEIFG.RX_EMPTY."]
pub struct RX_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EMPTY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RX_IGNORED` reader - 18:18\\]
Interrupt enable for RFCPEIFG.RX_IGNORED."]
pub struct RX_IGNORED_R(crate::FieldReader<bool, bool>);
impl RX_IGNORED_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_IGNORED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_IGNORED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_IGNORED` writer - 18:18\\]
Interrupt enable for RFCPEIFG.RX_IGNORED."]
pub struct RX_IGNORED_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_IGNORED_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RX_NOK` reader - 17:17\\]
Interrupt enable for RFCPEIFG.RX_NOK."]
pub struct RX_NOK_R(crate::FieldReader<bool, bool>);
impl RX_NOK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_NOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_NOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_NOK` writer - 17:17\\]
Interrupt enable for RFCPEIFG.RX_NOK."]
pub struct RX_NOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_NOK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RX_OK` reader - 16:16\\]
Interrupt enable for RFCPEIFG.RX_OK."]
pub struct RX_OK_R(crate::FieldReader<bool, bool>);
impl RX_OK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_OK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_OK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_OK` writer - 16:16\\]
Interrupt enable for RFCPEIFG.RX_OK."]
pub struct RX_OK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IRQ15` reader - 15:15\\]
Interrupt enable for RFCPEIFG.IRQ15."]
pub struct IRQ15_R(crate::FieldReader<bool, bool>);
impl IRQ15_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQ15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ15` writer - 15:15\\]
Interrupt enable for RFCPEIFG.IRQ15."]
pub struct IRQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IRQ14` reader - 14:14\\]
Interrupt enable for RFCPEIFG.IRQ14."]
pub struct IRQ14_R(crate::FieldReader<bool, bool>);
impl IRQ14_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQ14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ14` writer - 14:14\\]
Interrupt enable for RFCPEIFG.IRQ14."]
pub struct IRQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IRQ13` reader - 13:13\\]
Interrupt enable for RFCPEIFG.IRQ13."]
pub struct IRQ13_R(crate::FieldReader<bool, bool>);
impl IRQ13_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQ13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ13` writer - 13:13\\]
Interrupt enable for RFCPEIFG.IRQ13."]
pub struct IRQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IRQ12` reader - 12:12\\]
Interrupt enable for RFCPEIFG.IRQ12."]
pub struct IRQ12_R(crate::FieldReader<bool, bool>);
impl IRQ12_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQ12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ12` writer - 12:12\\]
Interrupt enable for RFCPEIFG.IRQ12."]
pub struct IRQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TX_BUFFER_CHANGED` reader - 11:11\\]
Interrupt enable for RFCPEIFG.TX_BUFFER_CHANGED."]
pub struct TX_BUFFER_CHANGED_R(crate::FieldReader<bool, bool>);
impl TX_BUFFER_CHANGED_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_BUFFER_CHANGED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BUFFER_CHANGED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BUFFER_CHANGED` writer - 11:11\\]
Interrupt enable for RFCPEIFG.TX_BUFFER_CHANGED."]
pub struct TX_BUFFER_CHANGED_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BUFFER_CHANGED_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TX_ENTRY_DONE` reader - 10:10\\]
Interrupt enable for RFCPEIFG.TX_ENTRY_DONE."]
pub struct TX_ENTRY_DONE_R(crate::FieldReader<bool, bool>);
impl TX_ENTRY_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_ENTRY_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_ENTRY_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ENTRY_DONE` writer - 10:10\\]
Interrupt enable for RFCPEIFG.TX_ENTRY_DONE."]
pub struct TX_ENTRY_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ENTRY_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TX_RETRANS` reader - 9:9\\]
Interrupt enable for RFCPEIFG.TX_RETRANS."]
pub struct TX_RETRANS_R(crate::FieldReader<bool, bool>);
impl TX_RETRANS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_RETRANS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_RETRANS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_RETRANS` writer - 9:9\\]
Interrupt enable for RFCPEIFG.TX_RETRANS."]
pub struct TX_RETRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RETRANS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TX_CTRL_ACK_ACK` reader - 8:8\\]
Interrupt enable for RFCPEIFG.TX_CTRL_ACK_ACK."]
pub struct TX_CTRL_ACK_ACK_R(crate::FieldReader<bool, bool>);
impl TX_CTRL_ACK_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_CTRL_ACK_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CTRL_ACK_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CTRL_ACK_ACK` writer - 8:8\\]
Interrupt enable for RFCPEIFG.TX_CTRL_ACK_ACK."]
pub struct TX_CTRL_ACK_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CTRL_ACK_ACK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TX_CTRL_ACK` reader - 7:7\\]
Interrupt enable for RFCPEIFG.TX_CTRL_ACK."]
pub struct TX_CTRL_ACK_R(crate::FieldReader<bool, bool>);
impl TX_CTRL_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_CTRL_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CTRL_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CTRL_ACK` writer - 7:7\\]
Interrupt enable for RFCPEIFG.TX_CTRL_ACK."]
pub struct TX_CTRL_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CTRL_ACK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TX_CTRL` reader - 6:6\\]
Interrupt enable for RFCPEIFG.TX_CTRL."]
pub struct TX_CTRL_R(crate::FieldReader<bool, bool>);
impl TX_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CTRL` writer - 6:6\\]
Interrupt enable for RFCPEIFG.TX_CTRL."]
pub struct TX_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CTRL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TX_ACK` reader - 5:5\\]
Interrupt enable for RFCPEIFG.TX_ACK."]
pub struct TX_ACK_R(crate::FieldReader<bool, bool>);
impl TX_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ACK` writer - 5:5\\]
Interrupt enable for RFCPEIFG.TX_ACK."]
pub struct TX_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ACK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TX_DONE` reader - 4:4\\]
Interrupt enable for RFCPEIFG.TX_DONE."]
pub struct TX_DONE_R(crate::FieldReader<bool, bool>);
impl TX_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DONE` writer - 4:4\\]
Interrupt enable for RFCPEIFG.TX_DONE."]
pub struct TX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LAST_FG_COMMAND_DONE` reader - 3:3\\]
Interrupt enable for RFCPEIFG.LAST_FG_COMMAND_DONE."]
pub struct LAST_FG_COMMAND_DONE_R(crate::FieldReader<bool, bool>);
impl LAST_FG_COMMAND_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LAST_FG_COMMAND_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LAST_FG_COMMAND_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LAST_FG_COMMAND_DONE` writer - 3:3\\]
Interrupt enable for RFCPEIFG.LAST_FG_COMMAND_DONE."]
pub struct LAST_FG_COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_FG_COMMAND_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FG_COMMAND_DONE` reader - 2:2\\]
Interrupt enable for RFCPEIFG.FG_COMMAND_DONE."]
pub struct FG_COMMAND_DONE_R(crate::FieldReader<bool, bool>);
impl FG_COMMAND_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FG_COMMAND_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FG_COMMAND_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FG_COMMAND_DONE` writer - 2:2\\]
Interrupt enable for RFCPEIFG.FG_COMMAND_DONE."]
pub struct FG_COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> FG_COMMAND_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LAST_COMMAND_DONE` reader - 1:1\\]
Interrupt enable for RFCPEIFG.LAST_COMMAND_DONE."]
pub struct LAST_COMMAND_DONE_R(crate::FieldReader<bool, bool>);
impl LAST_COMMAND_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LAST_COMMAND_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LAST_COMMAND_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LAST_COMMAND_DONE` writer - 1:1\\]
Interrupt enable for RFCPEIFG.LAST_COMMAND_DONE."]
pub struct LAST_COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_COMMAND_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `COMMAND_DONE` reader - 0:0\\]
Interrupt enable for RFCPEIFG.COMMAND_DONE."]
pub struct COMMAND_DONE_R(crate::FieldReader<bool, bool>);
impl COMMAND_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMMAND_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND_DONE` writer - 0:0\\]
Interrupt enable for RFCPEIFG.COMMAND_DONE."]
pub struct COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
Interrupt enable for RFCPEIFG.INTERNAL_ERROR."]
    #[inline(always)]
    pub fn internal_error(&self) -> INTERNAL_ERROR_R {
        INTERNAL_ERROR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Interrupt enable for RFCPEIFG.BOOT_DONE."]
    #[inline(always)]
    pub fn boot_done(&self) -> BOOT_DONE_R {
        BOOT_DONE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Interrupt enable for RFCPEIFG.MODULES_UNLOCKED."]
    #[inline(always)]
    pub fn modules_unlocked(&self) -> MODULES_UNLOCKED_R {
        MODULES_UNLOCKED_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Interrupt enable for RFCPEIFG.SYNTH_NO_LOCK."]
    #[inline(always)]
    pub fn synth_no_lock(&self) -> SYNTH_NO_LOCK_R {
        SYNTH_NO_LOCK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Interrupt enable for RFCPEIFG.IRQ27."]
    #[inline(always)]
    pub fn irq27(&self) -> IRQ27_R {
        IRQ27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Interrupt enable for RFCPEIFG.RX_ABORTED."]
    #[inline(always)]
    pub fn rx_aborted(&self) -> RX_ABORTED_R {
        RX_ABORTED_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Interrupt enable for RFCPEIFG.RX_N_DATA_WRITTEN."]
    #[inline(always)]
    pub fn rx_n_data_written(&self) -> RX_N_DATA_WRITTEN_R {
        RX_N_DATA_WRITTEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Interrupt enable for RFCPEIFG.RX_DATA_WRITTEN."]
    #[inline(always)]
    pub fn rx_data_written(&self) -> RX_DATA_WRITTEN_R {
        RX_DATA_WRITTEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Interrupt enable for RFCPEIFG.RX_ENTRY_DONE."]
    #[inline(always)]
    pub fn rx_entry_done(&self) -> RX_ENTRY_DONE_R {
        RX_ENTRY_DONE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt enable for RFCPEIFG.RX_BUF_FULL."]
    #[inline(always)]
    pub fn rx_buf_full(&self) -> RX_BUF_FULL_R {
        RX_BUF_FULL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Interrupt enable for RFCPEIFG.RX_CTRL_ACK."]
    #[inline(always)]
    pub fn rx_ctrl_ack(&self) -> RX_CTRL_ACK_R {
        RX_CTRL_ACK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Interrupt enable for RFCPEIFG.RX_CTRL."]
    #[inline(always)]
    pub fn rx_ctrl(&self) -> RX_CTRL_R {
        RX_CTRL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt enable for RFCPEIFG.RX_EMPTY."]
    #[inline(always)]
    pub fn rx_empty(&self) -> RX_EMPTY_R {
        RX_EMPTY_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt enable for RFCPEIFG.RX_IGNORED."]
    #[inline(always)]
    pub fn rx_ignored(&self) -> RX_IGNORED_R {
        RX_IGNORED_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt enable for RFCPEIFG.RX_NOK."]
    #[inline(always)]
    pub fn rx_nok(&self) -> RX_NOK_R {
        RX_NOK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt enable for RFCPEIFG.RX_OK."]
    #[inline(always)]
    pub fn rx_ok(&self) -> RX_OK_R {
        RX_OK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt enable for RFCPEIFG.IRQ15."]
    #[inline(always)]
    pub fn irq15(&self) -> IRQ15_R {
        IRQ15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt enable for RFCPEIFG.IRQ14."]
    #[inline(always)]
    pub fn irq14(&self) -> IRQ14_R {
        IRQ14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt enable for RFCPEIFG.IRQ13."]
    #[inline(always)]
    pub fn irq13(&self) -> IRQ13_R {
        IRQ13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt enable for RFCPEIFG.IRQ12."]
    #[inline(always)]
    pub fn irq12(&self) -> IRQ12_R {
        IRQ12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt enable for RFCPEIFG.TX_BUFFER_CHANGED."]
    #[inline(always)]
    pub fn tx_buffer_changed(&self) -> TX_BUFFER_CHANGED_R {
        TX_BUFFER_CHANGED_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt enable for RFCPEIFG.TX_ENTRY_DONE."]
    #[inline(always)]
    pub fn tx_entry_done(&self) -> TX_ENTRY_DONE_R {
        TX_ENTRY_DONE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt enable for RFCPEIFG.TX_RETRANS."]
    #[inline(always)]
    pub fn tx_retrans(&self) -> TX_RETRANS_R {
        TX_RETRANS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt enable for RFCPEIFG.TX_CTRL_ACK_ACK."]
    #[inline(always)]
    pub fn tx_ctrl_ack_ack(&self) -> TX_CTRL_ACK_ACK_R {
        TX_CTRL_ACK_ACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt enable for RFCPEIFG.TX_CTRL_ACK."]
    #[inline(always)]
    pub fn tx_ctrl_ack(&self) -> TX_CTRL_ACK_R {
        TX_CTRL_ACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt enable for RFCPEIFG.TX_CTRL."]
    #[inline(always)]
    pub fn tx_ctrl(&self) -> TX_CTRL_R {
        TX_CTRL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt enable for RFCPEIFG.TX_ACK."]
    #[inline(always)]
    pub fn tx_ack(&self) -> TX_ACK_R {
        TX_ACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt enable for RFCPEIFG.TX_DONE."]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt enable for RFCPEIFG.LAST_FG_COMMAND_DONE."]
    #[inline(always)]
    pub fn last_fg_command_done(&self) -> LAST_FG_COMMAND_DONE_R {
        LAST_FG_COMMAND_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt enable for RFCPEIFG.FG_COMMAND_DONE."]
    #[inline(always)]
    pub fn fg_command_done(&self) -> FG_COMMAND_DONE_R {
        FG_COMMAND_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt enable for RFCPEIFG.LAST_COMMAND_DONE."]
    #[inline(always)]
    pub fn last_command_done(&self) -> LAST_COMMAND_DONE_R {
        LAST_COMMAND_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Interrupt enable for RFCPEIFG.COMMAND_DONE."]
    #[inline(always)]
    pub fn command_done(&self) -> COMMAND_DONE_R {
        COMMAND_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Interrupt enable for RFCPEIFG.INTERNAL_ERROR."]
    #[inline(always)]
    pub fn internal_error(&mut self) -> INTERNAL_ERROR_W {
        INTERNAL_ERROR_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Interrupt enable for RFCPEIFG.BOOT_DONE."]
    #[inline(always)]
    pub fn boot_done(&mut self) -> BOOT_DONE_W {
        BOOT_DONE_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Interrupt enable for RFCPEIFG.MODULES_UNLOCKED."]
    #[inline(always)]
    pub fn modules_unlocked(&mut self) -> MODULES_UNLOCKED_W {
        MODULES_UNLOCKED_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Interrupt enable for RFCPEIFG.SYNTH_NO_LOCK."]
    #[inline(always)]
    pub fn synth_no_lock(&mut self) -> SYNTH_NO_LOCK_W {
        SYNTH_NO_LOCK_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Interrupt enable for RFCPEIFG.IRQ27."]
    #[inline(always)]
    pub fn irq27(&mut self) -> IRQ27_W {
        IRQ27_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
Interrupt enable for RFCPEIFG.RX_ABORTED."]
    #[inline(always)]
    pub fn rx_aborted(&mut self) -> RX_ABORTED_W {
        RX_ABORTED_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Interrupt enable for RFCPEIFG.RX_N_DATA_WRITTEN."]
    #[inline(always)]
    pub fn rx_n_data_written(&mut self) -> RX_N_DATA_WRITTEN_W {
        RX_N_DATA_WRITTEN_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Interrupt enable for RFCPEIFG.RX_DATA_WRITTEN."]
    #[inline(always)]
    pub fn rx_data_written(&mut self) -> RX_DATA_WRITTEN_W {
        RX_DATA_WRITTEN_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Interrupt enable for RFCPEIFG.RX_ENTRY_DONE."]
    #[inline(always)]
    pub fn rx_entry_done(&mut self) -> RX_ENTRY_DONE_W {
        RX_ENTRY_DONE_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt enable for RFCPEIFG.RX_BUF_FULL."]
    #[inline(always)]
    pub fn rx_buf_full(&mut self) -> RX_BUF_FULL_W {
        RX_BUF_FULL_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Interrupt enable for RFCPEIFG.RX_CTRL_ACK."]
    #[inline(always)]
    pub fn rx_ctrl_ack(&mut self) -> RX_CTRL_ACK_W {
        RX_CTRL_ACK_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Interrupt enable for RFCPEIFG.RX_CTRL."]
    #[inline(always)]
    pub fn rx_ctrl(&mut self) -> RX_CTRL_W {
        RX_CTRL_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt enable for RFCPEIFG.RX_EMPTY."]
    #[inline(always)]
    pub fn rx_empty(&mut self) -> RX_EMPTY_W {
        RX_EMPTY_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt enable for RFCPEIFG.RX_IGNORED."]
    #[inline(always)]
    pub fn rx_ignored(&mut self) -> RX_IGNORED_W {
        RX_IGNORED_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt enable for RFCPEIFG.RX_NOK."]
    #[inline(always)]
    pub fn rx_nok(&mut self) -> RX_NOK_W {
        RX_NOK_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt enable for RFCPEIFG.RX_OK."]
    #[inline(always)]
    pub fn rx_ok(&mut self) -> RX_OK_W {
        RX_OK_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt enable for RFCPEIFG.IRQ15."]
    #[inline(always)]
    pub fn irq15(&mut self) -> IRQ15_W {
        IRQ15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt enable for RFCPEIFG.IRQ14."]
    #[inline(always)]
    pub fn irq14(&mut self) -> IRQ14_W {
        IRQ14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt enable for RFCPEIFG.IRQ13."]
    #[inline(always)]
    pub fn irq13(&mut self) -> IRQ13_W {
        IRQ13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt enable for RFCPEIFG.IRQ12."]
    #[inline(always)]
    pub fn irq12(&mut self) -> IRQ12_W {
        IRQ12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt enable for RFCPEIFG.TX_BUFFER_CHANGED."]
    #[inline(always)]
    pub fn tx_buffer_changed(&mut self) -> TX_BUFFER_CHANGED_W {
        TX_BUFFER_CHANGED_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt enable for RFCPEIFG.TX_ENTRY_DONE."]
    #[inline(always)]
    pub fn tx_entry_done(&mut self) -> TX_ENTRY_DONE_W {
        TX_ENTRY_DONE_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt enable for RFCPEIFG.TX_RETRANS."]
    #[inline(always)]
    pub fn tx_retrans(&mut self) -> TX_RETRANS_W {
        TX_RETRANS_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt enable for RFCPEIFG.TX_CTRL_ACK_ACK."]
    #[inline(always)]
    pub fn tx_ctrl_ack_ack(&mut self) -> TX_CTRL_ACK_ACK_W {
        TX_CTRL_ACK_ACK_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt enable for RFCPEIFG.TX_CTRL_ACK."]
    #[inline(always)]
    pub fn tx_ctrl_ack(&mut self) -> TX_CTRL_ACK_W {
        TX_CTRL_ACK_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt enable for RFCPEIFG.TX_CTRL."]
    #[inline(always)]
    pub fn tx_ctrl(&mut self) -> TX_CTRL_W {
        TX_CTRL_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt enable for RFCPEIFG.TX_ACK."]
    #[inline(always)]
    pub fn tx_ack(&mut self) -> TX_ACK_W {
        TX_ACK_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt enable for RFCPEIFG.TX_DONE."]
    #[inline(always)]
    pub fn tx_done(&mut self) -> TX_DONE_W {
        TX_DONE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt enable for RFCPEIFG.LAST_FG_COMMAND_DONE."]
    #[inline(always)]
    pub fn last_fg_command_done(&mut self) -> LAST_FG_COMMAND_DONE_W {
        LAST_FG_COMMAND_DONE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt enable for RFCPEIFG.FG_COMMAND_DONE."]
    #[inline(always)]
    pub fn fg_command_done(&mut self) -> FG_COMMAND_DONE_W {
        FG_COMMAND_DONE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt enable for RFCPEIFG.LAST_COMMAND_DONE."]
    #[inline(always)]
    pub fn last_command_done(&mut self) -> LAST_COMMAND_DONE_W {
        LAST_COMMAND_DONE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Interrupt enable for RFCPEIFG.COMMAND_DONE."]
    #[inline(always)]
    pub fn command_done(&mut self) -> COMMAND_DONE_W {
        COMMAND_DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable For Command and Packet Engine Generated Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcpeien](index.html) module"]
pub struct RFCPEIEN_SPEC;
impl crate::RegisterSpec for RFCPEIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcpeien::R](R) reader structure"]
impl crate::Readable for RFCPEIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcpeien::W](W) writer structure"]
impl crate::Writable for RFCPEIEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFCPEIEN to value 0xffff_ffff"]
impl crate::Resettable for RFCPEIEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
