#[doc = "Register `REQDONE` reader"]
pub struct R(crate::R<REQDONE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REQDONE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REQDONE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REQDONE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REQDONE` writer"]
pub struct W(crate::W<REQDONE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REQDONE_SPEC>;
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
impl From<crate::W<REQDONE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REQDONE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHNLS` reader - 31:0\\]
Reflects the uDMA done status for the given channel, channel \\[Ch\\]. It's a sticky done bit. Unless cleared by writing a 1, it holds the value of 1. Read as: Bit \\[Ch\\]
= 0: Request has not completed for channel Ch Bit \\[Ch\\]
= 1: Request has completed for the channel Ch Writing a 1 to individual bits would clear the corresponding bit. Write as: Bit \\[Ch\\]
= 0: No effect. Bit \\[Ch\\]
= 1: The corresponding \\[Ch\\]
bit is cleared and is set to 0"]
pub struct CHNLS_R(crate::FieldReader<u32, u32>);
impl CHNLS_R {
    pub(crate) fn new(bits: u32) -> Self {
        CHNLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHNLS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHNLS` writer - 31:0\\]
Reflects the uDMA done status for the given channel, channel \\[Ch\\]. It's a sticky done bit. Unless cleared by writing a 1, it holds the value of 1. Read as: Bit \\[Ch\\]
= 0: Request has not completed for channel Ch Bit \\[Ch\\]
= 1: Request has completed for the channel Ch Writing a 1 to individual bits would clear the corresponding bit. Write as: Bit \\[Ch\\]
= 0: No effect. Bit \\[Ch\\]
= 1: The corresponding \\[Ch\\]
bit is cleared and is set to 0"]
pub struct CHNLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reflects the uDMA done status for the given channel, channel \\[Ch\\]. It's a sticky done bit. Unless cleared by writing a 1, it holds the value of 1. Read as: Bit \\[Ch\\]
= 0: Request has not completed for channel Ch Bit \\[Ch\\]
= 1: Request has completed for the channel Ch Writing a 1 to individual bits would clear the corresponding bit. Write as: Bit \\[Ch\\]
= 0: No effect. Bit \\[Ch\\]
= 1: The corresponding \\[Ch\\]
bit is cleared and is set to 0"]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reflects the uDMA done status for the given channel, channel \\[Ch\\]. It's a sticky done bit. Unless cleared by writing a 1, it holds the value of 1. Read as: Bit \\[Ch\\]
= 0: Request has not completed for channel Ch Bit \\[Ch\\]
= 1: Request has completed for the channel Ch Writing a 1 to individual bits would clear the corresponding bit. Write as: Bit \\[Ch\\]
= 0: No effect. Bit \\[Ch\\]
= 1: The corresponding \\[Ch\\]
bit is cleared and is set to 0"]
    #[inline(always)]
    pub fn chnls(&mut self) -> CHNLS_W {
        CHNLS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Request Done\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqdone](index.html) module"]
pub struct REQDONE_SPEC;
impl crate::RegisterSpec for REQDONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reqdone::R](R) reader structure"]
impl crate::Readable for REQDONE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reqdone::W](W) writer structure"]
impl crate::Writable for REQDONE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REQDONE to value 0"]
impl crate::Resettable for REQDONE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}