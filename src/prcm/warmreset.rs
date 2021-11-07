#[doc = "Register `WARMRESET` reader"]
pub struct R(crate::R<WARMRESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WARMRESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WARMRESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WARMRESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WARMRESET` writer"]
pub struct W(crate::W<WARMRESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WARMRESET_SPEC>;
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
impl From<crate::W<WARMRESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WARMRESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_R(crate::FieldReader<u32, u32>);
impl RESERVED3_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Field `WR_TO_PINRESET` reader - 2:2\\]
0: No action 1: A warm system reset event triggered by the below listed sources will result in an emulated pin reset. Warm reset sources included: ICEPick sysreset System CPU reset request, CPU_SCS:AIRCR.SYSRESETREQ System CPU Lockup WDT timeout An active ICEPick block system reset will gate all sources except ICEPick sysreset SW can read AON_SYSCTL:RESETCTL.RESET_SRC to find the source of the last reset resulting in a full power up sequence. WARMRESET in this register is set in the scenario that WR_TO_PINRESET=1 and one of the above listed sources is triggered."]
pub struct WR_TO_PINRESET_R(crate::FieldReader<bool, bool>);
impl WR_TO_PINRESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_TO_PINRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_TO_PINRESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_TO_PINRESET` writer - 2:2\\]
0: No action 1: A warm system reset event triggered by the below listed sources will result in an emulated pin reset. Warm reset sources included: ICEPick sysreset System CPU reset request, CPU_SCS:AIRCR.SYSRESETREQ System CPU Lockup WDT timeout An active ICEPick block system reset will gate all sources except ICEPick sysreset SW can read AON_SYSCTL:RESETCTL.RESET_SRC to find the source of the last reset resulting in a full power up sequence. WARMRESET in this register is set in the scenario that WR_TO_PINRESET=1 and one of the above listed sources is triggered."]
pub struct WR_TO_PINRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_TO_PINRESET_W<'a> {
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
#[doc = "Field `LOCKUP_STAT` reader - 1:1\\]
0: No registred event 1: A system CPU LOCKUP event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
pub struct LOCKUP_STAT_R(crate::FieldReader<bool, bool>);
impl LOCKUP_STAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKUP_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKUP_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKUP_STAT` writer - 1:1\\]
0: No registred event 1: A system CPU LOCKUP event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
pub struct LOCKUP_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKUP_STAT_W<'a> {
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
#[doc = "Field `WDT_STAT` reader - 0:0\\]
0: No registered event 1: A WDT event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
pub struct WDT_STAT_R(crate::FieldReader<bool, bool>);
impl WDT_STAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDT_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_STAT` writer - 0:0\\]
0: No registered event 1: A WDT event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
pub struct WDT_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_STAT_W<'a> {
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
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
0: No action 1: A warm system reset event triggered by the below listed sources will result in an emulated pin reset. Warm reset sources included: ICEPick sysreset System CPU reset request, CPU_SCS:AIRCR.SYSRESETREQ System CPU Lockup WDT timeout An active ICEPick block system reset will gate all sources except ICEPick sysreset SW can read AON_SYSCTL:RESETCTL.RESET_SRC to find the source of the last reset resulting in a full power up sequence. WARMRESET in this register is set in the scenario that WR_TO_PINRESET=1 and one of the above listed sources is triggered."]
    #[inline(always)]
    pub fn wr_to_pinreset(&self) -> WR_TO_PINRESET_R {
        WR_TO_PINRESET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No registred event 1: A system CPU LOCKUP event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
    #[inline(always)]
    pub fn lockup_stat(&self) -> LOCKUP_STAT_R {
        LOCKUP_STAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
0: No registered event 1: A WDT event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
    #[inline(always)]
    pub fn wdt_stat(&self) -> WDT_STAT_R {
        WDT_STAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
0: No action 1: A warm system reset event triggered by the below listed sources will result in an emulated pin reset. Warm reset sources included: ICEPick sysreset System CPU reset request, CPU_SCS:AIRCR.SYSRESETREQ System CPU Lockup WDT timeout An active ICEPick block system reset will gate all sources except ICEPick sysreset SW can read AON_SYSCTL:RESETCTL.RESET_SRC to find the source of the last reset resulting in a full power up sequence. WARMRESET in this register is set in the scenario that WR_TO_PINRESET=1 and one of the above listed sources is triggered."]
    #[inline(always)]
    pub fn wr_to_pinreset(&mut self) -> WR_TO_PINRESET_W {
        WR_TO_PINRESET_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: No registred event 1: A system CPU LOCKUP event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
    #[inline(always)]
    pub fn lockup_stat(&mut self) -> LOCKUP_STAT_W {
        LOCKUP_STAT_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: No registered event 1: A WDT event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
    #[inline(always)]
    pub fn wdt_stat(&mut self) -> WDT_STAT_W {
        WDT_STAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WARM Reset Control And Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [warmreset](index.html) module"]
pub struct WARMRESET_SPEC;
impl crate::RegisterSpec for WARMRESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [warmreset::R](R) reader structure"]
impl crate::Readable for WARMRESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [warmreset::W](W) writer structure"]
impl crate::Writable for WARMRESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WARMRESET to value 0"]
impl crate::Resettable for WARMRESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
