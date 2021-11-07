#[doc = "Register `AIFFMTCFG` reader"]
pub struct R(crate::R<AIFFMTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIFFMTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIFFMTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIFFMTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIFFMTCFG` writer"]
pub struct W(crate::W<AIFFMTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIFFMTCFG_SPEC>;
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
impl From<crate::W<AIFFMTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIFFMTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_R(crate::FieldReader<u16, u16>);
impl RESERVED16_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED16_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `DATA_DELAY` reader - 15:8\\]
The number of BCLK periods between a WCLK edge and MSB of the first word in a phase: 0x00: LJF and DSP format 0x01: I2S and DSP format 0x02: RJF format ... 0xFF: RJF format Note: When 0, MSB of the next word will be output in the idle period between LSB of the previous word and the start of the next word. Otherwise logical 0 will be output until the data delay has expired."]
pub struct DATA_DELAY_R(crate::FieldReader<u8, u8>);
impl DATA_DELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_DELAY` writer - 15:8\\]
The number of BCLK periods between a WCLK edge and MSB of the first word in a phase: 0x00: LJF and DSP format 0x01: I2S and DSP format 0x02: RJF format ... 0xFF: RJF format Note: When 0, MSB of the next word will be output in the idle period between LSB of the previous word and the start of the next word. Otherwise logical 0 will be output until the data delay has expired."]
pub struct DATA_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "7:7\\]
The size of each word stored to or loaded from memory:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEM_LEN_24_A {
    #[doc = "1: 24-bit (one 8 bit and one 16 bit locked access per sample)"]
    _24BIT = 1,
    #[doc = "0: 16-bit (one 16 bit access per sample)"]
    _16BIT = 0,
}
impl From<MEM_LEN_24_A> for bool {
    #[inline(always)]
    fn from(variant: MEM_LEN_24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM_LEN_24` reader - 7:7\\]
The size of each word stored to or loaded from memory:"]
pub struct MEM_LEN_24_R(crate::FieldReader<bool, MEM_LEN_24_A>);
impl MEM_LEN_24_R {
    pub(crate) fn new(bits: bool) -> Self {
        MEM_LEN_24_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEM_LEN_24_A {
        match self.bits {
            true => MEM_LEN_24_A::_24BIT,
            false => MEM_LEN_24_A::_16BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_24BIT`"]
    #[inline(always)]
    pub fn is_24bit(&self) -> bool {
        **self == MEM_LEN_24_A::_24BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        **self == MEM_LEN_24_A::_16BIT
    }
}
impl core::ops::Deref for MEM_LEN_24_R {
    type Target = crate::FieldReader<bool, MEM_LEN_24_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_LEN_24` writer - 7:7\\]
The size of each word stored to or loaded from memory:"]
pub struct MEM_LEN_24_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_LEN_24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEM_LEN_24_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "24-bit (one 8 bit and one 16 bit locked access per sample)"]
    #[inline(always)]
    pub fn _24bit(self) -> &'a mut W {
        self.variant(MEM_LEN_24_A::_24BIT)
    }
    #[doc = "16-bit (one 16 bit access per sample)"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(MEM_LEN_24_A::_16BIT)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "6:6\\]
On the serial audio interface, data (and wclk) is sampled and clocked out on opposite edges of BCLK.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPL_EDGE_A {
    #[doc = "1: Data is sampled on the positive edge and clocked out on the negative edge."]
    POS = 1,
    #[doc = "0: Data is sampled on the negative edge and clocked out on the positive edge."]
    NEG = 0,
}
impl From<SMPL_EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: SMPL_EDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPL_EDGE` reader - 6:6\\]
On the serial audio interface, data (and wclk) is sampled and clocked out on opposite edges of BCLK."]
pub struct SMPL_EDGE_R(crate::FieldReader<bool, SMPL_EDGE_A>);
impl SMPL_EDGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMPL_EDGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPL_EDGE_A {
        match self.bits {
            true => SMPL_EDGE_A::POS,
            false => SMPL_EDGE_A::NEG,
        }
    }
    #[doc = "Checks if the value of the field is `POS`"]
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        **self == SMPL_EDGE_A::POS
    }
    #[doc = "Checks if the value of the field is `NEG`"]
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        **self == SMPL_EDGE_A::NEG
    }
}
impl core::ops::Deref for SMPL_EDGE_R {
    type Target = crate::FieldReader<bool, SMPL_EDGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPL_EDGE` writer - 6:6\\]
On the serial audio interface, data (and wclk) is sampled and clocked out on opposite edges of BCLK."]
pub struct SMPL_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPL_EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPL_EDGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data is sampled on the positive edge and clocked out on the negative edge."]
    #[inline(always)]
    pub fn pos(self) -> &'a mut W {
        self.variant(SMPL_EDGE_A::POS)
    }
    #[doc = "Data is sampled on the negative edge and clocked out on the positive edge."]
    #[inline(always)]
    pub fn neg(self) -> &'a mut W {
        self.variant(SMPL_EDGE_A::NEG)
    }
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
#[doc = "Field `DUAL_PHASE` reader - 5:5\\]
Selects dual- or single-phase format. 0: Single-phase: DSP format 1: Dual-phase: I2S, LJF and RJF formats"]
pub struct DUAL_PHASE_R(crate::FieldReader<bool, bool>);
impl DUAL_PHASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUAL_PHASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUAL_PHASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUAL_PHASE` writer - 5:5\\]
Selects dual- or single-phase format. 0: Single-phase: DSP format 1: Dual-phase: I2S, LJF and RJF formats"]
pub struct DUAL_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL_PHASE_W<'a> {
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
#[doc = "Field `WORD_LEN` reader - 4:0\\]
Number of bits per word (8-24): In single-phase format, this is the exact number of bits per word. In dual-phase format, this is the maximum number of bits per word. Values below 8 and above 24 give undefined behavior. Data written to memory is always aligned to 16 or 24 bits as defined by MEM_LEN_24. Bit widths that differ from this alignment will either be truncated or zero padded."]
pub struct WORD_LEN_R(crate::FieldReader<u8, u8>);
impl WORD_LEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        WORD_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WORD_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WORD_LEN` writer - 4:0\\]
Number of bits per word (8-24): In single-phase format, this is the exact number of bits per word. In dual-phase format, this is the maximum number of bits per word. Values below 8 and above 24 give undefined behavior. Data written to memory is always aligned to 16 or 24 bits as defined by MEM_LEN_24. Bit widths that differ from this alignment will either be truncated or zero padded."]
pub struct WORD_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WORD_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The number of BCLK periods between a WCLK edge and MSB of the first word in a phase: 0x00: LJF and DSP format 0x01: I2S and DSP format 0x02: RJF format ... 0xFF: RJF format Note: When 0, MSB of the next word will be output in the idle period between LSB of the previous word and the start of the next word. Otherwise logical 0 will be output until the data delay has expired."]
    #[inline(always)]
    pub fn data_delay(&self) -> DATA_DELAY_R {
        DATA_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
The size of each word stored to or loaded from memory:"]
    #[inline(always)]
    pub fn mem_len_24(&self) -> MEM_LEN_24_R {
        MEM_LEN_24_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
On the serial audio interface, data (and wclk) is sampled and clocked out on opposite edges of BCLK."]
    #[inline(always)]
    pub fn smpl_edge(&self) -> SMPL_EDGE_R {
        SMPL_EDGE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Selects dual- or single-phase format. 0: Single-phase: DSP format 1: Dual-phase: I2S, LJF and RJF formats"]
    #[inline(always)]
    pub fn dual_phase(&self) -> DUAL_PHASE_R {
        DUAL_PHASE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - 4:0\\]
Number of bits per word (8-24): In single-phase format, this is the exact number of bits per word. In dual-phase format, this is the maximum number of bits per word. Values below 8 and above 24 give undefined behavior. Data written to memory is always aligned to 16 or 24 bits as defined by MEM_LEN_24. Bit widths that differ from this alignment will either be truncated or zero padded."]
    #[inline(always)]
    pub fn word_len(&self) -> WORD_LEN_R {
        WORD_LEN_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
The number of BCLK periods between a WCLK edge and MSB of the first word in a phase: 0x00: LJF and DSP format 0x01: I2S and DSP format 0x02: RJF format ... 0xFF: RJF format Note: When 0, MSB of the next word will be output in the idle period between LSB of the previous word and the start of the next word. Otherwise logical 0 will be output until the data delay has expired."]
    #[inline(always)]
    pub fn data_delay(&mut self) -> DATA_DELAY_W {
        DATA_DELAY_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
The size of each word stored to or loaded from memory:"]
    #[inline(always)]
    pub fn mem_len_24(&mut self) -> MEM_LEN_24_W {
        MEM_LEN_24_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
On the serial audio interface, data (and wclk) is sampled and clocked out on opposite edges of BCLK."]
    #[inline(always)]
    pub fn smpl_edge(&mut self) -> SMPL_EDGE_W {
        SMPL_EDGE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Selects dual- or single-phase format. 0: Single-phase: DSP format 1: Dual-phase: I2S, LJF and RJF formats"]
    #[inline(always)]
    pub fn dual_phase(&mut self) -> DUAL_PHASE_W {
        DUAL_PHASE_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\]
Number of bits per word (8-24): In single-phase format, this is the exact number of bits per word. In dual-phase format, this is the maximum number of bits per word. Values below 8 and above 24 give undefined behavior. Data written to memory is always aligned to 16 or 24 bits as defined by MEM_LEN_24. Bit widths that differ from this alignment will either be truncated or zero padded."]
    #[inline(always)]
    pub fn word_len(&mut self) -> WORD_LEN_W {
        WORD_LEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serial Interface Format Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aiffmtcfg](index.html) module"]
pub struct AIFFMTCFG_SPEC;
impl crate::RegisterSpec for AIFFMTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aiffmtcfg::R](R) reader structure"]
impl crate::Readable for AIFFMTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aiffmtcfg::W](W) writer structure"]
impl crate::Writable for AIFFMTCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AIFFMTCFG to value 0x0170"]
impl crate::Resettable for AIFFMTCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0170
    }
}
