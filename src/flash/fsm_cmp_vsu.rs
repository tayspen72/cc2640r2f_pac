#[doc = "Register `FSM_CMP_VSU` reader"]
pub struct R(crate::R<FSM_CMP_VSU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_CMP_VSU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_CMP_VSU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_CMP_VSU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_CMP_VSU` writer"]
pub struct W(crate::W<FSM_CMP_VSU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_CMP_VSU_SPEC>;
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
impl From<crate::W<FSM_CMP_VSU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_CMP_VSU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED16_R(crate::FieldReader<u16, u16>);
impl RESERVED16_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED16_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `ADD_EXZ` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct ADD_EXZ_R(crate::FieldReader<u8, u8>);
impl ADD_EXZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADD_EXZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD_EXZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD_EXZ` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct ADD_EXZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_EXZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED0_R(crate::FieldReader<u16, u16>);
impl RESERVED0_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn add_exz(&self) -> ADD_EXZ_R {
        ADD_EXZ_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn add_exz(&mut self) -> ADD_EXZ_W {
        ADD_EXZ_W { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_cmp_vsu](index.html) module"]
pub struct FSM_CMP_VSU_SPEC;
impl crate::RegisterSpec for FSM_CMP_VSU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_cmp_vsu::R](R) reader structure"]
impl crate::Readable for FSM_CMP_VSU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_cmp_vsu::W](W) writer structure"]
impl crate::Writable for FSM_CMP_VSU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_CMP_VSU to value 0"]
impl crate::Resettable for FSM_CMP_VSU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
