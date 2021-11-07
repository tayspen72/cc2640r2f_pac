#[doc = "Register `CR0` reader"]
pub struct R(crate::R<CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR0` writer"]
pub struct W(crate::W<CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR0_SPEC>;
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
impl From<crate::W<CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCR` reader - 15:8\\]
Serial clock rate: This is used to generate the transmit and receive bit rate of the SSI. The bit rate is (SSI's clock frequency)/((SCR+1)*CPSR.CPSDVSR). SCR is a value from 0-255."]
pub struct SCR_R(crate::FieldReader<u8, u8>);
impl SCR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCR` writer - 15:8\\]
Serial clock rate: This is used to generate the transmit and receive bit rate of the SSI. The bit rate is (SSI's clock frequency)/((SCR+1)*CPSR.CPSDVSR). SCR is a value from 0-255."]
pub struct SCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "7:7\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPH_A {
    #[doc = "1: Data is captured on the second clock edge transition."]
    _2ND_CLK_EDGE = 1,
    #[doc = "0: Data is captured on the first clock edge transition."]
    _1ST_CLK_EDGE = 0,
}
impl From<SPH_A> for bool {
    #[inline(always)]
    fn from(variant: SPH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPH` reader - 7:7\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
pub struct SPH_R(crate::FieldReader<bool, SPH_A>);
impl SPH_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPH_A {
        match self.bits {
            true => SPH_A::_2ND_CLK_EDGE,
            false => SPH_A::_1ST_CLK_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `_2ND_CLK_EDGE`"]
    #[inline(always)]
    pub fn is_2nd_clk_edge(&self) -> bool {
        **self == SPH_A::_2ND_CLK_EDGE
    }
    #[doc = "Checks if the value of the field is `_1ST_CLK_EDGE`"]
    #[inline(always)]
    pub fn is_1st_clk_edge(&self) -> bool {
        **self == SPH_A::_1ST_CLK_EDGE
    }
}
impl core::ops::Deref for SPH_R {
    type Target = crate::FieldReader<bool, SPH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPH` writer - 7:7\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
pub struct SPH_W<'a> {
    w: &'a mut W,
}
impl<'a> SPH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data is captured on the second clock edge transition."]
    #[inline(always)]
    pub fn _2nd_clk_edge(self) -> &'a mut W {
        self.variant(SPH_A::_2ND_CLK_EDGE)
    }
    #[doc = "Data is captured on the first clock edge transition."]
    #[inline(always)]
    pub fn _1st_clk_edge(self) -> &'a mut W {
        self.variant(SPH_A::_1ST_CLK_EDGE)
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
CLKOUT polarity (Motorola SPI frame format only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPO_A {
    #[doc = "1: SSI produces a steady state HIGH value on the CLKOUT pin when data is not being transferred."]
    HIGH = 1,
    #[doc = "0: SSI produces a steady state LOW value on the\nCLKOUT pin when data is not being transferred."]
    LOW = 0,
}
impl From<SPO_A> for bool {
    #[inline(always)]
    fn from(variant: SPO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPO` reader - 6:6\\]
CLKOUT polarity (Motorola SPI frame format only)"]
pub struct SPO_R(crate::FieldReader<bool, SPO_A>);
impl SPO_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPO_A {
        match self.bits {
            true => SPO_A::HIGH,
            false => SPO_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SPO_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SPO_A::LOW
    }
}
impl core::ops::Deref for SPO_R {
    type Target = crate::FieldReader<bool, SPO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPO` writer - 6:6\\]
CLKOUT polarity (Motorola SPI frame format only)"]
pub struct SPO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SSI produces a steady state HIGH value on the CLKOUT pin when data is not being transferred."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPO_A::HIGH)
    }
    #[doc = "SSI produces a steady state LOW value on the CLKOUT pin when data is not being transferred."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPO_A::LOW)
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
#[doc = "5:4\\]
Frame format. The supported frame formats are Motorola SPI, TI synchronous serial and National Microwire. Value 0'b11 is reserved and shall not be used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FRF_A {
    #[doc = "2: National Microwire frame format"]
    NATIONAL_MICROWIRE = 2,
    #[doc = "1: TI synchronous serial frame format"]
    TI_SYNC_SERIAL = 1,
    #[doc = "0: Motorola SPI frame format"]
    MOTOROLA_SPI = 0,
}
impl From<FRF_A> for u8 {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FRF` reader - 5:4\\]
Frame format. The supported frame formats are Motorola SPI, TI synchronous serial and National Microwire. Value 0'b11 is reserved and shall not be used."]
pub struct FRF_R(crate::FieldReader<u8, FRF_A>);
impl FRF_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FRF_A> {
        match self.bits {
            2 => Some(FRF_A::NATIONAL_MICROWIRE),
            1 => Some(FRF_A::TI_SYNC_SERIAL),
            0 => Some(FRF_A::MOTOROLA_SPI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NATIONAL_MICROWIRE`"]
    #[inline(always)]
    pub fn is_national_microwire(&self) -> bool {
        **self == FRF_A::NATIONAL_MICROWIRE
    }
    #[doc = "Checks if the value of the field is `TI_SYNC_SERIAL`"]
    #[inline(always)]
    pub fn is_ti_sync_serial(&self) -> bool {
        **self == FRF_A::TI_SYNC_SERIAL
    }
    #[doc = "Checks if the value of the field is `MOTOROLA_SPI`"]
    #[inline(always)]
    pub fn is_motorola_spi(&self) -> bool {
        **self == FRF_A::MOTOROLA_SPI
    }
}
impl core::ops::Deref for FRF_R {
    type Target = crate::FieldReader<u8, FRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRF` writer - 5:4\\]
Frame format. The supported frame formats are Motorola SPI, TI synchronous serial and National Microwire. Value 0'b11 is reserved and shall not be used."]
pub struct FRF_W<'a> {
    w: &'a mut W,
}
impl<'a> FRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "National Microwire frame format"]
    #[inline(always)]
    pub fn national_microwire(self) -> &'a mut W {
        self.variant(FRF_A::NATIONAL_MICROWIRE)
    }
    #[doc = "TI synchronous serial frame format"]
    #[inline(always)]
    pub fn ti_sync_serial(self) -> &'a mut W {
        self.variant(FRF_A::TI_SYNC_SERIAL)
    }
    #[doc = "Motorola SPI frame format"]
    #[inline(always)]
    pub fn motorola_spi(self) -> &'a mut W {
        self.variant(FRF_A::MOTOROLA_SPI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "3:0\\]
Data Size Select. Values 0b0000, 0b0001, 0b0010 are reserved and shall not be used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSS_A {
    #[doc = "15: 16-bit data"]
    _16_BIT = 15,
    #[doc = "14: 15-bit data"]
    _15_BIT = 14,
    #[doc = "13: 14-bit data"]
    _14_BIT = 13,
    #[doc = "12: 13-bit data"]
    _13_BIT = 12,
    #[doc = "11: 12-bit data"]
    _12_BIT = 11,
    #[doc = "10: 11-bit data"]
    _11_BIT = 10,
    #[doc = "9: 10-bit data"]
    _10_BIT = 9,
    #[doc = "8: 9-bit data"]
    _9_BIT = 8,
    #[doc = "7: 8-bit data"]
    _8_BIT = 7,
    #[doc = "6: 7-bit data"]
    _7_BIT = 6,
    #[doc = "5: 6-bit data"]
    _6_BIT = 5,
    #[doc = "4: 5-bit data"]
    _5_BIT = 4,
    #[doc = "3: 4-bit data"]
    _4_BIT = 3,
}
impl From<DSS_A> for u8 {
    #[inline(always)]
    fn from(variant: DSS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DSS` reader - 3:0\\]
Data Size Select. Values 0b0000, 0b0001, 0b0010 are reserved and shall not be used."]
pub struct DSS_R(crate::FieldReader<u8, DSS_A>);
impl DSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSS_A> {
        match self.bits {
            15 => Some(DSS_A::_16_BIT),
            14 => Some(DSS_A::_15_BIT),
            13 => Some(DSS_A::_14_BIT),
            12 => Some(DSS_A::_13_BIT),
            11 => Some(DSS_A::_12_BIT),
            10 => Some(DSS_A::_11_BIT),
            9 => Some(DSS_A::_10_BIT),
            8 => Some(DSS_A::_9_BIT),
            7 => Some(DSS_A::_8_BIT),
            6 => Some(DSS_A::_7_BIT),
            5 => Some(DSS_A::_6_BIT),
            4 => Some(DSS_A::_5_BIT),
            3 => Some(DSS_A::_4_BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        **self == DSS_A::_16_BIT
    }
    #[doc = "Checks if the value of the field is `_15_BIT`"]
    #[inline(always)]
    pub fn is_15_bit(&self) -> bool {
        **self == DSS_A::_15_BIT
    }
    #[doc = "Checks if the value of the field is `_14_BIT`"]
    #[inline(always)]
    pub fn is_14_bit(&self) -> bool {
        **self == DSS_A::_14_BIT
    }
    #[doc = "Checks if the value of the field is `_13_BIT`"]
    #[inline(always)]
    pub fn is_13_bit(&self) -> bool {
        **self == DSS_A::_13_BIT
    }
    #[doc = "Checks if the value of the field is `_12_BIT`"]
    #[inline(always)]
    pub fn is_12_bit(&self) -> bool {
        **self == DSS_A::_12_BIT
    }
    #[doc = "Checks if the value of the field is `_11_BIT`"]
    #[inline(always)]
    pub fn is_11_bit(&self) -> bool {
        **self == DSS_A::_11_BIT
    }
    #[doc = "Checks if the value of the field is `_10_BIT`"]
    #[inline(always)]
    pub fn is_10_bit(&self) -> bool {
        **self == DSS_A::_10_BIT
    }
    #[doc = "Checks if the value of the field is `_9_BIT`"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        **self == DSS_A::_9_BIT
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        **self == DSS_A::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_7_BIT`"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        **self == DSS_A::_7_BIT
    }
    #[doc = "Checks if the value of the field is `_6_BIT`"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        **self == DSS_A::_6_BIT
    }
    #[doc = "Checks if the value of the field is `_5_BIT`"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        **self == DSS_A::_5_BIT
    }
    #[doc = "Checks if the value of the field is `_4_BIT`"]
    #[inline(always)]
    pub fn is_4_bit(&self) -> bool {
        **self == DSS_A::_4_BIT
    }
}
impl core::ops::Deref for DSS_R {
    type Target = crate::FieldReader<u8, DSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSS` writer - 3:0\\]
Data Size Select. Values 0b0000, 0b0001, 0b0010 are reserved and shall not be used."]
pub struct DSS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(DSS_A::_16_BIT)
    }
    #[doc = "15-bit data"]
    #[inline(always)]
    pub fn _15_bit(self) -> &'a mut W {
        self.variant(DSS_A::_15_BIT)
    }
    #[doc = "14-bit data"]
    #[inline(always)]
    pub fn _14_bit(self) -> &'a mut W {
        self.variant(DSS_A::_14_BIT)
    }
    #[doc = "13-bit data"]
    #[inline(always)]
    pub fn _13_bit(self) -> &'a mut W {
        self.variant(DSS_A::_13_BIT)
    }
    #[doc = "12-bit data"]
    #[inline(always)]
    pub fn _12_bit(self) -> &'a mut W {
        self.variant(DSS_A::_12_BIT)
    }
    #[doc = "11-bit data"]
    #[inline(always)]
    pub fn _11_bit(self) -> &'a mut W {
        self.variant(DSS_A::_11_BIT)
    }
    #[doc = "10-bit data"]
    #[inline(always)]
    pub fn _10_bit(self) -> &'a mut W {
        self.variant(DSS_A::_10_BIT)
    }
    #[doc = "9-bit data"]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut W {
        self.variant(DSS_A::_9_BIT)
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(DSS_A::_8_BIT)
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut W {
        self.variant(DSS_A::_7_BIT)
    }
    #[doc = "6-bit data"]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut W {
        self.variant(DSS_A::_6_BIT)
    }
    #[doc = "5-bit data"]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut W {
        self.variant(DSS_A::_5_BIT)
    }
    #[doc = "4-bit data"]
    #[inline(always)]
    pub fn _4_bit(self) -> &'a mut W {
        self.variant(DSS_A::_4_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - 15:8\\]
Serial clock rate: This is used to generate the transmit and receive bit rate of the SSI. The bit rate is (SSI's clock frequency)/((SCR+1)*CPSR.CPSDVSR). SCR is a value from 0-255."]
    #[inline(always)]
    pub fn scr(&self) -> SCR_R {
        SCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
    #[inline(always)]
    pub fn sph(&self) -> SPH_R {
        SPH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
CLKOUT polarity (Motorola SPI frame format only)"]
    #[inline(always)]
    pub fn spo(&self) -> SPO_R {
        SPO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Frame format. The supported frame formats are Motorola SPI, TI synchronous serial and National Microwire. Value 0'b11 is reserved and shall not be used."]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Data Size Select. Values 0b0000, 0b0001, 0b0010 are reserved and shall not be used."]
    #[inline(always)]
    pub fn dss(&self) -> DSS_R {
        DSS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 15:8\\]
Serial clock rate: This is used to generate the transmit and receive bit rate of the SSI. The bit rate is (SSI's clock frequency)/((SCR+1)*CPSR.CPSDVSR). SCR is a value from 0-255."]
    #[inline(always)]
    pub fn scr(&mut self) -> SCR_W {
        SCR_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
    #[inline(always)]
    pub fn sph(&mut self) -> SPH_W {
        SPH_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
CLKOUT polarity (Motorola SPI frame format only)"]
    #[inline(always)]
    pub fn spo(&mut self) -> SPO_W {
        SPO_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Frame format. The supported frame formats are Motorola SPI, TI synchronous serial and National Microwire. Value 0'b11 is reserved and shall not be used."]
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W {
        FRF_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Data Size Select. Values 0b0000, 0b0001, 0b0010 are reserved and shall not be used."]
    #[inline(always)]
    pub fn dss(&mut self) -> DSS_W {
        DSS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](index.html) module"]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr0::R](R) reader structure"]
impl crate::Readable for CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr0::W](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
