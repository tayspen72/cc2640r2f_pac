#[doc = "Register `FSM_SECTOR2` reader"]
pub struct R(crate::R<FSM_SECTOR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_SECTOR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_SECTOR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_SECTOR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_SECTOR2` writer"]
pub struct W(crate::W<FSM_SECTOR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_SECTOR2_SPEC>;
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
impl From<crate::W<FSM_SECTOR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_SECTOR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSM_SECTOR2` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct FSM_SECTOR2_R(crate::FieldReader<u32, u32>);
impl FSM_SECTOR2_R {
    pub(crate) fn new(bits: u32) -> Self {
        FSM_SECTOR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSM_SECTOR2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSM_SECTOR2` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct FSM_SECTOR2_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_SECTOR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_sector2(&self) -> FSM_SECTOR2_R {
        FSM_SECTOR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_sector2(&mut self) -> FSM_SECTOR2_W {
        FSM_SECTOR2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_sector2](index.html) module"]
pub struct FSM_SECTOR2_SPEC;
impl crate::RegisterSpec for FSM_SECTOR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_sector2::R](R) reader structure"]
impl crate::Readable for FSM_SECTOR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_sector2::W](W) writer structure"]
impl crate::Writable for FSM_SECTOR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_SECTOR2 to value 0"]
impl crate::Resettable for FSM_SECTOR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
