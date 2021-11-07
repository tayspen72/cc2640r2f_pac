#[doc = "Register `PDSTAT1` reader"]
pub struct R(crate::R<PDSTAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSTAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDSTAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDSTAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDSTAT1` writer"]
pub struct W(crate::W<PDSTAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDSTAT1_SPEC>;
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
impl From<crate::W<PDSTAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDSTAT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED5_R(crate::FieldReader<u32, u32>);
impl RESERVED5_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED5_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED5` writer - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | ((value as u32 & 0x07ff_ffff) << 5);
        self.w
    }
}
#[doc = "Field `BUS_ON` reader - 4:4\\]
0: BUS domain not accessible 1: BUS domain is currently accessible"]
pub struct BUS_ON_R(crate::FieldReader<bool, bool>);
impl BUS_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUS_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUS_ON` writer - 4:4\\]
0: BUS domain not accessible 1: BUS domain is currently accessible"]
pub struct BUS_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_ON_W<'a> {
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
#[doc = "Field `VIMS_MODE` reader - 3:3\\]
0: VIMS domain not accessible 1: VIMS domain is currently accessible"]
pub struct VIMS_MODE_R(crate::FieldReader<bool, bool>);
impl VIMS_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VIMS_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VIMS_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VIMS_MODE` writer - 3:3\\]
0: VIMS domain not accessible 1: VIMS domain is currently accessible"]
pub struct VIMS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> VIMS_MODE_W<'a> {
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
#[doc = "Field `RFC_ON` reader - 2:2\\]
0: RFC domain not accessible 1: RFC domain is currently accessible"]
pub struct RFC_ON_R(crate::FieldReader<bool, bool>);
impl RFC_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFC_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFC_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFC_ON` writer - 2:2\\]
0: RFC domain not accessible 1: RFC domain is currently accessible"]
pub struct RFC_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> RFC_ON_W<'a> {
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
#[doc = "Field `CPU_ON` reader - 1:1\\]
0: CPU and BUS domain not accessible 1: CPU and BUS domains are both currently accessible"]
pub struct CPU_ON_R(crate::FieldReader<bool, bool>);
impl CPU_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_ON` writer - 1:1\\]
0: CPU and BUS domain not accessible 1: CPU and BUS domains are both currently accessible"]
pub struct CPU_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_ON_W<'a> {
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
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
    #[doc = "Bit 4 - 4:4\\]
0: BUS domain not accessible 1: BUS domain is currently accessible"]
    #[inline(always)]
    pub fn bus_on(&self) -> BUS_ON_R {
        BUS_ON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: VIMS domain not accessible 1: VIMS domain is currently accessible"]
    #[inline(always)]
    pub fn vims_mode(&self) -> VIMS_MODE_R {
        VIMS_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: RFC domain not accessible 1: RFC domain is currently accessible"]
    #[inline(always)]
    pub fn rfc_on(&self) -> RFC_ON_R {
        RFC_ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: CPU and BUS domain not accessible 1: CPU and BUS domains are both currently accessible"]
    #[inline(always)]
    pub fn cpu_on(&self) -> CPU_ON_R {
        CPU_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
0: BUS domain not accessible 1: BUS domain is currently accessible"]
    #[inline(always)]
    pub fn bus_on(&mut self) -> BUS_ON_W {
        BUS_ON_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
0: VIMS domain not accessible 1: VIMS domain is currently accessible"]
    #[inline(always)]
    pub fn vims_mode(&mut self) -> VIMS_MODE_W {
        VIMS_MODE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
0: RFC domain not accessible 1: RFC domain is currently accessible"]
    #[inline(always)]
    pub fn rfc_on(&mut self) -> RFC_ON_W {
        RFC_ON_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: CPU and BUS domain not accessible 1: CPU and BUS domains are both currently accessible"]
    #[inline(always)]
    pub fn cpu_on(&mut self) -> CPU_ON_W {
        CPU_ON_W { w: self }
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
#[doc = "Power Manager Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdstat1](index.html) module"]
pub struct PDSTAT1_SPEC;
impl crate::RegisterSpec for PDSTAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdstat1::R](R) reader structure"]
impl crate::Readable for PDSTAT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdstat1::W](W) writer structure"]
impl crate::Writable for PDSTAT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDSTAT1 to value 0x1a"]
impl crate::Resettable for PDSTAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1a
    }
}
