#[doc = "Register `IMAGE_VALID_CONF` reader"]
pub struct R(crate::R<IMAGE_VALID_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMAGE_VALID_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMAGE_VALID_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMAGE_VALID_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMAGE_VALID_CONF` writer"]
pub struct W(crate::W<IMAGE_VALID_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMAGE_VALID_CONF_SPEC>;
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
impl From<crate::W<IMAGE_VALID_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMAGE_VALID_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMAGE_VALID` reader - 31:0\\]
This field must have a value of 0x00000000 in order for enabling the boot sequence to transfer control to a flash image. A non-zero value forces the boot sequence to call the boot loader. For CC2640R2: This field must have the address value of the start of the flash vector table in order for enabling the boot sequence to transfer control to a flash image. Any illegal vector table start address value forces the boot sequence to call the boot loader. Note that if any other legal vector table start address value than 0x0 is selected the PRCM:WARMRESET.WR_TO_PINRESET must be set to 1."]
pub struct IMAGE_VALID_R(crate::FieldReader<u32, u32>);
impl IMAGE_VALID_R {
    pub(crate) fn new(bits: u32) -> Self {
        IMAGE_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMAGE_VALID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMAGE_VALID` writer - 31:0\\]
This field must have a value of 0x00000000 in order for enabling the boot sequence to transfer control to a flash image. A non-zero value forces the boot sequence to call the boot loader. For CC2640R2: This field must have the address value of the start of the flash vector table in order for enabling the boot sequence to transfer control to a flash image. Any illegal vector table start address value forces the boot sequence to call the boot loader. Note that if any other legal vector table start address value than 0x0 is selected the PRCM:WARMRESET.WR_TO_PINRESET must be set to 1."]
pub struct IMAGE_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> IMAGE_VALID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This field must have a value of 0x00000000 in order for enabling the boot sequence to transfer control to a flash image. A non-zero value forces the boot sequence to call the boot loader. For CC2640R2: This field must have the address value of the start of the flash vector table in order for enabling the boot sequence to transfer control to a flash image. Any illegal vector table start address value forces the boot sequence to call the boot loader. Note that if any other legal vector table start address value than 0x0 is selected the PRCM:WARMRESET.WR_TO_PINRESET must be set to 1."]
    #[inline(always)]
    pub fn image_valid(&self) -> IMAGE_VALID_R {
        IMAGE_VALID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This field must have a value of 0x00000000 in order for enabling the boot sequence to transfer control to a flash image. A non-zero value forces the boot sequence to call the boot loader. For CC2640R2: This field must have the address value of the start of the flash vector table in order for enabling the boot sequence to transfer control to a flash image. Any illegal vector table start address value forces the boot sequence to call the boot loader. Note that if any other legal vector table start address value than 0x0 is selected the PRCM:WARMRESET.WR_TO_PINRESET must be set to 1."]
    #[inline(always)]
    pub fn image_valid(&mut self) -> IMAGE_VALID_W {
        IMAGE_VALID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Image Valid\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [image_valid_conf](index.html) module"]
pub struct IMAGE_VALID_CONF_SPEC;
impl crate::RegisterSpec for IMAGE_VALID_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [image_valid_conf::R](R) reader structure"]
impl crate::Readable for IMAGE_VALID_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [image_valid_conf::W](W) writer structure"]
impl crate::Writable for IMAGE_VALID_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMAGE_VALID_CONF to value 0xffff_ffff"]
impl crate::Resettable for IMAGE_VALID_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
