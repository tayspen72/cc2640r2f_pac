#[doc = "Register `AESCTL` reader"]
pub struct R(crate::R<AESCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESCTL` writer"]
pub struct W(crate::W<AESCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESCTL_SPEC>;
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
impl From<crate::W<AESCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTEXT_RDY` reader - 31:31\\]
If 1, this status bit indicates that the context data registers can be overwritten and the Host is permitted to write the next context. Writing a context means writing either a mode, the crypto length or AESDATALEN1.LEN_MSW, AESDATALEN0.LEN_LSW length registers"]
pub struct CONTEXT_RDY_R(crate::FieldReader<bool, bool>);
impl CONTEXT_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONTEXT_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONTEXT_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONTEXT_RDY` writer - 31:31\\]
If 1, this status bit indicates that the context data registers can be overwritten and the Host is permitted to write the next context. Writing a context means writing either a mode, the crypto length or AESDATALEN1.LEN_MSW, AESDATALEN0.LEN_LSW length registers"]
pub struct CONTEXT_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTEXT_RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `SAVED_CONTEXT_RDY` reader - 30:30\\]
If read as 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if SAVE_CONTEXT is set to 1. The bit is mutually exclusive with CONTEXT_RDY. Writing 1 clears the bit to zero, indicating the Crypto peripheral can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes will be ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the Crypto peripheral for TAG read DMA operations. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub struct SAVED_CONTEXT_RDY_R(crate::FieldReader<bool, bool>);
impl SAVED_CONTEXT_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAVED_CONTEXT_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAVED_CONTEXT_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAVED_CONTEXT_RDY` writer - 30:30\\]
If read as 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if SAVE_CONTEXT is set to 1. The bit is mutually exclusive with CONTEXT_RDY. Writing 1 clears the bit to zero, indicating the Crypto peripheral can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes will be ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the Crypto peripheral for TAG read DMA operations. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub struct SAVED_CONTEXT_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SAVED_CONTEXT_RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `SAVE_CONTEXT` reader - 29:29\\]
IV must be read before the AES engine can start a new operation."]
pub struct SAVE_CONTEXT_R(crate::FieldReader<bool, bool>);
impl SAVE_CONTEXT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAVE_CONTEXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAVE_CONTEXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAVE_CONTEXT` writer - 29:29\\]
IV must be read before the AES engine can start a new operation."]
pub struct SAVE_CONTEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> SAVE_CONTEXT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `RESERVED25` reader - 28:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED25_R(crate::FieldReader<u8, u8>);
impl RESERVED25_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED25_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED25` writer - 28:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED25_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | ((value as u32 & 0x0f) << 25);
        self.w
    }
}
#[doc = "Field `CCM_M` reader - 24:22\\]
Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times the value of CCM_M plus one. Note: The Crypto peripheral always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
pub struct CCM_M_R(crate::FieldReader<u8, u8>);
impl CCM_M_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCM_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCM_M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCM_M` writer - 24:22\\]
Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times the value of CCM_M plus one. Note: The Crypto peripheral always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
pub struct CCM_M_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | ((value as u32 & 0x07) << 22);
        self.w
    }
}
#[doc = "Field `CCM_L` reader - 21:19\\]
Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM_L plus one. All values are supported."]
pub struct CCM_L_R(crate::FieldReader<u8, u8>);
impl CCM_L_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCM_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCM_L_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCM_L` writer - 21:19\\]
Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM_L plus one. All values are supported."]
pub struct CCM_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Field `CCM` reader - 18:18\\]
AES-CCM mode enable. AES-CCM is a combined mode, using AES for both authentication and encryption. Note: Selecting AES-CCM mode requires writing of AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
pub struct CCM_R(crate::FieldReader<bool, bool>);
impl CCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCM` writer - 18:18\\]
AES-CCM mode enable. AES-CCM is a combined mode, using AES for both authentication and encryption. Note: Selecting AES-CCM mode requires writing of AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
pub struct CCM_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CBC_MAC` reader - 15:15\\]
MAC mode enable. The DIR bit must be set to 1 for this mode. Selecting this mode requires writing the AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW registers after all other registers."]
pub struct CBC_MAC_R(crate::FieldReader<bool, bool>);
impl CBC_MAC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CBC_MAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBC_MAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBC_MAC` writer - 15:15\\]
MAC mode enable. The DIR bit must be set to 1 for this mode. Selecting this mode requires writing the AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW registers after all other registers."]
pub struct CBC_MAC_W<'a> {
    w: &'a mut W,
}
impl<'a> CBC_MAC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `RESERVED9` reader - 14:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED9_R(crate::FieldReader<u8, u8>);
impl RESERVED9_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED9` writer - 14:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 9)) | ((value as u32 & 0x3f) << 9);
        self.w
    }
}
#[doc = "8:7\\]
Specifies the counter width for AES-CTR mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTR_WIDTH_A {
    #[doc = "3: 128 bits"]
    _128_BIT = 3,
    #[doc = "2: 96 bits"]
    _96_BIT = 2,
    #[doc = "1: 64 bits"]
    _64_BIT = 1,
    #[doc = "0: 32 bits"]
    _32_BIT = 0,
}
impl From<CTR_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: CTR_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CTR_WIDTH` reader - 8:7\\]
Specifies the counter width for AES-CTR mode"]
pub struct CTR_WIDTH_R(crate::FieldReader<u8, CTR_WIDTH_A>);
impl CTR_WIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTR_WIDTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTR_WIDTH_A {
        match self.bits {
            3 => CTR_WIDTH_A::_128_BIT,
            2 => CTR_WIDTH_A::_96_BIT,
            1 => CTR_WIDTH_A::_64_BIT,
            0 => CTR_WIDTH_A::_32_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_128_BIT`"]
    #[inline(always)]
    pub fn is_128_bit(&self) -> bool {
        **self == CTR_WIDTH_A::_128_BIT
    }
    #[doc = "Checks if the value of the field is `_96_BIT`"]
    #[inline(always)]
    pub fn is_96_bit(&self) -> bool {
        **self == CTR_WIDTH_A::_96_BIT
    }
    #[doc = "Checks if the value of the field is `_64_BIT`"]
    #[inline(always)]
    pub fn is_64_bit(&self) -> bool {
        **self == CTR_WIDTH_A::_64_BIT
    }
    #[doc = "Checks if the value of the field is `_32_BIT`"]
    #[inline(always)]
    pub fn is_32_bit(&self) -> bool {
        **self == CTR_WIDTH_A::_32_BIT
    }
}
impl core::ops::Deref for CTR_WIDTH_R {
    type Target = crate::FieldReader<u8, CTR_WIDTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR_WIDTH` writer - 8:7\\]
Specifies the counter width for AES-CTR mode"]
pub struct CTR_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTR_WIDTH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn _128_bit(self) -> &'a mut W {
        self.variant(CTR_WIDTH_A::_128_BIT)
    }
    #[doc = "96 bits"]
    #[inline(always)]
    pub fn _96_bit(self) -> &'a mut W {
        self.variant(CTR_WIDTH_A::_96_BIT)
    }
    #[doc = "64 bits"]
    #[inline(always)]
    pub fn _64_bit(self) -> &'a mut W {
        self.variant(CTR_WIDTH_A::_64_BIT)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _32_bit(self) -> &'a mut W {
        self.variant(CTR_WIDTH_A::_32_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `CTR` reader - 6:6\\]
AES-CTR mode enable This bit must also be set for CCM, when encryption/decryption is required."]
pub struct CTR_R(crate::FieldReader<bool, bool>);
impl CTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR` writer - 6:6\\]
AES-CTR mode enable This bit must also be set for CCM, when encryption/decryption is required."]
pub struct CTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CBC` reader - 5:5\\]
CBC mode enable"]
pub struct CBC_R(crate::FieldReader<bool, bool>);
impl CBC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBC` writer - 5:5\\]
CBC mode enable"]
pub struct CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> CBC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `KEY_SIZE` reader - 4:3\\]
This field specifies the key size. The key size is automatically configured when a new key is loaded via the key store module. 00 = N/A - reserved 01 = 128 bits 10 = N/A - reserved 11 = N/A - reserved For the Crypto peripheral this field is fixed to 128 bits."]
pub struct KEY_SIZE_R(crate::FieldReader<u8, u8>);
impl KEY_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_SIZE` writer - 4:3\\]
This field specifies the key size. The key size is automatically configured when a new key is loaded via the key store module. 00 = N/A - reserved 01 = 128 bits 10 = N/A - reserved 11 = N/A - reserved For the Crypto peripheral this field is fixed to 128 bits."]
pub struct KEY_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `DIR` reader - 2:2\\]
Direction. 0 : Decrypt operation is performed. 1 : Encrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
pub struct DIR_R(crate::FieldReader<bool, bool>);
impl DIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - 2:2\\]
Direction. 0 : Decrypt operation is performed. 1 : Encrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
#[doc = "Field `INPUT_RDY` reader - 1:1\\]
If read as 1, this status bit indicates that the 16-byte AES input buffer is empty. The Host is permitted to write the next block of data. Writing a 0 clears the bit to zero and indicates that the AES engine can use the provided input data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. After reset, this bit is 0. After writing a context (note 1), this bit will become 1. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub struct INPUT_RDY_R(crate::FieldReader<bool, bool>);
impl INPUT_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        INPUT_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPUT_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPUT_RDY` writer - 1:1\\]
If read as 1, this status bit indicates that the 16-byte AES input buffer is empty. The Host is permitted to write the next block of data. Writing a 0 clears the bit to zero and indicates that the AES engine can use the provided input data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. After reset, this bit is 0. After writing a context (note 1), this bit will become 1. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub struct INPUT_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_RDY_W<'a> {
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
#[doc = "Field `OUTPUT_RDY` reader - 0:0\\]
If read as 1, this status bit indicates that an AES output block is available to be retrieved by the Host. Writing a 0 clears the bit to zero and indicates that output data is read by the Host. The AES engine can provide a next output data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub struct OUTPUT_RDY_R(crate::FieldReader<bool, bool>);
impl OUTPUT_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTPUT_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTPUT_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTPUT_RDY` writer - 0:0\\]
If read as 1, this status bit indicates that an AES output block is available to be retrieved by the Host. Writing a 0 clears the bit to zero and indicates that output data is read by the Host. The AES engine can provide a next output data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
pub struct OUTPUT_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_RDY_W<'a> {
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
    #[doc = "Bit 31 - 31:31\\]
If 1, this status bit indicates that the context data registers can be overwritten and the Host is permitted to write the next context. Writing a context means writing either a mode, the crypto length or AESDATALEN1.LEN_MSW, AESDATALEN0.LEN_LSW length registers"]
    #[inline(always)]
    pub fn context_rdy(&self) -> CONTEXT_RDY_R {
        CONTEXT_RDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
If read as 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if SAVE_CONTEXT is set to 1. The bit is mutually exclusive with CONTEXT_RDY. Writing 1 clears the bit to zero, indicating the Crypto peripheral can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes will be ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the Crypto peripheral for TAG read DMA operations. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    pub fn saved_context_rdy(&self) -> SAVED_CONTEXT_RDY_R {
        SAVED_CONTEXT_RDY_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
IV must be read before the AES engine can start a new operation."]
    #[inline(always)]
    pub fn save_context(&self) -> SAVE_CONTEXT_R {
        SAVE_CONTEXT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 25:28 - 28:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bits 22:24 - 24:22\\]
Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times the value of CCM_M plus one. Note: The Crypto peripheral always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    pub fn ccm_m(&self) -> CCM_M_R {
        CCM_M_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM_L plus one. All values are supported."]
    #[inline(always)]
    pub fn ccm_l(&self) -> CCM_L_R {
        CCM_L_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
AES-CCM mode enable. AES-CCM is a combined mode, using AES for both authentication and encryption. Note: Selecting AES-CCM mode requires writing of AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
MAC mode enable. The DIR bit must be set to 1 for this mode. Selecting this mode requires writing the AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW registers after all other registers."]
    #[inline(always)]
    pub fn cbc_mac(&self) -> CBC_MAC_R {
        CBC_MAC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 9:14 - 14:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bits 7:8 - 8:7\\]
Specifies the counter width for AES-CTR mode"]
    #[inline(always)]
    pub fn ctr_width(&self) -> CTR_WIDTH_R {
        CTR_WIDTH_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
AES-CTR mode enable This bit must also be set for CCM, when encryption/decryption is required."]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
CBC mode enable"]
    #[inline(always)]
    pub fn cbc(&self) -> CBC_R {
        CBC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
This field specifies the key size. The key size is automatically configured when a new key is loaded via the key store module. 00 = N/A - reserved 01 = 128 bits 10 = N/A - reserved 11 = N/A - reserved For the Crypto peripheral this field is fixed to 128 bits."]
    #[inline(always)]
    pub fn key_size(&self) -> KEY_SIZE_R {
        KEY_SIZE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Direction. 0 : Decrypt operation is performed. 1 : Encrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If read as 1, this status bit indicates that the 16-byte AES input buffer is empty. The Host is permitted to write the next block of data. Writing a 0 clears the bit to zero and indicates that the AES engine can use the provided input data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. After reset, this bit is 0. After writing a context (note 1), this bit will become 1. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    pub fn input_rdy(&self) -> INPUT_RDY_R {
        INPUT_RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
If read as 1, this status bit indicates that an AES output block is available to be retrieved by the Host. Writing a 0 clears the bit to zero and indicates that output data is read by the Host. The AES engine can provide a next output data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    pub fn output_rdy(&self) -> OUTPUT_RDY_R {
        OUTPUT_RDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
If 1, this status bit indicates that the context data registers can be overwritten and the Host is permitted to write the next context. Writing a context means writing either a mode, the crypto length or AESDATALEN1.LEN_MSW, AESDATALEN0.LEN_LSW length registers"]
    #[inline(always)]
    pub fn context_rdy(&mut self) -> CONTEXT_RDY_W {
        CONTEXT_RDY_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
If read as 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if SAVE_CONTEXT is set to 1. The bit is mutually exclusive with CONTEXT_RDY. Writing 1 clears the bit to zero, indicating the Crypto peripheral can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes will be ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the Crypto peripheral for TAG read DMA operations. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    pub fn saved_context_rdy(&mut self) -> SAVED_CONTEXT_RDY_W {
        SAVED_CONTEXT_RDY_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\]
IV must be read before the AES engine can start a new operation."]
    #[inline(always)]
    pub fn save_context(&mut self) -> SAVE_CONTEXT_W {
        SAVE_CONTEXT_W { w: self }
    }
    #[doc = "Bits 25:28 - 28:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&mut self) -> RESERVED25_W {
        RESERVED25_W { w: self }
    }
    #[doc = "Bits 22:24 - 24:22\\]
Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times the value of CCM_M plus one. Note: The Crypto peripheral always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    pub fn ccm_m(&mut self) -> CCM_M_W {
        CCM_M_W { w: self }
    }
    #[doc = "Bits 19:21 - 21:19\\]
Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM_L plus one. All values are supported."]
    #[inline(always)]
    pub fn ccm_l(&mut self) -> CCM_L_W {
        CCM_L_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\]
AES-CCM mode enable. AES-CCM is a combined mode, using AES for both authentication and encryption. Note: Selecting AES-CCM mode requires writing of AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
    #[inline(always)]
    pub fn ccm(&mut self) -> CCM_W {
        CCM_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\]
MAC mode enable. The DIR bit must be set to 1 for this mode. Selecting this mode requires writing the AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW registers after all other registers."]
    #[inline(always)]
    pub fn cbc_mac(&mut self) -> CBC_MAC_W {
        CBC_MAC_W { w: self }
    }
    #[doc = "Bits 9:14 - 14:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bits 7:8 - 8:7\\]
Specifies the counter width for AES-CTR mode"]
    #[inline(always)]
    pub fn ctr_width(&mut self) -> CTR_WIDTH_W {
        CTR_WIDTH_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
AES-CTR mode enable This bit must also be set for CCM, when encryption/decryption is required."]
    #[inline(always)]
    pub fn ctr(&mut self) -> CTR_W {
        CTR_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
CBC mode enable"]
    #[inline(always)]
    pub fn cbc(&mut self) -> CBC_W {
        CBC_W { w: self }
    }
    #[doc = "Bits 3:4 - 4:3\\]
This field specifies the key size. The key size is automatically configured when a new key is loaded via the key store module. 00 = N/A - reserved 01 = 128 bits 10 = N/A - reserved 11 = N/A - reserved For the Crypto peripheral this field is fixed to 128 bits."]
    #[inline(always)]
    pub fn key_size(&mut self) -> KEY_SIZE_W {
        KEY_SIZE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Direction. 0 : Decrypt operation is performed. 1 : Encrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
If read as 1, this status bit indicates that the 16-byte AES input buffer is empty. The Host is permitted to write the next block of data. Writing a 0 clears the bit to zero and indicates that the AES engine can use the provided input data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. After reset, this bit is 0. After writing a context (note 1), this bit will become 1. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    pub fn input_rdy(&mut self) -> INPUT_RDY_W {
        INPUT_RDY_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
If read as 1, this status bit indicates that an AES output block is available to be retrieved by the Host. Writing a 0 clears the bit to zero and indicates that output data is read by the Host. The AES engine can provide a next output data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    pub fn output_rdy(&mut self) -> OUTPUT_RDY_W {
        OUTPUT_RDY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Input/Output Buffer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesctl](index.html) module"]
pub struct AESCTL_SPEC;
impl crate::RegisterSpec for AESCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aesctl::R](R) reader structure"]
impl crate::Readable for AESCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesctl::W](W) writer structure"]
impl crate::Writable for AESCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESCTL to value 0x8000_0000"]
impl crate::Resettable for AESCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
