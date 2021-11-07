#[doc = "Register `MISC_CONF_1` reader"]
pub struct R(crate::R<MISC_CONF_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_CONF_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_CONF_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_CONF_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_CONF_1` writer"]
pub struct W(crate::W<MISC_CONF_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_CONF_1_SPEC>;
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
impl From<crate::W<MISC_CONF_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_CONF_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEVICE_MINOR_REV` reader - 7:0\\]
HW minor revision number (a value of 0xFF shall be treated equally to 0x00). Any test of this field by SW should be implemented as a 'greater or equal' comparison as signed integer. Value may change without warning."]
pub struct DEVICE_MINOR_REV_R(crate::FieldReader<u8, u8>);
impl DEVICE_MINOR_REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEVICE_MINOR_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVICE_MINOR_REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVICE_MINOR_REV` writer - 7:0\\]
HW minor revision number (a value of 0xFF shall be treated equally to 0x00). Any test of this field by SW should be implemented as a 'greater or equal' comparison as signed integer. Value may change without warning."]
pub struct DEVICE_MINOR_REV_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVICE_MINOR_REV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
HW minor revision number (a value of 0xFF shall be treated equally to 0x00). Any test of this field by SW should be implemented as a 'greater or equal' comparison as signed integer. Value may change without warning."]
    #[inline(always)]
    pub fn device_minor_rev(&self) -> DEVICE_MINOR_REV_R {
        DEVICE_MINOR_REV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
HW minor revision number (a value of 0xFF shall be treated equally to 0x00). Any test of this field by SW should be implemented as a 'greater or equal' comparison as signed integer. Value may change without warning."]
    #[inline(always)]
    pub fn device_minor_rev(&mut self) -> DEVICE_MINOR_REV_W {
        DEVICE_MINOR_REV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Misc configurations\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_conf_1](index.html) module"]
pub struct MISC_CONF_1_SPEC;
impl crate::RegisterSpec for MISC_CONF_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_conf_1::R](R) reader structure"]
impl crate::Readable for MISC_CONF_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_conf_1::W](W) writer structure"]
impl crate::Writable for MISC_CONF_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC_CONF_1 to value 0xffff_ff00"]
impl crate::Resettable for MISC_CONF_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ff00
    }
}
