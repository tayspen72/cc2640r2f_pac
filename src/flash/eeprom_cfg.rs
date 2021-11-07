#[doc = "Register `EEPROM_CFG` reader"]
pub struct R(crate::R<EEPROM_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEPROM_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEPROM_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEPROM_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEPROM_CFG` writer"]
pub struct W(crate::W<EEPROM_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEPROM_CFG_SPEC>;
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
impl From<crate::W<EEPROM_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEPROM_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTOSTART_GRACE` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct AUTOSTART_GRACE_R(crate::FieldReader<u32, u32>);
impl AUTOSTART_GRACE_R {
    pub(crate) fn new(bits: u32) -> Self {
        AUTOSTART_GRACE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOSTART_GRACE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOSTART_GRACE` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct AUTOSTART_GRACE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSTART_GRACE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn autostart_grace(&self) -> AUTOSTART_GRACE_R {
        AUTOSTART_GRACE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn autostart_grace(&mut self) -> AUTOSTART_GRACE_W {
        AUTOSTART_GRACE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eeprom_cfg](index.html) module"]
pub struct EEPROM_CFG_SPEC;
impl crate::RegisterSpec for EEPROM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eeprom_cfg::R](R) reader structure"]
impl crate::Readable for EEPROM_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eeprom_cfg::W](W) writer structure"]
impl crate::Writable for EEPROM_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEPROM_CFG to value 0x0001_0000"]
impl crate::Resettable for EEPROM_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
    }
}
