#[doc = "Register `DMACH1EXTADDR` reader"]
pub struct R(crate::R<DMACH1EXTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACH1EXTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACH1EXTADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACH1EXTADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACH1EXTADDR` writer"]
pub struct W(crate::W<DMACH1EXTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACH1EXTADDR_SPEC>;
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
impl From<crate::W<DMACH1EXTADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACH1EXTADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - 31:0\\]
Channel external address value. Holds the last updated external address after being sent to the master interface."]
pub struct ADDR_R(crate::FieldReader<u32, u32>);
impl ADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - 31:0\\]
Channel external address value. Holds the last updated external address after being sent to the master interface."]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel external address value. Holds the last updated external address after being sent to the master interface."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel external address value. Holds the last updated external address after being sent to the master interface."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel 1 External Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach1extaddr](index.html) module"]
pub struct DMACH1EXTADDR_SPEC;
impl crate::RegisterSpec for DMACH1EXTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmach1extaddr::R](R) reader structure"]
impl crate::Readable for DMACH1EXTADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmach1extaddr::W](W) writer structure"]
impl crate::Writable for DMACH1EXTADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACH1EXTADDR to value 0"]
impl crate::Resettable for DMACH1EXTADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
