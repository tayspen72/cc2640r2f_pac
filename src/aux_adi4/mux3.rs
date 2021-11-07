#[doc = "Register `MUX3` reader"]
pub struct R(crate::R<MUX3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUX3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUX3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUX3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUX3` writer"]
pub struct W(crate::W<MUX3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUX3_SPEC>;
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
impl From<crate::W<MUX3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUX3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "7:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCCOMPB_IN_A {
    #[doc = "128: Internal. Only to be used through TI provided API."]
    AUXIO0 = 128,
    #[doc = "64: Internal. Only to be used through TI provided API."]
    AUXIO1 = 64,
    #[doc = "32: Internal. Only to be used through TI provided API."]
    AUXIO2 = 32,
    #[doc = "16: Internal. Only to be used through TI provided API."]
    AUXIO3 = 16,
    #[doc = "8: Internal. Only to be used through TI provided API."]
    AUXIO4 = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    AUXIO5 = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    AUXIO6 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    AUXIO7 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    NC = 0,
}
impl From<ADCCOMPB_IN_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCCOMPB_IN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADCCOMPB_IN` reader - 7:0\\]
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
            128 => Some(ADCCOMPB_IN_A::AUXIO0),
            64 => Some(ADCCOMPB_IN_A::AUXIO1),
            32 => Some(ADCCOMPB_IN_A::AUXIO2),
            16 => Some(ADCCOMPB_IN_A::AUXIO3),
            8 => Some(ADCCOMPB_IN_A::AUXIO4),
            4 => Some(ADCCOMPB_IN_A::AUXIO5),
            2 => Some(ADCCOMPB_IN_A::AUXIO6),
            1 => Some(ADCCOMPB_IN_A::AUXIO7),
            0 => Some(ADCCOMPB_IN_A::NC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline(always)]
    pub fn is_auxio0(&self) -> bool {
        **self == ADCCOMPB_IN_A::AUXIO0
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline(always)]
    pub fn is_auxio1(&self) -> bool {
        **self == ADCCOMPB_IN_A::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline(always)]
    pub fn is_auxio2(&self) -> bool {
        **self == ADCCOMPB_IN_A::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline(always)]
    pub fn is_auxio3(&self) -> bool {
        **self == ADCCOMPB_IN_A::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline(always)]
    pub fn is_auxio4(&self) -> bool {
        **self == ADCCOMPB_IN_A::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline(always)]
    pub fn is_auxio5(&self) -> bool {
        **self == ADCCOMPB_IN_A::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline(always)]
    pub fn is_auxio6(&self) -> bool {
        **self == ADCCOMPB_IN_A::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline(always)]
    pub fn is_auxio7(&self) -> bool {
        **self == ADCCOMPB_IN_A::AUXIO7
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
#[doc = "Field `ADCCOMPB_IN` writer - 7:0\\]
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
    pub fn auxio0(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO0)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO3)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO4)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO5)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO6)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::AUXIO7)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(ADCCOMPB_IN_A::NC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adccompb_in(&self) -> ADCCOMPB_IN_R {
        ADCCOMPB_IN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adccompb_in(&mut self) -> ADCCOMPB_IN_W {
        ADCCOMPB_IN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux3](index.html) module"]
pub struct MUX3_SPEC;
impl crate::RegisterSpec for MUX3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mux3::R](R) reader structure"]
impl crate::Readable for MUX3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mux3::W](W) writer structure"]
impl crate::Writable for MUX3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MUX3 to value 0"]
impl crate::Resettable for MUX3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
