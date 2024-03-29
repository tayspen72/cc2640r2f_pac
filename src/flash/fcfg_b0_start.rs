#[doc = "Register `FCFG_B0_START` reader"]
pub struct R(crate::R<FCFG_B0_START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFG_B0_START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFG_B0_START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFG_B0_START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFG_B0_START` writer"]
pub struct W(crate::W<FCFG_B0_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFG_B0_START_SPEC>;
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
impl From<crate::W<FCFG_B0_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFG_B0_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B0_MAX_SECTOR` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub struct B0_MAX_SECTOR_R(crate::FieldReader<u8, u8>);
impl B0_MAX_SECTOR_R {
    pub(crate) fn new(bits: u8) -> Self {
        B0_MAX_SECTOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B0_MAX_SECTOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B0_MAX_SECTOR` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub struct B0_MAX_SECTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> B0_MAX_SECTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `B0_MUX_FACTOR` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub struct B0_MUX_FACTOR_R(crate::FieldReader<u8, u8>);
impl B0_MUX_FACTOR_R {
    pub(crate) fn new(bits: u8) -> Self {
        B0_MUX_FACTOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B0_MUX_FACTOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B0_MUX_FACTOR` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub struct B0_MUX_FACTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> B0_MUX_FACTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `B0_START_ADDR` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub struct B0_START_ADDR_R(crate::FieldReader<u32, u32>);
impl B0_START_ADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        B0_START_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B0_START_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B0_START_ADDR` writer - 23:0\\]
Internal. Only to be used through TI provided API."]
pub struct B0_START_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> B0_START_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_max_sector(&self) -> B0_MAX_SECTOR_R {
        B0_MAX_SECTOR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_mux_factor(&self) -> B0_MUX_FACTOR_R {
        B0_MUX_FACTOR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_start_addr(&self) -> B0_START_ADDR_R {
        B0_START_ADDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_max_sector(&mut self) -> B0_MAX_SECTOR_W {
        B0_MAX_SECTOR_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_mux_factor(&mut self) -> B0_MUX_FACTOR_W {
        B0_MUX_FACTOR_W { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_start_addr(&mut self) -> B0_START_ADDR_W {
        B0_START_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_b0_start](index.html) module"]
pub struct FCFG_B0_START_SPEC;
impl crate::RegisterSpec for FCFG_B0_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfg_b0_start::R](R) reader structure"]
impl crate::Readable for FCFG_B0_START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcfg_b0_start::W](W) writer structure"]
impl crate::Writable for FCFG_B0_START_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCFG_B0_START to value 0x0200_0000"]
impl crate::Resettable for FCFG_B0_START_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0000
    }
}
