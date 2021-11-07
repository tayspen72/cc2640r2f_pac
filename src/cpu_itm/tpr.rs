#[doc = "Register `TPR` reader"]
pub struct R(crate::R<TPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPR` writer"]
pub struct W(crate::W<TPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPR_SPEC>;
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
impl From<crate::W<TPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_R(crate::FieldReader<u32, u32>);
impl RESERVED4_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | ((value as u32 & 0x0fff_ffff) << 4);
        self.w
    }
}
#[doc = "Field `PRIVMASK` reader - 3:0\\]
Bit mask to enable unprivileged (User) access to ITM stimulus ports: Bit \\[0\\]
enables stimulus ports 0, 1, ..., and 7. Bit \\[1\\]
enables stimulus ports 8, 9, ..., and 15. Bit \\[2\\]
enables stimulus ports 16, 17, ..., and 23. Bit \\[3\\]
enables stimulus ports 24, 25, ..., and 31. 0: User access allowed to stimulus ports 1: Privileged access only to stimulus ports"]
pub struct PRIVMASK_R(crate::FieldReader<u8, u8>);
impl PRIVMASK_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRIVMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIVMASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIVMASK` writer - 3:0\\]
Bit mask to enable unprivileged (User) access to ITM stimulus ports: Bit \\[0\\]
enables stimulus ports 0, 1, ..., and 7. Bit \\[1\\]
enables stimulus ports 8, 9, ..., and 15. Bit \\[2\\]
enables stimulus ports 16, 17, ..., and 23. Bit \\[3\\]
enables stimulus ports 24, 25, ..., and 31. 0: User access allowed to stimulus ports 1: Privileged access only to stimulus ports"]
pub struct PRIVMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIVMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Bit mask to enable unprivileged (User) access to ITM stimulus ports: Bit \\[0\\]
enables stimulus ports 0, 1, ..., and 7. Bit \\[1\\]
enables stimulus ports 8, 9, ..., and 15. Bit \\[2\\]
enables stimulus ports 16, 17, ..., and 23. Bit \\[3\\]
enables stimulus ports 24, 25, ..., and 31. 0: User access allowed to stimulus ports 1: Privileged access only to stimulus ports"]
    #[inline(always)]
    pub fn privmask(&self) -> PRIVMASK_R {
        PRIVMASK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Bit mask to enable unprivileged (User) access to ITM stimulus ports: Bit \\[0\\]
enables stimulus ports 0, 1, ..., and 7. Bit \\[1\\]
enables stimulus ports 8, 9, ..., and 15. Bit \\[2\\]
enables stimulus ports 16, 17, ..., and 23. Bit \\[3\\]
enables stimulus ports 24, 25, ..., and 31. 0: User access allowed to stimulus ports 1: Privileged access only to stimulus ports"]
    #[inline(always)]
    pub fn privmask(&mut self) -> PRIVMASK_W {
        PRIVMASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trace Privilege This register is used to enable an operating system to control which stimulus ports are accessible by user code. This register can only be used in privileged mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpr](index.html) module"]
pub struct TPR_SPEC;
impl crate::RegisterSpec for TPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpr::R](R) reader structure"]
impl crate::Readable for TPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpr::W](W) writer structure"]
impl crate::Writable for TPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPR to value 0"]
impl crate::Resettable for TPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
