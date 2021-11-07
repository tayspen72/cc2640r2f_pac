#[doc = "Register `SLEEPCTL` reader"]
pub struct R(crate::R<SLEEPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLEEPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLEEPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLEEPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLEEPCTL` writer"]
pub struct W(crate::W<SLEEPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLEEPCTL_SPEC>;
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
impl From<crate::W<SLEEPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLEEPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1_R(crate::FieldReader<u32, u32>);
impl RESERVED1_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | ((value as u32 & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Field `IO_PAD_SLEEP_DIS` reader - 0:0\\]
Controls the I/O pad sleep mode. The boot code will set this bitfield automatically unless waking up from a SHUTDOWN ( RESETCTL.WU_FROM_SD is set ). 0: I/O pad sleep mode is enabled, ie all pads are latched and can not toggle. 1: I/O pad sleep mode is disabled Application software may want to reconfigure the state for all IO's before setting this bitfield upon waking up from a SHUTDOWN."]
pub struct IO_PAD_SLEEP_DIS_R(crate::FieldReader<bool, bool>);
impl IO_PAD_SLEEP_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        IO_PAD_SLEEP_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_PAD_SLEEP_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_PAD_SLEEP_DIS` writer - 0:0\\]
Controls the I/O pad sleep mode. The boot code will set this bitfield automatically unless waking up from a SHUTDOWN ( RESETCTL.WU_FROM_SD is set ). 0: I/O pad sleep mode is enabled, ie all pads are latched and can not toggle. 1: I/O pad sleep mode is disabled Application software may want to reconfigure the state for all IO's before setting this bitfield upon waking up from a SHUTDOWN."]
pub struct IO_PAD_SLEEP_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_PAD_SLEEP_DIS_W<'a> {
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
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\]
Controls the I/O pad sleep mode. The boot code will set this bitfield automatically unless waking up from a SHUTDOWN ( RESETCTL.WU_FROM_SD is set ). 0: I/O pad sleep mode is enabled, ie all pads are latched and can not toggle. 1: I/O pad sleep mode is disabled Application software may want to reconfigure the state for all IO's before setting this bitfield upon waking up from a SHUTDOWN."]
    #[inline(always)]
    pub fn io_pad_sleep_dis(&self) -> IO_PAD_SLEEP_DIS_R {
        IO_PAD_SLEEP_DIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Controls the I/O pad sleep mode. The boot code will set this bitfield automatically unless waking up from a SHUTDOWN ( RESETCTL.WU_FROM_SD is set ). 0: I/O pad sleep mode is enabled, ie all pads are latched and can not toggle. 1: I/O pad sleep mode is disabled Application software may want to reconfigure the state for all IO's before setting this bitfield upon waking up from a SHUTDOWN."]
    #[inline(always)]
    pub fn io_pad_sleep_dis(&mut self) -> IO_PAD_SLEEP_DIS_W {
        IO_PAD_SLEEP_DIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Mode This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleepctl](index.html) module"]
pub struct SLEEPCTL_SPEC;
impl crate::RegisterSpec for SLEEPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sleepctl::R](R) reader structure"]
impl crate::Readable for SLEEPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sleepctl::W](W) writer structure"]
impl crate::Writable for SLEEPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLEEPCTL to value 0"]
impl crate::Resettable for SLEEPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
