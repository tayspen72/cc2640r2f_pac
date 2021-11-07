#[doc = "Register `FLASH_NUMBER` reader"]
pub struct R(crate::R<FLASH_NUMBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_NUMBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_NUMBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_NUMBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_NUMBER` writer"]
pub struct W(crate::W<FLASH_NUMBER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_NUMBER_SPEC>;
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
impl From<crate::W<FLASH_NUMBER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_NUMBER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOT_NUMBER` reader - 31:0\\]
Number of the manufacturing lot that produced this unit."]
pub struct LOT_NUMBER_R(crate::FieldReader<u32, u32>);
impl LOT_NUMBER_R {
    pub(crate) fn new(bits: u32) -> Self {
        LOT_NUMBER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOT_NUMBER_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOT_NUMBER` writer - 31:0\\]
Number of the manufacturing lot that produced this unit."]
pub struct LOT_NUMBER_W<'a> {
    w: &'a mut W,
}
impl<'a> LOT_NUMBER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Number of the manufacturing lot that produced this unit."]
    #[inline(always)]
    pub fn lot_number(&self) -> LOT_NUMBER_R {
        LOT_NUMBER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Number of the manufacturing lot that produced this unit."]
    #[inline(always)]
    pub fn lot_number(&mut self) -> LOT_NUMBER_W {
        LOT_NUMBER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH_NUMBER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_number](index.html) module"]
pub struct FLASH_NUMBER_SPEC;
impl crate::RegisterSpec for FLASH_NUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_number::R](R) reader structure"]
impl crate::Readable for FLASH_NUMBER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_number::W](W) writer structure"]
impl crate::Writable for FLASH_NUMBER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_NUMBER to value 0"]
impl crate::Resettable for FLASH_NUMBER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
