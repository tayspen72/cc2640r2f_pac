#[doc = "Register `PRECNT` reader"]
pub struct R(crate::R<PRECNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRECNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRECNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRECNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRECNT` writer"]
pub struct W(crate::W<PRECNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRECNT_SPEC>;
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
impl From<crate::W<PRECNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRECNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_R(crate::FieldReader<u16, u16>);
impl RESERVED16_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED16_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CNT` reader - 15:0\\]
Prescaler counter value. Write a value to CNT to capture the value of the 16-bit prescaler counter into CNT. Read CNT to get the captured value. The read value gets 1 LSB uncertainty if the event source level rises when you release the reset. You must capture the prescaler counter value when the event source level is stable, either high or low: - Disable AUX I/O input buffer to clamp AUXIO event low. - Disable COMPA to clamp AUX_COMPA event low. The read value can in general get 1 LSB uncertainty when you gate the event source asynchronously. Please note the following: - The prescaler counter is reset to 2 by PRECTL.RESET_N. - The captured value is 2 when the number of rising edges on prescaler input is less than 3. Otherwise, captured value equals number of event pulses - 1."]
pub struct CNT_R(crate::FieldReader<u16, u16>);
impl CNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT` writer - 15:0\\]
Prescaler counter value. Write a value to CNT to capture the value of the 16-bit prescaler counter into CNT. Read CNT to get the captured value. The read value gets 1 LSB uncertainty if the event source level rises when you release the reset. You must capture the prescaler counter value when the event source level is stable, either high or low: - Disable AUX I/O input buffer to clamp AUXIO event low. - Disable COMPA to clamp AUX_COMPA event low. The read value can in general get 1 LSB uncertainty when you gate the event source asynchronously. Please note the following: - The prescaler counter is reset to 2 by PRECTL.RESET_N. - The captured value is 2 when the number of rising edges on prescaler input is less than 3. Otherwise, captured value equals number of event pulses - 1."]
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Prescaler counter value. Write a value to CNT to capture the value of the 16-bit prescaler counter into CNT. Read CNT to get the captured value. The read value gets 1 LSB uncertainty if the event source level rises when you release the reset. You must capture the prescaler counter value when the event source level is stable, either high or low: - Disable AUX I/O input buffer to clamp AUXIO event low. - Disable COMPA to clamp AUX_COMPA event low. The read value can in general get 1 LSB uncertainty when you gate the event source asynchronously. Please note the following: - The prescaler counter is reset to 2 by PRECTL.RESET_N. - The captured value is 2 when the number of rising edges on prescaler input is less than 3. Otherwise, captured value equals number of event pulses - 1."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Prescaler counter value. Write a value to CNT to capture the value of the 16-bit prescaler counter into CNT. Read CNT to get the captured value. The read value gets 1 LSB uncertainty if the event source level rises when you release the reset. You must capture the prescaler counter value when the event source level is stable, either high or low: - Disable AUX I/O input buffer to clamp AUXIO event low. - Disable COMPA to clamp AUX_COMPA event low. The read value can in general get 1 LSB uncertainty when you gate the event source asynchronously. Please note the following: - The prescaler counter is reset to 2 by PRECTL.RESET_N. - The captured value is 2 when the number of rising edges on prescaler input is less than 3. Otherwise, captured value equals number of event pulses - 1."]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prescaler Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [precnt](index.html) module"]
pub struct PRECNT_SPEC;
impl crate::RegisterSpec for PRECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [precnt::R](R) reader structure"]
impl crate::Readable for PRECNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [precnt::W](W) writer structure"]
impl crate::Writable for PRECNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRECNT to value 0"]
impl crate::Resettable for PRECNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
