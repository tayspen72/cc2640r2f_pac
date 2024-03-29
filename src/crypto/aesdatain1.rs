#[doc = "Register `AESDATAIN1` reader"]
pub struct R(crate::R<AESDATAIN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESDATAIN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESDATAIN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESDATAIN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESDATAIN1` writer"]
pub struct W(crate::W<AESDATAIN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESDATAIN1_SPEC>;
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
impl From<crate::W<AESDATAIN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESDATAIN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - 31:0\\]
Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[63:32\\]
of \\[127:0\\]
For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 < n <= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
pub struct DATA_R(crate::FieldReader<u32, u32>);
impl DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - 31:0\\]
Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[63:32\\]
of \\[127:0\\]
For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 < n <= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[63:32\\]
of \\[127:0\\]
For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 < n <= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[63:32\\]
of \\[127:0\\]
For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 < n <= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Data Input/Output 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdatain1](index.html) module"]
pub struct AESDATAIN1_SPEC;
impl crate::RegisterSpec for AESDATAIN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aesdatain1::R](R) reader structure"]
impl crate::Readable for AESDATAIN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesdatain1::W](W) writer structure"]
impl crate::Writable for AESDATAIN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESDATAIN1 to value 0"]
impl crate::Resettable for AESDATAIN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
