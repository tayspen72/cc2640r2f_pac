#[doc = "Register `DMACTL` reader"]
pub struct R(crate::R<DMACTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTL` writer"]
pub struct W(crate::W<DMACTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTL_SPEC>;
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
impl From<crate::W<DMACTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
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
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
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
#[doc = "2:2\\]
UDMA0 Request mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQ_MODE_A {
    #[doc = "1: Single requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    SINGLE = 1,
    #[doc = "0: Burst requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    BURST = 0,
}
impl From<REQ_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: REQ_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQ_MODE` reader - 2:2\\]
UDMA0 Request mode"]
pub struct REQ_MODE_R(crate::FieldReader<bool, REQ_MODE_A>);
impl REQ_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REQ_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQ_MODE_A {
        match self.bits {
            true => REQ_MODE_A::SINGLE,
            false => REQ_MODE_A::BURST,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == REQ_MODE_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        **self == REQ_MODE_A::BURST
    }
}
impl core::ops::Deref for REQ_MODE_R {
    type Target = crate::FieldReader<bool, REQ_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQ_MODE` writer - 2:2\\]
UDMA0 Request mode"]
pub struct REQ_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REQ_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(REQ_MODE_A::SINGLE)
    }
    #[doc = "Burst requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    #[inline(always)]
    pub fn burst(self) -> &'a mut W {
        self.variant(REQ_MODE_A::BURST)
    }
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
#[doc = "Field `EN` reader - 1:1\\]
uDMA ADC interface enable. 0: Disable UDMA0 interface to ADC. 1: Enable UDMA0 interface to ADC."]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - 1:1\\]
uDMA ADC interface enable. 0: Disable UDMA0 interface to ADC. 1: Enable UDMA0 interface to ADC."]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "0:0\\]
Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "1: UDMA0 trigger event will be generated when the ADC FIFO is almost full (3/4 full)."]
    FIFO_ALMOST_FULL = 1,
    #[doc = "0: UDMA0 trigger event will be generated when there are samples in the ADC FIFO."]
    FIFO_NOT_EMPTY = 0,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - 0:0\\]
Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data."]
pub struct SEL_R(crate::FieldReader<bool, SEL_A>);
impl SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            true => SEL_A::FIFO_ALMOST_FULL,
            false => SEL_A::FIFO_NOT_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_ALMOST_FULL`"]
    #[inline(always)]
    pub fn is_fifo_almost_full(&self) -> bool {
        **self == SEL_A::FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `FIFO_NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_fifo_not_empty(&self) -> bool {
        **self == SEL_A::FIFO_NOT_EMPTY
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<bool, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - 0:0\\]
Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data."]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "UDMA0 trigger event will be generated when the ADC FIFO is almost full (3/4 full)."]
    #[inline(always)]
    pub fn fifo_almost_full(self) -> &'a mut W {
        self.variant(SEL_A::FIFO_ALMOST_FULL)
    }
    #[doc = "UDMA0 trigger event will be generated when there are samples in the ADC FIFO."]
    #[inline(always)]
    pub fn fifo_not_empty(self) -> &'a mut W {
        self.variant(SEL_A::FIFO_NOT_EMPTY)
    }
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
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
UDMA0 Request mode"]
    #[inline(always)]
    pub fn req_mode(&self) -> REQ_MODE_R {
        REQ_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
uDMA ADC interface enable. 0: Disable UDMA0 interface to ADC. 1: Enable UDMA0 interface to ADC."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
UDMA0 Request mode"]
    #[inline(always)]
    pub fn req_mode(&mut self) -> REQ_MODE_W {
        REQ_MODE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
uDMA ADC interface enable. 0: Disable UDMA0 interface to ADC. 1: Enable UDMA0 interface to ADC."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direct Memory Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl](index.html) module"]
pub struct DMACTL_SPEC;
impl crate::RegisterSpec for DMACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmactl::R](R) reader structure"]
impl crate::Readable for DMACTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactl::W](W) writer structure"]
impl crate::Writable for DMACTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACTL to value 0"]
impl crate::Resettable for DMACTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
