#[doc = "Register `ID_PFR0` reader"]
pub struct R(crate::R<ID_PFR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_PFR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_PFR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_PFR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ID_PFR0` writer"]
pub struct W(crate::W<ID_PFR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ID_PFR0_SPEC>;
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
impl From<crate::W<ID_PFR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ID_PFR0_SPEC>) -> Self {
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
#[doc = "Field `STATE1` reader - 7:4\\]
State1 (T-bit == 1) 0x0: N/A 0x1: N/A 0x2: Thumb-2 encoding with the 16-bit basic instructions plus 32-bit Buncond/BL but no other 32-bit basic instructions (Note non-basic 32-bit instructions can be added using the appropriate instruction attribute, but other 32-bit basic instructions cannot.) 0x3: Thumb-2 encoding with all Thumb-2 basic instructions"]
pub struct STATE1_R(crate::FieldReader<u8, u8>);
impl STATE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        STATE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE1` writer - 7:4\\]
State1 (T-bit == 1) 0x0: N/A 0x1: N/A 0x2: Thumb-2 encoding with the 16-bit basic instructions plus 32-bit Buncond/BL but no other 32-bit basic instructions (Note non-basic 32-bit instructions can be added using the appropriate instruction attribute, but other 32-bit basic instructions cannot.) 0x3: Thumb-2 encoding with all Thumb-2 basic instructions"]
pub struct STATE1_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `STATE0` reader - 3:0\\]
State0 (T-bit == 0) 0x0: No ARM encoding 0x1: N/A"]
pub struct STATE0_R(crate::FieldReader<u8, u8>);
impl STATE0_R {
    pub(crate) fn new(bits: u8) -> Self {
        STATE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE0` writer - 3:0\\]
State0 (T-bit == 0) 0x0: No ARM encoding 0x1: N/A"]
pub struct STATE0_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
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
    #[doc = "Bits 4:7 - 7:4\\]
State1 (T-bit == 1) 0x0: N/A 0x1: N/A 0x2: Thumb-2 encoding with the 16-bit basic instructions plus 32-bit Buncond/BL but no other 32-bit basic instructions (Note non-basic 32-bit instructions can be added using the appropriate instruction attribute, but other 32-bit basic instructions cannot.) 0x3: Thumb-2 encoding with all Thumb-2 basic instructions"]
    #[inline(always)]
    pub fn state1(&self) -> STATE1_R {
        STATE1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
State0 (T-bit == 0) 0x0: No ARM encoding 0x1: N/A"]
    #[inline(always)]
    pub fn state0(&self) -> STATE0_R {
        STATE0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
State1 (T-bit == 1) 0x0: N/A 0x1: N/A 0x2: Thumb-2 encoding with the 16-bit basic instructions plus 32-bit Buncond/BL but no other 32-bit basic instructions (Note non-basic 32-bit instructions can be added using the appropriate instruction attribute, but other 32-bit basic instructions cannot.) 0x3: Thumb-2 encoding with all Thumb-2 basic instructions"]
    #[inline(always)]
    pub fn state1(&mut self) -> STATE1_W {
        STATE1_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
State0 (T-bit == 0) 0x0: No ARM encoding 0x1: N/A"]
    #[inline(always)]
    pub fn state0(&mut self) -> STATE0_W {
        STATE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Processor Feature 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_pfr0](index.html) module"]
pub struct ID_PFR0_SPEC;
impl crate::RegisterSpec for ID_PFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_pfr0::R](R) reader structure"]
impl crate::Readable for ID_PFR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [id_pfr0::W](W) writer structure"]
impl crate::Writable for ID_PFR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ID_PFR0 to value 0x30"]
impl crate::Resettable for ID_PFR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}
