#[doc = "Register `VTOR` reader"]
pub struct R(crate::R<VTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VTOR` writer"]
pub struct W(crate::W<VTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VTOR_SPEC>;
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
impl From<crate::W<VTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED30_R(crate::FieldReader<u8, u8>);
impl RESERVED30_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED30_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED30` writer - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED30_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `TBLOFF` reader - 29:7\\]
Bits 29 down to 7 of the vector table base offset."]
pub struct TBLOFF_R(crate::FieldReader<u32, u32>);
impl TBLOFF_R {
    pub(crate) fn new(bits: u32) -> Self {
        TBLOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBLOFF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBLOFF` writer - 29:7\\]
Bits 29 down to 7 of the vector table base offset."]
pub struct TBLOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBLOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 7)) | ((value as u32 & 0x007f_ffff) << 7);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 6:0\\]
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
#[doc = "Field `RESERVED0` writer - 6:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> RESERVED30_R {
        RESERVED30_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 7:29 - 29:7\\]
Bits 29 down to 7 of the vector table base offset."]
    #[inline(always)]
    pub fn tbloff(&self) -> TBLOFF_R {
        TBLOFF_R::new(((self.bits >> 7) & 0x007f_ffff) as u32)
    }
    #[doc = "Bits 0:6 - 6:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&mut self) -> RESERVED30_W {
        RESERVED30_W { w: self }
    }
    #[doc = "Bits 7:29 - 29:7\\]
Bits 29 down to 7 of the vector table base offset."]
    #[inline(always)]
    pub fn tbloff(&mut self) -> TBLOFF_W {
        TBLOFF_W { w: self }
    }
    #[doc = "Bits 0:6 - 6:0\\]
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
#[doc = "Vector Table Offset This register is used to relocated the vector table base address. The vector table base offset determines the offset from the bottom of the memory map. The two most significant bits and the seven least significant bits of the vector table base offset must be 0. The portion of vector table base offset that is allowed to change is TBLOFF.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vtor](index.html) module"]
pub struct VTOR_SPEC;
impl crate::RegisterSpec for VTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vtor::R](R) reader structure"]
impl crate::Readable for VTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vtor::W](W) writer structure"]
impl crate::Writable for VTOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VTOR to value 0"]
impl crate::Resettable for VTOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
