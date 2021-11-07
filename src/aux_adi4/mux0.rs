#[doc = "Register `MUX0` reader"]
pub struct R(crate::R<MUX0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUX0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUX0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUX0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUX0` writer"]
pub struct W(crate::W<MUX0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUX0_SPEC>;
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
impl From<crate::W<MUX0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUX0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED4` reader - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_R(crate::FieldReader<u8, u8>);
impl RESERVED4_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED4` writer - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u8 & 0x0f) << 4);
        self.w
    }
}
#[doc = "3:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMPA_REF_A {
    #[doc = "8: Internal. Only to be used through TI provided API."]
    ADCVREFP = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    VDDS = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    VSS = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    DCOUPL = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    NC = 0,
}
impl From<COMPA_REF_A> for u8 {
    #[inline(always)]
    fn from(variant: COMPA_REF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMPA_REF` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct COMPA_REF_R(crate::FieldReader<u8, COMPA_REF_A>);
impl COMPA_REF_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMPA_REF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMPA_REF_A> {
        match self.bits {
            8 => Some(COMPA_REF_A::ADCVREFP),
            4 => Some(COMPA_REF_A::VDDS),
            2 => Some(COMPA_REF_A::VSS),
            1 => Some(COMPA_REF_A::DCOUPL),
            0 => Some(COMPA_REF_A::NC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADCVREFP`"]
    #[inline(always)]
    pub fn is_adcvrefp(&self) -> bool {
        **self == COMPA_REF_A::ADCVREFP
    }
    #[doc = "Checks if the value of the field is `VDDS`"]
    #[inline(always)]
    pub fn is_vdds(&self) -> bool {
        **self == COMPA_REF_A::VDDS
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        **self == COMPA_REF_A::VSS
    }
    #[doc = "Checks if the value of the field is `DCOUPL`"]
    #[inline(always)]
    pub fn is_dcoupl(&self) -> bool {
        **self == COMPA_REF_A::DCOUPL
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        **self == COMPA_REF_A::NC
    }
}
impl core::ops::Deref for COMPA_REF_R {
    type Target = crate::FieldReader<u8, COMPA_REF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPA_REF` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct COMPA_REF_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPA_REF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPA_REF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adcvrefp(self) -> &'a mut W {
        self.variant(COMPA_REF_A::ADCVREFP)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vdds(self) -> &'a mut W {
        self.variant(COMPA_REF_A::VDDS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(COMPA_REF_A::VSS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcoupl(self) -> &'a mut W {
        self.variant(COMPA_REF_A::DCOUPL)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(COMPA_REF_A::NC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn compa_ref(&self) -> COMPA_REF_R {
        COMPA_REF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn compa_ref(&mut self) -> COMPA_REF_W {
        COMPA_REF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux0](index.html) module"]
pub struct MUX0_SPEC;
impl crate::RegisterSpec for MUX0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mux0::R](R) reader structure"]
impl crate::Readable for MUX0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mux0::W](W) writer structure"]
impl crate::Writable for MUX0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MUX0 to value 0"]
impl crate::Resettable for MUX0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}