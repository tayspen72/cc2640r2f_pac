#[doc = "Register `AUXIOLATCH` reader"]
pub struct R(crate::R<AUXIOLATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXIOLATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXIOLATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXIOLATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXIOLATCH` writer"]
pub struct W(crate::W<AUXIOLATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXIOLATCH_SPEC>;
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
impl From<crate::W<AUXIOLATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXIOLATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "0:0\\]
Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "1: Latches are transparent ( open )"]
    TRANSP = 1,
    #[doc = "0: Latches are static ( closed )"]
    STATIC = 0,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - 0:0\\]
Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins."]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            true => EN_A::TRANSP,
            false => EN_A::STATIC,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSP`"]
    #[inline(always)]
    pub fn is_transp(&self) -> bool {
        **self == EN_A::TRANSP
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        **self == EN_A::STATIC
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - 0:0\\]
Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins."]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Latches are transparent ( open )"]
    #[inline(always)]
    pub fn transp(self) -> &'a mut W {
        self.variant(EN_A::TRANSP)
    }
    #[doc = "Latches are static ( closed )"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(EN_A::STATIC)
    }
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
    #[doc = "Bit 0 - 0:0\\]
Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUX Input Output Latch Controls latching of signals between AUX_AIODIO0/AUX_AIODIO1 and AON_IOC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxiolatch](index.html) module"]
pub struct AUXIOLATCH_SPEC;
impl crate::RegisterSpec for AUXIOLATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auxiolatch::R](R) reader structure"]
impl crate::Readable for AUXIOLATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxiolatch::W](W) writer structure"]
impl crate::Writable for AUXIOLATCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUXIOLATCH to value 0"]
impl crate::Resettable for AUXIOLATCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
