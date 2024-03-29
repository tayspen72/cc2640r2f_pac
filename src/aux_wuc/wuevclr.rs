#[doc = "Register `WUEVCLR` reader"]
pub struct R(crate::R<WUEVCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUEVCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUEVCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUEVCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUEVCLR` writer"]
pub struct W(crate::W<WUEVCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUEVCLR_SPEC>;
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
impl From<crate::W<WUEVCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUEVCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AON_RTC_CH2` reader - 2:2\\]
Set to clear the WUEVFLAGS.AON_RTC_CH2 wake-up event. Note that if RTC channel 2 is also set as source for AON_PROG_WU this field can also clear WUEVFLAGS.AON_PROG_WU This bit must remain set until WUEVFLAGS.AON_RTC_CH2 returns to 0."]
pub struct AON_RTC_CH2_R(crate::FieldReader<bool, bool>);
impl AON_RTC_CH2_R {
    pub(crate) fn new(bits: bool) -> Self {
        AON_RTC_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AON_RTC_CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AON_RTC_CH2` writer - 2:2\\]
Set to clear the WUEVFLAGS.AON_RTC_CH2 wake-up event. Note that if RTC channel 2 is also set as source for AON_PROG_WU this field can also clear WUEVFLAGS.AON_PROG_WU This bit must remain set until WUEVFLAGS.AON_RTC_CH2 returns to 0."]
pub struct AON_RTC_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_RTC_CH2_W<'a> {
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
#[doc = "Field `AON_SW` reader - 1:1\\]
Set to clear the WUEVFLAGS.AON_SW wake-up event. This bit must remain set until WUEVFLAGS.AON_SW returns to 0."]
pub struct AON_SW_R(crate::FieldReader<bool, bool>);
impl AON_SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        AON_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AON_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AON_SW` writer - 1:1\\]
Set to clear the WUEVFLAGS.AON_SW wake-up event. This bit must remain set until WUEVFLAGS.AON_SW returns to 0."]
pub struct AON_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_SW_W<'a> {
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
#[doc = "Field `AON_PROG_WU` reader - 0:0\\]
Set to clear the WUEVFLAGS.AON_PROG_WU wake-up event. Note only if an IO event is selected as wake-up event, is it possible to use this field to clear the source. Other sources cannot be cleared using this field. The IO pin needs to be assigned to AUX in the IOC and the input enable for the pin needs to be set in AIODIO0 or AIODIO1 for this clearing to take effect. This bit must remain set until WUEVFLAGS.AON_PROG_WU returns to 0."]
pub struct AON_PROG_WU_R(crate::FieldReader<bool, bool>);
impl AON_PROG_WU_R {
    pub(crate) fn new(bits: bool) -> Self {
        AON_PROG_WU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AON_PROG_WU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AON_PROG_WU` writer - 0:0\\]
Set to clear the WUEVFLAGS.AON_PROG_WU wake-up event. Note only if an IO event is selected as wake-up event, is it possible to use this field to clear the source. Other sources cannot be cleared using this field. The IO pin needs to be assigned to AUX in the IOC and the input enable for the pin needs to be set in AIODIO0 or AIODIO1 for this clearing to take effect. This bit must remain set until WUEVFLAGS.AON_PROG_WU returns to 0."]
pub struct AON_PROG_WU_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_PROG_WU_W<'a> {
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
    #[doc = "Bit 2 - 2:2\\]
Set to clear the WUEVFLAGS.AON_RTC_CH2 wake-up event. Note that if RTC channel 2 is also set as source for AON_PROG_WU this field can also clear WUEVFLAGS.AON_PROG_WU This bit must remain set until WUEVFLAGS.AON_RTC_CH2 returns to 0."]
    #[inline(always)]
    pub fn aon_rtc_ch2(&self) -> AON_RTC_CH2_R {
        AON_RTC_CH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set to clear the WUEVFLAGS.AON_SW wake-up event. This bit must remain set until WUEVFLAGS.AON_SW returns to 0."]
    #[inline(always)]
    pub fn aon_sw(&self) -> AON_SW_R {
        AON_SW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Set to clear the WUEVFLAGS.AON_PROG_WU wake-up event. Note only if an IO event is selected as wake-up event, is it possible to use this field to clear the source. Other sources cannot be cleared using this field. The IO pin needs to be assigned to AUX in the IOC and the input enable for the pin needs to be set in AIODIO0 or AIODIO1 for this clearing to take effect. This bit must remain set until WUEVFLAGS.AON_PROG_WU returns to 0."]
    #[inline(always)]
    pub fn aon_prog_wu(&self) -> AON_PROG_WU_R {
        AON_PROG_WU_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - 2:2\\]
Set to clear the WUEVFLAGS.AON_RTC_CH2 wake-up event. Note that if RTC channel 2 is also set as source for AON_PROG_WU this field can also clear WUEVFLAGS.AON_PROG_WU This bit must remain set until WUEVFLAGS.AON_RTC_CH2 returns to 0."]
    #[inline(always)]
    pub fn aon_rtc_ch2(&mut self) -> AON_RTC_CH2_W {
        AON_RTC_CH2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Set to clear the WUEVFLAGS.AON_SW wake-up event. This bit must remain set until WUEVFLAGS.AON_SW returns to 0."]
    #[inline(always)]
    pub fn aon_sw(&mut self) -> AON_SW_W {
        AON_SW_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Set to clear the WUEVFLAGS.AON_PROG_WU wake-up event. Note only if an IO event is selected as wake-up event, is it possible to use this field to clear the source. Other sources cannot be cleared using this field. The IO pin needs to be assigned to AUX in the IOC and the input enable for the pin needs to be set in AIODIO0 or AIODIO1 for this clearing to take effect. This bit must remain set until WUEVFLAGS.AON_PROG_WU returns to 0."]
    #[inline(always)]
    pub fn aon_prog_wu(&mut self) -> AON_PROG_WU_W {
        AON_PROG_WU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake-up Event Clear Clears wake-up events from the AON domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wuevclr](index.html) module"]
pub struct WUEVCLR_SPEC;
impl crate::RegisterSpec for WUEVCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wuevclr::R](R) reader structure"]
impl crate::Readable for WUEVCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wuevclr::W](W) writer structure"]
impl crate::Writable for WUEVCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WUEVCLR to value 0"]
impl crate::Resettable for WUEVCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
