#[doc = "Register `RFCPEISL` reader"]
pub struct R(crate::R<RFCPEISL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCPEISL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCPEISL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCPEISL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFCPEISL` writer"]
pub struct W(crate::W<RFCPEISL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCPEISL_SPEC>;
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
impl From<crate::W<RFCPEISL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFCPEISL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "31:31\\]
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERNAL_ERROR_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<INTERNAL_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: INTERNAL_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTERNAL_ERROR` reader - 31:31\\]
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
pub struct INTERNAL_ERROR_R(crate::FieldReader<bool, INTERNAL_ERROR_A>);
impl INTERNAL_ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTERNAL_ERROR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERNAL_ERROR_A {
        match self.bits {
            true => INTERNAL_ERROR_A::CPE1,
            false => INTERNAL_ERROR_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == INTERNAL_ERROR_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == INTERNAL_ERROR_A::CPE0
    }
}
impl core::ops::Deref for INTERNAL_ERROR_R {
    type Target = crate::FieldReader<bool, INTERNAL_ERROR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERNAL_ERROR` writer - 31:31\\]
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
pub struct INTERNAL_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTERNAL_ERROR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(INTERNAL_ERROR_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(INTERNAL_ERROR_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<BOOT_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: BOOT_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOT_DONE` reader - 30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
pub struct BOOT_DONE_R(crate::FieldReader<bool, BOOT_DONE_A>);
impl BOOT_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT_DONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_DONE_A {
        match self.bits {
            true => BOOT_DONE_A::CPE1,
            false => BOOT_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == BOOT_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == BOOT_DONE_A::CPE0
    }
}
impl core::ops::Deref for BOOT_DONE_R {
    type Target = crate::FieldReader<bool, BOOT_DONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT_DONE` writer - 30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
pub struct BOOT_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOT_DONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(BOOT_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(BOOT_DONE_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODULES_UNLOCKED_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<MODULES_UNLOCKED_A> for bool {
    #[inline(always)]
    fn from(variant: MODULES_UNLOCKED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODULES_UNLOCKED` reader - 29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
pub struct MODULES_UNLOCKED_R(crate::FieldReader<bool, MODULES_UNLOCKED_A>);
impl MODULES_UNLOCKED_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODULES_UNLOCKED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODULES_UNLOCKED_A {
        match self.bits {
            true => MODULES_UNLOCKED_A::CPE1,
            false => MODULES_UNLOCKED_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == MODULES_UNLOCKED_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == MODULES_UNLOCKED_A::CPE0
    }
}
impl core::ops::Deref for MODULES_UNLOCKED_R {
    type Target = crate::FieldReader<bool, MODULES_UNLOCKED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODULES_UNLOCKED` writer - 29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
pub struct MODULES_UNLOCKED_W<'a> {
    w: &'a mut W,
}
impl<'a> MODULES_UNLOCKED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODULES_UNLOCKED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(MODULES_UNLOCKED_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(MODULES_UNLOCKED_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNTH_NO_LOCK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<SYNTH_NO_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: SYNTH_NO_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNTH_NO_LOCK` reader - 28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
pub struct SYNTH_NO_LOCK_R(crate::FieldReader<bool, SYNTH_NO_LOCK_A>);
impl SYNTH_NO_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNTH_NO_LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNTH_NO_LOCK_A {
        match self.bits {
            true => SYNTH_NO_LOCK_A::CPE1,
            false => SYNTH_NO_LOCK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == SYNTH_NO_LOCK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == SYNTH_NO_LOCK_A::CPE0
    }
}
impl core::ops::Deref for SYNTH_NO_LOCK_R {
    type Target = crate::FieldReader<bool, SYNTH_NO_LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNTH_NO_LOCK` writer - 28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
pub struct SYNTH_NO_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNTH_NO_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNTH_NO_LOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(SYNTH_NO_LOCK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(SYNTH_NO_LOCK_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ27_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<IRQ27_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ27` reader - 27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
pub struct IRQ27_R(crate::FieldReader<bool, IRQ27_A>);
impl IRQ27_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQ27_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ27_A {
        match self.bits {
            true => IRQ27_A::CPE1,
            false => IRQ27_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == IRQ27_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == IRQ27_A::CPE0
    }
}
impl core::ops::Deref for IRQ27_R {
    type Target = crate::FieldReader<bool, IRQ27_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ27` writer - 27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
pub struct IRQ27_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQ27_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ27_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ27_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ABORTED_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_ABORTED_A> for bool {
    #[inline(always)]
    fn from(variant: RX_ABORTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_ABORTED` reader - 26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
pub struct RX_ABORTED_R(crate::FieldReader<bool, RX_ABORTED_A>);
impl RX_ABORTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_ABORTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_ABORTED_A {
        match self.bits {
            true => RX_ABORTED_A::CPE1,
            false => RX_ABORTED_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == RX_ABORTED_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == RX_ABORTED_A::CPE0
    }
}
impl core::ops::Deref for RX_ABORTED_R {
    type Target = crate::FieldReader<bool, RX_ABORTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ABORTED` writer - 26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
pub struct RX_ABORTED_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ABORTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_ABORTED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_ABORTED_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_ABORTED_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_N_DATA_WRITTEN_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_N_DATA_WRITTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_N_DATA_WRITTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_N_DATA_WRITTEN` reader - 25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
pub struct RX_N_DATA_WRITTEN_R(crate::FieldReader<bool, RX_N_DATA_WRITTEN_A>);
impl RX_N_DATA_WRITTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_N_DATA_WRITTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_N_DATA_WRITTEN_A {
        match self.bits {
            true => RX_N_DATA_WRITTEN_A::CPE1,
            false => RX_N_DATA_WRITTEN_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == RX_N_DATA_WRITTEN_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == RX_N_DATA_WRITTEN_A::CPE0
    }
}
impl core::ops::Deref for RX_N_DATA_WRITTEN_R {
    type Target = crate::FieldReader<bool, RX_N_DATA_WRITTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_N_DATA_WRITTEN` writer - 25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
pub struct RX_N_DATA_WRITTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_N_DATA_WRITTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_N_DATA_WRITTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_N_DATA_WRITTEN_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_N_DATA_WRITTEN_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DATA_WRITTEN_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_DATA_WRITTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_DATA_WRITTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_DATA_WRITTEN` reader - 24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
pub struct RX_DATA_WRITTEN_R(crate::FieldReader<bool, RX_DATA_WRITTEN_A>);
impl RX_DATA_WRITTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_DATA_WRITTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_DATA_WRITTEN_A {
        match self.bits {
            true => RX_DATA_WRITTEN_A::CPE1,
            false => RX_DATA_WRITTEN_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == RX_DATA_WRITTEN_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == RX_DATA_WRITTEN_A::CPE0
    }
}
impl core::ops::Deref for RX_DATA_WRITTEN_R {
    type Target = crate::FieldReader<bool, RX_DATA_WRITTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DATA_WRITTEN` writer - 24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
pub struct RX_DATA_WRITTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_WRITTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_DATA_WRITTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_DATA_WRITTEN_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_DATA_WRITTEN_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ENTRY_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_ENTRY_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: RX_ENTRY_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_ENTRY_DONE` reader - 23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
pub struct RX_ENTRY_DONE_R(crate::FieldReader<bool, RX_ENTRY_DONE_A>);
impl RX_ENTRY_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_ENTRY_DONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_ENTRY_DONE_A {
        match self.bits {
            true => RX_ENTRY_DONE_A::CPE1,
            false => RX_ENTRY_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == RX_ENTRY_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == RX_ENTRY_DONE_A::CPE0
    }
}
impl core::ops::Deref for RX_ENTRY_DONE_R {
    type Target = crate::FieldReader<bool, RX_ENTRY_DONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ENTRY_DONE` writer - 23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
pub struct RX_ENTRY_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ENTRY_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_ENTRY_DONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_ENTRY_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_ENTRY_DONE_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_BUF_FULL_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_BUF_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_BUF_FULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_BUF_FULL` reader - 22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
pub struct RX_BUF_FULL_R(crate::FieldReader<bool, RX_BUF_FULL_A>);
impl RX_BUF_FULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_BUF_FULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_BUF_FULL_A {
        match self.bits {
            true => RX_BUF_FULL_A::CPE1,
            false => RX_BUF_FULL_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == RX_BUF_FULL_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == RX_BUF_FULL_A::CPE0
    }
}
impl core::ops::Deref for RX_BUF_FULL_R {
    type Target = crate::FieldReader<bool, RX_BUF_FULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BUF_FULL` writer - 22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
pub struct RX_BUF_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BUF_FULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_BUF_FULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_BUF_FULL_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_BUF_FULL_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_CTRL_ACK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_CTRL_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: RX_CTRL_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_CTRL_ACK` reader - 21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
pub struct RX_CTRL_ACK_R(crate::FieldReader<bool, RX_CTRL_ACK_A>);
impl RX_CTRL_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_CTRL_ACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_CTRL_ACK_A {
        match self.bits {
            true => RX_CTRL_ACK_A::CPE1,
            false => RX_CTRL_ACK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == RX_CTRL_ACK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == RX_CTRL_ACK_A::CPE0
    }
}
impl core::ops::Deref for RX_CTRL_ACK_R {
    type Target = crate::FieldReader<bool, RX_CTRL_ACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CTRL_ACK` writer - 21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
pub struct RX_CTRL_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CTRL_ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_CTRL_ACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_CTRL_ACK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_CTRL_ACK_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_CTRL_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_CTRL` reader - 20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
pub struct RX_CTRL_R(crate::FieldReader<bool, RX_CTRL_A>);
impl RX_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_CTRL_A {
        match self.bits {
            true => RX_CTRL_A::CPE1,
            false => RX_CTRL_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == RX_CTRL_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == RX_CTRL_A::CPE0
    }
}
impl core::ops::Deref for RX_CTRL_R {
    type Target = crate::FieldReader<bool, RX_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CTRL` writer - 20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
pub struct RX_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_CTRL_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_CTRL_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_EMPTY_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: RX_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_EMPTY` reader - 19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
pub struct RX_EMPTY_R(crate::FieldReader<bool, RX_EMPTY_A>);
impl RX_EMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_EMPTY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_EMPTY_A {
        match self.bits {
            true => RX_EMPTY_A::CPE1,
            false => RX_EMPTY_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == RX_EMPTY_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == RX_EMPTY_A::CPE0
    }
}
impl core::ops::Deref for RX_EMPTY_R {
    type Target = crate::FieldReader<bool, RX_EMPTY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_EMPTY` writer - 19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
pub struct RX_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EMPTY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_EMPTY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_EMPTY_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_EMPTY_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_IGNORED_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_IGNORED_A> for bool {
    #[inline(always)]
    fn from(variant: RX_IGNORED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_IGNORED` reader - 18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
pub struct RX_IGNORED_R(crate::FieldReader<bool, RX_IGNORED_A>);
impl RX_IGNORED_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_IGNORED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_IGNORED_A {
        match self.bits {
            true => RX_IGNORED_A::CPE1,
            false => RX_IGNORED_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == RX_IGNORED_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == RX_IGNORED_A::CPE0
    }
}
impl core::ops::Deref for RX_IGNORED_R {
    type Target = crate::FieldReader<bool, RX_IGNORED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_IGNORED` writer - 18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
pub struct RX_IGNORED_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_IGNORED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_IGNORED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_IGNORED_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_IGNORED_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_NOK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_NOK_A> for bool {
    #[inline(always)]
    fn from(variant: RX_NOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_NOK` reader - 17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
pub struct RX_NOK_R(crate::FieldReader<bool, RX_NOK_A>);
impl RX_NOK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_NOK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_NOK_A {
        match self.bits {
            true => RX_NOK_A::CPE1,
            false => RX_NOK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == RX_NOK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == RX_NOK_A::CPE0
    }
}
impl core::ops::Deref for RX_NOK_R {
    type Target = crate::FieldReader<bool, RX_NOK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_NOK` writer - 17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
pub struct RX_NOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_NOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_NOK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_NOK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_NOK_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_OK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<RX_OK_A> for bool {
    #[inline(always)]
    fn from(variant: RX_OK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_OK` reader - 16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
pub struct RX_OK_R(crate::FieldReader<bool, RX_OK_A>);
impl RX_OK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_OK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_OK_A {
        match self.bits {
            true => RX_OK_A::CPE1,
            false => RX_OK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == RX_OK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == RX_OK_A::CPE0
    }
}
impl core::ops::Deref for RX_OK_R {
    type Target = crate::FieldReader<bool, RX_OK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_OK` writer - 16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
pub struct RX_OK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_OK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(RX_OK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(RX_OK_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ15_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<IRQ15_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ15` reader - 15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
pub struct IRQ15_R(crate::FieldReader<bool, IRQ15_A>);
impl IRQ15_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQ15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ15_A {
        match self.bits {
            true => IRQ15_A::CPE1,
            false => IRQ15_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == IRQ15_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == IRQ15_A::CPE0
    }
}
impl core::ops::Deref for IRQ15_R {
    type Target = crate::FieldReader<bool, IRQ15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ15` writer - 15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
pub struct IRQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQ15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ15_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ15_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ14_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<IRQ14_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ14` reader - 14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
pub struct IRQ14_R(crate::FieldReader<bool, IRQ14_A>);
impl IRQ14_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQ14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ14_A {
        match self.bits {
            true => IRQ14_A::CPE1,
            false => IRQ14_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == IRQ14_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == IRQ14_A::CPE0
    }
}
impl core::ops::Deref for IRQ14_R {
    type Target = crate::FieldReader<bool, IRQ14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ14` writer - 14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
pub struct IRQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQ14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ14_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ14_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "13:13\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ13_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<IRQ13_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ13` reader - 13:13\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use."]
pub struct IRQ13_R(crate::FieldReader<bool, IRQ13_A>);
impl IRQ13_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQ13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ13_A {
        match self.bits {
            true => IRQ13_A::CPE1,
            false => IRQ13_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == IRQ13_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == IRQ13_A::CPE0
    }
}
impl core::ops::Deref for IRQ13_R {
    type Target = crate::FieldReader<bool, IRQ13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ13` writer - 13:13\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use."]
pub struct IRQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQ13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ13_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ13_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "12:12\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ12_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<IRQ12_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ12` reader - 12:12\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use."]
pub struct IRQ12_R(crate::FieldReader<bool, IRQ12_A>);
impl IRQ12_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQ12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ12_A {
        match self.bits {
            true => IRQ12_A::CPE1,
            false => IRQ12_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == IRQ12_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == IRQ12_A::CPE0
    }
}
impl core::ops::Deref for IRQ12_R {
    type Target = crate::FieldReader<bool, IRQ12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ12` writer - 12:12\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use."]
pub struct IRQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQ12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(IRQ12_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(IRQ12_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_BUFFER_CHANGED_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_BUFFER_CHANGED_A> for bool {
    #[inline(always)]
    fn from(variant: TX_BUFFER_CHANGED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_BUFFER_CHANGED` reader - 11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
pub struct TX_BUFFER_CHANGED_R(crate::FieldReader<bool, TX_BUFFER_CHANGED_A>);
impl TX_BUFFER_CHANGED_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_BUFFER_CHANGED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_BUFFER_CHANGED_A {
        match self.bits {
            true => TX_BUFFER_CHANGED_A::CPE1,
            false => TX_BUFFER_CHANGED_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == TX_BUFFER_CHANGED_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == TX_BUFFER_CHANGED_A::CPE0
    }
}
impl core::ops::Deref for TX_BUFFER_CHANGED_R {
    type Target = crate::FieldReader<bool, TX_BUFFER_CHANGED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BUFFER_CHANGED` writer - 11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
pub struct TX_BUFFER_CHANGED_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BUFFER_CHANGED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_BUFFER_CHANGED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_BUFFER_CHANGED_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_BUFFER_CHANGED_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_ENTRY_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_ENTRY_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_ENTRY_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_ENTRY_DONE` reader - 10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
pub struct TX_ENTRY_DONE_R(crate::FieldReader<bool, TX_ENTRY_DONE_A>);
impl TX_ENTRY_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_ENTRY_DONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_ENTRY_DONE_A {
        match self.bits {
            true => TX_ENTRY_DONE_A::CPE1,
            false => TX_ENTRY_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == TX_ENTRY_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == TX_ENTRY_DONE_A::CPE0
    }
}
impl core::ops::Deref for TX_ENTRY_DONE_R {
    type Target = crate::FieldReader<bool, TX_ENTRY_DONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ENTRY_DONE` writer - 10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
pub struct TX_ENTRY_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ENTRY_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_ENTRY_DONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_ENTRY_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_ENTRY_DONE_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_RETRANS_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_RETRANS_A> for bool {
    #[inline(always)]
    fn from(variant: TX_RETRANS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_RETRANS` reader - 9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
pub struct TX_RETRANS_R(crate::FieldReader<bool, TX_RETRANS_A>);
impl TX_RETRANS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_RETRANS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_RETRANS_A {
        match self.bits {
            true => TX_RETRANS_A::CPE1,
            false => TX_RETRANS_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == TX_RETRANS_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == TX_RETRANS_A::CPE0
    }
}
impl core::ops::Deref for TX_RETRANS_R {
    type Target = crate::FieldReader<bool, TX_RETRANS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_RETRANS` writer - 9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
pub struct TX_RETRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RETRANS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_RETRANS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_RETRANS_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_RETRANS_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_CTRL_ACK_ACK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_CTRL_ACK_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: TX_CTRL_ACK_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_CTRL_ACK_ACK` reader - 8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
pub struct TX_CTRL_ACK_ACK_R(crate::FieldReader<bool, TX_CTRL_ACK_ACK_A>);
impl TX_CTRL_ACK_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_CTRL_ACK_ACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_CTRL_ACK_ACK_A {
        match self.bits {
            true => TX_CTRL_ACK_ACK_A::CPE1,
            false => TX_CTRL_ACK_ACK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == TX_CTRL_ACK_ACK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == TX_CTRL_ACK_ACK_A::CPE0
    }
}
impl core::ops::Deref for TX_CTRL_ACK_ACK_R {
    type Target = crate::FieldReader<bool, TX_CTRL_ACK_ACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CTRL_ACK_ACK` writer - 8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
pub struct TX_CTRL_ACK_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CTRL_ACK_ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_CTRL_ACK_ACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_CTRL_ACK_ACK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_CTRL_ACK_ACK_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_CTRL_ACK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_CTRL_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: TX_CTRL_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_CTRL_ACK` reader - 7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
pub struct TX_CTRL_ACK_R(crate::FieldReader<bool, TX_CTRL_ACK_A>);
impl TX_CTRL_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_CTRL_ACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_CTRL_ACK_A {
        match self.bits {
            true => TX_CTRL_ACK_A::CPE1,
            false => TX_CTRL_ACK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == TX_CTRL_ACK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == TX_CTRL_ACK_A::CPE0
    }
}
impl core::ops::Deref for TX_CTRL_ACK_R {
    type Target = crate::FieldReader<bool, TX_CTRL_ACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CTRL_ACK` writer - 7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
pub struct TX_CTRL_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CTRL_ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_CTRL_ACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_CTRL_ACK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_CTRL_ACK_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_CTRL_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: TX_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_CTRL` reader - 6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
pub struct TX_CTRL_R(crate::FieldReader<bool, TX_CTRL_A>);
impl TX_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_CTRL_A {
        match self.bits {
            true => TX_CTRL_A::CPE1,
            false => TX_CTRL_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == TX_CTRL_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == TX_CTRL_A::CPE0
    }
}
impl core::ops::Deref for TX_CTRL_R {
    type Target = crate::FieldReader<bool, TX_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CTRL` writer - 6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
pub struct TX_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_CTRL_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_CTRL_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_ACK_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: TX_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_ACK` reader - 5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
pub struct TX_ACK_R(crate::FieldReader<bool, TX_ACK_A>);
impl TX_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_ACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_ACK_A {
        match self.bits {
            true => TX_ACK_A::CPE1,
            false => TX_ACK_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == TX_ACK_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == TX_ACK_A::CPE0
    }
}
impl core::ops::Deref for TX_ACK_R {
    type Target = crate::FieldReader<bool, TX_ACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ACK` writer - 5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
pub struct TX_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_ACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_ACK_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_ACK_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<TX_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_DONE` reader - 4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
pub struct TX_DONE_R(crate::FieldReader<bool, TX_DONE_A>);
impl TX_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_DONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_DONE_A {
        match self.bits {
            true => TX_DONE_A::CPE1,
            false => TX_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == TX_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == TX_DONE_A::CPE0
    }
}
impl core::ops::Deref for TX_DONE_R {
    type Target = crate::FieldReader<bool, TX_DONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DONE` writer - 4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
pub struct TX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_DONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(TX_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(TX_DONE_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LAST_FG_COMMAND_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<LAST_FG_COMMAND_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: LAST_FG_COMMAND_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LAST_FG_COMMAND_DONE` reader - 3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
pub struct LAST_FG_COMMAND_DONE_R(crate::FieldReader<bool, LAST_FG_COMMAND_DONE_A>);
impl LAST_FG_COMMAND_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LAST_FG_COMMAND_DONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LAST_FG_COMMAND_DONE_A {
        match self.bits {
            true => LAST_FG_COMMAND_DONE_A::CPE1,
            false => LAST_FG_COMMAND_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == LAST_FG_COMMAND_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == LAST_FG_COMMAND_DONE_A::CPE0
    }
}
impl core::ops::Deref for LAST_FG_COMMAND_DONE_R {
    type Target = crate::FieldReader<bool, LAST_FG_COMMAND_DONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LAST_FG_COMMAND_DONE` writer - 3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
pub struct LAST_FG_COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_FG_COMMAND_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LAST_FG_COMMAND_DONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(LAST_FG_COMMAND_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(LAST_FG_COMMAND_DONE_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FG_COMMAND_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<FG_COMMAND_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: FG_COMMAND_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FG_COMMAND_DONE` reader - 2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
pub struct FG_COMMAND_DONE_R(crate::FieldReader<bool, FG_COMMAND_DONE_A>);
impl FG_COMMAND_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FG_COMMAND_DONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FG_COMMAND_DONE_A {
        match self.bits {
            true => FG_COMMAND_DONE_A::CPE1,
            false => FG_COMMAND_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == FG_COMMAND_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == FG_COMMAND_DONE_A::CPE0
    }
}
impl core::ops::Deref for FG_COMMAND_DONE_R {
    type Target = crate::FieldReader<bool, FG_COMMAND_DONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FG_COMMAND_DONE` writer - 2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
pub struct FG_COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> FG_COMMAND_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FG_COMMAND_DONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(FG_COMMAND_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(FG_COMMAND_DONE_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LAST_COMMAND_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<LAST_COMMAND_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: LAST_COMMAND_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LAST_COMMAND_DONE` reader - 1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
pub struct LAST_COMMAND_DONE_R(crate::FieldReader<bool, LAST_COMMAND_DONE_A>);
impl LAST_COMMAND_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LAST_COMMAND_DONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LAST_COMMAND_DONE_A {
        match self.bits {
            true => LAST_COMMAND_DONE_A::CPE1,
            false => LAST_COMMAND_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == LAST_COMMAND_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == LAST_COMMAND_DONE_A::CPE0
    }
}
impl core::ops::Deref for LAST_COMMAND_DONE_R {
    type Target = crate::FieldReader<bool, LAST_COMMAND_DONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LAST_COMMAND_DONE` writer - 1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
pub struct LAST_COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_COMMAND_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LAST_COMMAND_DONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(LAST_COMMAND_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(LAST_COMMAND_DONE_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMMAND_DONE_A {
    #[doc = "1: Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    CPE1 = 1,
    #[doc = "0: Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    CPE0 = 0,
}
impl From<COMMAND_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: COMMAND_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMMAND_DONE` reader - 0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
pub struct COMMAND_DONE_R(crate::FieldReader<bool, COMMAND_DONE_A>);
impl COMMAND_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMMAND_DONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMMAND_DONE_A {
        match self.bits {
            true => COMMAND_DONE_A::CPE1,
            false => COMMAND_DONE_A::CPE0,
        }
    }
    #[doc = "Checks if the value of the field is `CPE1`"]
    #[inline(always)]
    pub fn is_cpe1(&self) -> bool {
        **self == COMMAND_DONE_A::CPE1
    }
    #[doc = "Checks if the value of the field is `CPE0`"]
    #[inline(always)]
    pub fn is_cpe0(&self) -> bool {
        **self == COMMAND_DONE_A::CPE0
    }
}
impl core::ops::Deref for COMMAND_DONE_R {
    type Target = crate::FieldReader<bool, COMMAND_DONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND_DONE` writer - 0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
pub struct COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMMAND_DONE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector"]
    #[inline(always)]
    pub fn cpe1(self) -> &'a mut W {
        self.variant(COMMAND_DONE_A::CPE1)
    }
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector"]
    #[inline(always)]
    pub fn cpe0(self) -> &'a mut W {
        self.variant(COMMAND_DONE_A::CPE0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
    #[inline(always)]
    pub fn internal_error(&self) -> INTERNAL_ERROR_R {
        INTERNAL_ERROR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
    #[inline(always)]
    pub fn boot_done(&self) -> BOOT_DONE_R {
        BOOT_DONE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
    #[inline(always)]
    pub fn modules_unlocked(&self) -> MODULES_UNLOCKED_R {
        MODULES_UNLOCKED_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
    #[inline(always)]
    pub fn synth_no_lock(&self) -> SYNTH_NO_LOCK_R {
        SYNTH_NO_LOCK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
    #[inline(always)]
    pub fn irq27(&self) -> IRQ27_R {
        IRQ27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
    #[inline(always)]
    pub fn rx_aborted(&self) -> RX_ABORTED_R {
        RX_ABORTED_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    pub fn rx_n_data_written(&self) -> RX_N_DATA_WRITTEN_R {
        RX_N_DATA_WRITTEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    pub fn rx_data_written(&self) -> RX_DATA_WRITTEN_R {
        RX_DATA_WRITTEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    pub fn rx_entry_done(&self) -> RX_ENTRY_DONE_R {
        RX_ENTRY_DONE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
    #[inline(always)]
    pub fn rx_buf_full(&self) -> RX_BUF_FULL_R {
        RX_BUF_FULL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    pub fn rx_ctrl_ack(&self) -> RX_CTRL_ACK_R {
        RX_CTRL_ACK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
    #[inline(always)]
    pub fn rx_ctrl(&self) -> RX_CTRL_R {
        RX_CTRL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
    #[inline(always)]
    pub fn rx_empty(&self) -> RX_EMPTY_R {
        RX_EMPTY_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
    #[inline(always)]
    pub fn rx_ignored(&self) -> RX_IGNORED_R {
        RX_IGNORED_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
    #[inline(always)]
    pub fn rx_nok(&self) -> RX_NOK_R {
        RX_NOK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
    #[inline(always)]
    pub fn rx_ok(&self) -> RX_OK_R {
        RX_OK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
    #[inline(always)]
    pub fn irq15(&self) -> IRQ15_R {
        IRQ15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
    #[inline(always)]
    pub fn irq14(&self) -> IRQ14_R {
        IRQ14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use."]
    #[inline(always)]
    pub fn irq13(&self) -> IRQ13_R {
        IRQ13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use."]
    #[inline(always)]
    pub fn irq12(&self) -> IRQ12_R {
        IRQ12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
    #[inline(always)]
    pub fn tx_buffer_changed(&self) -> TX_BUFFER_CHANGED_R {
        TX_BUFFER_CHANGED_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    pub fn tx_entry_done(&self) -> TX_ENTRY_DONE_R {
        TX_ENTRY_DONE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
    #[inline(always)]
    pub fn tx_retrans(&self) -> TX_RETRANS_R {
        TX_RETRANS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl_ack_ack(&self) -> TX_CTRL_ACK_ACK_R {
        TX_CTRL_ACK_ACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl_ack(&self) -> TX_CTRL_ACK_R {
        TX_CTRL_ACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl(&self) -> TX_CTRL_R {
        TX_CTRL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ack(&self) -> TX_ACK_R {
        TX_ACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn last_fg_command_done(&self) -> LAST_FG_COMMAND_DONE_R {
        LAST_FG_COMMAND_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn fg_command_done(&self) -> FG_COMMAND_DONE_R {
        FG_COMMAND_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn last_command_done(&self) -> LAST_COMMAND_DONE_R {
        LAST_COMMAND_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn command_done(&self) -> COMMAND_DONE_R {
        COMMAND_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
    #[inline(always)]
    pub fn internal_error(&mut self) -> INTERNAL_ERROR_W {
        INTERNAL_ERROR_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
    #[inline(always)]
    pub fn boot_done(&mut self) -> BOOT_DONE_W {
        BOOT_DONE_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
    #[inline(always)]
    pub fn modules_unlocked(&mut self) -> MODULES_UNLOCKED_W {
        MODULES_UNLOCKED_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\]
Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
    #[inline(always)]
    pub fn synth_no_lock(&mut self) -> SYNTH_NO_LOCK_W {
        SYNTH_NO_LOCK_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
    #[inline(always)]
    pub fn irq27(&mut self) -> IRQ27_W {
        IRQ27_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
    #[inline(always)]
    pub fn rx_aborted(&mut self) -> RX_ABORTED_W {
        RX_ABORTED_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    pub fn rx_n_data_written(&mut self) -> RX_N_DATA_WRITTEN_W {
        RX_N_DATA_WRITTEN_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    pub fn rx_data_written(&mut self) -> RX_DATA_WRITTEN_W {
        RX_DATA_WRITTEN_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    pub fn rx_entry_done(&mut self) -> RX_ENTRY_DONE_W {
        RX_ENTRY_DONE_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
    #[inline(always)]
    pub fn rx_buf_full(&mut self) -> RX_BUF_FULL_W {
        RX_BUF_FULL_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    pub fn rx_ctrl_ack(&mut self) -> RX_CTRL_ACK_W {
        RX_CTRL_ACK_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
    #[inline(always)]
    pub fn rx_ctrl(&mut self) -> RX_CTRL_W {
        RX_CTRL_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
    #[inline(always)]
    pub fn rx_empty(&mut self) -> RX_EMPTY_W {
        RX_EMPTY_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
    #[inline(always)]
    pub fn rx_ignored(&mut self) -> RX_IGNORED_W {
        RX_IGNORED_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
    #[inline(always)]
    pub fn rx_nok(&mut self) -> RX_NOK_W {
        RX_NOK_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
    #[inline(always)]
    pub fn rx_ok(&mut self) -> RX_OK_W {
        RX_OK_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
    #[inline(always)]
    pub fn irq15(&mut self) -> IRQ15_W {
        IRQ15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
    #[inline(always)]
    pub fn irq14(&mut self) -> IRQ14_W {
        IRQ14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use."]
    #[inline(always)]
    pub fn irq13(&mut self) -> IRQ13_W {
        IRQ13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use."]
    #[inline(always)]
    pub fn irq12(&mut self) -> IRQ12_W {
        IRQ12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
    #[inline(always)]
    pub fn tx_buffer_changed(&mut self) -> TX_BUFFER_CHANGED_W {
        TX_BUFFER_CHANGED_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    pub fn tx_entry_done(&mut self) -> TX_ENTRY_DONE_W {
        TX_ENTRY_DONE_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
    #[inline(always)]
    pub fn tx_retrans(&mut self) -> TX_RETRANS_W {
        TX_RETRANS_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl_ack_ack(&mut self) -> TX_CTRL_ACK_ACK_W {
        TX_CTRL_ACK_ACK_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl_ack(&mut self) -> TX_CTRL_ACK_W {
        TX_CTRL_ACK_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
    #[inline(always)]
    pub fn tx_ctrl(&mut self) -> TX_CTRL_W {
        TX_CTRL_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
    #[inline(always)]
    pub fn tx_ack(&mut self) -> TX_ACK_W {
        TX_ACK_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
    #[inline(always)]
    pub fn tx_done(&mut self) -> TX_DONE_W {
        TX_DONE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn last_fg_command_done(&mut self) -> LAST_FG_COMMAND_DONE_W {
        LAST_FG_COMMAND_DONE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn fg_command_done(&mut self) -> FG_COMMAND_DONE_W {
        FG_COMMAND_DONE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub fn last_command_done(&mut self) -> LAST_COMMAND_DONE_W {
        LAST_COMMAND_DONE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
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
#[doc = "Interrupt Vector Selection For Command and Packet Engine Generated Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcpeisl](index.html) module"]
pub struct RFCPEISL_SPEC;
impl crate::RegisterSpec for RFCPEISL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcpeisl::R](R) reader structure"]
impl crate::Readable for RFCPEISL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcpeisl::W](W) writer structure"]
impl crate::Writable for RFCPEISL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFCPEISL to value 0xffff_0000"]
impl crate::Resettable for RFCPEISL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_0000
    }
}
