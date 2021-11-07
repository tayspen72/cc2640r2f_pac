#[doc = "Register `FSM_MODE` reader"]
pub struct R(crate::R<FSM_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_MODE` writer"]
pub struct W(crate::W<FSM_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_MODE_SPEC>;
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
impl From<crate::W<FSM_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED20` reader - 31:20\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED20_R(crate::FieldReader<u16, u16>);
impl RESERVED20_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED20_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED20` writer - 31:20\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED20_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | ((value as u32 & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Field `RDV_SUBMODE` reader - 19:18\\]
Internal. Only to be used through TI provided API."]
pub struct RDV_SUBMODE_R(crate::FieldReader<u8, u8>);
impl RDV_SUBMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDV_SUBMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDV_SUBMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDV_SUBMODE` writer - 19:18\\]
Internal. Only to be used through TI provided API."]
pub struct RDV_SUBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDV_SUBMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `PGM_SUBMODE` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub struct PGM_SUBMODE_R(crate::FieldReader<u8, u8>);
impl PGM_SUBMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PGM_SUBMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGM_SUBMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGM_SUBMODE` writer - 17:16\\]
Internal. Only to be used through TI provided API."]
pub struct PGM_SUBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_SUBMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `ERA_SUBMODE` reader - 15:14\\]
Internal. Only to be used through TI provided API."]
pub struct ERA_SUBMODE_R(crate::FieldReader<u8, u8>);
impl ERA_SUBMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ERA_SUBMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERA_SUBMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERA_SUBMODE` writer - 15:14\\]
Internal. Only to be used through TI provided API."]
pub struct ERA_SUBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERA_SUBMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `SUBMODE` reader - 13:12\\]
Internal. Only to be used through TI provided API."]
pub struct SUBMODE_R(crate::FieldReader<u8, u8>);
impl SUBMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SUBMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUBMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUBMODE` writer - 13:12\\]
Internal. Only to be used through TI provided API."]
pub struct SUBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `SAV_PGM_CMD` reader - 11:9\\]
Internal. Only to be used through TI provided API."]
pub struct SAV_PGM_CMD_R(crate::FieldReader<u8, u8>);
impl SAV_PGM_CMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAV_PGM_CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAV_PGM_CMD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAV_PGM_CMD` writer - 11:9\\]
Internal. Only to be used through TI provided API."]
pub struct SAV_PGM_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SAV_PGM_CMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `SAV_ERA_MODE` reader - 8:6\\]
Internal. Only to be used through TI provided API."]
pub struct SAV_ERA_MODE_R(crate::FieldReader<u8, u8>);
impl SAV_ERA_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAV_ERA_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAV_ERA_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAV_ERA_MODE` writer - 8:6\\]
Internal. Only to be used through TI provided API."]
pub struct SAV_ERA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAV_ERA_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `MODE` reader - 5:3\\]
Internal. Only to be used through TI provided API."]
pub struct MODE_R(crate::FieldReader<u8, u8>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - 5:3\\]
Internal. Only to be used through TI provided API."]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `CMD` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub struct CMD_R(crate::FieldReader<u8, u8>);
impl CMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 18:19 - 19:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rdv_submode(&self) -> RDV_SUBMODE_R {
        RDV_SUBMODE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_submode(&self) -> PGM_SUBMODE_R {
        PGM_SUBMODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_submode(&self) -> ERA_SUBMODE_R {
        ERA_SUBMODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn submode(&self) -> SUBMODE_R {
        SUBMODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sav_pgm_cmd(&self) -> SAV_PGM_CMD_R {
        SAV_PGM_CMD_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sav_era_mode(&self) -> SAV_ERA_MODE_R {
        SAV_ERA_MODE_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bits 18:19 - 19:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rdv_submode(&mut self) -> RDV_SUBMODE_W {
        RDV_SUBMODE_W { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_submode(&mut self) -> PGM_SUBMODE_W {
        PGM_SUBMODE_W { w: self }
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_submode(&mut self) -> ERA_SUBMODE_W {
        ERA_SUBMODE_W { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn submode(&mut self) -> SUBMODE_W {
        SUBMODE_W { w: self }
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sav_pgm_cmd(&mut self) -> SAV_PGM_CMD_W {
        SAV_PGM_CMD_W { w: self }
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sav_era_mode(&mut self) -> SAV_ERA_MODE_W {
        SAV_ERA_MODE_W { w: self }
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_mode](index.html) module"]
pub struct FSM_MODE_SPEC;
impl crate::RegisterSpec for FSM_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_mode::R](R) reader structure"]
impl crate::Readable for FSM_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_mode::W](W) writer structure"]
impl crate::Writable for FSM_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_MODE to value 0"]
impl crate::Resettable for FSM_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
