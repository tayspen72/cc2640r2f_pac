#[doc = "Register `RTCSUBSECINCCTL` reader"]
pub struct R(crate::R<RTCSUBSECINCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCSUBSECINCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCSUBSECINCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCSUBSECINCCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCSUBSECINCCTL` writer"]
pub struct W(crate::W<RTCSUBSECINCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCSUBSECINCCTL_SPEC>;
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
impl From<crate::W<RTCSUBSECINCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCSUBSECINCCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPD_ACK` reader - 1:1\\]
Acknowledgment of the UPD_REQ."]
pub struct UPD_ACK_R(crate::FieldReader<bool, bool>);
impl UPD_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPD_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPD_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPD_ACK` writer - 1:1\\]
Acknowledgment of the UPD_REQ."]
pub struct UPD_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> UPD_ACK_W<'a> {
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
#[doc = "Field `UPD_REQ` reader - 0:0\\]
Signal that a new real time counter sub second increment value is available 0: New sub second increment is not available 1: New sub second increment is available This bit must not be modified unless UPD_ACK matches the current value."]
pub struct UPD_REQ_R(crate::FieldReader<bool, bool>);
impl UPD_REQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPD_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPD_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPD_REQ` writer - 0:0\\]
Signal that a new real time counter sub second increment value is available 0: New sub second increment is not available 1: New sub second increment is available This bit must not be modified unless UPD_ACK matches the current value."]
pub struct UPD_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> UPD_REQ_W<'a> {
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
    #[doc = "Bit 1 - 1:1\\]
Acknowledgment of the UPD_REQ."]
    #[inline(always)]
    pub fn upd_ack(&self) -> UPD_ACK_R {
        UPD_ACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Signal that a new real time counter sub second increment value is available 0: New sub second increment is not available 1: New sub second increment is available This bit must not be modified unless UPD_ACK matches the current value."]
    #[inline(always)]
    pub fn upd_req(&self) -> UPD_REQ_R {
        UPD_REQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
Acknowledgment of the UPD_REQ."]
    #[inline(always)]
    pub fn upd_ack(&mut self) -> UPD_ACK_W {
        UPD_ACK_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Signal that a new real time counter sub second increment value is available 0: New sub second increment is not available 1: New sub second increment is available This bit must not be modified unless UPD_ACK matches the current value."]
    #[inline(always)]
    pub fn upd_req(&mut self) -> UPD_REQ_W {
        UPD_REQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Time Counter Sub Second Increment Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsubsecincctl](index.html) module"]
pub struct RTCSUBSECINCCTL_SPEC;
impl crate::RegisterSpec for RTCSUBSECINCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcsubsecincctl::R](R) reader structure"]
impl crate::Readable for RTCSUBSECINCCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcsubsecincctl::W](W) writer structure"]
impl crate::Writable for RTCSUBSECINCCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCSUBSECINCCTL to value 0"]
impl crate::Resettable for RTCSUBSECINCCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
