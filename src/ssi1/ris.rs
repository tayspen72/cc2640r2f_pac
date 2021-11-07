#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RIS` writer"]
pub struct W(crate::W<RIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RIS_SPEC>;
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
impl From<crate::W<RIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXRIS` reader - 3:3\\]
Raw transmit FIFO interrupt status: The transmit interrupt is asserted when there are four or fewer valid entries in the transmit FIFO. The transmit interrupt is not qualified with the SSI enable signal. Therefore one of the following ways can be used: - data can be written to the transmit FIFO prior to enabling the SSI and the interrupts. - SSI and interrupts can be enabled so that data can be written to the transmit FIFO by an interrupt service routine."]
pub struct TXRIS_R(crate::FieldReader<bool, bool>);
impl TXRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRIS` writer - 3:3\\]
Raw transmit FIFO interrupt status: The transmit interrupt is asserted when there are four or fewer valid entries in the transmit FIFO. The transmit interrupt is not qualified with the SSI enable signal. Therefore one of the following ways can be used: - data can be written to the transmit FIFO prior to enabling the SSI and the interrupts. - SSI and interrupts can be enabled so that data can be written to the transmit FIFO by an interrupt service routine."]
pub struct TXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RXRIS` reader - 2:2\\]
Raw interrupt state of receive FIFO interrupt: The receive interrupt is asserted when there are four or more valid entries in the receive FIFO."]
pub struct RXRIS_R(crate::FieldReader<bool, bool>);
impl RXRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRIS` writer - 2:2\\]
Raw interrupt state of receive FIFO interrupt: The receive interrupt is asserted when there are four or more valid entries in the receive FIFO."]
pub struct RXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRIS_W<'a> {
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
#[doc = "Field `RTRIS` reader - 1:1\\]
Raw interrupt state of receive timeout interrupt: The receive timeout interrupt is asserted when the receive FIFO is not empty and SSI has remained idle for a fixed 32 bit period. This mechanism can be used to notify the user that data is still present in the receive FIFO and requires servicing. This interrupt is deasserted if the receive FIFO becomes empty by subsequent reads, or if new data is received on RXD. It can also be cleared by writing to ICR.RTIC."]
pub struct RTRIS_R(crate::FieldReader<bool, bool>);
impl RTRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTRIS` writer - 1:1\\]
Raw interrupt state of receive timeout interrupt: The receive timeout interrupt is asserted when the receive FIFO is not empty and SSI has remained idle for a fixed 32 bit period. This mechanism can be used to notify the user that data is still present in the receive FIFO and requires servicing. This interrupt is deasserted if the receive FIFO becomes empty by subsequent reads, or if new data is received on RXD. It can also be cleared by writing to ICR.RTIC."]
pub struct RTRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTRIS_W<'a> {
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
#[doc = "Field `RORRIS` reader - 0:0\\]
Raw interrupt state of receive overrun interrupt: The receive overrun interrupt is asserted when the FIFO is already full and an additional data frame is received, causing an overrun of the FIFO. Data is over-written in the receive shift register, but not the FIFO so the FIFO contents stay valid. It can also be cleared by writing to ICR.RORIC."]
pub struct RORRIS_R(crate::FieldReader<bool, bool>);
impl RORRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RORRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RORRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RORRIS` writer - 0:0\\]
Raw interrupt state of receive overrun interrupt: The receive overrun interrupt is asserted when the FIFO is already full and an additional data frame is received, causing an overrun of the FIFO. Data is over-written in the receive shift register, but not the FIFO so the FIFO contents stay valid. It can also be cleared by writing to ICR.RORIC."]
pub struct RORRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RORRIS_W<'a> {
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
    #[doc = "Bit 3 - 3:3\\]
Raw transmit FIFO interrupt status: The transmit interrupt is asserted when there are four or fewer valid entries in the transmit FIFO. The transmit interrupt is not qualified with the SSI enable signal. Therefore one of the following ways can be used: - data can be written to the transmit FIFO prior to enabling the SSI and the interrupts. - SSI and interrupts can be enabled so that data can be written to the transmit FIFO by an interrupt service routine."]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw interrupt state of receive FIFO interrupt: The receive interrupt is asserted when there are four or more valid entries in the receive FIFO."]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Raw interrupt state of receive timeout interrupt: The receive timeout interrupt is asserted when the receive FIFO is not empty and SSI has remained idle for a fixed 32 bit period. This mechanism can be used to notify the user that data is still present in the receive FIFO and requires servicing. This interrupt is deasserted if the receive FIFO becomes empty by subsequent reads, or if new data is received on RXD. It can also be cleared by writing to ICR.RTIC."]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Raw interrupt state of receive overrun interrupt: The receive overrun interrupt is asserted when the FIFO is already full and an additional data frame is received, causing an overrun of the FIFO. Data is over-written in the receive shift register, but not the FIFO so the FIFO contents stay valid. It can also be cleared by writing to ICR.RORIC."]
    #[inline(always)]
    pub fn rorris(&self) -> RORRIS_R {
        RORRIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 3:3\\]
Raw transmit FIFO interrupt status: The transmit interrupt is asserted when there are four or fewer valid entries in the transmit FIFO. The transmit interrupt is not qualified with the SSI enable signal. Therefore one of the following ways can be used: - data can be written to the transmit FIFO prior to enabling the SSI and the interrupts. - SSI and interrupts can be enabled so that data can be written to the transmit FIFO by an interrupt service routine."]
    #[inline(always)]
    pub fn txris(&mut self) -> TXRIS_W {
        TXRIS_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Raw interrupt state of receive FIFO interrupt: The receive interrupt is asserted when there are four or more valid entries in the receive FIFO."]
    #[inline(always)]
    pub fn rxris(&mut self) -> RXRIS_W {
        RXRIS_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Raw interrupt state of receive timeout interrupt: The receive timeout interrupt is asserted when the receive FIFO is not empty and SSI has remained idle for a fixed 32 bit period. This mechanism can be used to notify the user that data is still present in the receive FIFO and requires servicing. This interrupt is deasserted if the receive FIFO becomes empty by subsequent reads, or if new data is received on RXD. It can also be cleared by writing to ICR.RTIC."]
    #[inline(always)]
    pub fn rtris(&mut self) -> RTRIS_W {
        RTRIS_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Raw interrupt state of receive overrun interrupt: The receive overrun interrupt is asserted when the FIFO is already full and an additional data frame is received, causing an overrun of the FIFO. Data is over-written in the receive shift register, but not the FIFO so the FIFO contents stay valid. It can also be cleared by writing to ICR.RORIC."]
    #[inline(always)]
    pub fn rorris(&mut self) -> RORRIS_W {
        RORRIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ris::W](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RIS to value 0x08"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
