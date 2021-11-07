#[doc = "Register `FSPRD` reader"]
pub struct R(crate::R<FSPRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSPRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSPRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSPRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSPRD` writer"]
pub struct W(crate::W<FSPRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSPRD_SPEC>;
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
impl From<crate::W<FSPRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSPRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS_PREEMPT` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_PREEMPT_R(crate::FieldReader<u16, u16>);
impl DIS_PREEMPT_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIS_PREEMPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_PREEMPT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_PREEMPT` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct DIS_PREEMPT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_PREEMPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `RMBSEM` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct RMBSEM_R(crate::FieldReader<u8, u8>);
impl RMBSEM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RMBSEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMBSEM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMBSEM` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct RMBSEM_W<'a> {
    w: &'a mut W,
}
impl<'a> RMBSEM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `RESERVED2` reader - 7:2\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED2_R(crate::FieldReader<u8, u8>);
impl RESERVED2_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED2` writer - 7:2\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | ((value as u32 & 0x3f) << 2);
        self.w
    }
}
#[doc = "Field `RM1` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub struct RM1_R(crate::FieldReader<bool, bool>);
impl RM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RM1` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub struct RM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RM1_W<'a> {
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
#[doc = "Field `RM0` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct RM0_R(crate::FieldReader<bool, bool>);
impl RM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RM0` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct RM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RM0_W<'a> {
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
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_preempt(&self) -> DIS_PREEMPT_R {
        DIS_PREEMPT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rmbsem(&self) -> RMBSEM_R {
        RMBSEM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rm1(&self) -> RM1_R {
        RM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rm0(&self) -> RM0_R {
        RM0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_preempt(&mut self) -> DIS_PREEMPT_W {
        DIS_PREEMPT_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rmbsem(&mut self) -> RMBSEM_W {
        RMBSEM_W { w: self }
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rm1(&mut self) -> RM1_W {
        RM1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rm0(&mut self) -> RM0_W {
        RM0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsprd](index.html) module"]
pub struct FSPRD_SPEC;
impl crate::RegisterSpec for FSPRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsprd::R](R) reader structure"]
impl crate::Readable for FSPRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsprd::W](W) writer structure"]
impl crate::Writable for FSPRD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSPRD to value 0"]
impl crate::Resettable for FSPRD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
