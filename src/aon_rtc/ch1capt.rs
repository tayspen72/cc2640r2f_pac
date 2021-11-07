#[doc = "Register `CH1CAPT` reader"]
pub struct R(crate::R<CH1CAPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1CAPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1CAPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1CAPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1CAPT` writer"]
pub struct W(crate::W<CH1CAPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1CAPT_SPEC>;
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
impl From<crate::W<CH1CAPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1CAPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC` reader - 31:16\\]
Value of SEC.VALUE bits 15:0 at capture time."]
pub struct SEC_R(crate::FieldReader<u16, u16>);
impl SEC_R {
    pub(crate) fn new(bits: u16) -> Self {
        SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC` writer - 31:16\\]
Value of SEC.VALUE bits 15:0 at capture time."]
pub struct SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `SUBSEC` reader - 15:0\\]
Value of SUBSEC.VALUE bits 31:16 at capture time."]
pub struct SUBSEC_R(crate::FieldReader<u16, u16>);
impl SUBSEC_R {
    pub(crate) fn new(bits: u16) -> Self {
        SUBSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUBSEC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUBSEC` writer - 15:0\\]
Value of SUBSEC.VALUE bits 31:16 at capture time."]
pub struct SUBSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBSEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Value of SEC.VALUE bits 15:0 at capture time."]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Value of SUBSEC.VALUE bits 31:16 at capture time."]
    #[inline(always)]
    pub fn subsec(&self) -> SUBSEC_R {
        SUBSEC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Value of SEC.VALUE bits 15:0 at capture time."]
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W {
        SEC_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Value of SUBSEC.VALUE bits 31:16 at capture time."]
    #[inline(always)]
    pub fn subsec(&mut self) -> SUBSEC_W {
        SUBSEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 1 Capture Value If CHCTL.CH1_EN = 1and CHCTL.CH1_CAPT_EN = 1, capture occurs on each rising edge of the event selected in AON_EVENT:RTCSEL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1capt](index.html) module"]
pub struct CH1CAPT_SPEC;
impl crate::RegisterSpec for CH1CAPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1capt::R](R) reader structure"]
impl crate::Readable for CH1CAPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1capt::W](W) writer structure"]
impl crate::Writable for CH1CAPT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH1CAPT to value 0"]
impl crate::Resettable for CH1CAPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
