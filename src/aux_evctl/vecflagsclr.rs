#[doc = "Register `VECFLAGSCLR` reader"]
pub struct R(crate::R<VECFLAGSCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VECFLAGSCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VECFLAGSCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VECFLAGSCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VECFLAGSCLR` writer"]
pub struct W(crate::W<VECFLAGSCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VECFLAGSCLR_SPEC>;
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
impl From<crate::W<VECFLAGSCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VECFLAGSCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_R(crate::FieldReader<u32, u32>);
impl RESERVED4_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | ((value as u32 & 0x0fff_ffff) << 4);
        self.w
    }
}
#[doc = "Field `VEC3` reader - 3:3\\]
Clear vector flag 3. 0: No effect. 1: Clear VECFLAGS.VEC3. Read value is 0."]
pub struct VEC3_R(crate::FieldReader<bool, bool>);
impl VEC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        VEC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VEC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VEC3` writer - 3:3\\]
Clear vector flag 3. 0: No effect. 1: Clear VECFLAGS.VEC3. Read value is 0."]
pub struct VEC3_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC3_W<'a> {
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
#[doc = "Field `VEC2` reader - 2:2\\]
Clear vector flag 2. 0: No effect. 1: Clear VECFLAGS.VEC2. Read value is 0."]
pub struct VEC2_R(crate::FieldReader<bool, bool>);
impl VEC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        VEC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VEC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VEC2` writer - 2:2\\]
Clear vector flag 2. 0: No effect. 1: Clear VECFLAGS.VEC2. Read value is 0."]
pub struct VEC2_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC2_W<'a> {
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
#[doc = "Field `VEC1` reader - 1:1\\]
Clear vector flag 1. 0: No effect. 1: Clear VECFLAGS.VEC1. Read value is 0."]
pub struct VEC1_R(crate::FieldReader<bool, bool>);
impl VEC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        VEC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VEC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VEC1` writer - 1:1\\]
Clear vector flag 1. 0: No effect. 1: Clear VECFLAGS.VEC1. Read value is 0."]
pub struct VEC1_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC1_W<'a> {
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
#[doc = "Field `VEC0` reader - 0:0\\]
Clear vector flag 0. 0: No effect. 1: Clear VECFLAGS.VEC0. Read value is 0."]
pub struct VEC0_R(crate::FieldReader<bool, bool>);
impl VEC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        VEC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VEC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VEC0` writer - 0:0\\]
Clear vector flag 0. 0: No effect. 1: Clear VECFLAGS.VEC0. Read value is 0."]
pub struct VEC0_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC0_W<'a> {
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
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 3 - 3:3\\]
Clear vector flag 3. 0: No effect. 1: Clear VECFLAGS.VEC3. Read value is 0."]
    #[inline(always)]
    pub fn vec3(&self) -> VEC3_R {
        VEC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Clear vector flag 2. 0: No effect. 1: Clear VECFLAGS.VEC2. Read value is 0."]
    #[inline(always)]
    pub fn vec2(&self) -> VEC2_R {
        VEC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear vector flag 1. 0: No effect. 1: Clear VECFLAGS.VEC1. Read value is 0."]
    #[inline(always)]
    pub fn vec1(&self) -> VEC1_R {
        VEC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Clear vector flag 0. 0: No effect. 1: Clear VECFLAGS.VEC0. Read value is 0."]
    #[inline(always)]
    pub fn vec0(&self) -> VEC0_R {
        VEC0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Clear vector flag 3. 0: No effect. 1: Clear VECFLAGS.VEC3. Read value is 0."]
    #[inline(always)]
    pub fn vec3(&mut self) -> VEC3_W {
        VEC3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Clear vector flag 2. 0: No effect. 1: Clear VECFLAGS.VEC2. Read value is 0."]
    #[inline(always)]
    pub fn vec2(&mut self) -> VEC2_W {
        VEC2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Clear vector flag 1. 0: No effect. 1: Clear VECFLAGS.VEC1. Read value is 0."]
    #[inline(always)]
    pub fn vec1(&mut self) -> VEC1_W {
        VEC1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Clear vector flag 0. 0: No effect. 1: Clear VECFLAGS.VEC0. Read value is 0."]
    #[inline(always)]
    pub fn vec0(&mut self) -> VEC0_W {
        VEC0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vector Flags Clear Strobes for clearing flags in VECFLAGS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vecflagsclr](index.html) module"]
pub struct VECFLAGSCLR_SPEC;
impl crate::RegisterSpec for VECFLAGSCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vecflagsclr::R](R) reader structure"]
impl crate::Readable for VECFLAGSCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vecflagsclr::W](W) writer structure"]
impl crate::Writable for VECFLAGSCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VECFLAGSCLR to value 0"]
impl crate::Resettable for VECFLAGSCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
