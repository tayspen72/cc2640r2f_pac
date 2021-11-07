#[doc = "Register `PWRSTAT` reader"]
pub struct R(crate::R<PWRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRSTAT` writer"]
pub struct W(crate::W<PWRSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRSTAT_SPEC>;
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
impl From<crate::W<PWRSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED10` reader - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED10_R(crate::FieldReader<u32, u32>);
impl RESERVED10_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED10_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED10` writer - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED10_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | ((value as u32 & 0x003f_ffff) << 10);
        self.w
    }
}
#[doc = "Field `AUX_PWR_DWN` reader - 9:9\\]
Indicates the AUX powerdown state when AUX domain is powered up. 0: Active mode 1: AUX Powerdown request has been granted"]
pub struct AUX_PWR_DWN_R(crate::FieldReader<bool, bool>);
impl AUX_PWR_DWN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUX_PWR_DWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUX_PWR_DWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUX_PWR_DWN` writer - 9:9\\]
Indicates the AUX powerdown state when AUX domain is powered up. 0: Active mode 1: AUX Powerdown request has been granted"]
pub struct AUX_PWR_DWN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_PWR_DWN_W<'a> {
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
#[doc = "Field `RESERVED7` reader - 8:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED7_R(crate::FieldReader<u8, u8>);
impl RESERVED7_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED7` writer - 8:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `JTAG_PD_ON` reader - 6:6\\]
Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on"]
pub struct JTAG_PD_ON_R(crate::FieldReader<bool, bool>);
impl JTAG_PD_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        JTAG_PD_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAG_PD_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JTAG_PD_ON` writer - 6:6\\]
Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on"]
pub struct JTAG_PD_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_PD_ON_W<'a> {
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
#[doc = "Field `AUX_PD_ON` reader - 5:5\\]
Indicates AUX power state: 0: AUX is not ready for use ( may be powered off or in power state transition ) 1: AUX is powered on, connected to bus and ready for use,"]
pub struct AUX_PD_ON_R(crate::FieldReader<bool, bool>);
impl AUX_PD_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUX_PD_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUX_PD_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUX_PD_ON` writer - 5:5\\]
Indicates AUX power state: 0: AUX is not ready for use ( may be powered off or in power state transition ) 1: AUX is powered on, connected to bus and ready for use,"]
pub struct AUX_PD_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_PD_ON_W<'a> {
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
#[doc = "Field `MCU_PD_ON` reader - 4:4\\]
Indicates MCU power state: 0: MCU Power sequencing is not yet finalized and MCU_AONIF registers may not be reliable 1: MCU Power sequencing is finalized and all MCU_AONIF registers are reliable"]
pub struct MCU_PD_ON_R(crate::FieldReader<bool, bool>);
impl MCU_PD_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCU_PD_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_PD_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_PD_ON` writer - 4:4\\]
Indicates MCU power state: 0: MCU Power sequencing is not yet finalized and MCU_AONIF registers may not be reliable 1: MCU Power sequencing is finalized and all MCU_AONIF registers are reliable"]
pub struct MCU_PD_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_PD_ON_W<'a> {
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
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_R(crate::FieldReader<bool, bool>);
impl RESERVED3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
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
#[doc = "Field `AUX_BUS_CONNECTED` reader - 2:2\\]
Indicates that AUX Bus is connected: 0: AUX bus is not connected 1: AUX bus is connected ( idle_ack = 0 )"]
pub struct AUX_BUS_CONNECTED_R(crate::FieldReader<bool, bool>);
impl AUX_BUS_CONNECTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUX_BUS_CONNECTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUX_BUS_CONNECTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUX_BUS_CONNECTED` writer - 2:2\\]
Indicates that AUX Bus is connected: 0: AUX bus is not connected 1: AUX bus is connected ( idle_ack = 0 )"]
pub struct AUX_BUS_CONNECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_BUS_CONNECTED_W<'a> {
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
#[doc = "Field `AUX_RESET_DONE` reader - 1:1\\]
Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released"]
pub struct AUX_RESET_DONE_R(crate::FieldReader<bool, bool>);
impl AUX_RESET_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUX_RESET_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUX_RESET_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUX_RESET_DONE` writer - 1:1\\]
Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released"]
pub struct AUX_RESET_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_RESET_DONE_W<'a> {
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
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_R(crate::FieldReader<bool, bool>);
impl RESERVED0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
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
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 9 - 9:9\\]
Indicates the AUX powerdown state when AUX domain is powered up. 0: Active mode 1: AUX Powerdown request has been granted"]
    #[inline(always)]
    pub fn aux_pwr_dwn(&self) -> AUX_PWR_DWN_R {
        AUX_PWR_DWN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on"]
    #[inline(always)]
    pub fn jtag_pd_on(&self) -> JTAG_PD_ON_R {
        JTAG_PD_ON_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates AUX power state: 0: AUX is not ready for use ( may be powered off or in power state transition ) 1: AUX is powered on, connected to bus and ready for use,"]
    #[inline(always)]
    pub fn aux_pd_on(&self) -> AUX_PD_ON_R {
        AUX_PD_ON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates MCU power state: 0: MCU Power sequencing is not yet finalized and MCU_AONIF registers may not be reliable 1: MCU Power sequencing is finalized and all MCU_AONIF registers are reliable"]
    #[inline(always)]
    pub fn mcu_pd_on(&self) -> MCU_PD_ON_R {
        MCU_PD_ON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates that AUX Bus is connected: 0: AUX bus is not connected 1: AUX bus is connected ( idle_ack = 0 )"]
    #[inline(always)]
    pub fn aux_bus_connected(&self) -> AUX_BUS_CONNECTED_R {
        AUX_BUS_CONNECTED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released"]
    #[inline(always)]
    pub fn aux_reset_done(&self) -> AUX_RESET_DONE_R {
        AUX_RESET_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&mut self) -> RESERVED10_W {
        RESERVED10_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Indicates the AUX powerdown state when AUX domain is powered up. 0: Active mode 1: AUX Powerdown request has been granted"]
    #[inline(always)]
    pub fn aux_pwr_dwn(&mut self) -> AUX_PWR_DWN_W {
        AUX_PWR_DWN_W { w: self }
    }
    #[doc = "Bits 7:8 - 8:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on"]
    #[inline(always)]
    pub fn jtag_pd_on(&mut self) -> JTAG_PD_ON_W {
        JTAG_PD_ON_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates AUX power state: 0: AUX is not ready for use ( may be powered off or in power state transition ) 1: AUX is powered on, connected to bus and ready for use,"]
    #[inline(always)]
    pub fn aux_pd_on(&mut self) -> AUX_PD_ON_W {
        AUX_PD_ON_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates MCU power state: 0: MCU Power sequencing is not yet finalized and MCU_AONIF registers may not be reliable 1: MCU Power sequencing is finalized and all MCU_AONIF registers are reliable"]
    #[inline(always)]
    pub fn mcu_pd_on(&mut self) -> MCU_PD_ON_W {
        MCU_PD_ON_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates that AUX Bus is connected: 0: AUX bus is not connected 1: AUX bus is connected ( idle_ack = 0 )"]
    #[inline(always)]
    pub fn aux_bus_connected(&mut self) -> AUX_BUS_CONNECTED_W {
        AUX_BUS_CONNECTED_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released"]
    #[inline(always)]
    pub fn aux_reset_done(&mut self) -> AUX_RESET_DONE_W {
        AUX_RESET_DONE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Status This register is used to monitor various power management related signals in AON. Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrstat](index.html) module"]
pub struct PWRSTAT_SPEC;
impl crate::RegisterSpec for PWRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrstat::R](R) reader structure"]
impl crate::Readable for PWRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrstat::W](W) writer structure"]
impl crate::Writable for PWRSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRSTAT to value 0x0380_0000"]
impl crate::Resettable for PWRSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0380_0000
    }
}
