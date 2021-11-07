#[doc = "Register `RFHWIEN` reader"]
pub struct R(crate::R<RFHWIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFHWIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFHWIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFHWIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFHWIEN` writer"]
pub struct W(crate::W<RFHWIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFHWIEN_SPEC>;
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
impl From<crate::W<RFHWIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFHWIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED20` reader - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED20_R(crate::FieldReader<u16, u16>);
impl RESERVED20_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED20_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED20` writer - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED20_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | ((value as u32 & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Field `RATCH7` reader - 19:19\\]
Interrupt enable for RFHWIFG.RATCH7."]
pub struct RATCH7_R(crate::FieldReader<bool, bool>);
impl RATCH7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RATCH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RATCH7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RATCH7` writer - 19:19\\]
Interrupt enable for RFHWIFG.RATCH7."]
pub struct RATCH7_W<'a> {
    w: &'a mut W,
}
impl<'a> RATCH7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `RATCH6` reader - 18:18\\]
Interrupt enable for RFHWIFG.RATCH6."]
pub struct RATCH6_R(crate::FieldReader<bool, bool>);
impl RATCH6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RATCH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RATCH6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RATCH6` writer - 18:18\\]
Interrupt enable for RFHWIFG.RATCH6."]
pub struct RATCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> RATCH6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `RATCH5` reader - 17:17\\]
Interrupt enable for RFHWIFG.RATCH5."]
pub struct RATCH5_R(crate::FieldReader<bool, bool>);
impl RATCH5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RATCH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RATCH5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RATCH5` writer - 17:17\\]
Interrupt enable for RFHWIFG.RATCH5."]
pub struct RATCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> RATCH5_W<'a> {
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
#[doc = "Field `RATCH4` reader - 16:16\\]
Interrupt enable for RFHWIFG.RATCH4."]
pub struct RATCH4_R(crate::FieldReader<bool, bool>);
impl RATCH4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RATCH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RATCH4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RATCH4` writer - 16:16\\]
Interrupt enable for RFHWIFG.RATCH4."]
pub struct RATCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> RATCH4_W<'a> {
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
#[doc = "Field `RATCH3` reader - 15:15\\]
Interrupt enable for RFHWIFG.RATCH3."]
pub struct RATCH3_R(crate::FieldReader<bool, bool>);
impl RATCH3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RATCH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RATCH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RATCH3` writer - 15:15\\]
Interrupt enable for RFHWIFG.RATCH3."]
pub struct RATCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RATCH3_W<'a> {
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
#[doc = "Field `RATCH2` reader - 14:14\\]
Interrupt enable for RFHWIFG.RATCH2."]
pub struct RATCH2_R(crate::FieldReader<bool, bool>);
impl RATCH2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RATCH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RATCH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RATCH2` writer - 14:14\\]
Interrupt enable for RFHWIFG.RATCH2."]
pub struct RATCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> RATCH2_W<'a> {
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
#[doc = "Field `RATCH1` reader - 13:13\\]
Interrupt enable for RFHWIFG.RATCH1."]
pub struct RATCH1_R(crate::FieldReader<bool, bool>);
impl RATCH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RATCH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RATCH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RATCH1` writer - 13:13\\]
Interrupt enable for RFHWIFG.RATCH1."]
pub struct RATCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RATCH1_W<'a> {
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
#[doc = "Field `RATCH0` reader - 12:12\\]
Interrupt enable for RFHWIFG.RATCH0."]
pub struct RATCH0_R(crate::FieldReader<bool, bool>);
impl RATCH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RATCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RATCH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RATCH0` writer - 12:12\\]
Interrupt enable for RFHWIFG.RATCH0."]
pub struct RATCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RATCH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `RFESOFT2` reader - 11:11\\]
Interrupt enable for RFHWIFG.RFESOFT2."]
pub struct RFESOFT2_R(crate::FieldReader<bool, bool>);
impl RFESOFT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFESOFT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFESOFT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFESOFT2` writer - 11:11\\]
Interrupt enable for RFHWIFG.RFESOFT2."]
pub struct RFESOFT2_W<'a> {
    w: &'a mut W,
}
impl<'a> RFESOFT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `RFESOFT1` reader - 10:10\\]
Interrupt enable for RFHWIFG.RFESOFT1."]
pub struct RFESOFT1_R(crate::FieldReader<bool, bool>);
impl RFESOFT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFESOFT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFESOFT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFESOFT1` writer - 10:10\\]
Interrupt enable for RFHWIFG.RFESOFT1."]
pub struct RFESOFT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RFESOFT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `RFESOFT0` reader - 9:9\\]
Interrupt enable for RFHWIFG.RFESOFT0."]
pub struct RFESOFT0_R(crate::FieldReader<bool, bool>);
impl RFESOFT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFESOFT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFESOFT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFESOFT0` writer - 9:9\\]
Interrupt enable for RFHWIFG.RFESOFT0."]
pub struct RFESOFT0_W<'a> {
    w: &'a mut W,
}
impl<'a> RFESOFT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `RFEDONE` reader - 8:8\\]
Interrupt enable for RFHWIFG.RFEDONE."]
pub struct RFEDONE_R(crate::FieldReader<bool, bool>);
impl RFEDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFEDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFEDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFEDONE` writer - 8:8\\]
Interrupt enable for RFHWIFG.RFEDONE."]
pub struct RFEDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFEDONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `TRCTK` reader - 6:6\\]
Interrupt enable for RFHWIFG.TRCTK."]
pub struct TRCTK_R(crate::FieldReader<bool, bool>);
impl TRCTK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRCTK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRCTK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRCTK` writer - 6:6\\]
Interrupt enable for RFHWIFG.TRCTK."]
pub struct TRCTK_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCTK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `MDMSOFT` reader - 5:5\\]
Interrupt enable for RFHWIFG.MDMSOFT."]
pub struct MDMSOFT_R(crate::FieldReader<bool, bool>);
impl MDMSOFT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDMSOFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDMSOFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDMSOFT` writer - 5:5\\]
Interrupt enable for RFHWIFG.MDMSOFT."]
pub struct MDMSOFT_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMSOFT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `MDMOUT` reader - 4:4\\]
Interrupt enable for RFHWIFG.MDMOUT."]
pub struct MDMOUT_R(crate::FieldReader<bool, bool>);
impl MDMOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDMOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDMOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDMOUT` writer - 4:4\\]
Interrupt enable for RFHWIFG.MDMOUT."]
pub struct MDMOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `MDMIN` reader - 3:3\\]
Interrupt enable for RFHWIFG.MDMIN."]
pub struct MDMIN_R(crate::FieldReader<bool, bool>);
impl MDMIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDMIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDMIN` writer - 3:3\\]
Interrupt enable for RFHWIFG.MDMIN."]
pub struct MDMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMIN_W<'a> {
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
#[doc = "Field `MDMDONE` reader - 2:2\\]
Interrupt enable for RFHWIFG.MDMDONE."]
pub struct MDMDONE_R(crate::FieldReader<bool, bool>);
impl MDMDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDMDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDMDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDMDONE` writer - 2:2\\]
Interrupt enable for RFHWIFG.MDMDONE."]
pub struct MDMDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMDONE_W<'a> {
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
#[doc = "Field `FSCA` reader - 1:1\\]
Interrupt enable for RFHWIFG.FSCA."]
pub struct FSCA_R(crate::FieldReader<bool, bool>);
impl FSCA_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSCA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSCA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSCA` writer - 1:1\\]
Interrupt enable for RFHWIFG.FSCA."]
pub struct FSCA_W<'a> {
    w: &'a mut W,
}
impl<'a> FSCA_W<'a> {
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
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_R(crate::FieldReader<bool, bool>);
impl RESERVED0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
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
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt enable for RFHWIFG.RATCH7."]
    #[inline(always)]
    pub fn ratch7(&self) -> RATCH7_R {
        RATCH7_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt enable for RFHWIFG.RATCH6."]
    #[inline(always)]
    pub fn ratch6(&self) -> RATCH6_R {
        RATCH6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt enable for RFHWIFG.RATCH5."]
    #[inline(always)]
    pub fn ratch5(&self) -> RATCH5_R {
        RATCH5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt enable for RFHWIFG.RATCH4."]
    #[inline(always)]
    pub fn ratch4(&self) -> RATCH4_R {
        RATCH4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt enable for RFHWIFG.RATCH3."]
    #[inline(always)]
    pub fn ratch3(&self) -> RATCH3_R {
        RATCH3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt enable for RFHWIFG.RATCH2."]
    #[inline(always)]
    pub fn ratch2(&self) -> RATCH2_R {
        RATCH2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt enable for RFHWIFG.RATCH1."]
    #[inline(always)]
    pub fn ratch1(&self) -> RATCH1_R {
        RATCH1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt enable for RFHWIFG.RATCH0."]
    #[inline(always)]
    pub fn ratch0(&self) -> RATCH0_R {
        RATCH0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt enable for RFHWIFG.RFESOFT2."]
    #[inline(always)]
    pub fn rfesoft2(&self) -> RFESOFT2_R {
        RFESOFT2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt enable for RFHWIFG.RFESOFT1."]
    #[inline(always)]
    pub fn rfesoft1(&self) -> RFESOFT1_R {
        RFESOFT1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt enable for RFHWIFG.RFESOFT0."]
    #[inline(always)]
    pub fn rfesoft0(&self) -> RFESOFT0_R {
        RFESOFT0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt enable for RFHWIFG.RFEDONE."]
    #[inline(always)]
    pub fn rfedone(&self) -> RFEDONE_R {
        RFEDONE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt enable for RFHWIFG.TRCTK."]
    #[inline(always)]
    pub fn trctk(&self) -> TRCTK_R {
        TRCTK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt enable for RFHWIFG.MDMSOFT."]
    #[inline(always)]
    pub fn mdmsoft(&self) -> MDMSOFT_R {
        MDMSOFT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt enable for RFHWIFG.MDMOUT."]
    #[inline(always)]
    pub fn mdmout(&self) -> MDMOUT_R {
        MDMOUT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt enable for RFHWIFG.MDMIN."]
    #[inline(always)]
    pub fn mdmin(&self) -> MDMIN_R {
        MDMIN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt enable for RFHWIFG.MDMDONE."]
    #[inline(always)]
    pub fn mdmdone(&self) -> MDMDONE_R {
        MDMDONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt enable for RFHWIFG.FSCA."]
    #[inline(always)]
    pub fn fsca(&self) -> FSCA_R {
        FSCA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt enable for RFHWIFG.RATCH7."]
    #[inline(always)]
    pub fn ratch7(&mut self) -> RATCH7_W {
        RATCH7_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt enable for RFHWIFG.RATCH6."]
    #[inline(always)]
    pub fn ratch6(&mut self) -> RATCH6_W {
        RATCH6_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt enable for RFHWIFG.RATCH5."]
    #[inline(always)]
    pub fn ratch5(&mut self) -> RATCH5_W {
        RATCH5_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt enable for RFHWIFG.RATCH4."]
    #[inline(always)]
    pub fn ratch4(&mut self) -> RATCH4_W {
        RATCH4_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt enable for RFHWIFG.RATCH3."]
    #[inline(always)]
    pub fn ratch3(&mut self) -> RATCH3_W {
        RATCH3_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt enable for RFHWIFG.RATCH2."]
    #[inline(always)]
    pub fn ratch2(&mut self) -> RATCH2_W {
        RATCH2_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt enable for RFHWIFG.RATCH1."]
    #[inline(always)]
    pub fn ratch1(&mut self) -> RATCH1_W {
        RATCH1_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt enable for RFHWIFG.RATCH0."]
    #[inline(always)]
    pub fn ratch0(&mut self) -> RATCH0_W {
        RATCH0_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt enable for RFHWIFG.RFESOFT2."]
    #[inline(always)]
    pub fn rfesoft2(&mut self) -> RFESOFT2_W {
        RFESOFT2_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt enable for RFHWIFG.RFESOFT1."]
    #[inline(always)]
    pub fn rfesoft1(&mut self) -> RFESOFT1_W {
        RFESOFT1_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt enable for RFHWIFG.RFESOFT0."]
    #[inline(always)]
    pub fn rfesoft0(&mut self) -> RFESOFT0_W {
        RFESOFT0_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt enable for RFHWIFG.RFEDONE."]
    #[inline(always)]
    pub fn rfedone(&mut self) -> RFEDONE_W {
        RFEDONE_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt enable for RFHWIFG.TRCTK."]
    #[inline(always)]
    pub fn trctk(&mut self) -> TRCTK_W {
        TRCTK_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt enable for RFHWIFG.MDMSOFT."]
    #[inline(always)]
    pub fn mdmsoft(&mut self) -> MDMSOFT_W {
        MDMSOFT_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt enable for RFHWIFG.MDMOUT."]
    #[inline(always)]
    pub fn mdmout(&mut self) -> MDMOUT_W {
        MDMOUT_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt enable for RFHWIFG.MDMIN."]
    #[inline(always)]
    pub fn mdmin(&mut self) -> MDMIN_W {
        MDMIN_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt enable for RFHWIFG.MDMDONE."]
    #[inline(always)]
    pub fn mdmdone(&mut self) -> MDMDONE_W {
        MDMDONE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt enable for RFHWIFG.FSCA."]
    #[inline(always)]
    pub fn fsca(&mut self) -> FSCA_W {
        FSCA_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable For RF Hardware Modules\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfhwien](index.html) module"]
pub struct RFHWIEN_SPEC;
impl crate::RegisterSpec for RFHWIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfhwien::R](R) reader structure"]
impl crate::Readable for RFHWIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfhwien::W](W) writer structure"]
impl crate::Writable for RFHWIEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFHWIEN to value 0"]
impl crate::Resettable for RFHWIEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
