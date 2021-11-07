#[doc = "Register `FCFG_WRAPPER` reader"]
pub struct R(crate::R<FCFG_WRAPPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFG_WRAPPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFG_WRAPPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFG_WRAPPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFG_WRAPPER` writer"]
pub struct W(crate::W<FCFG_WRAPPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFG_WRAPPER_SPEC>;
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
impl From<crate::W<FCFG_WRAPPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFG_WRAPPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAMILY_TYPE` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct FAMILY_TYPE_R(crate::FieldReader<u8, u8>);
impl FAMILY_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FAMILY_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAMILY_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAMILY_TYPE` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct FAMILY_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> FAMILY_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `RESERVED21` reader - 23:21\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED21_R(crate::FieldReader<u8, u8>);
impl RESERVED21_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED21_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED21` writer - 23:21\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED21_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `MEM_MAP` reader - 20:20\\]
Internal. Only to be used through TI provided API."]
pub struct MEM_MAP_R(crate::FieldReader<bool, bool>);
impl MEM_MAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        MEM_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_MAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_MAP` writer - 20:20\\]
Internal. Only to be used through TI provided API."]
pub struct MEM_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CPU2` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct CPU2_R(crate::FieldReader<u8, u8>);
impl CPU2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPU2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU2` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct CPU2_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `EE_IN_MAIN` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct EE_IN_MAIN_R(crate::FieldReader<u8, u8>);
impl EE_IN_MAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE_IN_MAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE_IN_MAIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE_IN_MAIN` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct EE_IN_MAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EE_IN_MAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `ROM` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub struct ROM_R(crate::FieldReader<bool, bool>);
impl ROM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub struct ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `IFLUSH` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub struct IFLUSH_R(crate::FieldReader<bool, bool>);
impl IFLUSH_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFLUSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFLUSH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFLUSH` writer - 10:10\\]
Internal. Only to be used through TI provided API."]
pub struct IFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> IFLUSH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SIL3` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub struct SIL3_R(crate::FieldReader<bool, bool>);
impl SIL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIL3` writer - 9:9\\]
Internal. Only to be used through TI provided API."]
pub struct SIL3_W<'a> {
    w: &'a mut W,
}
impl<'a> SIL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `ECCA` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct ECCA_R(crate::FieldReader<bool, bool>);
impl ECCA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCA` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct ECCA_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `AUTO_SUSP` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub struct AUTO_SUSP_R(crate::FieldReader<u8, u8>);
impl AUTO_SUSP_R {
    pub(crate) fn new(bits: u8) -> Self {
        AUTO_SUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTO_SUSP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTO_SUSP` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub struct AUTO_SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_SUSP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `UERR` reader - 5:4\\]
Internal. Only to be used through TI provided API."]
pub struct UERR_R(crate::FieldReader<u8, u8>);
impl UERR_R {
    pub(crate) fn new(bits: u8) -> Self {
        UERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UERR` writer - 5:4\\]
Internal. Only to be used through TI provided API."]
pub struct UERR_W<'a> {
    w: &'a mut W,
}
impl<'a> UERR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `CPU_TYPE1` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct CPU_TYPE1_R(crate::FieldReader<u8, u8>);
impl CPU_TYPE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPU_TYPE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TYPE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TYPE1` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct CPU_TYPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TYPE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn family_type(&self) -> FAMILY_TYPE_R {
        FAMILY_TYPE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved21(&self) -> RESERVED21_R {
        RESERVED21_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mem_map(&self) -> MEM_MAP_R {
        MEM_MAP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cpu2(&self) -> CPU2_R {
        CPU2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ee_in_main(&self) -> EE_IN_MAIN_R {
        EE_IN_MAIN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iflush(&self) -> IFLUSH_R {
        IFLUSH_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sil3(&self) -> SIL3_R {
        SIL3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ecca(&self) -> ECCA_R {
        ECCA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auto_susp(&self) -> AUTO_SUSP_R {
        AUTO_SUSP_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn uerr(&self) -> UERR_R {
        UERR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cpu_type1(&self) -> CPU_TYPE1_R {
        CPU_TYPE1_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn family_type(&mut self) -> FAMILY_TYPE_W {
        FAMILY_TYPE_W { w: self }
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved21(&mut self) -> RESERVED21_W {
        RESERVED21_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mem_map(&mut self) -> MEM_MAP_W {
        MEM_MAP_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cpu2(&mut self) -> CPU2_W {
        CPU2_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ee_in_main(&mut self) -> EE_IN_MAIN_W {
        EE_IN_MAIN_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W {
        ROM_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iflush(&mut self) -> IFLUSH_W {
        IFLUSH_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sil3(&mut self) -> SIL3_W {
        SIL3_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ecca(&mut self) -> ECCA_W {
        ECCA_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auto_susp(&mut self) -> AUTO_SUSP_W {
        AUTO_SUSP_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn uerr(&mut self) -> UERR_W {
        UERR_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cpu_type1(&mut self) -> CPU_TYPE1_W {
        CPU_TYPE1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_wrapper](index.html) module"]
pub struct FCFG_WRAPPER_SPEC;
impl crate::RegisterSpec for FCFG_WRAPPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfg_wrapper::R](R) reader structure"]
impl crate::Readable for FCFG_WRAPPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcfg_wrapper::W](W) writer structure"]
impl crate::Writable for FCFG_WRAPPER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCFG_WRAPPER to value 0x5000_9007"]
impl crate::Resettable for FCFG_WRAPPER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5000_9007
    }
}
