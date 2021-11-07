#[doc = "Register `FLASH_C_E_P_R` reader"]
pub struct R(crate::R<FLASH_C_E_P_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_C_E_P_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_C_E_P_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_C_E_P_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_C_E_P_R` writer"]
pub struct W(crate::W<FLASH_C_E_P_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_C_E_P_R_SPEC>;
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
impl From<crate::W<FLASH_C_E_P_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_C_E_P_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RVSU` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct RVSU_R(crate::FieldReader<u8, u8>);
impl RVSU_R {
    pub(crate) fn new(bits: u8) -> Self {
        RVSU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RVSU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RVSU` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct RVSU_W<'a> {
    w: &'a mut W,
}
impl<'a> RVSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `PV_ACCESS` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub struct PV_ACCESS_R(crate::FieldReader<u8, u8>);
impl PV_ACCESS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PV_ACCESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PV_ACCESS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PV_ACCESS` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub struct PV_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> PV_ACCESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `A_EXEZ_SETUP` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct A_EXEZ_SETUP_R(crate::FieldReader<u8, u8>);
impl A_EXEZ_SETUP_R {
    pub(crate) fn new(bits: u8) -> Self {
        A_EXEZ_SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for A_EXEZ_SETUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `A_EXEZ_SETUP` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct A_EXEZ_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> A_EXEZ_SETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `CVSU` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub struct CVSU_R(crate::FieldReader<u16, u16>);
impl CVSU_R {
    pub(crate) fn new(bits: u16) -> Self {
        CVSU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CVSU_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CVSU` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub struct CVSU_W<'a> {
    w: &'a mut W,
}
impl<'a> CVSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvsu(&self) -> RVSU_R {
        RVSU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pv_access(&self) -> PV_ACCESS_R {
        PV_ACCESS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn a_exez_setup(&self) -> A_EXEZ_SETUP_R {
        A_EXEZ_SETUP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cvsu(&self) -> CVSU_R {
        CVSU_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvsu(&mut self) -> RVSU_W {
        RVSU_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pv_access(&mut self) -> PV_ACCESS_W {
        PV_ACCESS_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn a_exez_setup(&mut self) -> A_EXEZ_SETUP_W {
        A_EXEZ_SETUP_W { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cvsu(&mut self) -> CVSU_W {
        CVSU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_c_e_p_r](index.html) module"]
pub struct FLASH_C_E_P_R_SPEC;
impl crate::RegisterSpec for FLASH_C_E_P_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_c_e_p_r::R](R) reader structure"]
impl crate::Readable for FLASH_C_E_P_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_c_e_p_r::W](W) writer structure"]
impl crate::Writable for FLASH_C_E_P_R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_C_E_P_R to value 0x0a0a_2000"]
impl crate::Resettable for FLASH_C_E_P_R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a0a_2000
    }
}
