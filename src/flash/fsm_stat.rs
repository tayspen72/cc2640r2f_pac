#[doc = "Register `FSM_STAT` reader"]
pub struct R(crate::R<FSM_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_STAT` writer"]
pub struct W(crate::W<FSM_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_STAT_SPEC>;
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
impl From<crate::W<FSM_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED3_R(crate::FieldReader<u32, u32>);
impl RESERVED3_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 31:3\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Field `NON_OP` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub struct NON_OP_R(crate::FieldReader<bool, bool>);
impl NON_OP_R {
    pub(crate) fn new(bits: bool) -> Self {
        NON_OP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NON_OP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NON_OP` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub struct NON_OP_W<'a> {
    w: &'a mut W,
}
impl<'a> NON_OP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `OVR_PUL_CNT` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub struct OVR_PUL_CNT_R(crate::FieldReader<bool, bool>);
impl OVR_PUL_CNT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVR_PUL_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_PUL_CNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_PUL_CNT` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub struct OVR_PUL_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_PUL_CNT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `INV_DAT` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct INV_DAT_R(crate::FieldReader<bool, bool>);
impl INV_DAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INV_DAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INV_DAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV_DAT` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct INV_DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_DAT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn non_op(&self) -> NON_OP_R {
        NON_OP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ovr_pul_cnt(&self) -> OVR_PUL_CNT_R {
        OVR_PUL_CNT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn inv_dat(&self) -> INV_DAT_R {
        INV_DAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn non_op(&mut self) -> NON_OP_W {
        NON_OP_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ovr_pul_cnt(&mut self) -> OVR_PUL_CNT_W {
        OVR_PUL_CNT_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn inv_dat(&mut self) -> INV_DAT_W {
        INV_DAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_stat](index.html) module"]
pub struct FSM_STAT_SPEC;
impl crate::RegisterSpec for FSM_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_stat::R](R) reader structure"]
impl crate::Readable for FSM_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_stat::W](W) writer structure"]
impl crate::Writable for FSM_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_STAT to value 0x04"]
impl crate::Resettable for FSM_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
