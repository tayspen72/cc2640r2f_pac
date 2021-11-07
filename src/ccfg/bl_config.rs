#[doc = "Register `BL_CONFIG` reader"]
pub struct R(crate::R<BL_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BL_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BL_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BL_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BL_CONFIG` writer"]
pub struct W(crate::W<BL_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BL_CONFIG_SPEC>;
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
impl From<crate::W<BL_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BL_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOTLOADER_ENABLE` reader - 31:24\\]
Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is non-zero or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
pub struct BOOTLOADER_ENABLE_R(crate::FieldReader<u8, u8>);
impl BOOTLOADER_ENABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BOOTLOADER_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOTLOADER_ENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOTLOADER_ENABLE` writer - 31:24\\]
Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is non-zero or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
pub struct BOOTLOADER_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTLOADER_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `BL_LEVEL` reader - 16:16\\]
Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
pub struct BL_LEVEL_R(crate::FieldReader<bool, bool>);
impl BL_LEVEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BL_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BL_LEVEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BL_LEVEL` writer - 16:16\\]
Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
pub struct BL_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_LEVEL_W<'a> {
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
#[doc = "Field `BL_PIN_NUMBER` reader - 15:8\\]
DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
pub struct BL_PIN_NUMBER_R(crate::FieldReader<u8, u8>);
impl BL_PIN_NUMBER_R {
    pub(crate) fn new(bits: u8) -> Self {
        BL_PIN_NUMBER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BL_PIN_NUMBER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BL_PIN_NUMBER` writer - 15:8\\]
DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
pub struct BL_PIN_NUMBER_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_PIN_NUMBER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BL_ENABLE` reader - 7:0\\]
Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
pub struct BL_ENABLE_R(crate::FieldReader<u8, u8>);
impl BL_ENABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BL_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BL_ENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BL_ENABLE` writer - 7:0\\]
Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
pub struct BL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is non-zero or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
    #[inline(always)]
    pub fn bootloader_enable(&self) -> BOOTLOADER_ENABLE_R {
        BOOTLOADER_ENABLE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
    #[inline(always)]
    pub fn bl_level(&self) -> BL_LEVEL_R {
        BL_LEVEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
    #[inline(always)]
    pub fn bl_pin_number(&self) -> BL_PIN_NUMBER_R {
        BL_PIN_NUMBER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
    #[inline(always)]
    pub fn bl_enable(&self) -> BL_ENABLE_R {
        BL_ENABLE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is non-zero or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
    #[inline(always)]
    pub fn bootloader_enable(&mut self) -> BOOTLOADER_ENABLE_W {
        BOOTLOADER_ENABLE_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
    #[inline(always)]
    pub fn bl_level(&mut self) -> BL_LEVEL_W {
        BL_LEVEL_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
    #[inline(always)]
    pub fn bl_pin_number(&mut self) -> BL_PIN_NUMBER_W {
        BL_PIN_NUMBER_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
    #[inline(always)]
    pub fn bl_enable(&mut self) -> BL_ENABLE_W {
        BL_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bl_config](index.html) module"]
pub struct BL_CONFIG_SPEC;
impl crate::RegisterSpec for BL_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bl_config::R](R) reader structure"]
impl crate::Readable for BL_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bl_config::W](W) writer structure"]
impl crate::Writable for BL_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BL_CONFIG to value 0xc5ff_ffff"]
impl crate::Resettable for BL_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc5ff_ffff
    }
}
