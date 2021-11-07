#[doc = "Register `VECFLAGS` reader"]
pub struct R(crate::R<VECFLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VECFLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VECFLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VECFLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VECFLAGS` writer"]
pub struct W(crate::W<VECFLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VECFLAGS_SPEC>;
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
impl From<crate::W<VECFLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VECFLAGS_SPEC>) -> Self {
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
Vector flag 3. The vector flag is set if the edge selected VECCFG1.VEC3_POL occurs on the event selected in VECCFG1.VEC3_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC3."]
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
Vector flag 3. The vector flag is set if the edge selected VECCFG1.VEC3_POL occurs on the event selected in VECCFG1.VEC3_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC3."]
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
Vector flag 2. The vector flag is set if the edge selected VECCFG1.VEC2_POL occurs on the event selected in VECCFG1.VEC2_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC2."]
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
Vector flag 2. The vector flag is set if the edge selected VECCFG1.VEC2_POL occurs on the event selected in VECCFG1.VEC2_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC2."]
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
Vector flag 1. The vector flag is set if the edge selected VECCFG0.VEC1_POL occurs on the event selected in VECCFG0.VEC1_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC1."]
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
Vector flag 1. The vector flag is set if the edge selected VECCFG0.VEC1_POL occurs on the event selected in VECCFG0.VEC1_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC1."]
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
Vector flag 0. The vector flag is set if the edge selected VECCFG0.VEC0_POL occurs on the event selected in VECCFG0.VEC0_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC0."]
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
Vector flag 0. The vector flag is set if the edge selected VECCFG0.VEC0_POL occurs on the event selected in VECCFG0.VEC0_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC0."]
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
Vector flag 3. The vector flag is set if the edge selected VECCFG1.VEC3_POL occurs on the event selected in VECCFG1.VEC3_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC3."]
    #[inline(always)]
    pub fn vec3(&self) -> VEC3_R {
        VEC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Vector flag 2. The vector flag is set if the edge selected VECCFG1.VEC2_POL occurs on the event selected in VECCFG1.VEC2_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC2."]
    #[inline(always)]
    pub fn vec2(&self) -> VEC2_R {
        VEC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Vector flag 1. The vector flag is set if the edge selected VECCFG0.VEC1_POL occurs on the event selected in VECCFG0.VEC1_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC1."]
    #[inline(always)]
    pub fn vec1(&self) -> VEC1_R {
        VEC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Vector flag 0. The vector flag is set if the edge selected VECCFG0.VEC0_POL occurs on the event selected in VECCFG0.VEC0_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC0."]
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
Vector flag 3. The vector flag is set if the edge selected VECCFG1.VEC3_POL occurs on the event selected in VECCFG1.VEC3_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC3."]
    #[inline(always)]
    pub fn vec3(&mut self) -> VEC3_W {
        VEC3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Vector flag 2. The vector flag is set if the edge selected VECCFG1.VEC2_POL occurs on the event selected in VECCFG1.VEC2_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC2."]
    #[inline(always)]
    pub fn vec2(&mut self) -> VEC2_W {
        VEC2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Vector flag 1. The vector flag is set if the edge selected VECCFG0.VEC1_POL occurs on the event selected in VECCFG0.VEC1_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC1."]
    #[inline(always)]
    pub fn vec1(&mut self) -> VEC1_W {
        VEC1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Vector flag 0. The vector flag is set if the edge selected VECCFG0.VEC0_POL occurs on the event selected in VECCFG0.VEC0_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC0."]
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
#[doc = "Vector Flags If a vector flag becomes 1 and AUX_SCE sleeps, AUX_SCE will wake up and execute the corresponding vector. The vector with the lowest index will execute first if multiple vectors flags are set. AUX_SCE must return to sleep to execute the next vector. During execution of a vector, AUX_SCE must clear the vector flag that triggered execution. Write 1 to bit index n in VECFLAGSCLR to clear vector flag n.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vecflags](index.html) module"]
pub struct VECFLAGS_SPEC;
impl crate::RegisterSpec for VECFLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vecflags::R](R) reader structure"]
impl crate::Readable for VECFLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vecflags::W](W) writer structure"]
impl crate::Writable for VECFLAGS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VECFLAGS to value 0"]
impl crate::Resettable for VECFLAGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
