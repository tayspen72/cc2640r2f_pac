#[doc = "Register `ADCREF0` reader"]
pub struct R(crate::R<ADCREF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCREF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCREF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCREF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCREF0` writer"]
pub struct W(crate::W<ADCREF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCREF0_SPEC>;
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
impl From<crate::W<ADCREF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCREF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED7_R(crate::FieldReader<bool, bool>);
impl RESERVED7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `REF_ON_IDLE` reader - 6:6\\]
Keep ADCREF powered up in IDLE state when ADC0.SMPL_MODE = 0. Set to 1 if ADC0.SMPL_CYCLE_EXP is less than 6 (21.3us sampling time)"]
pub struct REF_ON_IDLE_R(crate::FieldReader<bool, bool>);
impl REF_ON_IDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REF_ON_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REF_ON_IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REF_ON_IDLE` writer - 6:6\\]
Keep ADCREF powered up in IDLE state when ADC0.SMPL_MODE = 0. Set to 1 if ADC0.SMPL_CYCLE_EXP is less than 6 (21.3us sampling time)"]
pub struct REF_ON_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_ON_IDLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `IOMUX` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub struct IOMUX_R(crate::FieldReader<bool, bool>);
impl IOMUX_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOMUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOMUX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOMUX` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub struct IOMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `EXT` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub struct EXT_R(crate::FieldReader<bool, bool>);
impl EXT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub struct EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `SRC` reader - 3:3\\]
ADC reference source: 0: Fixed reference = 4.3V 1: Relative reference = VDDS"]
pub struct SRC_R(crate::FieldReader<bool, bool>);
impl SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC` writer - 3:3\\]
ADC reference source: 0: Fixed reference = 4.3V 1: Relative reference = VDDS"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RESERVED1` reader - 2:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1_R(crate::FieldReader<u8, u8>);
impl RESERVED1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED1` writer - 2:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u8 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `EN` reader - 0:0\\]
ADC reference module enable: 0: ADC reference module powered down 1: ADC reference module enabled"]
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
#[doc = "Field `EN` writer - 0:0\\]
ADC reference module enable: 0: ADC reference module powered down 1: ADC reference module enabled"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Keep ADCREF powered up in IDLE state when ADC0.SMPL_MODE = 0. Set to 1 if ADC0.SMPL_CYCLE_EXP is less than 6 (21.3us sampling time)"]
    #[inline(always)]
    pub fn ref_on_idle(&self) -> REF_ON_IDLE_R {
        REF_ON_IDLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iomux(&self) -> IOMUX_R {
        IOMUX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
ADC reference source: 0: Fixed reference = 4.3V 1: Relative reference = VDDS"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
ADC reference module enable: 0: ADC reference module powered down 1: ADC reference module enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Keep ADCREF powered up in IDLE state when ADC0.SMPL_MODE = 0. Set to 1 if ADC0.SMPL_CYCLE_EXP is less than 6 (21.3us sampling time)"]
    #[inline(always)]
    pub fn ref_on_idle(&mut self) -> REF_ON_IDLE_W {
        REF_ON_IDLE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iomux(&mut self) -> IOMUX_W {
        IOMUX_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ext(&mut self) -> EXT_W {
        EXT_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
ADC reference source: 0: Fixed reference = 4.3V 1: Relative reference = VDDS"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Bits 1:2 - 2:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
ADC reference module enable: 0: ADC reference module powered down 1: ADC reference module enabled"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Reference 0 Control reference used by the ADC. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcref0](index.html) module"]
pub struct ADCREF0_SPEC;
impl crate::RegisterSpec for ADCREF0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adcref0::R](R) reader structure"]
impl crate::Readable for ADCREF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcref0::W](W) writer structure"]
impl crate::Writable for ADCREF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCREF0 to value 0"]
impl crate::Resettable for ADCREF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
