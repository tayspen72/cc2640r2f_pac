#[doc = "Register `REG3_2` reader"]
pub struct R(crate::R<REG3_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG3_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG3_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG3_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG3_2` writer"]
pub struct W(crate::W<REG3_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG3_2_SPEC>;
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
impl From<crate::W<REG3_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG3_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG3` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct REG3_R(crate::FieldReader<u16, u16>);
impl REG3_R {
    pub(crate) fn new(bits: u16) -> Self {
        REG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG3` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct REG3_W<'a> {
    w: &'a mut W,
}
impl<'a> REG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `REG2` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct REG2_R(crate::FieldReader<u16, u16>);
impl REG2_R {
    pub(crate) fn new(bits: u16) -> Self {
        REG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG2` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct REG2_W<'a> {
    w: &'a mut W,
}
impl<'a> REG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg3(&self) -> REG3_R {
        REG3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg2(&self) -> REG2_R {
        REG2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg3(&mut self) -> REG3_W {
        REG3_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg2(&mut self) -> REG2_W {
        REG2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg3_2](index.html) module"]
pub struct REG3_2_SPEC;
impl crate::RegisterSpec for REG3_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg3_2::R](R) reader structure"]
impl crate::Readable for REG3_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg3_2::W](W) writer structure"]
impl crate::Writable for REG3_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG3_2 to value 0"]
impl crate::Resettable for REG3_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
