#[doc = "Register `PWRDWNACK` reader"]
pub struct R(crate::R<PWRDWNACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRDWNACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRDWNACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRDWNACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRDWNACK` writer"]
pub struct W(crate::W<PWRDWNACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRDWNACK_SPEC>;
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
impl From<crate::W<PWRDWNACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRDWNACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACK` reader - 0:0\\]
Power down acknowledgment. Indicates whether the power down request given by PWRDWNREQ.REQ is captured by the AON domain or not 0: AUX can assume that the system is in active mode 1: The request for power down is acknowledged and the AUX must act like the system is in power down mode and power supply is limited The system CPU cannot use this bit since the bus bridge between MCU domain and AUX domain is always disconnected when this bit is set. For AUX_SCE use only"]
pub struct ACK_R(crate::FieldReader<bool, bool>);
impl ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACK` writer - 0:0\\]
Power down acknowledgment. Indicates whether the power down request given by PWRDWNREQ.REQ is captured by the AON domain or not 0: AUX can assume that the system is in active mode 1: The request for power down is acknowledged and the AUX must act like the system is in power down mode and power supply is limited The system CPU cannot use this bit since the bus bridge between MCU domain and AUX domain is always disconnected when this bit is set. For AUX_SCE use only"]
pub struct ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_W<'a> {
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
    #[doc = "Bit 0 - 0:0\\]
Power down acknowledgment. Indicates whether the power down request given by PWRDWNREQ.REQ is captured by the AON domain or not 0: AUX can assume that the system is in active mode 1: The request for power down is acknowledged and the AUX must act like the system is in power down mode and power supply is limited The system CPU cannot use this bit since the bus bridge between MCU domain and AUX domain is always disconnected when this bit is set. For AUX_SCE use only"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Power down acknowledgment. Indicates whether the power down request given by PWRDWNREQ.REQ is captured by the AON domain or not 0: AUX can assume that the system is in active mode 1: The request for power down is acknowledged and the AUX must act like the system is in power down mode and power supply is limited The system CPU cannot use this bit since the bus bridge between MCU domain and AUX domain is always disconnected when this bit is set. For AUX_SCE use only"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Down Acknowledgment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrdwnack](index.html) module"]
pub struct PWRDWNACK_SPEC;
impl crate::RegisterSpec for PWRDWNACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrdwnack::R](R) reader structure"]
impl crate::Readable for PWRDWNACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrdwnack::W](W) writer structure"]
impl crate::Writable for PWRDWNACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRDWNACK to value 0"]
impl crate::Resettable for PWRDWNACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
