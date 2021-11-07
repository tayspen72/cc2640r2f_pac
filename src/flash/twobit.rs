#[doc = "Register `TWOBIT` reader"]
pub struct R(crate::R<TWOBIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWOBIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWOBIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWOBIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWOBIT` writer"]
pub struct W(crate::W<TWOBIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWOBIT_SPEC>;
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
impl From<crate::W<TWOBIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWOBIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FROMN` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub struct FROMN_R(crate::FieldReader<u32, u32>);
impl FROMN_R {
    pub(crate) fn new(bits: u32) -> Self {
        FROMN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FROMN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FROMN` writer - 31:1\\]
Internal. Only to be used through TI provided API."]
pub struct FROMN_W<'a> {
    w: &'a mut W,
}
impl<'a> FROMN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | ((value as u32 & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Field `FROM0` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct FROM0_R(crate::FieldReader<bool, bool>);
impl FROM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FROM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FROM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FROM0` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct FROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> FROM0_W<'a> {
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
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fromn(&self) -> FROMN_R {
        FROMN_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn from0(&self) -> FROM0_R {
        FROM0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fromn(&mut self) -> FROMN_W {
        FROMN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn from0(&mut self) -> FROM0_W {
        FROM0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twobit](index.html) module"]
pub struct TWOBIT_SPEC;
impl crate::RegisterSpec for TWOBIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twobit::R](R) reader structure"]
impl crate::Readable for TWOBIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twobit::W](W) writer structure"]
impl crate::Writable for TWOBIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWOBIT to value 0"]
impl crate::Resettable for TWOBIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
