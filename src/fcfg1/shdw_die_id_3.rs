#[doc = "Register `SHDW_DIE_ID_3` reader"]
pub struct R(crate::R<SHDW_DIE_ID_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHDW_DIE_ID_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHDW_DIE_ID_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHDW_DIE_ID_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHDW_DIE_ID_3` writer"]
pub struct W(crate::W<SHDW_DIE_ID_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHDW_DIE_ID_3_SPEC>;
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
impl From<crate::W<SHDW_DIE_ID_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHDW_DIE_ID_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID_127_96` reader - 31:0\\]
Shadow of DIE_ID_3 register in eFuse row number 6"]
pub struct ID_127_96_R(crate::FieldReader<u32, u32>);
impl ID_127_96_R {
    pub(crate) fn new(bits: u32) -> Self {
        ID_127_96_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_127_96_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID_127_96` writer - 31:0\\]
Shadow of DIE_ID_3 register in eFuse row number 6"]
pub struct ID_127_96_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_127_96_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Shadow of DIE_ID_3 register in eFuse row number 6"]
    #[inline(always)]
    pub fn id_127_96(&self) -> ID_127_96_R {
        ID_127_96_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Shadow of DIE_ID_3 register in eFuse row number 6"]
    #[inline(always)]
    pub fn id_127_96(&mut self) -> ID_127_96_W {
        ID_127_96_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow of DIE_ID_3 register in eFuse\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_die_id_3](index.html) module"]
pub struct SHDW_DIE_ID_3_SPEC;
impl crate::RegisterSpec for SHDW_DIE_ID_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shdw_die_id_3::R](R) reader structure"]
impl crate::Readable for SHDW_DIE_ID_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shdw_die_id_3::W](W) writer structure"]
impl crate::Writable for SHDW_DIE_ID_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHDW_DIE_ID_3 to value 0"]
impl crate::Resettable for SHDW_DIE_ID_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
