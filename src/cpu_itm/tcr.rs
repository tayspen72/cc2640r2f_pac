#[doc = "Register `TCR` reader"]
pub struct R(crate::R<TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR` writer"]
pub struct W(crate::W<TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR_SPEC>;
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
impl From<crate::W<TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED24_R(crate::FieldReader<u8, u8>);
impl RESERVED24_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED24_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `BUSY` reader - 23:23\\]
Set when ITM events present and being drained."]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` writer - 23:23\\]
Set when ITM events present and being drained."]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `ATBID` reader - 22:16\\]
Trace Bus ID for CoreSight system. Optional identifier for multi-source trace stream formatting. If multi-source trace is in use, this field must be written with a non-zero value."]
pub struct ATBID_R(crate::FieldReader<u8, u8>);
impl ATBID_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATBID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATBID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATBID` writer - 22:16\\]
Trace Bus ID for CoreSight system. Optional identifier for multi-source trace stream formatting. If multi-source trace is in use, this field must be written with a non-zero value."]
pub struct ATBID_W<'a> {
    w: &'a mut W,
}
impl<'a> ATBID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED10` reader - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED10_R(crate::FieldReader<u8, u8>);
impl RESERVED10_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED10` writer - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED10_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | ((value as u32 & 0x3f) << 10);
        self.w
    }
}
#[doc = "9:8\\]
Timestamp prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSPRESCALE_A {
    #[doc = "3: Divide by 64"]
    DIV64 = 3,
    #[doc = "2: Divide by 16"]
    DIV16 = 2,
    #[doc = "1: Divide by 4"]
    DIV4 = 1,
    #[doc = "0: No prescaling"]
    NOPRESCALING = 0,
}
impl From<TSPRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: TSPRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSPRESCALE` reader - 9:8\\]
Timestamp prescaler"]
pub struct TSPRESCALE_R(crate::FieldReader<u8, TSPRESCALE_A>);
impl TSPRESCALE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSPRESCALE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSPRESCALE_A {
        match self.bits {
            3 => TSPRESCALE_A::DIV64,
            2 => TSPRESCALE_A::DIV16,
            1 => TSPRESCALE_A::DIV4,
            0 => TSPRESCALE_A::NOPRESCALING,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == TSPRESCALE_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == TSPRESCALE_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == TSPRESCALE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `NOPRESCALING`"]
    #[inline(always)]
    pub fn is_noprescaling(&self) -> bool {
        **self == TSPRESCALE_A::NOPRESCALING
    }
}
impl core::ops::Deref for TSPRESCALE_R {
    type Target = crate::FieldReader<u8, TSPRESCALE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSPRESCALE` writer - 9:8\\]
Timestamp prescaler"]
pub struct TSPRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSPRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSPRESCALE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(TSPRESCALE_A::DIV64)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(TSPRESCALE_A::DIV16)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(TSPRESCALE_A::DIV4)
    }
    #[doc = "No prescaling"]
    #[inline(always)]
    pub fn noprescaling(self) -> &'a mut W {
        self.variant(TSPRESCALE_A::NOPRESCALING)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `RESERVED5` reader - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED5_R(crate::FieldReader<u8, u8>);
impl RESERVED5_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED5` writer - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `SWOENA` reader - 4:4\\]
Enables asynchronous clocking of the timestamp counter (when TSENA = 1). If TSENA = 0, writing this bit to 1 does not enable asynchronous clocking of the timestamp counter. 0x0: Mode disabled. Timestamp counter uses system clock from the core and counts continuously. 0x1: Timestamp counter uses lineout (data related) clock from TPIU interface. The timestamp counter is held in reset while the output line is idle."]
pub struct SWOENA_R(crate::FieldReader<bool, bool>);
impl SWOENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWOENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWOENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWOENA` writer - 4:4\\]
Enables asynchronous clocking of the timestamp counter (when TSENA = 1). If TSENA = 0, writing this bit to 1 does not enable asynchronous clocking of the timestamp counter. 0x0: Mode disabled. Timestamp counter uses system clock from the core and counts continuously. 0x1: Timestamp counter uses lineout (data related) clock from TPIU interface. The timestamp counter is held in reset while the output line is idle."]
pub struct SWOENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SWOENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `DWTENA` reader - 3:3\\]
Enables the DWT stimulus (hardware event packet emission to the TPIU from the DWT)"]
pub struct DWTENA_R(crate::FieldReader<bool, bool>);
impl DWTENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DWTENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DWTENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DWTENA` writer - 3:3\\]
Enables the DWT stimulus (hardware event packet emission to the TPIU from the DWT)"]
pub struct DWTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DWTENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SYNCENA` reader - 2:2\\]
Enables synchronization packet transmission for a synchronous TPIU. CPU_DWT:CTRL.SYNCTAP must be configured for the correct synchronization speed."]
pub struct SYNCENA_R(crate::FieldReader<bool, bool>);
impl SYNCENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNCENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCENA` writer - 2:2\\]
Enables synchronization packet transmission for a synchronous TPIU. CPU_DWT:CTRL.SYNCTAP must be configured for the correct synchronization speed."]
pub struct SYNCENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCENA_W<'a> {
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
#[doc = "Field `TSENA` reader - 1:1\\]
Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of two million cycles. This provides a time reference for packets and inter-packet gaps. If SWOENA (bit \\[4\\]) is set, timestamps are triggered by activity on the internal trace bus only. In this case there is no regular timestamp output when the ITM is idle."]
pub struct TSENA_R(crate::FieldReader<bool, bool>);
impl TSENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENA` writer - 1:1\\]
Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of two million cycles. This provides a time reference for packets and inter-packet gaps. If SWOENA (bit \\[4\\]) is set, timestamps are triggered by activity on the internal trace bus only. In this case there is no regular timestamp output when the ITM is idle."]
pub struct TSENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENA_W<'a> {
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
#[doc = "Field `ITMENA` reader - 0:0\\]
Enables ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
pub struct ITMENA_R(crate::FieldReader<bool, bool>);
impl ITMENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITMENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITMENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITMENA` writer - 0:0\\]
Enables ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
pub struct ITMENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ITMENA_W<'a> {
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
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Set when ITM events present and being drained."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Trace Bus ID for CoreSight system. Optional identifier for multi-source trace stream formatting. If multi-source trace is in use, this field must be written with a non-zero value."]
    #[inline(always)]
    pub fn atbid(&self) -> ATBID_R {
        ATBID_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Timestamp prescaler"]
    #[inline(always)]
    pub fn tsprescale(&self) -> TSPRESCALE_R {
        TSPRESCALE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables asynchronous clocking of the timestamp counter (when TSENA = 1). If TSENA = 0, writing this bit to 1 does not enable asynchronous clocking of the timestamp counter. 0x0: Mode disabled. Timestamp counter uses system clock from the core and counts continuously. 0x1: Timestamp counter uses lineout (data related) clock from TPIU interface. The timestamp counter is held in reset while the output line is idle."]
    #[inline(always)]
    pub fn swoena(&self) -> SWOENA_R {
        SWOENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables the DWT stimulus (hardware event packet emission to the TPIU from the DWT)"]
    #[inline(always)]
    pub fn dwtena(&self) -> DWTENA_R {
        DWTENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables synchronization packet transmission for a synchronous TPIU. CPU_DWT:CTRL.SYNCTAP must be configured for the correct synchronization speed."]
    #[inline(always)]
    pub fn syncena(&self) -> SYNCENA_R {
        SYNCENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of two million cycles. This provides a time reference for packets and inter-packet gaps. If SWOENA (bit \\[4\\]) is set, timestamps are triggered by activity on the internal trace bus only. In this case there is no regular timestamp output when the ITM is idle."]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Enables ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
    #[inline(always)]
    pub fn itmena(&self) -> ITMENA_R {
        ITMENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Set when ITM events present and being drained."]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bits 16:22 - 22:16\\]
Trace Bus ID for CoreSight system. Optional identifier for multi-source trace stream formatting. If multi-source trace is in use, this field must be written with a non-zero value."]
    #[inline(always)]
    pub fn atbid(&mut self) -> ATBID_W {
        ATBID_W { w: self }
    }
    #[doc = "Bits 10:15 - 15:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&mut self) -> RESERVED10_W {
        RESERVED10_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
Timestamp prescaler"]
    #[inline(always)]
    pub fn tsprescale(&mut self) -> TSPRESCALE_W {
        TSPRESCALE_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Enables asynchronous clocking of the timestamp counter (when TSENA = 1). If TSENA = 0, writing this bit to 1 does not enable asynchronous clocking of the timestamp counter. 0x0: Mode disabled. Timestamp counter uses system clock from the core and counts continuously. 0x1: Timestamp counter uses lineout (data related) clock from TPIU interface. The timestamp counter is held in reset while the output line is idle."]
    #[inline(always)]
    pub fn swoena(&mut self) -> SWOENA_W {
        SWOENA_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Enables the DWT stimulus (hardware event packet emission to the TPIU from the DWT)"]
    #[inline(always)]
    pub fn dwtena(&mut self) -> DWTENA_W {
        DWTENA_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Enables synchronization packet transmission for a synchronous TPIU. CPU_DWT:CTRL.SYNCTAP must be configured for the correct synchronization speed."]
    #[inline(always)]
    pub fn syncena(&mut self) -> SYNCENA_W {
        SYNCENA_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of two million cycles. This provides a time reference for packets and inter-packet gaps. If SWOENA (bit \\[4\\]) is set, timestamps are triggered by activity on the internal trace bus only. In this case there is no regular timestamp output when the ITM is idle."]
    #[inline(always)]
    pub fn tsena(&mut self) -> TSENA_W {
        TSENA_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Enables ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
    #[inline(always)]
    pub fn itmena(&mut self) -> ITMENA_W {
        ITMENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trace Control Use this register to configure and control ITM transfers. This register can only be written in privilege mode. DWT is not enabled in the ITM block. However, DWT stimulus entry into the FIFO is controlled by DWTENA. If DWT requires timestamping, the TSENA bit must be set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](index.html) module"]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr::R](R) reader structure"]
impl crate::Readable for TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr::W](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
