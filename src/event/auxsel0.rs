#[doc = "Register `AUXSEL0` reader"]
pub struct R(crate::R<AUXSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXSEL0` writer"]
pub struct W(crate::W<AUXSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXSEL0_SPEC>;
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
impl From<crate::W<AUXSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "121: Always asserted"]
    ALWAYS_ACTIVE = 121,
    #[doc = "19: GPT1B interrupt event, controlled by GPT1:TBMR"]
    GPT1B = 19,
    #[doc = "18: GPT1A interrupt event, controlled by GPT1:TAMR"]
    GPT1A = 18,
    #[doc = "17: GPT0B interrupt event, controlled by GPT0:TBMR"]
    GPT0B = 17,
    #[doc = "16: GPT0A interrupt event, controlled by GPT0:TAMR"]
    GPT0A = 16,
    #[doc = "15: GPT3B interrupt event, controlled by GPT3:TBMR"]
    GPT3B = 15,
    #[doc = "14: GPT3A interrupt event, controlled by GPT3:TAMR"]
    GPT3A = 14,
    #[doc = "13: GPT2B interrupt event, controlled by GPT2:TBMR"]
    GPT2B = 13,
    #[doc = "12: GPT2A interrupt event, controlled by GPT2:TAMR"]
    GPT2A = 12,
    #[doc = "0: Always inactive"]
    NONE = 0,
}
impl From<EV_A> for u8 {
    #[inline(always)]
    fn from(variant: EV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EV` reader - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub struct EV_R(crate::FieldReader<u8, EV_A>);
impl EV_R {
    pub(crate) fn new(bits: u8) -> Self {
        EV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EV_A> {
        match self.bits {
            121 => Some(EV_A::ALWAYS_ACTIVE),
            19 => Some(EV_A::GPT1B),
            18 => Some(EV_A::GPT1A),
            17 => Some(EV_A::GPT0B),
            16 => Some(EV_A::GPT0A),
            15 => Some(EV_A::GPT3B),
            14 => Some(EV_A::GPT3A),
            13 => Some(EV_A::GPT2B),
            12 => Some(EV_A::GPT2A),
            0 => Some(EV_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ACTIVE`"]
    #[inline(always)]
    pub fn is_always_active(&self) -> bool {
        **self == EV_A::ALWAYS_ACTIVE
    }
    #[doc = "Checks if the value of the field is `GPT1B`"]
    #[inline(always)]
    pub fn is_gpt1b(&self) -> bool {
        **self == EV_A::GPT1B
    }
    #[doc = "Checks if the value of the field is `GPT1A`"]
    #[inline(always)]
    pub fn is_gpt1a(&self) -> bool {
        **self == EV_A::GPT1A
    }
    #[doc = "Checks if the value of the field is `GPT0B`"]
    #[inline(always)]
    pub fn is_gpt0b(&self) -> bool {
        **self == EV_A::GPT0B
    }
    #[doc = "Checks if the value of the field is `GPT0A`"]
    #[inline(always)]
    pub fn is_gpt0a(&self) -> bool {
        **self == EV_A::GPT0A
    }
    #[doc = "Checks if the value of the field is `GPT3B`"]
    #[inline(always)]
    pub fn is_gpt3b(&self) -> bool {
        **self == EV_A::GPT3B
    }
    #[doc = "Checks if the value of the field is `GPT3A`"]
    #[inline(always)]
    pub fn is_gpt3a(&self) -> bool {
        **self == EV_A::GPT3A
    }
    #[doc = "Checks if the value of the field is `GPT2B`"]
    #[inline(always)]
    pub fn is_gpt2b(&self) -> bool {
        **self == EV_A::GPT2B
    }
    #[doc = "Checks if the value of the field is `GPT2A`"]
    #[inline(always)]
    pub fn is_gpt2a(&self) -> bool {
        **self == EV_A::GPT2A
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == EV_A::NONE
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
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub struct EV_W<'a> {
    w: &'a mut W,
}
impl<'a> EV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Always asserted"]
    #[inline(always)]
    pub fn always_active(self) -> &'a mut W {
        self.variant(EV_A::ALWAYS_ACTIVE)
    }
    #[doc = "GPT1B interrupt event, controlled by GPT1:TBMR"]
    #[inline(always)]
    pub fn gpt1b(self) -> &'a mut W {
        self.variant(EV_A::GPT1B)
    }
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR"]
    #[inline(always)]
    pub fn gpt1a(self) -> &'a mut W {
        self.variant(EV_A::GPT1A)
    }
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR"]
    #[inline(always)]
    pub fn gpt0b(self) -> &'a mut W {
        self.variant(EV_A::GPT0B)
    }
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR"]
    #[inline(always)]
    pub fn gpt0a(self) -> &'a mut W {
        self.variant(EV_A::GPT0A)
    }
    #[doc = "GPT3B interrupt event, controlled by GPT3:TBMR"]
    #[inline(always)]
    pub fn gpt3b(self) -> &'a mut W {
        self.variant(EV_A::GPT3B)
    }
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR"]
    #[inline(always)]
    pub fn gpt3a(self) -> &'a mut W {
        self.variant(EV_A::GPT3A)
    }
    #[doc = "GPT2B interrupt event, controlled by GPT2:TBMR"]
    #[inline(always)]
    pub fn gpt2b(self) -> &'a mut W {
        self.variant(EV_A::GPT2B)
    }
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR"]
    #[inline(always)]
    pub fn gpt2a(self) -> &'a mut W {
        self.variant(EV_A::GPT2A)
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(EV_A::NONE)
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
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
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
#[doc = "Output Selection for AUX Subscriber 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxsel0](index.html) module"]
pub struct AUXSEL0_SPEC;
impl crate::RegisterSpec for AUXSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auxsel0::R](R) reader structure"]
impl crate::Readable for AUXSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxsel0::W](W) writer structure"]
impl crate::Writable for AUXSEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUXSEL0 to value 0x10"]
impl crate::Resettable for AUXSEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
