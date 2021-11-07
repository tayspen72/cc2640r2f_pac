#[doc = "Register `AMPCOMPCTL` reader"]
pub struct R(crate::R<AMPCOMPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPCOMPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPCOMPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPCOMPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPCOMPCTL` writer"]
pub struct W(crate::W<AMPCOMPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPCOMPCTL_SPEC>;
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
impl From<crate::W<AMPCOMPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPCOMPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPARE31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE31_R(crate::FieldReader<bool, bool>);
impl SPARE31_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPARE31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE31` writer - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE31_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `AMPCOMP_REQ_MODE` reader - 30:30\\]
Internal. Only to be used through TI provided API."]
pub struct AMPCOMP_REQ_MODE_R(crate::FieldReader<bool, bool>);
impl AMPCOMP_REQ_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMPCOMP_REQ_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMPCOMP_REQ_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMPCOMP_REQ_MODE` writer - 30:30\\]
Internal. Only to be used through TI provided API."]
pub struct AMPCOMP_REQ_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPCOMP_REQ_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "29:28\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AMPCOMP_FSM_UPDATE_RATE_A {
    #[doc = "3: Internal. Only to be used through TI provided API."]
    _250KHZ = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    _500KHZ = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    _1MHZ = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    _2MHZ = 0,
}
impl From<AMPCOMP_FSM_UPDATE_RATE_A> for u8 {
    #[inline(always)]
    fn from(variant: AMPCOMP_FSM_UPDATE_RATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AMPCOMP_FSM_UPDATE_RATE` reader - 29:28\\]
Internal. Only to be used through TI provided API."]
pub struct AMPCOMP_FSM_UPDATE_RATE_R(crate::FieldReader<u8, AMPCOMP_FSM_UPDATE_RATE_A>);
impl AMPCOMP_FSM_UPDATE_RATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        AMPCOMP_FSM_UPDATE_RATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPCOMP_FSM_UPDATE_RATE_A {
        match self.bits {
            3 => AMPCOMP_FSM_UPDATE_RATE_A::_250KHZ,
            2 => AMPCOMP_FSM_UPDATE_RATE_A::_500KHZ,
            1 => AMPCOMP_FSM_UPDATE_RATE_A::_1MHZ,
            0 => AMPCOMP_FSM_UPDATE_RATE_A::_2MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_250KHZ`"]
    #[inline(always)]
    pub fn is_250khz(&self) -> bool {
        **self == AMPCOMP_FSM_UPDATE_RATE_A::_250KHZ
    }
    #[doc = "Checks if the value of the field is `_500KHZ`"]
    #[inline(always)]
    pub fn is_500khz(&self) -> bool {
        **self == AMPCOMP_FSM_UPDATE_RATE_A::_500KHZ
    }
    #[doc = "Checks if the value of the field is `_1MHZ`"]
    #[inline(always)]
    pub fn is_1mhz(&self) -> bool {
        **self == AMPCOMP_FSM_UPDATE_RATE_A::_1MHZ
    }
    #[doc = "Checks if the value of the field is `_2MHZ`"]
    #[inline(always)]
    pub fn is_2mhz(&self) -> bool {
        **self == AMPCOMP_FSM_UPDATE_RATE_A::_2MHZ
    }
}
impl core::ops::Deref for AMPCOMP_FSM_UPDATE_RATE_R {
    type Target = crate::FieldReader<u8, AMPCOMP_FSM_UPDATE_RATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMPCOMP_FSM_UPDATE_RATE` writer - 29:28\\]
Internal. Only to be used through TI provided API."]
pub struct AMPCOMP_FSM_UPDATE_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPCOMP_FSM_UPDATE_RATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AMPCOMP_FSM_UPDATE_RATE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _250khz(self) -> &'a mut W {
        self.variant(AMPCOMP_FSM_UPDATE_RATE_A::_250KHZ)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _500khz(self) -> &'a mut W {
        self.variant(AMPCOMP_FSM_UPDATE_RATE_A::_500KHZ)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _1mhz(self) -> &'a mut W {
        self.variant(AMPCOMP_FSM_UPDATE_RATE_A::_1MHZ)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _2mhz(self) -> &'a mut W {
        self.variant(AMPCOMP_FSM_UPDATE_RATE_A::_2MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `AMPCOMP_SW_CTRL` reader - 27:27\\]
Internal. Only to be used through TI provided API."]
pub struct AMPCOMP_SW_CTRL_R(crate::FieldReader<bool, bool>);
impl AMPCOMP_SW_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMPCOMP_SW_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMPCOMP_SW_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMPCOMP_SW_CTRL` writer - 27:27\\]
Internal. Only to be used through TI provided API."]
pub struct AMPCOMP_SW_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPCOMP_SW_CTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `AMPCOMP_SW_EN` reader - 26:26\\]
Internal. Only to be used through TI provided API."]
pub struct AMPCOMP_SW_EN_R(crate::FieldReader<bool, bool>);
impl AMPCOMP_SW_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMPCOMP_SW_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMPCOMP_SW_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMPCOMP_SW_EN` writer - 26:26\\]
Internal. Only to be used through TI provided API."]
pub struct AMPCOMP_SW_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPCOMP_SW_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `RESERVED24` reader - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED24_R(crate::FieldReader<u8, u8>);
impl RESERVED24_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED24_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED24` writer - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `IBIAS_OFFSET` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub struct IBIAS_OFFSET_R(crate::FieldReader<u8, u8>);
impl IBIAS_OFFSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        IBIAS_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBIAS_OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBIAS_OFFSET` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub struct IBIAS_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIAS_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `IBIAS_INIT` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct IBIAS_INIT_R(crate::FieldReader<u8, u8>);
impl IBIAS_INIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        IBIAS_INIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBIAS_INIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBIAS_INIT` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct IBIAS_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIAS_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `LPM_IBIAS_WAIT_CNT_FINAL` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct LPM_IBIAS_WAIT_CNT_FINAL_R(crate::FieldReader<u8, u8>);
impl LPM_IBIAS_WAIT_CNT_FINAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPM_IBIAS_WAIT_CNT_FINAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_IBIAS_WAIT_CNT_FINAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM_IBIAS_WAIT_CNT_FINAL` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct LPM_IBIAS_WAIT_CNT_FINAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_IBIAS_WAIT_CNT_FINAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CAP_STEP` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub struct CAP_STEP_R(crate::FieldReader<u8, u8>);
impl CAP_STEP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAP_STEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP_STEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP_STEP` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub struct CAP_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `IBIASCAP_HPTOLP_OL_CNT` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct IBIASCAP_HPTOLP_OL_CNT_R(crate::FieldReader<u8, u8>);
impl IBIASCAP_HPTOLP_OL_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        IBIASCAP_HPTOLP_OL_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBIASCAP_HPTOLP_OL_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBIASCAP_HPTOLP_OL_CNT` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct IBIASCAP_HPTOLP_OL_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIASCAP_HPTOLP_OL_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare31(&self) -> SPARE31_R {
        SPARE31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_req_mode(&self) -> AMPCOMP_REQ_MODE_R {
        AMPCOMP_REQ_MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_fsm_update_rate(&self) -> AMPCOMP_FSM_UPDATE_RATE_R {
        AMPCOMP_FSM_UPDATE_RATE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_sw_ctrl(&self) -> AMPCOMP_SW_CTRL_R {
        AMPCOMP_SW_CTRL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_sw_en(&self) -> AMPCOMP_SW_EN_R {
        AMPCOMP_SW_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_offset(&self) -> IBIAS_OFFSET_R {
        IBIAS_OFFSET_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_init(&self) -> IBIAS_INIT_R {
        IBIAS_INIT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt_final(&self) -> LPM_IBIAS_WAIT_CNT_FINAL_R {
        LPM_IBIAS_WAIT_CNT_FINAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cap_step(&self) -> CAP_STEP_R {
        CAP_STEP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibiascap_hptolp_ol_cnt(&self) -> IBIASCAP_HPTOLP_OL_CNT_R {
        IBIASCAP_HPTOLP_OL_CNT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare31(&mut self) -> SPARE31_W {
        SPARE31_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_req_mode(&mut self) -> AMPCOMP_REQ_MODE_W {
        AMPCOMP_REQ_MODE_W { w: self }
    }
    #[doc = "Bits 28:29 - 29:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_fsm_update_rate(&mut self) -> AMPCOMP_FSM_UPDATE_RATE_W {
        AMPCOMP_FSM_UPDATE_RATE_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_sw_ctrl(&mut self) -> AMPCOMP_SW_CTRL_W {
        AMPCOMP_SW_CTRL_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_sw_en(&mut self) -> AMPCOMP_SW_EN_W {
        AMPCOMP_SW_EN_W { w: self }
    }
    #[doc = "Bits 24:25 - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_offset(&mut self) -> IBIAS_OFFSET_W {
        IBIAS_OFFSET_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_init(&mut self) -> IBIAS_INIT_W {
        IBIAS_INIT_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt_final(&mut self) -> LPM_IBIAS_WAIT_CNT_FINAL_W {
        LPM_IBIAS_WAIT_CNT_FINAL_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cap_step(&mut self) -> CAP_STEP_W {
        CAP_STEP_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibiascap_hptolp_ol_cnt(&mut self) -> IBIASCAP_HPTOLP_OL_CNT_W {
        IBIASCAP_HPTOLP_OL_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Amplitude Compensation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcompctl](index.html) module"]
pub struct AMPCOMPCTL_SPEC;
impl crate::RegisterSpec for AMPCOMPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ampcompctl::R](R) reader structure"]
impl crate::Readable for AMPCOMPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ampcompctl::W](W) writer structure"]
impl crate::Writable for AMPCOMPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AMPCOMPCTL to value 0"]
impl crate::Resettable for AMPCOMPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
