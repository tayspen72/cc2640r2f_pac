#[doc = "Register `JTAGCFG` reader"]
pub struct R(crate::R<JTAGCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JTAGCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JTAGCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JTAGCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JTAGCFG` writer"]
pub struct W(crate::W<JTAGCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JTAGCFG_SPEC>;
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
impl From<crate::W<JTAGCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JTAGCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED9_R(crate::FieldReader<u32, u32>);
impl RESERVED9_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED9_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | ((value as u32 & 0x007f_ffff) << 9);
        self.w
    }
}
#[doc = "Field `JTAG_PD_FORCE_ON` reader - 8:8\\]
Controls JTAG PowerDomain power state: 0: Controlled exclusively by debug subsystem. (JTAG Powerdomain will be powered off unless a debugger is attached) 1: JTAG Power Domain is forced on, independent of debug subsystem. NB: The reset value causes JTAG Power Domain to be powered on by default. Software must clear this bit to turn off the JTAG Power Domain"]
pub struct JTAG_PD_FORCE_ON_R(crate::FieldReader<bool, bool>);
impl JTAG_PD_FORCE_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        JTAG_PD_FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAG_PD_FORCE_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JTAG_PD_FORCE_ON` writer - 8:8\\]
Controls JTAG PowerDomain power state: 0: Controlled exclusively by debug subsystem. (JTAG Powerdomain will be powered off unless a debugger is attached) 1: JTAG Power Domain is forced on, independent of debug subsystem. NB: The reset value causes JTAG Power Domain to be powered on by default. Software must clear this bit to turn off the JTAG Power Domain"]
pub struct JTAG_PD_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_PD_FORCE_ON_W<'a> {
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
#[doc = "Field `RESERVED0` reader - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_R(crate::FieldReader<u8, u8>);
impl RESERVED0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls JTAG PowerDomain power state: 0: Controlled exclusively by debug subsystem. (JTAG Powerdomain will be powered off unless a debugger is attached) 1: JTAG Power Domain is forced on, independent of debug subsystem. NB: The reset value causes JTAG Power Domain to be powered on by default. Software must clear this bit to turn off the JTAG Power Domain"]
    #[inline(always)]
    pub fn jtag_pd_force_on(&self) -> JTAG_PD_FORCE_ON_R {
        JTAG_PD_FORCE_ON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Controls JTAG PowerDomain power state: 0: Controlled exclusively by debug subsystem. (JTAG Powerdomain will be powered off unless a debugger is attached) 1: JTAG Power Domain is forced on, independent of debug subsystem. NB: The reset value causes JTAG Power Domain to be powered on by default. Software must clear this bit to turn off the JTAG Power Domain"]
    #[inline(always)]
    pub fn jtag_pd_force_on(&mut self) -> JTAG_PD_FORCE_ON_W {
        JTAG_PD_FORCE_ON_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
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
#[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtagcfg](index.html) module"]
pub struct JTAGCFG_SPEC;
impl crate::RegisterSpec for JTAGCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jtagcfg::R](R) reader structure"]
impl crate::Readable for JTAGCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jtagcfg::W](W) writer structure"]
impl crate::Writable for JTAGCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JTAGCFG to value 0x0100"]
impl crate::Resettable for JTAGCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
