#[doc = "Register `DHCSR` reader"]
pub struct R(crate::R<DHCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DHCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DHCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DHCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DHCSR` writer"]
pub struct W(crate::W<DHCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DHCSR_SPEC>;
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
impl From<crate::W<DHCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DHCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED26` reader - 31:26\\]
Software should not rely on the value of a reserved. When writing to this register, 0x28 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub struct RESERVED26_R(crate::FieldReader<u8, u8>);
impl RESERVED26_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED26_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED26` writer - 31:26\\]
Software should not rely on the value of a reserved. When writing to this register, 0x28 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub struct RESERVED26_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | ((value as u32 & 0x3f) << 26);
        self.w
    }
}
#[doc = "Field `S_RESET_ST` reader - 25:25\\]
Indicates that the core has been reset, or is now being reset, since the last time this bit was read. This a sticky bit that clears on read. So, reading twice and getting 1 then 0 means it was reset in the past. Reading twice and getting 1 both times means that it is being reset now (held in reset still). When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub struct S_RESET_ST_R(crate::FieldReader<bool, bool>);
impl S_RESET_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_RESET_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_RESET_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_RESET_ST` writer - 25:25\\]
Indicates that the core has been reset, or is now being reset, since the last time this bit was read. This a sticky bit that clears on read. So, reading twice and getting 1 then 0 means it was reset in the past. Reading twice and getting 1 both times means that it is being reset now (held in reset still). When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub struct S_RESET_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> S_RESET_ST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `S_RETIRE_ST` reader - 24:24\\]
Indicates that an instruction has completed since last read. This is a sticky bit that clears on read. This determines if the core is stalled on a load/store or fetch. When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub struct S_RETIRE_ST_R(crate::FieldReader<bool, bool>);
impl S_RETIRE_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_RETIRE_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_RETIRE_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_RETIRE_ST` writer - 24:24\\]
Indicates that an instruction has completed since last read. This is a sticky bit that clears on read. This determines if the core is stalled on a load/store or fetch. When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub struct S_RETIRE_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> S_RETIRE_ST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RESERVED20` reader - 23:20\\]
Software should not rely on the value of a reserved. When writing to this register, 0x5 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub struct RESERVED20_R(crate::FieldReader<u8, u8>);
impl RESERVED20_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED20_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED20` writer - 23:20\\]
Software should not rely on the value of a reserved. When writing to this register, 0x5 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub struct RESERVED20_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `S_LOCKUP` reader - 19:19\\]
Reads as one if the core is running (not halted) and a lockup condition is present. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub struct S_LOCKUP_R(crate::FieldReader<bool, bool>);
impl S_LOCKUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_LOCKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_LOCKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_LOCKUP` writer - 19:19\\]
Reads as one if the core is running (not halted) and a lockup condition is present. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub struct S_LOCKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> S_LOCKUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `S_SLEEP` reader - 18:18\\]
Indicates that the core is sleeping (WFI, WFE, or **SLEEP-ON-EXIT**). Must use C_HALT to gain control or wait for interrupt to wake-up. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub struct S_SLEEP_R(crate::FieldReader<bool, bool>);
impl S_SLEEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_SLEEP` writer - 18:18\\]
Indicates that the core is sleeping (WFI, WFE, or **SLEEP-ON-EXIT**). Must use C_HALT to gain control or wait for interrupt to wake-up. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub struct S_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> S_SLEEP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `S_HALT` reader - 17:17\\]
The core is in debug state when this bit is set. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub struct S_HALT_R(crate::FieldReader<bool, bool>);
impl S_HALT_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_HALT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_HALT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_HALT` writer - 17:17\\]
The core is in debug state when this bit is set. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub struct S_HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> S_HALT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `S_REGRDY` reader - 16:16\\]
Register Read/Write on the Debug Core Register Selector register is available. Last transfer is complete. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub struct S_REGRDY_R(crate::FieldReader<bool, bool>);
impl S_REGRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_REGRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_REGRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_REGRDY` writer - 16:16\\]
Register Read/Write on the Debug Core Register Selector register is available. Last transfer is complete. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
pub struct S_REGRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> S_REGRDY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RESERVED6` reader - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED6_R(crate::FieldReader<u16, u16>);
impl RESERVED6_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED6_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED6` writer - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 6)) | ((value as u32 & 0x03ff) << 6);
        self.w
    }
}
#[doc = "Field `C_SNAPSTALL` reader - 5:5\\]
If the core is stalled on a load/store operation the stall ceases and the instruction is forced to complete. This enables Halting debug to gain control of the core. It can only be set if: C_DEBUGEN = 1 and C_HALT = 1. The core reads S_RETIRE_ST as 0. This indicates that no instruction has advanced. This prevents misuse. The bus state is Unpredictable when this is used. S_RETIRE_ST can detect core stalls on load/store operations."]
pub struct C_SNAPSTALL_R(crate::FieldReader<bool, bool>);
impl C_SNAPSTALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        C_SNAPSTALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C_SNAPSTALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C_SNAPSTALL` writer - 5:5\\]
If the core is stalled on a load/store operation the stall ceases and the instruction is forced to complete. This enables Halting debug to gain control of the core. It can only be set if: C_DEBUGEN = 1 and C_HALT = 1. The core reads S_RETIRE_ST as 0. This indicates that no instruction has advanced. This prevents misuse. The bus state is Unpredictable when this is used. S_RETIRE_ST can detect core stalls on load/store operations."]
pub struct C_SNAPSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> C_SNAPSTALL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RESERVED4` reader - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_R(crate::FieldReader<bool, bool>);
impl RESERVED4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED4` writer - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `C_MASKINTS` reader - 3:3\\]
Mask interrupts when stepping or running in halted debug. This masking does not affect NMI, fault exceptions and SVC caused by execution of the instructions. This bit must only be modified when the processor is halted (S_HALT == 1). C_MASKINTS must be set or cleared before halt is released (i.e., the writes to set or clear C_MASKINTS and to set or clear C_HALT must be separate). Modifying C_MASKINTS while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
pub struct C_MASKINTS_R(crate::FieldReader<bool, bool>);
impl C_MASKINTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        C_MASKINTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C_MASKINTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C_MASKINTS` writer - 3:3\\]
Mask interrupts when stepping or running in halted debug. This masking does not affect NMI, fault exceptions and SVC caused by execution of the instructions. This bit must only be modified when the processor is halted (S_HALT == 1). C_MASKINTS must be set or cleared before halt is released (i.e., the writes to set or clear C_MASKINTS and to set or clear C_HALT must be separate). Modifying C_MASKINTS while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
pub struct C_MASKINTS_W<'a> {
    w: &'a mut W,
}
impl<'a> C_MASKINTS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `C_STEP` reader - 2:2\\]
Steps the core in halted debug. When C_DEBUGEN = 0, this bit has no effect. Must only be modified when the processor is halted (S_HALT == 1). Modifying C_STEP while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
pub struct C_STEP_R(crate::FieldReader<bool, bool>);
impl C_STEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        C_STEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C_STEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C_STEP` writer - 2:2\\]
Steps the core in halted debug. When C_DEBUGEN = 0, this bit has no effect. Must only be modified when the processor is halted (S_HALT == 1). Modifying C_STEP while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
pub struct C_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> C_STEP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `C_HALT` reader - 1:1\\]
Halts the core. This bit is set automatically when the core Halts. For example Breakpoint. This bit clears on core reset."]
pub struct C_HALT_R(crate::FieldReader<bool, bool>);
impl C_HALT_R {
    pub(crate) fn new(bits: bool) -> Self {
        C_HALT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C_HALT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C_HALT` writer - 1:1\\]
Halts the core. This bit is set automatically when the core Halts. For example Breakpoint. This bit clears on core reset."]
pub struct C_HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> C_HALT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `C_DEBUGEN` reader - 0:0\\]
Enables debug. This can only be written by AHB-AP and not by the core. It is ignored when written by the core, which cannot set or clear it. The core must write a 1 to it when writing C_HALT to halt itself. The values of C_HALT, C_STEP and C_MASKINTS are ignored by hardware when C_DEBUGEN = 0. The read values for C_HALT, C_STEP and C_MASKINTS fields will be unknown to software when C_DEBUGEN = 0."]
pub struct C_DEBUGEN_R(crate::FieldReader<bool, bool>);
impl C_DEBUGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        C_DEBUGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C_DEBUGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C_DEBUGEN` writer - 0:0\\]
Enables debug. This can only be written by AHB-AP and not by the core. It is ignored when written by the core, which cannot set or clear it. The core must write a 1 to it when writing C_HALT to halt itself. The values of C_HALT, C_STEP and C_MASKINTS are ignored by hardware when C_DEBUGEN = 0. The read values for C_HALT, C_STEP and C_MASKINTS fields will be unknown to software when C_DEBUGEN = 0."]
pub struct C_DEBUGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> C_DEBUGEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. When writing to this register, 0x28 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn reserved26(&self) -> RESERVED26_R {
        RESERVED26_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
Indicates that the core has been reset, or is now being reset, since the last time this bit was read. This a sticky bit that clears on read. So, reading twice and getting 1 then 0 means it was reset in the past. Reading twice and getting 1 both times means that it is being reset now (held in reset still). When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_reset_st(&self) -> S_RESET_ST_R {
        S_RESET_ST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates that an instruction has completed since last read. This is a sticky bit that clears on read. This determines if the core is stalled on a load/store or fetch. When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_retire_st(&self) -> S_RETIRE_ST_R {
        S_RETIRE_ST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. When writing to this register, 0x5 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
Reads as one if the core is running (not halted) and a lockup condition is present. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_lockup(&self) -> S_LOCKUP_R {
        S_LOCKUP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Indicates that the core is sleeping (WFI, WFE, or **SLEEP-ON-EXIT**). Must use C_HALT to gain control or wait for interrupt to wake-up. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_sleep(&self) -> S_SLEEP_R {
        S_SLEEP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
The core is in debug state when this bit is set. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_halt(&self) -> S_HALT_R {
        S_HALT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Register Read/Write on the Debug Core Register Selector register is available. Last transfer is complete. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_regrdy(&self) -> S_REGRDY_R {
        S_REGRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 6:15 - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bit 5 - 5:5\\]
If the core is stalled on a load/store operation the stall ceases and the instruction is forced to complete. This enables Halting debug to gain control of the core. It can only be set if: C_DEBUGEN = 1 and C_HALT = 1. The core reads S_RETIRE_ST as 0. This indicates that no instruction has advanced. This prevents misuse. The bus state is Unpredictable when this is used. S_RETIRE_ST can detect core stalls on load/store operations."]
    #[inline(always)]
    pub fn c_snapstall(&self) -> C_SNAPSTALL_R {
        C_SNAPSTALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Mask interrupts when stepping or running in halted debug. This masking does not affect NMI, fault exceptions and SVC caused by execution of the instructions. This bit must only be modified when the processor is halted (S_HALT == 1). C_MASKINTS must be set or cleared before halt is released (i.e., the writes to set or clear C_MASKINTS and to set or clear C_HALT must be separate). Modifying C_MASKINTS while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
    #[inline(always)]
    pub fn c_maskints(&self) -> C_MASKINTS_R {
        C_MASKINTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Steps the core in halted debug. When C_DEBUGEN = 0, this bit has no effect. Must only be modified when the processor is halted (S_HALT == 1). Modifying C_STEP while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
    #[inline(always)]
    pub fn c_step(&self) -> C_STEP_R {
        C_STEP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Halts the core. This bit is set automatically when the core Halts. For example Breakpoint. This bit clears on core reset."]
    #[inline(always)]
    pub fn c_halt(&self) -> C_HALT_R {
        C_HALT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Enables debug. This can only be written by AHB-AP and not by the core. It is ignored when written by the core, which cannot set or clear it. The core must write a 1 to it when writing C_HALT to halt itself. The values of C_HALT, C_STEP and C_MASKINTS are ignored by hardware when C_DEBUGEN = 0. The read values for C_HALT, C_STEP and C_MASKINTS fields will be unknown to software when C_DEBUGEN = 0."]
    #[inline(always)]
    pub fn c_debugen(&self) -> C_DEBUGEN_R {
        C_DEBUGEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. When writing to this register, 0x28 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn reserved26(&mut self) -> RESERVED26_W {
        RESERVED26_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Indicates that the core has been reset, or is now being reset, since the last time this bit was read. This a sticky bit that clears on read. So, reading twice and getting 1 then 0 means it was reset in the past. Reading twice and getting 1 both times means that it is being reset now (held in reset still). When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_reset_st(&mut self) -> S_RESET_ST_W {
        S_RESET_ST_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates that an instruction has completed since last read. This is a sticky bit that clears on read. This determines if the core is stalled on a load/store or fetch. When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_retire_st(&mut self) -> S_RETIRE_ST_W {
        S_RETIRE_ST_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Software should not rely on the value of a reserved. When writing to this register, 0x5 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Reads as one if the core is running (not halted) and a lockup condition is present. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_lockup(&mut self) -> S_LOCKUP_W {
        S_LOCKUP_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Indicates that the core is sleeping (WFI, WFE, or **SLEEP-ON-EXIT**). Must use C_HALT to gain control or wait for interrupt to wake-up. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_sleep(&mut self) -> S_SLEEP_W {
        S_SLEEP_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
The core is in debug state when this bit is set. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_halt(&mut self) -> S_HALT_W {
        S_HALT_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Register Read/Write on the Debug Core Register Selector register is available. Last transfer is complete. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub fn s_regrdy(&mut self) -> S_REGRDY_W {
        S_REGRDY_W { w: self }
    }
    #[doc = "Bits 6:15 - 15:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
If the core is stalled on a load/store operation the stall ceases and the instruction is forced to complete. This enables Halting debug to gain control of the core. It can only be set if: C_DEBUGEN = 1 and C_HALT = 1. The core reads S_RETIRE_ST as 0. This indicates that no instruction has advanced. This prevents misuse. The bus state is Unpredictable when this is used. S_RETIRE_ST can detect core stalls on load/store operations."]
    #[inline(always)]
    pub fn c_snapstall(&mut self) -> C_SNAPSTALL_W {
        C_SNAPSTALL_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Mask interrupts when stepping or running in halted debug. This masking does not affect NMI, fault exceptions and SVC caused by execution of the instructions. This bit must only be modified when the processor is halted (S_HALT == 1). C_MASKINTS must be set or cleared before halt is released (i.e., the writes to set or clear C_MASKINTS and to set or clear C_HALT must be separate). Modifying C_MASKINTS while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
    #[inline(always)]
    pub fn c_maskints(&mut self) -> C_MASKINTS_W {
        C_MASKINTS_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Steps the core in halted debug. When C_DEBUGEN = 0, this bit has no effect. Must only be modified when the processor is halted (S_HALT == 1). Modifying C_STEP while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
    #[inline(always)]
    pub fn c_step(&mut self) -> C_STEP_W {
        C_STEP_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Halts the core. This bit is set automatically when the core Halts. For example Breakpoint. This bit clears on core reset."]
    #[inline(always)]
    pub fn c_halt(&mut self) -> C_HALT_W {
        C_HALT_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Enables debug. This can only be written by AHB-AP and not by the core. It is ignored when written by the core, which cannot set or clear it. The core must write a 1 to it when writing C_HALT to halt itself. The values of C_HALT, C_STEP and C_MASKINTS are ignored by hardware when C_DEBUGEN = 0. The read values for C_HALT, C_STEP and C_MASKINTS fields will be unknown to software when C_DEBUGEN = 0."]
    #[inline(always)]
    pub fn c_debugen(&mut self) -> C_DEBUGEN_W {
        C_DEBUGEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Halting Control and Status The purpose of this register is to provide status information about the state of the processor, enable core debug, halt and step the processor. For writes, 0xA05F must be written to higher half-word of this register, otherwise the write operation is ignored and no bits are written into the register. If not enabled for Halting mode, C_DEBUGEN = 1, all other fields are disabled. This register is not reset on a core reset. It is reset by a power-on reset. However, C_HALT always clears on a core reset. To halt on a reset, the following bits must be enabled: DEMCR.VC_CORERESET and C_DEBUGEN. Note that writes to this register in any size other than word are unpredictable. It is acceptable to read in any size, and it can be used to avoid or intentionally change a sticky bit. Behavior of the system when writing to this register while CPU is halted (i.e. C_DEBUGEN = 1 and S_HALT= 1): C_HALT=0, C_STEP=0, C_MASKINTS=0 Exit Debug state and start instruction execution. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=0, C_MASKINTS=1 Exit Debug state and start instruction execution. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=0 Exit Debug state, step an instruction and halt. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=1 Exit Debug state, step an instruction and halt. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=1, C_STEP=x, C_MASKINTS=x Remain in Debug state\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhcsr](index.html) module"]
pub struct DHCSR_SPEC;
impl crate::RegisterSpec for DHCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dhcsr::R](R) reader structure"]
impl crate::Readable for DHCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dhcsr::W](W) writer structure"]
impl crate::Writable for DHCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DHCSR to value 0"]
impl crate::Resettable for DHCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
