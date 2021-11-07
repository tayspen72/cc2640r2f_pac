#[doc = "Register `STCSR` reader"]
pub struct R(crate::R<STCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCSR` writer"]
pub struct W(crate::W<STCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCSR_SPEC>;
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
impl From<crate::W<STCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED17` reader - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED17_R(crate::FieldReader<u16, u16>);
impl RESERVED17_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED17_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED17` writer - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED17_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 17)) | ((value as u32 & 0x7fff) << 17);
        self.w
    }
}
#[doc = "Field `COUNTFLAG` reader - 16:16\\]
Returns 1 if timer counted to 0 since last time this was read. Clears on read by application of any part of the SysTick Control and Status Register. If read by the debugger using the DAP, this bit is cleared on read-only if the MasterType bit in the **AHB-AP** Control Register is set to 0. Otherwise, COUNTFLAG is not changed by the debugger read."]
pub struct COUNTFLAG_R(crate::FieldReader<bool, bool>);
impl COUNTFLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        COUNTFLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTFLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTFLAG` writer - 16:16\\]
Returns 1 if timer counted to 0 since last time this was read. Clears on read by application of any part of the SysTick Control and Status Register. If read by the debugger using the DAP, this bit is cleared on read-only if the MasterType bit in the **AHB-AP** Control Register is set to 0. Otherwise, COUNTFLAG is not changed by the debugger read."]
pub struct COUNTFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTFLAG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RESERVED3` reader - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_R(crate::FieldReader<u16, u16>);
impl RESERVED3_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 3)) | ((value as u32 & 0x1fff) << 3);
        self.w
    }
}
#[doc = "Field `CLKSOURCE` reader - 2:2\\]
Clock source: 0: External reference clock. 1: Core clock External clock is not available in this device. Writes to this field will be ignored."]
pub struct CLKSOURCE_R(crate::FieldReader<bool, bool>);
impl CLKSOURCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKSOURCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKSOURCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSOURCE` writer - 2:2\\]
Clock source: 0: External reference clock. 1: Core clock External clock is not available in this device. Writes to this field will be ignored."]
pub struct CLKSOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSOURCE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TICKINT` reader - 1:1\\]
0: Counting down to zero does not pend the SysTick handler. Software can use COUNTFLAG to determine if the SysTick handler has ever counted to zero. 1: Counting down to zero pends the SysTick handler."]
pub struct TICKINT_R(crate::FieldReader<bool, bool>);
impl TICKINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TICKINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TICKINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TICKINT` writer - 1:1\\]
0: Counting down to zero does not pend the SysTick handler. Software can use COUNTFLAG to determine if the SysTick handler has ever counted to zero. 1: Counting down to zero pends the SysTick handler."]
pub struct TICKINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TICKINT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ENABLE` reader - 0:0\\]
Enable SysTick counter 0: Counter disabled 1: Counter operates in a multi-shot way. That is, counter loads with the Reload value STRVR.RELOAD and then begins counting down. On reaching 0, it sets COUNTFLAG to 1 and optionally pends the SysTick handler, based on TICKINT. It then loads STRVR.RELOAD again, and begins counting."]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - 0:0\\]
Enable SysTick counter 0: Counter disabled 1: Counter operates in a multi-shot way. That is, counter loads with the Reload value STRVR.RELOAD and then begins counting down. On reaching 0, it sets COUNTFLAG to 1 and optionally pends the SysTick handler, based on TICKINT. It then loads STRVR.RELOAD again, and begins counting."]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Returns 1 if timer counted to 0 since last time this was read. Clears on read by application of any part of the SysTick Control and Status Register. If read by the debugger using the DAP, this bit is cleared on read-only if the MasterType bit in the **AHB-AP** Control Register is set to 0. Otherwise, COUNTFLAG is not changed by the debugger read."]
    #[inline(always)]
    pub fn countflag(&self) -> COUNTFLAG_R {
        COUNTFLAG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 3:15 - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bit 2 - 2:2\\]
Clock source: 0: External reference clock. 1: Core clock External clock is not available in this device. Writes to this field will be ignored."]
    #[inline(always)]
    pub fn clksource(&self) -> CLKSOURCE_R {
        CLKSOURCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Counting down to zero does not pend the SysTick handler. Software can use COUNTFLAG to determine if the SysTick handler has ever counted to zero. 1: Counting down to zero pends the SysTick handler."]
    #[inline(always)]
    pub fn tickint(&self) -> TICKINT_R {
        TICKINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Enable SysTick counter 0: Counter disabled 1: Counter operates in a multi-shot way. That is, counter loads with the Reload value STRVR.RELOAD and then begins counting down. On reaching 0, it sets COUNTFLAG to 1 and optionally pends the SysTick handler, based on TICKINT. It then loads STRVR.RELOAD again, and begins counting."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&mut self) -> RESERVED17_W {
        RESERVED17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Returns 1 if timer counted to 0 since last time this was read. Clears on read by application of any part of the SysTick Control and Status Register. If read by the debugger using the DAP, this bit is cleared on read-only if the MasterType bit in the **AHB-AP** Control Register is set to 0. Otherwise, COUNTFLAG is not changed by the debugger read."]
    #[inline(always)]
    pub fn countflag(&mut self) -> COUNTFLAG_W {
        COUNTFLAG_W { w: self }
    }
    #[doc = "Bits 3:15 - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Clock source: 0: External reference clock. 1: Core clock External clock is not available in this device. Writes to this field will be ignored."]
    #[inline(always)]
    pub fn clksource(&mut self) -> CLKSOURCE_W {
        CLKSOURCE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: Counting down to zero does not pend the SysTick handler. Software can use COUNTFLAG to determine if the SysTick handler has ever counted to zero. 1: Counting down to zero pends the SysTick handler."]
    #[inline(always)]
    pub fn tickint(&mut self) -> TICKINT_W {
        TICKINT_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Enable SysTick counter 0: Counter disabled 1: Counter operates in a multi-shot way. That is, counter loads with the Reload value STRVR.RELOAD and then begins counting down. On reaching 0, it sets COUNTFLAG to 1 and optionally pends the SysTick handler, based on TICKINT. It then loads STRVR.RELOAD again, and begins counting."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SysTick Control and Status This register enables the SysTick features and returns status flags related to SysTick.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcsr](index.html) module"]
pub struct STCSR_SPEC;
impl crate::RegisterSpec for STCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcsr::R](R) reader structure"]
impl crate::Readable for STCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stcsr::W](W) writer structure"]
impl crate::Writable for STCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STCSR to value 0x04"]
impl crate::Resettable for STCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
