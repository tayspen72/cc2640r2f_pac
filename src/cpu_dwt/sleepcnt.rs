#[doc = "Register `SLEEPCNT` reader"]
pub struct R(crate::R<SLEEPCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLEEPCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLEEPCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLEEPCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLEEPCNT` writer"]
pub struct W(crate::W<SLEEPCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLEEPCNT_SPEC>;
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
impl From<crate::W<SLEEPCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLEEPCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED8_R(crate::FieldReader<u32, u32>);
impl RESERVED8_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED8_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Field `SLEEPCNT` reader - 7:0\\]
Sleep counter. Counts the number of cycles during which the processor is sleeping. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.SLEEPEVTENA. Note that the sleep counter is clocked using CPU's free-running clock. In some power modes the free-running clock to CPU is gated to minimize power consumption. This means that the sleep counter will be invalid in these power modes."]
pub struct SLEEPCNT_R(crate::FieldReader<u8, u8>);
impl SLEEPCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLEEPCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEEPCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEPCNT` writer - 7:0\\]
Sleep counter. Counts the number of cycles during which the processor is sleeping. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.SLEEPEVTENA. Note that the sleep counter is clocked using CPU's free-running clock. In some power modes the free-running clock to CPU is gated to minimize power consumption. This means that the sleep counter will be invalid in these power modes."]
pub struct SLEEPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Sleep counter. Counts the number of cycles during which the processor is sleeping. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.SLEEPEVTENA. Note that the sleep counter is clocked using CPU's free-running clock. In some power modes the free-running clock to CPU is gated to minimize power consumption. This means that the sleep counter will be invalid in these power modes."]
    #[inline(always)]
    pub fn sleepcnt(&self) -> SLEEPCNT_R {
        SLEEPCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Sleep counter. Counts the number of cycles during which the processor is sleeping. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.SLEEPEVTENA. Note that the sleep counter is clocked using CPU's free-running clock. In some power modes the free-running clock to CPU is gated to minimize power consumption. This means that the sleep counter will be invalid in these power modes."]
    #[inline(always)]
    pub fn sleepcnt(&mut self) -> SLEEPCNT_W {
        SLEEPCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Count This register is used to count the total number of cycles during which the processor is sleeping.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleepcnt](index.html) module"]
pub struct SLEEPCNT_SPEC;
impl crate::RegisterSpec for SLEEPCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sleepcnt::R](R) reader structure"]
impl crate::Readable for SLEEPCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sleepcnt::W](W) writer structure"]
impl crate::Writable for SLEEPCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLEEPCNT to value 0"]
impl crate::Resettable for SLEEPCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
