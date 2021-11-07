#[doc = "Register `PWMCLKEN` reader"]
pub struct R(crate::R<PWMCLKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMCLKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMCLKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMCLKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWMCLKEN` writer"]
pub struct W(crate::W<PWMCLKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMCLKEN_SPEC>;
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
impl From<crate::W<PWMCLKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMCLKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED11_R(crate::FieldReader<u32, u32>);
impl RESERVED11_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED11_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED11` writer - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED11_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x001f_ffff << 11)) | ((value as u32 & 0x001f_ffff) << 11);
        self.w
    }
}
#[doc = "Field `RFCTRC` reader - 10:10\\]
Enable clock to the RF Core Tracer (RFCTRC) module."]
pub struct RFCTRC_R(crate::FieldReader<bool, bool>);
impl RFCTRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFCTRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCTRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFCTRC` writer - 10:10\\]
Enable clock to the RF Core Tracer (RFCTRC) module."]
pub struct RFCTRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCTRC_W<'a> {
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
#[doc = "Field `FSCA` reader - 9:9\\]
Enable clock to the Frequency Synthesizer Calibration Accelerator (FSCA) module."]
pub struct FSCA_R(crate::FieldReader<bool, bool>);
impl FSCA_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSCA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSCA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSCA` writer - 9:9\\]
Enable clock to the Frequency Synthesizer Calibration Accelerator (FSCA) module."]
pub struct FSCA_W<'a> {
    w: &'a mut W,
}
impl<'a> FSCA_W<'a> {
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
#[doc = "Field `PHA` reader - 8:8\\]
Enable clock to the Packet Handling Accelerator (PHA) module."]
pub struct PHA_R(crate::FieldReader<bool, bool>);
impl PHA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHA` writer - 8:8\\]
Enable clock to the Packet Handling Accelerator (PHA) module."]
pub struct PHA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHA_W<'a> {
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
#[doc = "Field `RAT` reader - 7:7\\]
Enable clock to the Radio Timer (RAT) module."]
pub struct RAT_R(crate::FieldReader<bool, bool>);
impl RAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAT` writer - 7:7\\]
Enable clock to the Radio Timer (RAT) module."]
pub struct RAT_W<'a> {
    w: &'a mut W,
}
impl<'a> RAT_W<'a> {
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
#[doc = "Field `RFERAM` reader - 6:6\\]
Enable clock to the RF Engine RAM module."]
pub struct RFERAM_R(crate::FieldReader<bool, bool>);
impl RFERAM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFERAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFERAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFERAM` writer - 6:6\\]
Enable clock to the RF Engine RAM module."]
pub struct RFERAM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFERAM_W<'a> {
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
#[doc = "Field `RFE` reader - 5:5\\]
Enable clock to the RF Engine (RFE) module."]
pub struct RFE_R(crate::FieldReader<bool, bool>);
impl RFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFE` writer - 5:5\\]
Enable clock to the RF Engine (RFE) module."]
pub struct RFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFE_W<'a> {
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
#[doc = "Field `MDMRAM` reader - 4:4\\]
Enable clock to the Modem RAM module."]
pub struct MDMRAM_R(crate::FieldReader<bool, bool>);
impl MDMRAM_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDMRAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDMRAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDMRAM` writer - 4:4\\]
Enable clock to the Modem RAM module."]
pub struct MDMRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMRAM_W<'a> {
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
#[doc = "Field `MDM` reader - 3:3\\]
Enable clock to the Modem (MDM) module."]
pub struct MDM_R(crate::FieldReader<bool, bool>);
impl MDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDM` writer - 3:3\\]
Enable clock to the Modem (MDM) module."]
pub struct MDM_W<'a> {
    w: &'a mut W,
}
impl<'a> MDM_W<'a> {
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
#[doc = "Field `CPERAM` reader - 2:2\\]
Enable clock to the Command and Packet Engine (CPE) RAM module. As part of RF Core initialization, set this bit together with CPE bit to enable CPE to boot."]
pub struct CPERAM_R(crate::FieldReader<bool, bool>);
impl CPERAM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPERAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPERAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPERAM` writer - 2:2\\]
Enable clock to the Command and Packet Engine (CPE) RAM module. As part of RF Core initialization, set this bit together with CPE bit to enable CPE to boot."]
pub struct CPERAM_W<'a> {
    w: &'a mut W,
}
impl<'a> CPERAM_W<'a> {
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
#[doc = "Field `CPE` reader - 1:1\\]
Enable processor clock (hclk) to the Command and Packet Engine (CPE). As part of RF Core initialization, set this bit together with CPERAM bit to enable CPE to boot."]
pub struct CPE_R(crate::FieldReader<bool, bool>);
impl CPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPE` writer - 1:1\\]
Enable processor clock (hclk) to the Command and Packet Engine (CPE). As part of RF Core initialization, set this bit together with CPERAM bit to enable CPE to boot."]
pub struct CPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPE_W<'a> {
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
#[doc = "Field `RFC` reader - 0:0\\]
Enable essential clocks for the RF Core interface. This includes the interconnect, the radio doorbell DBELL command interface, the power management (PWR) clock control module, and bus clock (sclk) for the CPE. To remove possibility of locking yourself out from the RF Core, this bit can not be cleared. If you need to disable all clocks to the RF Core, see the PRCM:RFCCLKG.CLK_EN register."]
pub struct RFC_R(crate::FieldReader<bool, bool>);
impl RFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFC` writer - 0:0\\]
Enable essential clocks for the RF Core interface. This includes the interconnect, the radio doorbell DBELL command interface, the power management (PWR) clock control module, and bus clock (sclk) for the CPE. To remove possibility of locking yourself out from the RF Core, this bit can not be cleared. If you need to disable all clocks to the RF Core, see the PRCM:RFCCLKG.CLK_EN register."]
pub struct RFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RFC_W<'a> {
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
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 11) & 0x001f_ffff) as u32)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable clock to the RF Core Tracer (RFCTRC) module."]
    #[inline(always)]
    pub fn rfctrc(&self) -> RFCTRC_R {
        RFCTRC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable clock to the Frequency Synthesizer Calibration Accelerator (FSCA) module."]
    #[inline(always)]
    pub fn fsca(&self) -> FSCA_R {
        FSCA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable clock to the Packet Handling Accelerator (PHA) module."]
    #[inline(always)]
    pub fn pha(&self) -> PHA_R {
        PHA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable clock to the Radio Timer (RAT) module."]
    #[inline(always)]
    pub fn rat(&self) -> RAT_R {
        RAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable clock to the RF Engine RAM module."]
    #[inline(always)]
    pub fn rferam(&self) -> RFERAM_R {
        RFERAM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable clock to the RF Engine (RFE) module."]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable clock to the Modem RAM module."]
    #[inline(always)]
    pub fn mdmram(&self) -> MDMRAM_R {
        MDMRAM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable clock to the Modem (MDM) module."]
    #[inline(always)]
    pub fn mdm(&self) -> MDM_R {
        MDM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable clock to the Command and Packet Engine (CPE) RAM module. As part of RF Core initialization, set this bit together with CPE bit to enable CPE to boot."]
    #[inline(always)]
    pub fn cperam(&self) -> CPERAM_R {
        CPERAM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable processor clock (hclk) to the Command and Packet Engine (CPE). As part of RF Core initialization, set this bit together with CPERAM bit to enable CPE to boot."]
    #[inline(always)]
    pub fn cpe(&self) -> CPE_R {
        CPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Enable essential clocks for the RF Core interface. This includes the interconnect, the radio doorbell DBELL command interface, the power management (PWR) clock control module, and bus clock (sclk) for the CPE. To remove possibility of locking yourself out from the RF Core, this bit can not be cleared. If you need to disable all clocks to the RF Core, see the PRCM:RFCCLKG.CLK_EN register."]
    #[inline(always)]
    pub fn rfc(&self) -> RFC_R {
        RFC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&mut self) -> RESERVED11_W {
        RESERVED11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Enable clock to the RF Core Tracer (RFCTRC) module."]
    #[inline(always)]
    pub fn rfctrc(&mut self) -> RFCTRC_W {
        RFCTRC_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Enable clock to the Frequency Synthesizer Calibration Accelerator (FSCA) module."]
    #[inline(always)]
    pub fn fsca(&mut self) -> FSCA_W {
        FSCA_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Enable clock to the Packet Handling Accelerator (PHA) module."]
    #[inline(always)]
    pub fn pha(&mut self) -> PHA_W {
        PHA_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Enable clock to the Radio Timer (RAT) module."]
    #[inline(always)]
    pub fn rat(&mut self) -> RAT_W {
        RAT_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Enable clock to the RF Engine RAM module."]
    #[inline(always)]
    pub fn rferam(&mut self) -> RFERAM_W {
        RFERAM_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Enable clock to the RF Engine (RFE) module."]
    #[inline(always)]
    pub fn rfe(&mut self) -> RFE_W {
        RFE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Enable clock to the Modem RAM module."]
    #[inline(always)]
    pub fn mdmram(&mut self) -> MDMRAM_W {
        MDMRAM_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Enable clock to the Modem (MDM) module."]
    #[inline(always)]
    pub fn mdm(&mut self) -> MDM_W {
        MDM_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Enable clock to the Command and Packet Engine (CPE) RAM module. As part of RF Core initialization, set this bit together with CPE bit to enable CPE to boot."]
    #[inline(always)]
    pub fn cperam(&mut self) -> CPERAM_W {
        CPERAM_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Enable processor clock (hclk) to the Command and Packet Engine (CPE). As part of RF Core initialization, set this bit together with CPERAM bit to enable CPE to boot."]
    #[inline(always)]
    pub fn cpe(&mut self) -> CPE_W {
        CPE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Enable essential clocks for the RF Core interface. This includes the interconnect, the radio doorbell DBELL command interface, the power management (PWR) clock control module, and bus clock (sclk) for the CPE. To remove possibility of locking yourself out from the RF Core, this bit can not be cleared. If you need to disable all clocks to the RF Core, see the PRCM:RFCCLKG.CLK_EN register."]
    #[inline(always)]
    pub fn rfc(&mut self) -> RFC_W {
        RFC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF Core Power Management and Clock Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmclken](index.html) module"]
pub struct PWMCLKEN_SPEC;
impl crate::RegisterSpec for PWMCLKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwmclken::R](R) reader structure"]
impl crate::Readable for PWMCLKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwmclken::W](W) writer structure"]
impl crate::Writable for PWMCLKEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWMCLKEN to value 0x01"]
impl crate::Resettable for PWMCLKEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
