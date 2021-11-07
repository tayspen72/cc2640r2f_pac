#[doc = "Register `IRQSTATMASK` reader"]
pub struct R(crate::R<IRQSTATMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQSTATMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQSTATMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQSTATMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQSTATMASK` writer"]
pub struct W(crate::W<IRQSTATMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQSTATMASK_SPEC>;
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
impl From<crate::W<IRQSTATMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQSTATMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_R(crate::FieldReader<u32, u32>);
impl RESERVED2_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "Field `SHUTDOWN_OVF` reader - 1:1\\]
Shutdown Overflow (result of IRQFLAGSTAT.SHUTDOWN_OVF AND'ed with IRQFLAGMASK.SHUTDOWN_OVF)"]
pub struct SHUTDOWN_OVF_R(crate::FieldReader<bool, bool>);
impl SHUTDOWN_OVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHUTDOWN_OVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHUTDOWN_OVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHUTDOWN_OVF` writer - 1:1\\]
Shutdown Overflow (result of IRQFLAGSTAT.SHUTDOWN_OVF AND'ed with IRQFLAGMASK.SHUTDOWN_OVF)"]
pub struct SHUTDOWN_OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> SHUTDOWN_OVF_W<'a> {
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
#[doc = "Field `RDY` reader - 0:0\\]
New random value available (result of IRQFLAGSTAT.RDY AND'ed with IRQFLAGMASK.RDY)"]
pub struct RDY_R(crate::FieldReader<bool, bool>);
impl RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDY` writer - 0:0\\]
New random value available (result of IRQFLAGSTAT.RDY AND'ed with IRQFLAGMASK.RDY)"]
pub struct RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RDY_W<'a> {
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
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 1 - 1:1\\]
Shutdown Overflow (result of IRQFLAGSTAT.SHUTDOWN_OVF AND'ed with IRQFLAGMASK.SHUTDOWN_OVF)"]
    #[inline(always)]
    pub fn shutdown_ovf(&self) -> SHUTDOWN_OVF_R {
        SHUTDOWN_OVF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
New random value available (result of IRQFLAGSTAT.RDY AND'ed with IRQFLAGMASK.RDY)"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Shutdown Overflow (result of IRQFLAGSTAT.SHUTDOWN_OVF AND'ed with IRQFLAGMASK.SHUTDOWN_OVF)"]
    #[inline(always)]
    pub fn shutdown_ovf(&mut self) -> SHUTDOWN_OVF_W {
        SHUTDOWN_OVF_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
New random value available (result of IRQFLAGSTAT.RDY AND'ed with IRQFLAGMASK.RDY)"]
    #[inline(always)]
    pub fn rdy(&mut self) -> RDY_W {
        RDY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status After Masking\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqstatmask](index.html) module"]
pub struct IRQSTATMASK_SPEC;
impl crate::RegisterSpec for IRQSTATMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqstatmask::R](R) reader structure"]
impl crate::Readable for IRQSTATMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqstatmask::W](W) writer structure"]
impl crate::Writable for IRQSTATMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQSTATMASK to value 0"]
impl crate::Resettable for IRQSTATMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
