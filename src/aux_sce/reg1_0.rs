#[doc = "Register `REG1_0` reader"]
pub struct R(crate::R<REG1_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG1_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG1_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG1_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG1_0` writer"]
pub struct W(crate::W<REG1_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG1_0_SPEC>;
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
impl From<crate::W<REG1_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG1_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG1` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct REG1_R(crate::FieldReader<u16, u16>);
impl REG1_R {
    pub(crate) fn new(bits: u16) -> Self {
        REG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG1` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct REG1_W<'a> {
    w: &'a mut W,
}
impl<'a> REG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `REG0` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct REG0_R(crate::FieldReader<u16, u16>);
impl REG0_R {
    pub(crate) fn new(bits: u16) -> Self {
        REG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG0` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct REG0_W<'a> {
    w: &'a mut W,
}
impl<'a> REG0_W<'a> {
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
    pub fn reg1(&self) -> REG1_R {
        REG1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg0(&self) -> REG0_R {
        REG0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg1(&mut self) -> REG1_W {
        REG1_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg0(&mut self) -> REG0_W {
        REG0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg1_0](index.html) module"]
pub struct REG1_0_SPEC;
impl crate::RegisterSpec for REG1_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg1_0::R](R) reader structure"]
impl crate::Readable for REG1_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg1_0::W](W) writer structure"]
impl crate::Writable for REG1_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG1_0 to value 0"]
impl crate::Resettable for REG1_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
