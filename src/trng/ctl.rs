#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTUP_CYCLES` reader - 31:16\\]
This field determines the number of samples (between 2^8 and 2^24) taken to gather entropy from the FROs during startup. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while TRNG_EN is 0. If 1 an update will be ignored."]
pub struct STARTUP_CYCLES_R(crate::FieldReader<u16, u16>);
impl STARTUP_CYCLES_R {
    pub(crate) fn new(bits: u16) -> Self {
        STARTUP_CYCLES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTUP_CYCLES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTUP_CYCLES` writer - 31:16\\]
This field determines the number of samples (between 2^8 and 2^24) taken to gather entropy from the FROs during startup. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while TRNG_EN is 0. If 1 an update will be ignored."]
pub struct STARTUP_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTUP_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED11` reader - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED11_R(crate::FieldReader<u8, u8>);
impl RESERVED11_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED11` writer - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED11_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Field `TRNG_EN` reader - 10:10\\]
0: Forces all TRNG logic back into the idle state immediately. 1: Starts TRNG, gathering entropy from the FROs for the number of samples determined by STARTUP_CYCLES."]
pub struct TRNG_EN_R(crate::FieldReader<bool, bool>);
impl TRNG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRNG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRNG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRNG_EN` writer - 10:10\\]
0: Forces all TRNG logic back into the idle state immediately. 1: Starts TRNG, gathering entropy from the FROs for the number of samples determined by STARTUP_CYCLES."]
pub struct TRNG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG_EN_W<'a> {
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
#[doc = "Field `RESERVED3` reader - 9:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_R(crate::FieldReader<u8, u8>);
impl RESERVED3_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 9:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 3)) | ((value as u32 & 0x7f) << 3);
        self.w
    }
}
#[doc = "Field `NO_LFSR_FB` reader - 2:2\\]
1: Remove XNOR feedback from the main LFSR, converting it into a normal shift register for the XOR-ed outputs of the FROs (shifting data in on the LSB side). A '1' also forces the LFSR to sample continuously. This bit can only be set to '1' when TEST_MODE is also set to '1' and should not be used for other than test purposes"]
pub struct NO_LFSR_FB_R(crate::FieldReader<bool, bool>);
impl NO_LFSR_FB_R {
    pub(crate) fn new(bits: bool) -> Self {
        NO_LFSR_FB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NO_LFSR_FB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NO_LFSR_FB` writer - 2:2\\]
1: Remove XNOR feedback from the main LFSR, converting it into a normal shift register for the XOR-ed outputs of the FROs (shifting data in on the LSB side). A '1' also forces the LFSR to sample continuously. This bit can only be set to '1' when TEST_MODE is also set to '1' and should not be used for other than test purposes"]
pub struct NO_LFSR_FB_W<'a> {
    w: &'a mut W,
}
impl<'a> NO_LFSR_FB_W<'a> {
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
#[doc = "Field `TEST_MODE` reader - 1:1\\]
1: Enables access to the TESTCNT and LFSR0/LFSR1/LFSR2 registers (the latter are automatically cleared before enabling access) and keeps IRQFLAGSTAT.NEED_CLOCK at '1'. This bit shall not be used unless you need to change the LFSR seed prior to creating a new random value. All other testing is done external to register control."]
pub struct TEST_MODE_R(crate::FieldReader<bool, bool>);
impl TEST_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEST_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_MODE` writer - 1:1\\]
1: Enables access to the TESTCNT and LFSR0/LFSR1/LFSR2 registers (the latter are automatically cleared before enabling access) and keeps IRQFLAGSTAT.NEED_CLOCK at '1'. This bit shall not be used unless you need to change the LFSR seed prior to creating a new random value. All other testing is done external to register control."]
pub struct TEST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_MODE_W<'a> {
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
    #[doc = "Bits 16:31 - 31:16\\]
This field determines the number of samples (between 2^8 and 2^24) taken to gather entropy from the FROs during startup. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while TRNG_EN is 0. If 1 an update will be ignored."]
    #[inline(always)]
    pub fn startup_cycles(&self) -> STARTUP_CYCLES_R {
        STARTUP_CYCLES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Forces all TRNG logic back into the idle state immediately. 1: Starts TRNG, gathering entropy from the FROs for the number of samples determined by STARTUP_CYCLES."]
    #[inline(always)]
    pub fn trng_en(&self) -> TRNG_EN_R {
        TRNG_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 3:9 - 9:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
1: Remove XNOR feedback from the main LFSR, converting it into a normal shift register for the XOR-ed outputs of the FROs (shifting data in on the LSB side). A '1' also forces the LFSR to sample continuously. This bit can only be set to '1' when TEST_MODE is also set to '1' and should not be used for other than test purposes"]
    #[inline(always)]
    pub fn no_lfsr_fb(&self) -> NO_LFSR_FB_R {
        NO_LFSR_FB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Enables access to the TESTCNT and LFSR0/LFSR1/LFSR2 registers (the latter are automatically cleared before enabling access) and keeps IRQFLAGSTAT.NEED_CLOCK at '1'. This bit shall not be used unless you need to change the LFSR seed prior to creating a new random value. All other testing is done external to register control."]
    #[inline(always)]
    pub fn test_mode(&self) -> TEST_MODE_R {
        TEST_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
This field determines the number of samples (between 2^8 and 2^24) taken to gather entropy from the FROs during startup. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while TRNG_EN is 0. If 1 an update will be ignored."]
    #[inline(always)]
    pub fn startup_cycles(&mut self) -> STARTUP_CYCLES_W {
        STARTUP_CYCLES_W { w: self }
    }
    #[doc = "Bits 11:15 - 15:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&mut self) -> RESERVED11_W {
        RESERVED11_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
0: Forces all TRNG logic back into the idle state immediately. 1: Starts TRNG, gathering entropy from the FROs for the number of samples determined by STARTUP_CYCLES."]
    #[inline(always)]
    pub fn trng_en(&mut self) -> TRNG_EN_W {
        TRNG_EN_W { w: self }
    }
    #[doc = "Bits 3:9 - 9:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
1: Remove XNOR feedback from the main LFSR, converting it into a normal shift register for the XOR-ed outputs of the FROs (shifting data in on the LSB side). A '1' also forces the LFSR to sample continuously. This bit can only be set to '1' when TEST_MODE is also set to '1' and should not be used for other than test purposes"]
    #[inline(always)]
    pub fn no_lfsr_fb(&mut self) -> NO_LFSR_FB_W {
        NO_LFSR_FB_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
1: Enables access to the TESTCNT and LFSR0/LFSR1/LFSR2 registers (the latter are automatically cleared before enabling access) and keeps IRQFLAGSTAT.NEED_CLOCK at '1'. This bit shall not be used unless you need to change the LFSR seed prior to creating a new random value. All other testing is done external to register control."]
    #[inline(always)]
    pub fn test_mode(&mut self) -> TEST_MODE_W {
        TEST_MODE_W { w: self }
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
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
