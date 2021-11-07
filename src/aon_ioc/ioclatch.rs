#[doc = "Register `IOCLATCH` reader"]
pub struct R(crate::R<IOCLATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCLATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOCLATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOCLATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOCLATCH` writer"]
pub struct W(crate::W<IOCLATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCLATCH_SPEC>;
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
impl From<crate::W<IOCLATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOCLATCH_SPEC>) -> Self {
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
#[doc = "0:0\\]
Controls latches between MCU IOC and AON_IOC. The latches are transparent by default. They must be closed prior to power off the domain(s) controlling the IOs in order to preserve IO values on external pins.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "1: Latches are transparent, meaning the value of the IO is directly controlled by the GPIO or peripheral value"]
    TRANSP = 1,
    #[doc = "0: Latches are static, meaning the current value on the IO pin is frozen by latches and kept even if GPIO module or a peripheral module is turned off"]
    STATIC = 0,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - 0:0\\]
Controls latches between MCU IOC and AON_IOC. The latches are transparent by default. They must be closed prior to power off the domain(s) controlling the IOs in order to preserve IO values on external pins."]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            true => EN_A::TRANSP,
            false => EN_A::STATIC,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSP`"]
    #[inline(always)]
    pub fn is_transp(&self) -> bool {
        **self == EN_A::TRANSP
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        **self == EN_A::STATIC
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - 0:0\\]
Controls latches between MCU IOC and AON_IOC. The latches are transparent by default. They must be closed prior to power off the domain(s) controlling the IOs in order to preserve IO values on external pins."]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Latches are transparent, meaning the value of the IO is directly controlled by the GPIO or peripheral value"]
    #[inline(always)]
    pub fn transp(self) -> &'a mut W {
        self.variant(EN_A::TRANSP)
    }
    #[doc = "Latches are static, meaning the current value on the IO pin is frozen by latches and kept even if GPIO module or a peripheral module is turned off"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(EN_A::STATIC)
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
Controls latches between MCU IOC and AON_IOC. The latches are transparent by default. They must be closed prior to power off the domain(s) controlling the IOs in order to preserve IO values on external pins."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
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
Controls latches between MCU IOC and AON_IOC. The latches are transparent by default. They must be closed prior to power off the domain(s) controlling the IOs in order to preserve IO values on external pins."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IO Latch Control Controls transparency of all latches holding I/O or configuration state from the MCU IOC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioclatch](index.html) module"]
pub struct IOCLATCH_SPEC;
impl crate::RegisterSpec for IOCLATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioclatch::R](R) reader structure"]
impl crate::Readable for IOCLATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioclatch::W](W) writer structure"]
impl crate::Writable for IOCLATCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOCLATCH to value 0x01"]
impl crate::Resettable for IOCLATCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
