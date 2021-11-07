#[doc = "Register `CPUIRQSEL8` reader"]
pub struct R(crate::R<CPUIRQSEL8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUIRQSEL8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUIRQSEL8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUIRQSEL8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUIRQSEL8` writer"]
pub struct W(crate::W<CPUIRQSEL8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUIRQSEL8_SPEC>;
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
impl From<crate::W<CPUIRQSEL8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUIRQSEL8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 35"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "35: SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    SSI1_COMB = 35,
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
            35 => Some(EV_A::SSI1_COMB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SSI1_COMB`"]
    #[inline(always)]
    pub fn is_ssi1_comb(&self) -> bool {
        **self == EV_A::SSI1_COMB
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
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    #[inline(always)]
    pub fn ssi1_comb(self) -> &'a mut W {
        self.variant(EV_A::SSI1_COMB)
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
#[doc = "Output Selection for CPU Interrupt 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel8](index.html) module"]
pub struct CPUIRQSEL8_SPEC;
impl crate::RegisterSpec for CPUIRQSEL8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpuirqsel8::R](R) reader structure"]
impl crate::Readable for CPUIRQSEL8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpuirqsel8::W](W) writer structure"]
impl crate::Writable for CPUIRQSEL8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPUIRQSEL8 to value 0x23"]
impl crate::Resettable for CPUIRQSEL8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x23
    }
}
