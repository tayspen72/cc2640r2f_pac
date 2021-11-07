#[doc = "Register `FBFALLBACK` reader"]
pub struct R(crate::R<FBFALLBACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBFALLBACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBFALLBACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBFALLBACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBFALLBACK` writer"]
pub struct W(crate::W<FBFALLBACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBFALLBACK_SPEC>;
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
impl From<crate::W<FBFALLBACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBFALLBACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED28` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED28_R(crate::FieldReader<u8, u8>);
impl RESERVED28_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED28_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED28` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED28_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `FSM_PWRSAV` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub struct FSM_PWRSAV_R(crate::FieldReader<u8, u8>);
impl FSM_PWRSAV_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSM_PWRSAV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSM_PWRSAV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSM_PWRSAV` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub struct FSM_PWRSAV_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_PWRSAV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `RESERVED20` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED20_R(crate::FieldReader<u8, u8>);
impl RESERVED20_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED20_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED20` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED20_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `REG_PWRSAV` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct REG_PWRSAV_R(crate::FieldReader<u8, u8>);
impl REG_PWRSAV_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_PWRSAV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_PWRSAV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_PWRSAV` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct REG_PWRSAV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_PWRSAV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `BANKPWR7` reader - 15:14\\]
Internal. Only to be used through TI provided API."]
pub struct BANKPWR7_R(crate::FieldReader<u8, u8>);
impl BANKPWR7_R {
    pub(crate) fn new(bits: u8) -> Self {
        BANKPWR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BANKPWR7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BANKPWR7` writer - 15:14\\]
Internal. Only to be used through TI provided API."]
pub struct BANKPWR7_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKPWR7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `BANKPWR6` reader - 13:12\\]
Internal. Only to be used through TI provided API."]
pub struct BANKPWR6_R(crate::FieldReader<u8, u8>);
impl BANKPWR6_R {
    pub(crate) fn new(bits: u8) -> Self {
        BANKPWR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BANKPWR6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BANKPWR6` writer - 13:12\\]
Internal. Only to be used through TI provided API."]
pub struct BANKPWR6_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKPWR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `BANKPWR5` reader - 11:10\\]
Internal. Only to be used through TI provided API."]
pub struct BANKPWR5_R(crate::FieldReader<u8, u8>);
impl BANKPWR5_R {
    pub(crate) fn new(bits: u8) -> Self {
        BANKPWR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BANKPWR5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BANKPWR5` writer - 11:10\\]
Internal. Only to be used through TI provided API."]
pub struct BANKPWR5_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKPWR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `BANKPWR4` reader - 9:8\\]
Internal. Only to be used through TI provided API."]
pub struct BANKPWR4_R(crate::FieldReader<u8, u8>);
impl BANKPWR4_R {
    pub(crate) fn new(bits: u8) -> Self {
        BANKPWR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BANKPWR4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BANKPWR4` writer - 9:8\\]
Internal. Only to be used through TI provided API."]
pub struct BANKPWR4_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKPWR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `BANKPWR3` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub struct BANKPWR3_R(crate::FieldReader<u8, u8>);
impl BANKPWR3_R {
    pub(crate) fn new(bits: u8) -> Self {
        BANKPWR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BANKPWR3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BANKPWR3` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub struct BANKPWR3_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKPWR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `BANKPWR2` reader - 5:4\\]
Internal. Only to be used through TI provided API."]
pub struct BANKPWR2_R(crate::FieldReader<u8, u8>);
impl BANKPWR2_R {
    pub(crate) fn new(bits: u8) -> Self {
        BANKPWR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BANKPWR2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BANKPWR2` writer - 5:4\\]
Internal. Only to be used through TI provided API."]
pub struct BANKPWR2_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKPWR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `BANKPWR1` reader - 3:2\\]
Internal. Only to be used through TI provided API."]
pub struct BANKPWR1_R(crate::FieldReader<u8, u8>);
impl BANKPWR1_R {
    pub(crate) fn new(bits: u8) -> Self {
        BANKPWR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BANKPWR1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BANKPWR1` writer - 3:2\\]
Internal. Only to be used through TI provided API."]
pub struct BANKPWR1_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKPWR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `BANKPWR0` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct BANKPWR0_R(crate::FieldReader<u8, u8>);
impl BANKPWR0_R {
    pub(crate) fn new(bits: u8) -> Self {
        BANKPWR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BANKPWR0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BANKPWR0` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct BANKPWR0_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKPWR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_pwrsav(&self) -> FSM_PWRSAV_R {
        FSM_PWRSAV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg_pwrsav(&self) -> REG_PWRSAV_R {
        REG_PWRSAV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr7(&self) -> BANKPWR7_R {
        BANKPWR7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr6(&self) -> BANKPWR6_R {
        BANKPWR6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr5(&self) -> BANKPWR5_R {
        BANKPWR5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr4(&self) -> BANKPWR4_R {
        BANKPWR4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr3(&self) -> BANKPWR3_R {
        BANKPWR3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr2(&self) -> BANKPWR2_R {
        BANKPWR2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr1(&self) -> BANKPWR1_R {
        BANKPWR1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr0(&self) -> BANKPWR0_R {
        BANKPWR0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&mut self) -> RESERVED28_W {
        RESERVED28_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_pwrsav(&mut self) -> FSM_PWRSAV_W {
        FSM_PWRSAV_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg_pwrsav(&mut self) -> REG_PWRSAV_W {
        REG_PWRSAV_W { w: self }
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr7(&mut self) -> BANKPWR7_W {
        BANKPWR7_W { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr6(&mut self) -> BANKPWR6_W {
        BANKPWR6_W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr5(&mut self) -> BANKPWR5_W {
        BANKPWR5_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr4(&mut self) -> BANKPWR4_W {
        BANKPWR4_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr3(&mut self) -> BANKPWR3_W {
        BANKPWR3_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr2(&mut self) -> BANKPWR2_W {
        BANKPWR2_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr1(&mut self) -> BANKPWR1_W {
        BANKPWR1_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankpwr0(&mut self) -> BANKPWR0_W {
        BANKPWR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbfallback](index.html) module"]
pub struct FBFALLBACK_SPEC;
impl crate::RegisterSpec for FBFALLBACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbfallback::R](R) reader structure"]
impl crate::Readable for FBFALLBACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbfallback::W](W) writer structure"]
impl crate::Writable for FBFALLBACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FBFALLBACK to value 0x0505_ffff"]
impl crate::Resettable for FBFALLBACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0505_ffff
    }
}
