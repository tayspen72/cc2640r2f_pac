#[doc = "Register `RFCSEL5` reader"]
pub struct R(crate::R<RFCSEL5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCSEL5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFCSEL5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFCSEL5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFCSEL5` writer"]
pub struct W(crate::W<RFCSEL5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCSEL5_SPEC>;
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
impl From<crate::W<RFCSEL5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFCSEL5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 66"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "66: GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    GPT2B_CMP = 66,
}
impl From<EV_A> for u8 {
    #[inline(always)]
    fn from(variant: EV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EV` reader - 6:0\\]
Read only selection value"]
pub struct EV_R(crate::FieldReader<u8, EV_A>);
impl EV_R {
    pub(crate) fn new(bits: u8) -> Self {
        EV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EV_A> {
        match self.bits {
            66 => Some(EV_A::GPT2B_CMP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPT2B_CMP`"]
    #[inline(always)]
    pub fn is_gpt2b_cmp(&self) -> bool {
        **self == EV_A::GPT2B_CMP
    }
}
impl core::ops::Deref for EV_R {
    type Target = crate::FieldReader<u8, EV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EV` writer - 6:0\\]
Read only selection value"]
pub struct EV_W<'a> {
    w: &'a mut W,
}
impl<'a> EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt2b_cmp(self) -> &'a mut W {
        self.variant(EV_A::GPT2B_CMP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn ev(&mut self) -> EV_W {
        EV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Selection for RFC Event 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcsel5](index.html) module"]
pub struct RFCSEL5_SPEC;
impl crate::RegisterSpec for RFCSEL5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfcsel5::R](R) reader structure"]
impl crate::Readable for RFCSEL5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfcsel5::W](W) writer structure"]
impl crate::Writable for RFCSEL5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFCSEL5 to value 0x42"]
impl crate::Resettable for RFCSEL5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x42
    }
}
