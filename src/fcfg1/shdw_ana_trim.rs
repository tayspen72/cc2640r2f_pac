#[doc = "Register `SHDW_ANA_TRIM` reader"]
pub struct R(crate::R<SHDW_ANA_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHDW_ANA_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHDW_ANA_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHDW_ANA_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHDW_ANA_TRIM` writer"]
pub struct W(crate::W<SHDW_ANA_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHDW_ANA_TRIM_SPEC>;
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
impl From<crate::W<SHDW_ANA_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHDW_ANA_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOD_BANDGAP_TRIM_CNF` reader - 26:25\\]
Internal. Only to be used through TI provided API."]
pub struct BOD_BANDGAP_TRIM_CNF_R(crate::FieldReader<u8, u8>);
impl BOD_BANDGAP_TRIM_CNF_R {
    pub(crate) fn new(bits: u8) -> Self {
        BOD_BANDGAP_TRIM_CNF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD_BANDGAP_TRIM_CNF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD_BANDGAP_TRIM_CNF` writer - 26:25\\]
Internal. Only to be used through TI provided API."]
pub struct BOD_BANDGAP_TRIM_CNF_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD_BANDGAP_TRIM_CNF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Field `VDDR_ENABLE_PG1` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub struct VDDR_ENABLE_PG1_R(crate::FieldReader<bool, bool>);
impl VDDR_ENABLE_PG1_R {
    pub(crate) fn new(bits: bool) -> Self {
        VDDR_ENABLE_PG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDR_ENABLE_PG1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDR_ENABLE_PG1` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub struct VDDR_ENABLE_PG1_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_ENABLE_PG1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `VDDR_OK_HYS` reader - 23:23\\]
Internal. Only to be used through TI provided API."]
pub struct VDDR_OK_HYS_R(crate::FieldReader<bool, bool>);
impl VDDR_OK_HYS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VDDR_OK_HYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDR_OK_HYS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDR_OK_HYS` writer - 23:23\\]
Internal. Only to be used through TI provided API."]
pub struct VDDR_OK_HYS_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_OK_HYS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `IPTAT_TRIM` reader - 22:21\\]
Internal. Only to be used through TI provided API."]
pub struct IPTAT_TRIM_R(crate::FieldReader<u8, u8>);
impl IPTAT_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        IPTAT_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPTAT_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPTAT_TRIM` writer - 22:21\\]
Internal. Only to be used through TI provided API."]
pub struct IPTAT_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> IPTAT_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `VDDR_TRIM` reader - 20:16\\]
Internal. Only to be used through TI provided API."]
pub struct VDDR_TRIM_R(crate::FieldReader<u8, u8>);
impl VDDR_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDDR_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDR_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDR_TRIM` writer - 20:16\\]
Internal. Only to be used through TI provided API."]
pub struct VDDR_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `TRIMBOD_INTMODE` reader - 15:11\\]
Internal. Only to be used through TI provided API."]
pub struct TRIMBOD_INTMODE_R(crate::FieldReader<u8, u8>);
impl TRIMBOD_INTMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIMBOD_INTMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIMBOD_INTMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIMBOD_INTMODE` writer - 15:11\\]
Internal. Only to be used through TI provided API."]
pub struct TRIMBOD_INTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMBOD_INTMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Field `TRIMBOD_EXTMODE` reader - 10:6\\]
Internal. Only to be used through TI provided API."]
pub struct TRIMBOD_EXTMODE_R(crate::FieldReader<u8, u8>);
impl TRIMBOD_EXTMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIMBOD_EXTMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIMBOD_EXTMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIMBOD_EXTMODE` writer - 10:6\\]
Internal. Only to be used through TI provided API."]
pub struct TRIMBOD_EXTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMBOD_EXTMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | ((value as u32 & 0x1f) << 6);
        self.w
    }
}
#[doc = "Field `TRIMTEMP` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub struct TRIMTEMP_R(crate::FieldReader<u8, u8>);
impl TRIMTEMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIMTEMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIMTEMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIMTEMP` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub struct TRIMTEMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMTEMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:26 - 26:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bod_bandgap_trim_cnf(&self) -> BOD_BANDGAP_TRIM_CNF_R {
        BOD_BANDGAP_TRIM_CNF_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_enable_pg1(&self) -> VDDR_ENABLE_PG1_R {
        VDDR_ENABLE_PG1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_ok_hys(&self) -> VDDR_OK_HYS_R {
        VDDR_OK_HYS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iptat_trim(&self) -> IPTAT_TRIM_R {
        IPTAT_TRIM_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim(&self) -> VDDR_TRIM_R {
        VDDR_TRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_intmode(&self) -> TRIMBOD_INTMODE_R {
        TRIMBOD_INTMODE_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - 10:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_extmode(&self) -> TRIMBOD_EXTMODE_R {
        TRIMBOD_EXTMODE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimtemp(&self) -> TRIMTEMP_R {
        TRIMTEMP_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:26 - 26:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bod_bandgap_trim_cnf(&mut self) -> BOD_BANDGAP_TRIM_CNF_W {
        BOD_BANDGAP_TRIM_CNF_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_enable_pg1(&mut self) -> VDDR_ENABLE_PG1_W {
        VDDR_ENABLE_PG1_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_ok_hys(&mut self) -> VDDR_OK_HYS_W {
        VDDR_OK_HYS_W { w: self }
    }
    #[doc = "Bits 21:22 - 22:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iptat_trim(&mut self) -> IPTAT_TRIM_W {
        IPTAT_TRIM_W { w: self }
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim(&mut self) -> VDDR_TRIM_W {
        VDDR_TRIM_W { w: self }
    }
    #[doc = "Bits 11:15 - 15:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_intmode(&mut self) -> TRIMBOD_INTMODE_W {
        TRIMBOD_INTMODE_W { w: self }
    }
    #[doc = "Bits 6:10 - 10:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_extmode(&mut self) -> TRIMBOD_EXTMODE_W {
        TRIMBOD_EXTMODE_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimtemp(&mut self) -> TRIMTEMP_W {
        TRIMTEMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_ana_trim](index.html) module"]
pub struct SHDW_ANA_TRIM_SPEC;
impl crate::RegisterSpec for SHDW_ANA_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shdw_ana_trim::R](R) reader structure"]
impl crate::Readable for SHDW_ANA_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shdw_ana_trim::W](W) writer structure"]
impl crate::Writable for SHDW_ANA_TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHDW_ANA_TRIM to value 0"]
impl crate::Resettable for SHDW_ANA_TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
