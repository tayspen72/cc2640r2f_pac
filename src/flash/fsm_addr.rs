#[doc = "Register `FSM_ADDR` reader"]
pub struct R(crate::R<FSM_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_ADDR` writer"]
pub struct W(crate::W<FSM_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_ADDR_SPEC>;
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
impl From<crate::W<FSM_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED31` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED31_R(crate::FieldReader<bool, bool>);
impl RESERVED31_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED31` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED31_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED31_W<'a> {
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
#[doc = "Field `BANK` reader - 30:28\\]
Internal. Only to be used through TI provided API."]
pub struct BANK_R(crate::FieldReader<u8, u8>);
impl BANK_R {
    pub(crate) fn new(bits: u8) -> Self {
        BANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BANK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BANK` writer - 30:28\\]
Internal. Only to be used through TI provided API."]
pub struct BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `CUR_ADDR` reader - 27:0\\]
Internal. Only to be used through TI provided API."]
pub struct CUR_ADDR_R(crate::FieldReader<u32, u32>);
impl CUR_ADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CUR_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CUR_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CUR_ADDR` writer - 27:0\\]
Internal. Only to be used through TI provided API."]
pub struct CUR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CUR_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | (value as u32 & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved31(&self) -> RESERVED31_R {
        RESERVED31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bank(&self) -> BANK_R {
        BANK_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 0:27 - 27:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cur_addr(&self) -> CUR_ADDR_R {
        CUR_ADDR_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved31(&mut self) -> RESERVED31_W {
        RESERVED31_W { w: self }
    }
    #[doc = "Bits 28:30 - 30:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bank(&mut self) -> BANK_W {
        BANK_W { w: self }
    }
    #[doc = "Bits 0:27 - 27:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cur_addr(&mut self) -> CUR_ADDR_W {
        CUR_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_addr](index.html) module"]
pub struct FSM_ADDR_SPEC;
impl crate::RegisterSpec for FSM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_addr::R](R) reader structure"]
impl crate::Readable for FSM_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_addr::W](W) writer structure"]
impl crate::Writable for FSM_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM_ADDR to value 0"]
impl crate::Resettable for FSM_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
