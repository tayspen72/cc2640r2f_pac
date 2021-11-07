#[doc = "Register `FMC_REV_ID` reader"]
pub struct R(crate::R<FMC_REV_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_REV_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_REV_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_REV_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_REV_ID` writer"]
pub struct W(crate::W<FMC_REV_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_REV_ID_SPEC>;
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
impl From<crate::W<FMC_REV_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_REV_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOD_VERSION` reader - 31:12\\]
Internal. Only to be used through TI provided API."]
pub struct MOD_VERSION_R(crate::FieldReader<u32, u32>);
impl MOD_VERSION_R {
    pub(crate) fn new(bits: u32) -> Self {
        MOD_VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOD_VERSION_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOD_VERSION` writer - 31:12\\]
Internal. Only to be used through TI provided API."]
pub struct MOD_VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_VERSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | ((value as u32 & 0x000f_ffff) << 12);
        self.w
    }
}
#[doc = "Field `CONFIG_CRC` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub struct CONFIG_CRC_R(crate::FieldReader<u16, u16>);
impl CONFIG_CRC_R {
    pub(crate) fn new(bits: u16) -> Self {
        CONFIG_CRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONFIG_CRC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONFIG_CRC` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub struct CONFIG_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFIG_CRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mod_version(&self) -> MOD_VERSION_R {
        MOD_VERSION_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn config_crc(&self) -> CONFIG_CRC_R {
        CONFIG_CRC_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mod_version(&mut self) -> MOD_VERSION_W {
        MOD_VERSION_W { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn config_crc(&mut self) -> CONFIG_CRC_W {
        CONFIG_CRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_rev_id](index.html) module"]
pub struct FMC_REV_ID_SPEC;
impl crate::RegisterSpec for FMC_REV_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_rev_id::R](R) reader structure"]
impl crate::Readable for FMC_REV_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_rev_id::W](W) writer structure"]
impl crate::Writable for FMC_REV_ID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_REV_ID to value 0"]
impl crate::Resettable for FMC_REV_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
