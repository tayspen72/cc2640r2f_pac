#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_R(crate::FieldReader<u16, u16>);
impl RESERVED16_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED16_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `EFUSE_BLANK` reader - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
pub struct EFUSE_BLANK_R(crate::FieldReader<bool, bool>);
impl EFUSE_BLANK_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFUSE_BLANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFUSE_BLANK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFUSE_BLANK` writer - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
pub struct EFUSE_BLANK_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_BLANK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `EFUSE_TIMEOUT` reader - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
pub struct EFUSE_TIMEOUT_R(crate::FieldReader<bool, bool>);
impl EFUSE_TIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFUSE_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFUSE_TIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFUSE_TIMEOUT` writer - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
pub struct EFUSE_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_TIMEOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `EFUSE_CRC_ERROR` reader - 13:13\\]
Efuse scanning resulted in scan chain CRC error. 0 : No CRC error 1 : CRC Error"]
pub struct EFUSE_CRC_ERROR_R(crate::FieldReader<bool, bool>);
impl EFUSE_CRC_ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFUSE_CRC_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFUSE_CRC_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFUSE_CRC_ERROR` writer - 13:13\\]
Efuse scanning resulted in scan chain CRC error. 0 : No CRC error 1 : CRC Error"]
pub struct EFUSE_CRC_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_CRC_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `EFUSE_ERRCODE` reader - 12:8\\]
Same as EFUSEERROR.CODE"]
pub struct EFUSE_ERRCODE_R(crate::FieldReader<u8, u8>);
impl EFUSE_ERRCODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        EFUSE_ERRCODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFUSE_ERRCODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFUSE_ERRCODE` writer - 12:8\\]
Same as EFUSEERROR.CODE"]
pub struct EFUSE_ERRCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_ERRCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `RESERVED3` reader - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_R(crate::FieldReader<u8, u8>);
impl RESERVED3_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `SAMHOLD_DIS` reader - 2:2\\]
Status indicator of flash sample and hold sequencing logic. This bit will go to 1 some delay after CFG.DIS_IDLE is set to 1. 0: Not disabled 1: Sample and hold disabled and stable"]
pub struct SAMHOLD_DIS_R(crate::FieldReader<bool, bool>);
impl SAMHOLD_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAMHOLD_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMHOLD_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMHOLD_DIS` writer - 2:2\\]
Status indicator of flash sample and hold sequencing logic. This bit will go to 1 some delay after CFG.DIS_IDLE is set to 1. 0: Not disabled 1: Sample and hold disabled and stable"]
pub struct SAMHOLD_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMHOLD_DIS_W<'a> {
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
#[doc = "Field `BUSY` reader - 1:1\\]
Fast version of the FMC FMSTAT.BUSY bit. This flag is valid immediately after the operation setting it (FMSTAT.BUSY is delayed some cycles) 0 : Not busy 1 : Busy"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` writer - 1:1\\]
Fast version of the FMC FMSTAT.BUSY bit. This flag is valid immediately after the operation setting it (FMSTAT.BUSY is delayed some cycles) 0 : Not busy 1 : Busy"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
#[doc = "Field `POWER_MODE` reader - 0:0\\]
Power state of the flash sub-system. 0 : Active 1 : Low power"]
pub struct POWER_MODE_R(crate::FieldReader<bool, bool>);
impl POWER_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        POWER_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POWER_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POWER_MODE` writer - 0:0\\]
Power state of the flash sub-system. 0 : Active 1 : Low power"]
pub struct POWER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_MODE_W<'a> {
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
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
    #[inline(always)]
    pub fn efuse_blank(&self) -> EFUSE_BLANK_R {
        EFUSE_BLANK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
    #[inline(always)]
    pub fn efuse_timeout(&self) -> EFUSE_TIMEOUT_R {
        EFUSE_TIMEOUT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Efuse scanning resulted in scan chain CRC error. 0 : No CRC error 1 : CRC Error"]
    #[inline(always)]
    pub fn efuse_crc_error(&self) -> EFUSE_CRC_ERROR_R {
        EFUSE_CRC_ERROR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Same as EFUSEERROR.CODE"]
    #[inline(always)]
    pub fn efuse_errcode(&self) -> EFUSE_ERRCODE_R {
        EFUSE_ERRCODE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Status indicator of flash sample and hold sequencing logic. This bit will go to 1 some delay after CFG.DIS_IDLE is set to 1. 0: Not disabled 1: Sample and hold disabled and stable"]
    #[inline(always)]
    pub fn samhold_dis(&self) -> SAMHOLD_DIS_R {
        SAMHOLD_DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Fast version of the FMC FMSTAT.BUSY bit. This flag is valid immediately after the operation setting it (FMSTAT.BUSY is delayed some cycles) 0 : Not busy 1 : Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Power state of the flash sub-system. 0 : Active 1 : Low power"]
    #[inline(always)]
    pub fn power_mode(&self) -> POWER_MODE_R {
        POWER_MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
    #[inline(always)]
    pub fn efuse_blank(&mut self) -> EFUSE_BLANK_W {
        EFUSE_BLANK_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
    #[inline(always)]
    pub fn efuse_timeout(&mut self) -> EFUSE_TIMEOUT_W {
        EFUSE_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Efuse scanning resulted in scan chain CRC error. 0 : No CRC error 1 : CRC Error"]
    #[inline(always)]
    pub fn efuse_crc_error(&mut self) -> EFUSE_CRC_ERROR_W {
        EFUSE_CRC_ERROR_W { w: self }
    }
    #[doc = "Bits 8:12 - 12:8\\]
Same as EFUSEERROR.CODE"]
    #[inline(always)]
    pub fn efuse_errcode(&mut self) -> EFUSE_ERRCODE_W {
        EFUSE_ERRCODE_W { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Status indicator of flash sample and hold sequencing logic. This bit will go to 1 some delay after CFG.DIS_IDLE is set to 1. 0: Not disabled 1: Sample and hold disabled and stable"]
    #[inline(always)]
    pub fn samhold_dis(&mut self) -> SAMHOLD_DIS_W {
        SAMHOLD_DIS_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Fast version of the FMC FMSTAT.BUSY bit. This flag is valid immediately after the operation setting it (FMSTAT.BUSY is delayed some cycles) 0 : Not busy 1 : Busy"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Power state of the flash sub-system. 0 : Active 1 : Low power"]
    #[inline(always)]
    pub fn power_mode(&mut self) -> POWER_MODE_W {
        POWER_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC and Efuse Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
