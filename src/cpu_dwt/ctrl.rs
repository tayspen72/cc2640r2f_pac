#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED26` reader - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
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
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
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
#[doc = "Field `NOCYCCNT` reader - 25:25\\]
When set, CYCCNT is not supported."]
pub struct NOCYCCNT_R(crate::FieldReader<bool, bool>);
impl NOCYCCNT_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOCYCCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOCYCCNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOCYCCNT` writer - 25:25\\]
When set, CYCCNT is not supported."]
pub struct NOCYCCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> NOCYCCNT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `NOPRFCNT` reader - 24:24\\]
When set, FOLDCNT, LSUCNT, SLEEPCNT, EXCCNT, and CPICNT are not supported."]
pub struct NOPRFCNT_R(crate::FieldReader<bool, bool>);
impl NOPRFCNT_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOPRFCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOPRFCNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOPRFCNT` writer - 24:24\\]
When set, FOLDCNT, LSUCNT, SLEEPCNT, EXCCNT, and CPICNT are not supported."]
pub struct NOPRFCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> NOPRFCNT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RESERVED23` reader - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED23_R(crate::FieldReader<bool, bool>);
impl RESERVED23_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED23` writer - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED23_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `CYCEVTENA` reader - 22:22\\]
Enables Cycle count event. Emits an event when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. This event is only emitted if PCSAMPLEENA is disabled. PCSAMPLEENA overrides the setting of this bit. 0: Cycle count events disabled 1: Cycle count events enabled"]
pub struct CYCEVTENA_R(crate::FieldReader<bool, bool>);
impl CYCEVTENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CYCEVTENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CYCEVTENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CYCEVTENA` writer - 22:22\\]
Enables Cycle count event. Emits an event when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. This event is only emitted if PCSAMPLEENA is disabled. PCSAMPLEENA overrides the setting of this bit. 0: Cycle count events disabled 1: Cycle count events enabled"]
pub struct CYCEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FOLDEVTENA` reader - 21:21\\]
Enables Folded instruction count event. Emits an event when FOLDCNT overflows (every 256 cycles of folded instructions). A folded instruction is one that does not incur even one cycle to execute. For example, an IT instruction is folded away and so does not use up one cycle. 0: Folded instruction count events disabled. 1: Folded instruction count events enabled."]
pub struct FOLDEVTENA_R(crate::FieldReader<bool, bool>);
impl FOLDEVTENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        FOLDEVTENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FOLDEVTENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FOLDEVTENA` writer - 21:21\\]
Enables Folded instruction count event. Emits an event when FOLDCNT overflows (every 256 cycles of folded instructions). A folded instruction is one that does not incur even one cycle to execute. For example, an IT instruction is folded away and so does not use up one cycle. 0: Folded instruction count events disabled. 1: Folded instruction count events enabled."]
pub struct FOLDEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> FOLDEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LSUEVTENA` reader - 20:20\\]
Enables LSU count event. Emits an event when LSUCNT overflows (every 256 cycles of LSU operation). LSU counts include all LSU costs after the initial cycle for the instruction. 0: LSU count events disabled. 1: LSU count events enabled."]
pub struct LSUEVTENA_R(crate::FieldReader<bool, bool>);
impl LSUEVTENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSUEVTENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSUEVTENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSUEVTENA` writer - 20:20\\]
Enables LSU count event. Emits an event when LSUCNT overflows (every 256 cycles of LSU operation). LSU counts include all LSU costs after the initial cycle for the instruction. 0: LSU count events disabled. 1: LSU count events enabled."]
pub struct LSUEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LSUEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SLEEPEVTENA` reader - 19:19\\]
Enables Sleep count event. Emits an event when SLEEPCNT overflows (every 256 cycles that the processor is sleeping). 0: Sleep count events disabled. 1: Sleep count events enabled."]
pub struct SLEEPEVTENA_R(crate::FieldReader<bool, bool>);
impl SLEEPEVTENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEEPEVTENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEEPEVTENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEPEVTENA` writer - 19:19\\]
Enables Sleep count event. Emits an event when SLEEPCNT overflows (every 256 cycles that the processor is sleeping). 0: Sleep count events disabled. 1: Sleep count events enabled."]
pub struct SLEEPEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXCEVTENA` reader - 18:18\\]
Enables Interrupt overhead event. Emits an event when EXCCNT overflows (every 256 cycles of interrupt overhead). 0x0: Interrupt overhead event disabled. 0x1: Interrupt overhead event enabled."]
pub struct EXCEVTENA_R(crate::FieldReader<bool, bool>);
impl EXCEVTENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXCEVTENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXCEVTENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXCEVTENA` writer - 18:18\\]
Enables Interrupt overhead event. Emits an event when EXCCNT overflows (every 256 cycles of interrupt overhead). 0x0: Interrupt overhead event disabled. 0x1: Interrupt overhead event enabled."]
pub struct EXCEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `CPIEVTENA` reader - 17:17\\]
Enables CPI count event. Emits an event when CPICNT overflows (every 256 cycles of multi-cycle instructions). 0: CPI counter events disabled. 1: CPI counter events enabled."]
pub struct CPIEVTENA_R(crate::FieldReader<bool, bool>);
impl CPIEVTENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPIEVTENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPIEVTENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPIEVTENA` writer - 17:17\\]
Enables CPI count event. Emits an event when CPICNT overflows (every 256 cycles of multi-cycle instructions). 0: CPI counter events disabled. 1: CPI counter events enabled."]
pub struct CPIEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPIEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXCTRCENA` reader - 16:16\\]
Enables Interrupt event tracing. 0: Interrupt event trace disabled. 1: Interrupt event trace enabled."]
pub struct EXCTRCENA_R(crate::FieldReader<bool, bool>);
impl EXCTRCENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXCTRCENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXCTRCENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXCTRCENA` writer - 16:16\\]
Enables Interrupt event tracing. 0: Interrupt event trace disabled. 1: Interrupt event trace enabled."]
pub struct EXCTRCENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCTRCENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RESERVED13` reader - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED13_R(crate::FieldReader<u8, u8>);
impl RESERVED13_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED13` writer - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED13_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Field `PCSAMPLEENA` reader - 12:12\\]
Enables PC Sampling event. A PC sample event is emitted when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. Enabling this bit overrides CYCEVTENA. 0: PC Sampling event disabled. 1: Sampling event enabled."]
pub struct PCSAMPLEENA_R(crate::FieldReader<bool, bool>);
impl PCSAMPLEENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSAMPLEENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSAMPLEENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSAMPLEENA` writer - 12:12\\]
Enables PC Sampling event. A PC sample event is emitted when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. Enabling this bit overrides CYCEVTENA. 0: PC Sampling event disabled. 1: Sampling event enabled."]
pub struct PCSAMPLEENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSAMPLEENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "11:10\\]
Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCTAP_A {
    #[doc = "3: Tap at bit 28 of CYCCNT"]
    BIT28 = 3,
    #[doc = "2: Tap at bit 26 of CYCCNT"]
    BIT26 = 2,
    #[doc = "1: Tap at bit 24 of CYCCNT"]
    BIT24 = 1,
    #[doc = "0: Disabled. No synchronization packets"]
    DIS = 0,
}
impl From<SYNCTAP_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCTAP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYNCTAP` reader - 11:10\\]
Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0)."]
pub struct SYNCTAP_R(crate::FieldReader<u8, SYNCTAP_A>);
impl SYNCTAP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNCTAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCTAP_A {
        match self.bits {
            3 => SYNCTAP_A::BIT28,
            2 => SYNCTAP_A::BIT26,
            1 => SYNCTAP_A::BIT24,
            0 => SYNCTAP_A::DIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BIT28`"]
    #[inline(always)]
    pub fn is_bit28(&self) -> bool {
        **self == SYNCTAP_A::BIT28
    }
    #[doc = "Checks if the value of the field is `BIT26`"]
    #[inline(always)]
    pub fn is_bit26(&self) -> bool {
        **self == SYNCTAP_A::BIT26
    }
    #[doc = "Checks if the value of the field is `BIT24`"]
    #[inline(always)]
    pub fn is_bit24(&self) -> bool {
        **self == SYNCTAP_A::BIT24
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == SYNCTAP_A::DIS
    }
}
impl core::ops::Deref for SYNCTAP_R {
    type Target = crate::FieldReader<u8, SYNCTAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCTAP` writer - 11:10\\]
Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0)."]
pub struct SYNCTAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCTAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCTAP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Tap at bit 28 of CYCCNT"]
    #[inline(always)]
    pub fn bit28(self) -> &'a mut W {
        self.variant(SYNCTAP_A::BIT28)
    }
    #[doc = "Tap at bit 26 of CYCCNT"]
    #[inline(always)]
    pub fn bit26(self) -> &'a mut W {
        self.variant(SYNCTAP_A::BIT26)
    }
    #[doc = "Tap at bit 24 of CYCCNT"]
    #[inline(always)]
    pub fn bit24(self) -> &'a mut W {
        self.variant(SYNCTAP_A::BIT24)
    }
    #[doc = "Disabled. No synchronization packets"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SYNCTAP_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "9:9\\]
Selects a tap on CYCCNT. These are spaced at bits \\[6\\]
and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CYCTAP_A {
    #[doc = "1: Selects bit \\[10\\]
to tap"]
    BIT10 = 1,
    #[doc = "0: Selects bit \\[6\\]
to tap"]
    BIT6 = 0,
}
impl From<CYCTAP_A> for bool {
    #[inline(always)]
    fn from(variant: CYCTAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CYCTAP` reader - 9:9\\]
Selects a tap on CYCCNT. These are spaced at bits \\[6\\]
and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA)."]
pub struct CYCTAP_R(crate::FieldReader<bool, CYCTAP_A>);
impl CYCTAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CYCTAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYCTAP_A {
        match self.bits {
            true => CYCTAP_A::BIT10,
            false => CYCTAP_A::BIT6,
        }
    }
    #[doc = "Checks if the value of the field is `BIT10`"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        **self == CYCTAP_A::BIT10
    }
    #[doc = "Checks if the value of the field is `BIT6`"]
    #[inline(always)]
    pub fn is_bit6(&self) -> bool {
        **self == CYCTAP_A::BIT6
    }
}
impl core::ops::Deref for CYCTAP_R {
    type Target = crate::FieldReader<bool, CYCTAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CYCTAP` writer - 9:9\\]
Selects a tap on CYCCNT. These are spaced at bits \\[6\\]
and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA)."]
pub struct CYCTAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCTAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CYCTAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selects bit \\[10\\]
to tap"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(CYCTAP_A::BIT10)
    }
    #[doc = "Selects bit \\[6\\]
to tap"]
    #[inline(always)]
    pub fn bit6(self) -> &'a mut W {
        self.variant(CYCTAP_A::BIT6)
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
#[doc = "Field `POSTCNT` reader - 8:5\\]
Post-scalar counter for CYCTAP. When the selected tapped bit changes from 0 to 1 or 1 to 0, the post scalar counter is down-counted when not 0. If 0, it triggers an event for PCSAMPLEENA or CYCEVTENA use. It also reloads with the value from POSTPRESET."]
pub struct POSTCNT_R(crate::FieldReader<u8, u8>);
impl POSTCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        POSTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POSTCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POSTCNT` writer - 8:5\\]
Post-scalar counter for CYCTAP. When the selected tapped bit changes from 0 to 1 or 1 to 0, the post scalar counter is down-counted when not 0. If 0, it triggers an event for PCSAMPLEENA or CYCEVTENA use. It also reloads with the value from POSTPRESET."]
pub struct POSTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> POSTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | ((value as u32 & 0x0f) << 5);
        self.w
    }
}
#[doc = "Field `POSTPRESET` reader - 4:1\\]
Reload value for post-scalar counter POSTCNT. When 0, events are triggered on each tap change (a power of 2). If this field has a non-0 value, it forms a count-down value, to be reloaded into POSTCNT each time it reaches 0. For example, a value 1 in this register means an event is formed every other tap change."]
pub struct POSTPRESET_R(crate::FieldReader<u8, u8>);
impl POSTPRESET_R {
    pub(crate) fn new(bits: u8) -> Self {
        POSTPRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POSTPRESET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POSTPRESET` writer - 4:1\\]
Reload value for post-scalar counter POSTCNT. When 0, events are triggered on each tap change (a power of 2). If this field has a non-0 value, it forms a count-down value, to be reloaded into POSTCNT each time it reaches 0. For example, a value 1 in this register means an event is formed every other tap change."]
pub struct POSTPRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> POSTPRESET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `CYCCNTENA` reader - 0:0\\]
Enable CYCCNT, allowing it to increment and generate synchronization and count events. If NOCYCCNT = 1, this bit reads zero and ignore writes."]
pub struct CYCCNTENA_R(crate::FieldReader<bool, bool>);
impl CYCCNTENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CYCCNTENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CYCCNTENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CYCCNTENA` writer - 0:0\\]
Enable CYCCNT, allowing it to increment and generate synchronization and count events. If NOCYCCNT = 1, this bit reads zero and ignore writes."]
pub struct CYCCNTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCCNTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved26(&self) -> RESERVED26_R {
        RESERVED26_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
When set, CYCCNT is not supported."]
    #[inline(always)]
    pub fn nocyccnt(&self) -> NOCYCCNT_R {
        NOCYCCNT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
When set, FOLDCNT, LSUCNT, SLEEPCNT, EXCCNT, and CPICNT are not supported."]
    #[inline(always)]
    pub fn noprfcnt(&self) -> NOPRFCNT_R {
        NOPRFCNT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&self) -> RESERVED23_R {
        RESERVED23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Enables Cycle count event. Emits an event when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. This event is only emitted if PCSAMPLEENA is disabled. PCSAMPLEENA overrides the setting of this bit. 0: Cycle count events disabled 1: Cycle count events enabled"]
    #[inline(always)]
    pub fn cycevtena(&self) -> CYCEVTENA_R {
        CYCEVTENA_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Enables Folded instruction count event. Emits an event when FOLDCNT overflows (every 256 cycles of folded instructions). A folded instruction is one that does not incur even one cycle to execute. For example, an IT instruction is folded away and so does not use up one cycle. 0: Folded instruction count events disabled. 1: Folded instruction count events enabled."]
    #[inline(always)]
    pub fn foldevtena(&self) -> FOLDEVTENA_R {
        FOLDEVTENA_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Enables LSU count event. Emits an event when LSUCNT overflows (every 256 cycles of LSU operation). LSU counts include all LSU costs after the initial cycle for the instruction. 0: LSU count events disabled. 1: LSU count events enabled."]
    #[inline(always)]
    pub fn lsuevtena(&self) -> LSUEVTENA_R {
        LSUEVTENA_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Enables Sleep count event. Emits an event when SLEEPCNT overflows (every 256 cycles that the processor is sleeping). 0: Sleep count events disabled. 1: Sleep count events enabled."]
    #[inline(always)]
    pub fn sleepevtena(&self) -> SLEEPEVTENA_R {
        SLEEPEVTENA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Enables Interrupt overhead event. Emits an event when EXCCNT overflows (every 256 cycles of interrupt overhead). 0x0: Interrupt overhead event disabled. 0x1: Interrupt overhead event enabled."]
    #[inline(always)]
    pub fn excevtena(&self) -> EXCEVTENA_R {
        EXCEVTENA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Enables CPI count event. Emits an event when CPICNT overflows (every 256 cycles of multi-cycle instructions). 0: CPI counter events disabled. 1: CPI counter events enabled."]
    #[inline(always)]
    pub fn cpievtena(&self) -> CPIEVTENA_R {
        CPIEVTENA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables Interrupt event tracing. 0: Interrupt event trace disabled. 1: Interrupt event trace enabled."]
    #[inline(always)]
    pub fn exctrcena(&self) -> EXCTRCENA_R {
        EXCTRCENA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Enables PC Sampling event. A PC sample event is emitted when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. Enabling this bit overrides CYCEVTENA. 0: PC Sampling event disabled. 1: Sampling event enabled."]
    #[inline(always)]
    pub fn pcsampleena(&self) -> PCSAMPLEENA_R {
        PCSAMPLEENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0)."]
    #[inline(always)]
    pub fn synctap(&self) -> SYNCTAP_R {
        SYNCTAP_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Selects a tap on CYCCNT. These are spaced at bits \\[6\\]
and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA)."]
    #[inline(always)]
    pub fn cyctap(&self) -> CYCTAP_R {
        CYCTAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Post-scalar counter for CYCTAP. When the selected tapped bit changes from 0 to 1 or 1 to 0, the post scalar counter is down-counted when not 0. If 0, it triggers an event for PCSAMPLEENA or CYCEVTENA use. It also reloads with the value from POSTPRESET."]
    #[inline(always)]
    pub fn postcnt(&self) -> POSTCNT_R {
        POSTCNT_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Reload value for post-scalar counter POSTCNT. When 0, events are triggered on each tap change (a power of 2). If this field has a non-0 value, it forms a count-down value, to be reloaded into POSTCNT each time it reaches 0. For example, a value 1 in this register means an event is formed every other tap change."]
    #[inline(always)]
    pub fn postpreset(&self) -> POSTPRESET_R {
        POSTPRESET_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Enable CYCCNT, allowing it to increment and generate synchronization and count events. If NOCYCCNT = 1, this bit reads zero and ignore writes."]
    #[inline(always)]
    pub fn cyccntena(&self) -> CYCCNTENA_R {
        CYCCNTENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved26(&mut self) -> RESERVED26_W {
        RESERVED26_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
When set, CYCCNT is not supported."]
    #[inline(always)]
    pub fn nocyccnt(&mut self) -> NOCYCCNT_W {
        NOCYCCNT_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
When set, FOLDCNT, LSUCNT, SLEEPCNT, EXCCNT, and CPICNT are not supported."]
    #[inline(always)]
    pub fn noprfcnt(&mut self) -> NOPRFCNT_W {
        NOPRFCNT_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&mut self) -> RESERVED23_W {
        RESERVED23_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\]
Enables Cycle count event. Emits an event when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. This event is only emitted if PCSAMPLEENA is disabled. PCSAMPLEENA overrides the setting of this bit. 0: Cycle count events disabled 1: Cycle count events enabled"]
    #[inline(always)]
    pub fn cycevtena(&mut self) -> CYCEVTENA_W {
        CYCEVTENA_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\]
Enables Folded instruction count event. Emits an event when FOLDCNT overflows (every 256 cycles of folded instructions). A folded instruction is one that does not incur even one cycle to execute. For example, an IT instruction is folded away and so does not use up one cycle. 0: Folded instruction count events disabled. 1: Folded instruction count events enabled."]
    #[inline(always)]
    pub fn foldevtena(&mut self) -> FOLDEVTENA_W {
        FOLDEVTENA_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Enables LSU count event. Emits an event when LSUCNT overflows (every 256 cycles of LSU operation). LSU counts include all LSU costs after the initial cycle for the instruction. 0: LSU count events disabled. 1: LSU count events enabled."]
    #[inline(always)]
    pub fn lsuevtena(&mut self) -> LSUEVTENA_W {
        LSUEVTENA_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Enables Sleep count event. Emits an event when SLEEPCNT overflows (every 256 cycles that the processor is sleeping). 0: Sleep count events disabled. 1: Sleep count events enabled."]
    #[inline(always)]
    pub fn sleepevtena(&mut self) -> SLEEPEVTENA_W {
        SLEEPEVTENA_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Enables Interrupt overhead event. Emits an event when EXCCNT overflows (every 256 cycles of interrupt overhead). 0x0: Interrupt overhead event disabled. 0x1: Interrupt overhead event enabled."]
    #[inline(always)]
    pub fn excevtena(&mut self) -> EXCEVTENA_W {
        EXCEVTENA_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Enables CPI count event. Emits an event when CPICNT overflows (every 256 cycles of multi-cycle instructions). 0: CPI counter events disabled. 1: CPI counter events enabled."]
    #[inline(always)]
    pub fn cpievtena(&mut self) -> CPIEVTENA_W {
        CPIEVTENA_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Enables Interrupt event tracing. 0: Interrupt event trace disabled. 1: Interrupt event trace enabled."]
    #[inline(always)]
    pub fn exctrcena(&mut self) -> EXCTRCENA_W {
        EXCTRCENA_W { w: self }
    }
    #[doc = "Bits 13:15 - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&mut self) -> RESERVED13_W {
        RESERVED13_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Enables PC Sampling event. A PC sample event is emitted when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. Enabling this bit overrides CYCEVTENA. 0: PC Sampling event disabled. 1: Sampling event enabled."]
    #[inline(always)]
    pub fn pcsampleena(&mut self) -> PCSAMPLEENA_W {
        PCSAMPLEENA_W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0)."]
    #[inline(always)]
    pub fn synctap(&mut self) -> SYNCTAP_W {
        SYNCTAP_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Selects a tap on CYCCNT. These are spaced at bits \\[6\\]
and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA)."]
    #[inline(always)]
    pub fn cyctap(&mut self) -> CYCTAP_W {
        CYCTAP_W { w: self }
    }
    #[doc = "Bits 5:8 - 8:5\\]
Post-scalar counter for CYCTAP. When the selected tapped bit changes from 0 to 1 or 1 to 0, the post scalar counter is down-counted when not 0. If 0, it triggers an event for PCSAMPLEENA or CYCEVTENA use. It also reloads with the value from POSTPRESET."]
    #[inline(always)]
    pub fn postcnt(&mut self) -> POSTCNT_W {
        POSTCNT_W { w: self }
    }
    #[doc = "Bits 1:4 - 4:1\\]
Reload value for post-scalar counter POSTCNT. When 0, events are triggered on each tap change (a power of 2). If this field has a non-0 value, it forms a count-down value, to be reloaded into POSTCNT each time it reaches 0. For example, a value 1 in this register means an event is formed every other tap change."]
    #[inline(always)]
    pub fn postpreset(&mut self) -> POSTPRESET_W {
        POSTPRESET_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Enable CYCCNT, allowing it to increment and generate synchronization and count events. If NOCYCCNT = 1, this bit reads zero and ignore writes."]
    #[inline(always)]
    pub fn cyccntena(&mut self) -> CYCCNTENA_W {
        CYCCNTENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Use the DWT Control Register to enable the DWT unit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x4000_0000"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
