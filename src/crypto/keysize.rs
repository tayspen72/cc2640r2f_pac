#[doc = "Register `KEYSIZE` reader"]
pub struct R(crate::R<KEYSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYSIZE` writer"]
pub struct W(crate::W<KEYSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYSIZE_SPEC>;
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
impl From<crate::W<KEYSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_R(crate::FieldReader<u32, u32>);
impl RESERVED2_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "1:0\\]
Key size When writing to this register, KEYWRITTENAREA will be reset. Note: For the Crypto peripheral this field is fixed to 128 bits. For software compatibility KEYWRITTENAREA will be reset when writing to this register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "3: Not supported"]
    _256_BIT = 3,
    #[doc = "2: Not supported"]
    _192_BIT = 2,
    #[doc = "1: 128 bits"]
    _128_BIT = 1,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SIZE` reader - 1:0\\]
Key size When writing to this register, KEYWRITTENAREA will be reset. Note: For the Crypto peripheral this field is fixed to 128 bits. For software compatibility KEYWRITTENAREA will be reset when writing to this register."]
pub struct SIZE_R(crate::FieldReader<u8, SIZE_A>);
impl SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIZE_A> {
        match self.bits {
            3 => Some(SIZE_A::_256_BIT),
            2 => Some(SIZE_A::_192_BIT),
            1 => Some(SIZE_A::_128_BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_256_BIT`"]
    #[inline(always)]
    pub fn is_256_bit(&self) -> bool {
        **self == SIZE_A::_256_BIT
    }
    #[doc = "Checks if the value of the field is `_192_BIT`"]
    #[inline(always)]
    pub fn is_192_bit(&self) -> bool {
        **self == SIZE_A::_192_BIT
    }
    #[doc = "Checks if the value of the field is `_128_BIT`"]
    #[inline(always)]
    pub fn is_128_bit(&self) -> bool {
        **self == SIZE_A::_128_BIT
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<u8, SIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIZE` writer - 1:0\\]
Key size When writing to this register, KEYWRITTENAREA will be reset. Note: For the Crypto peripheral this field is fixed to 128 bits. For software compatibility KEYWRITTENAREA will be reset when writing to this register."]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn _256_bit(self) -> &'a mut W {
        self.variant(SIZE_A::_256_BIT)
    }
    #[doc = "Not supported"]
    #[inline(always)]
    pub fn _192_bit(self) -> &'a mut W {
        self.variant(SIZE_A::_192_BIT)
    }
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn _128_bit(self) -> &'a mut W {
        self.variant(SIZE_A::_128_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Key size When writing to this register, KEYWRITTENAREA will be reset. Note: For the Crypto peripheral this field is fixed to 128 bits. For software compatibility KEYWRITTENAREA will be reset when writing to this register."]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Key size When writing to this register, KEYWRITTENAREA will be reset. Note: For the Crypto peripheral this field is fixed to 128 bits. For software compatibility KEYWRITTENAREA will be reset when writing to this register."]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Size This register defines the size of the keys that are written with DMA.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keysize](index.html) module"]
pub struct KEYSIZE_SPEC;
impl crate::RegisterSpec for KEYSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keysize::R](R) reader structure"]
impl crate::Readable for KEYSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keysize::W](W) writer structure"]
impl crate::Writable for KEYSIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYSIZE to value 0x01"]
impl crate::Resettable for KEYSIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
