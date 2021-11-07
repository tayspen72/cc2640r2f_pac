#[doc = "Register `MUX2` reader"]
pub struct R(crate::R<MUX2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUX2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUX2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUX2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUX2` writer"]
pub struct W(crate::W<MUX2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUX2_SPEC>;
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
impl From<crate::W<MUX2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUX2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "7:3\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCCOMPB_IN_A {
    #[doc = "16: Internal. Only to be used through TI provided API."]
    VDDS = 16,
    #[doc = "8: Internal. Only to be used through TI provided API."]
    VSS = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    DCOUPL = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    ATEST1 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    ATEST0 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    NC = 0,
}
impl From<ADCCOMPB_IN_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCCOMPB_IN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADCCOMPB_IN` reader - 7:3\\]
Internal. Only to be used through TI provided API."]
pub struct ADCCOMPB_IN_R(crate::FieldReader<u8, ADCCOMPB_IN_A>);
impl ADCCOMPB_IN_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCCOMPB_IN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCCOMPB_IN_A> {
        match self.bits {
            16 => Some(ADCCOMPB_IN_A::VDDS),
            8 => Some(ADCCOMPB_IN_A::VSS),
            4 => Some(ADCCOMPB_IN_A::DCOUPL),
            2 => Some(ADCCOMPB_IN_A::ATEST1),
            1 => Some(ADCCOMPB_IN_A::ATEST0),
            0 => Some(ADCCOMPB_IN_A::NC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VDDS`"]
    #[inline(always)]
    pub fn is_vdds(&self) -> bool {
        **self == ADCCOMPB_IN_A::VDDS
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        **self == ADCCOMPB_IN_A::VSS
    }
    #[doc = "Checks if the value of the field is `DCOUPL`"]
    #[inline(always)]
    pub fn is_dcoupl(&self) -> bool {
        **self == ADCCOMPB_IN_A::DCOUPL
    }
    #[doc = "Checks if the value of the field is `ATEST1`"]
    #[inline(always)]
    pub fn is_atest1(&self) -> bool {
        **self == ADCCOMPB_IN_A::ATEST1
    }
    #[doc = "Checks if the value of the field is `ATEST0`"]
    #[inline(always)]
    pub fn is_atest0(&self) -> bool {
        **self == ADCCOMPB_IN_A::ATEST0
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        **self == ADCCOMPB_IN_A::NC
    }
}
impl core::ops::Deref for ADCCOMPB_IN_R {
    type Target = crate::FieldReader<u8, ADCCOMPB_IN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCCOMPB_IN` writer - 7:3\\]
Internal. Only to be used through TI provided API."]
pub struct ADCCOMPB_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCCOMPB_IN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCCOMPB_IN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vdds(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::VDDS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::VSS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcoupl(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::DCOUPL)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atest1(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::ATEST1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atest0(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::ATEST0)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::NC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u8 & 0x1f) << 3);
        self.w
    }
}
#[doc = "2:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMPB_REF_A {
    #[doc = "4: Internal. Only to be used through TI provided API."]
    VDDS = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    VSS = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    DCOUPL = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    NC = 0,
}
impl From<COMPB_REF_A> for u8 {
    #[inline(always)]
    fn from(variant: COMPB_REF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMPB_REF` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub struct COMPB_REF_R(crate::FieldReader<u8, COMPB_REF_A>);
impl COMPB_REF_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMPB_REF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMPB_REF_A> {
        match self.bits {
            4 => Some(COMPB_REF_A::VDDS),
            2 => Some(COMPB_REF_A::VSS),
            1 => Some(COMPB_REF_A::DCOUPL),
            0 => Some(COMPB_REF_A::NC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VDDS`"]
    #[inline(always)]
    pub fn is_vdds(&self) -> bool {
        **self == COMPB_REF_A::VDDS
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        **self == COMPB_REF_A::VSS
    }
    #[doc = "Checks if the value of the field is `DCOUPL`"]
    #[inline(always)]
    pub fn is_dcoupl(&self) -> bool {
        **self == COMPB_REF_A::DCOUPL
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        **self == COMPB_REF_A::NC
    }
}
impl core::ops::Deref for COMPB_REF_R {
    type Target = crate::FieldReader<u8, COMPB_REF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPB_REF` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub struct COMPB_REF_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPB_REF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPB_REF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vdds(self) -> &'a mut W {
        self.variant(COMPB_REF_A::VDDS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(COMPB_REF_A::VSS)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dcoupl(self) -> &'a mut W {
        self.variant(COMPB_REF_A::DCOUPL)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(COMPB_REF_A::NC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adccompb_in(&self) -> ADCCOMPB_IN_R {
        ADCCOMPB_IN_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn compb_ref(&self) -> COMPB_REF_R {
        COMPB_REF_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adccompb_in(&mut self) -> ADCCOMPB_IN_W {
        ADCCOMPB_IN_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn compb_ref(&mut self) -> COMPB_REF_W {
        COMPB_REF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux2](index.html) module"]
pub struct MUX2_SPEC;
impl crate::RegisterSpec for MUX2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mux2::R](R) reader structure"]
impl crate::Readable for MUX2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mux2::W](W) writer structure"]
impl crate::Writable for MUX2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MUX2 to value 0"]
impl crate::Resettable for MUX2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
