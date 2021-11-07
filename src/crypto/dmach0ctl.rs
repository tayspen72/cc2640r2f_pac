#[doc = "Register `DMACH0CTL` reader"]
pub struct R(crate::R<DMACH0CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACH0CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACH0CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACH0CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACH0CTL` writer"]
pub struct W(crate::W<DMACH0CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACH0CTL_SPEC>;
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
impl From<crate::W<DMACH0CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACH0CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_R(crate::FieldReader<u32, u32>);
impl RESERVED2_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "1:1\\]
Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRIO_A {
    #[doc = "1: Priority high"]
    HIGH = 1,
    #[doc = "0: Priority low"]
    LOW = 0,
}
impl From<PRIO_A> for bool {
    #[inline(always)]
    fn from(variant: PRIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIO` reader - 1:1\\]
Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme."]
pub struct PRIO_R(crate::FieldReader<bool, PRIO_A>);
impl PRIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRIO_A {
        match self.bits {
            true => PRIO_A::HIGH,
            false => PRIO_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PRIO_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PRIO_A::LOW
    }
}
impl core::ops::Deref for PRIO_R {
    type Target = crate::FieldReader<bool, PRIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIO` writer - 1:1\\]
Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme."]
pub struct PRIO_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRIO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Priority high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PRIO_A::HIGH)
    }
    #[doc = "Priority low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PRIO_A::LOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "0:0\\]
DMA Channel 0 Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "1: Channel enabled"]
    EN = 1,
    #[doc = "0: Channel disabled"]
    DIS = 0,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - 0:0\\]
DMA Channel 0 Control"]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            true => EN_A::EN,
            false => EN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == EN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == EN_A::DIS
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - 0:0\\]
DMA Channel 0 Control"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN_A::EN)
    }
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN_A::DIS)
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
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
DMA Channel 0 Control"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme."]
    #[inline(always)]
    pub fn prio(&mut self) -> PRIO_W {
        PRIO_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
DMA Channel 0 Control"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel 0 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach0ctl](index.html) module"]
pub struct DMACH0CTL_SPEC;
impl crate::RegisterSpec for DMACH0CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmach0ctl::R](R) reader structure"]
impl crate::Readable for DMACH0CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmach0ctl::W](W) writer structure"]
impl crate::Writable for DMACH0CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACH0CTL to value 0"]
impl crate::Resettable for DMACH0CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
