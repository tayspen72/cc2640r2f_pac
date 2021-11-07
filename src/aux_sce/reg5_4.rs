#[doc = "Register `REG5_4` reader"]
pub struct R(crate::R<REG5_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG5_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG5_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG5_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG5_4` writer"]
pub struct W(crate::W<REG5_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG5_4_SPEC>;
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
impl From<crate::W<REG5_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG5_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG5` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct REG5_R(crate::FieldReader<u16, u16>);
impl REG5_R {
    pub(crate) fn new(bits: u16) -> Self {
        REG5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG5_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG5` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct REG5_W<'a> {
    w: &'a mut W,
}
impl<'a> REG5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `REG4` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct REG4_R(crate::FieldReader<u16, u16>);
impl REG4_R {
    pub(crate) fn new(bits: u16) -> Self {
        REG4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG4_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG4` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct REG4_W<'a> {
    w: &'a mut W,
}
impl<'a> REG4_W<'a> {
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
    pub fn reg5(&self) -> REG5_R {
        REG5_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg4(&self) -> REG4_R {
        REG4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg5(&mut self) -> REG5_W {
        REG5_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reg4(&mut self) -> REG4_W {
        REG4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg5_4](index.html) module"]
pub struct REG5_4_SPEC;
impl crate::RegisterSpec for REG5_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg5_4::R](R) reader structure"]
impl crate::Readable for REG5_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg5_4::W](W) writer structure"]
impl crate::Writable for REG5_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG5_4 to value 0"]
impl crate::Resettable for REG5_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
