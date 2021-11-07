#[doc = "Register `RAMRETEN` reader"]
pub struct R(crate::R<RAMRETEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMRETEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMRETEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMRETEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMRETEN` writer"]
pub struct W(crate::W<RAMRETEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMRETEN_SPEC>;
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
impl From<crate::W<RAMRETEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMRETEN_SPEC>) -> Self {
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
#[doc = "Field `RFC` reader - 2:2\\]
0: Retention for RFC SRAM disabled 1: Retention for RFC SRAM enabled Memories controlled: CPERAM MCERAM RFERAM"]
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
#[doc = "Field `RFC` writer - 2:2\\]
0: Retention for RFC SRAM disabled 1: Retention for RFC SRAM enabled Memories controlled: CPERAM MCERAM RFERAM"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `VIMS` reader - 1:0\\]
0: Memory retention disabled 1: Memory retention enabled Bit 0: VIMS_TRAM Bit 1: VIMS_CRAM Legal modes depend on settings in VIMS:CTL.MODE 00: VIMS:CTL.MODE must be OFF before DEEPSLEEP is asserted - must be set to CACHE or SPLIT mode after waking up again 01: VIMS:CTL.MODE must be GPRAM before DEEPSLEEP is asserted. Must remain in GPRAM mode after wake up, alternatively select OFF mode first and then CACHE or SPILT mode. 10: Illegal mode 11: No restrictions"]
pub struct VIMS_R(crate::FieldReader<u8, u8>);
impl VIMS_R {
    pub(crate) fn new(bits: u8) -> Self {
        VIMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VIMS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VIMS` writer - 1:0\\]
0: Memory retention disabled 1: Memory retention enabled Bit 0: VIMS_TRAM Bit 1: VIMS_CRAM Legal modes depend on settings in VIMS:CTL.MODE 00: VIMS:CTL.MODE must be OFF before DEEPSLEEP is asserted - must be set to CACHE or SPLIT mode after waking up again 01: VIMS:CTL.MODE must be GPRAM before DEEPSLEEP is asserted. Must remain in GPRAM mode after wake up, alternatively select OFF mode first and then CACHE or SPILT mode. 10: Illegal mode 11: No restrictions"]
pub struct VIMS_W<'a> {
    w: &'a mut W,
}
impl<'a> VIMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
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
0: Retention for RFC SRAM disabled 1: Retention for RFC SRAM enabled Memories controlled: CPERAM MCERAM RFERAM"]
    #[inline(always)]
    pub fn rfc(&self) -> RFC_R {
        RFC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - 1:0\\]
0: Memory retention disabled 1: Memory retention enabled Bit 0: VIMS_TRAM Bit 1: VIMS_CRAM Legal modes depend on settings in VIMS:CTL.MODE 00: VIMS:CTL.MODE must be OFF before DEEPSLEEP is asserted - must be set to CACHE or SPLIT mode after waking up again 01: VIMS:CTL.MODE must be GPRAM before DEEPSLEEP is asserted. Must remain in GPRAM mode after wake up, alternatively select OFF mode first and then CACHE or SPILT mode. 10: Illegal mode 11: No restrictions"]
    #[inline(always)]
    pub fn vims(&self) -> VIMS_R {
        VIMS_R::new((self.bits & 0x03) as u8)
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
0: Retention for RFC SRAM disabled 1: Retention for RFC SRAM enabled Memories controlled: CPERAM MCERAM RFERAM"]
    #[inline(always)]
    pub fn rfc(&mut self) -> RFC_W {
        RFC_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
0: Memory retention disabled 1: Memory retention enabled Bit 0: VIMS_TRAM Bit 1: VIMS_CRAM Legal modes depend on settings in VIMS:CTL.MODE 00: VIMS:CTL.MODE must be OFF before DEEPSLEEP is asserted - must be set to CACHE or SPLIT mode after waking up again 01: VIMS:CTL.MODE must be GPRAM before DEEPSLEEP is asserted. Must remain in GPRAM mode after wake up, alternatively select OFF mode first and then CACHE or SPILT mode. 10: Illegal mode 11: No restrictions"]
    #[inline(always)]
    pub fn vims(&mut self) -> VIMS_W {
        VIMS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Retention Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramreten](index.html) module"]
pub struct RAMRETEN_SPEC;
impl crate::RegisterSpec for RAMRETEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramreten::R](R) reader structure"]
impl crate::Readable for RAMRETEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramreten::W](W) writer structure"]
impl crate::Writable for RAMRETEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAMRETEN to value 0x03"]
impl crate::Resettable for RAMRETEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
