#[doc = "Register `MCUCFG` reader"]
pub struct R(crate::R<MCUCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUCFG` writer"]
pub struct W(crate::W<MCUCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUCFG_SPEC>;
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
impl From<crate::W<MCUCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED18` reader - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED18_R(crate::FieldReader<u16, u16>);
impl RESERVED18_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED18_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED18` writer - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED18_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 18)) | ((value as u32 & 0x3fff) << 18);
        self.w
    }
}
#[doc = "Field `VIRT_OFF` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub struct VIRT_OFF_R(crate::FieldReader<bool, bool>);
impl VIRT_OFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        VIRT_OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VIRT_OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VIRT_OFF` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub struct VIRT_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> VIRT_OFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `FIXED_WU_EN` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub struct FIXED_WU_EN_R(crate::FieldReader<bool, bool>);
impl FIXED_WU_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIXED_WU_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIXED_WU_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIXED_WU_EN` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub struct FIXED_WU_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIXED_WU_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED4` reader - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_R(crate::FieldReader<u16, u16>);
impl RESERVED4_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED4_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED4` writer - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | ((value as u32 & 0x0fff) << 4);
        self.w
    }
}
#[doc = "3:0\\]
MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAM_RET_EN_A {
    #[doc = "15: Retention on for all banks (SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2 and SRAM:BANK3)"]
    RET_FULL = 15,
    #[doc = "7: Retention on for SRAM:BANK0, SRAM:BANK1 and SRAM:BANK2"]
    RET_LEVEL3 = 7,
    #[doc = "3: Retention on for SRAM:BANK0 and SRAM:BANK1"]
    RET_LEVEL2 = 3,
    #[doc = "1: Retention on for SRAM:BANK0"]
    RET_LEVEL1 = 1,
    #[doc = "0: Retention is disabled"]
    RET_NONE = 0,
}
impl From<SRAM_RET_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAM_RET_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRAM_RET_EN` reader - 3:0\\]
MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off"]
pub struct SRAM_RET_EN_R(crate::FieldReader<u8, SRAM_RET_EN_A>);
impl SRAM_RET_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_RET_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRAM_RET_EN_A> {
        match self.bits {
            15 => Some(SRAM_RET_EN_A::RET_FULL),
            7 => Some(SRAM_RET_EN_A::RET_LEVEL3),
            3 => Some(SRAM_RET_EN_A::RET_LEVEL2),
            1 => Some(SRAM_RET_EN_A::RET_LEVEL1),
            0 => Some(SRAM_RET_EN_A::RET_NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RET_FULL`"]
    #[inline(always)]
    pub fn is_ret_full(&self) -> bool {
        **self == SRAM_RET_EN_A::RET_FULL
    }
    #[doc = "Checks if the value of the field is `RET_LEVEL3`"]
    #[inline(always)]
    pub fn is_ret_level3(&self) -> bool {
        **self == SRAM_RET_EN_A::RET_LEVEL3
    }
    #[doc = "Checks if the value of the field is `RET_LEVEL2`"]
    #[inline(always)]
    pub fn is_ret_level2(&self) -> bool {
        **self == SRAM_RET_EN_A::RET_LEVEL2
    }
    #[doc = "Checks if the value of the field is `RET_LEVEL1`"]
    #[inline(always)]
    pub fn is_ret_level1(&self) -> bool {
        **self == SRAM_RET_EN_A::RET_LEVEL1
    }
    #[doc = "Checks if the value of the field is `RET_NONE`"]
    #[inline(always)]
    pub fn is_ret_none(&self) -> bool {
        **self == SRAM_RET_EN_A::RET_NONE
    }
}
impl core::ops::Deref for SRAM_RET_EN_R {
    type Target = crate::FieldReader<u8, SRAM_RET_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_RET_EN` writer - 3:0\\]
MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off"]
pub struct SRAM_RET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_RET_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_RET_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Retention on for all banks (SRAM:BANK0, SRAM:BANK1 ,SRAM:BANK2 and SRAM:BANK3)"]
    #[inline(always)]
    pub fn ret_full(self) -> &'a mut W {
        self.variant(SRAM_RET_EN_A::RET_FULL)
    }
    #[doc = "Retention on for SRAM:BANK0, SRAM:BANK1 and SRAM:BANK2"]
    #[inline(always)]
    pub fn ret_level3(self) -> &'a mut W {
        self.variant(SRAM_RET_EN_A::RET_LEVEL3)
    }
    #[doc = "Retention on for SRAM:BANK0 and SRAM:BANK1"]
    #[inline(always)]
    pub fn ret_level2(self) -> &'a mut W {
        self.variant(SRAM_RET_EN_A::RET_LEVEL2)
    }
    #[doc = "Retention on for SRAM:BANK0"]
    #[inline(always)]
    pub fn ret_level1(self) -> &'a mut W {
        self.variant(SRAM_RET_EN_A::RET_LEVEL1)
    }
    #[doc = "Retention is disabled"]
    #[inline(always)]
    pub fn ret_none(self) -> &'a mut W {
        self.variant(SRAM_RET_EN_A::RET_NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn virt_off(&self) -> VIRT_OFF_R {
        VIRT_OFF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fixed_wu_en(&self) -> FIXED_WU_EN_R {
        FIXED_WU_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - 3:0\\]
MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off"]
    #[inline(always)]
    pub fn sram_ret_en(&self) -> SRAM_RET_EN_R {
        SRAM_RET_EN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&mut self) -> RESERVED18_W {
        RESERVED18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn virt_off(&mut self) -> VIRT_OFF_W {
        VIRT_OFF_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fixed_wu_en(&mut self) -> FIXED_WU_EN_W {
        FIXED_WU_EN_W { w: self }
    }
    #[doc = "Bits 4:15 - 15:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off"]
    #[inline(always)]
    pub fn sram_ret_en(&mut self) -> SRAM_RET_EN_W {
        SRAM_RET_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCU Configuration This register contains power management related bitfields for the MCU domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcucfg](index.html) module"]
pub struct MCUCFG_SPEC;
impl crate::RegisterSpec for MCUCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcucfg::R](R) reader structure"]
impl crate::Readable for MCUCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcucfg::W](W) writer structure"]
impl crate::Writable for MCUCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCUCFG to value 0x0f"]
impl crate::Resettable for MCUCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
