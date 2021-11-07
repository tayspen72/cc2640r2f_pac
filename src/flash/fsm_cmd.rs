#[doc = "Register `FSM_CMD` reader"]
pub struct R(crate::R<FSM_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_CMD` writer"]
pub struct W(crate::W<FSM_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_CMD_SPEC>;
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
impl From<crate::W<FSM_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED6_R(crate::FieldReader<u32, u32>);
impl RESERVED6_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED6_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED6` writer - 31:6\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | ((value as u32 & 0x03ff_ffff) << 6);
        self.w
    }
}
#[doc = "Field `FSMCMD` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub struct FSMCMD_R(crate::FieldReader<u8, u8>);
impl FSMCMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSMCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSMCMD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSMCMD` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub struct FSMCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMCMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsmcmd(&self) -> FSMCMD_R {
        FSMCMD_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsmcmd(&mut self) -> FSMCMD_W {
        FSMCMD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_cmd](index.html) module"]
pub struct FSM_CMD_SPEC;
impl crate::RegisterSpec for FSM_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_cmd::R](R) reader structure"]
impl crate::Readable for FSM_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_cmd::W](W) writer structure"]
impl crate::Writable for FSM_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_CMD to value 0"]
impl crate::Resettable for FSM_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
