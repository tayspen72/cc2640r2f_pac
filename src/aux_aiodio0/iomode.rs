#[doc = "Register `IOMODE` reader"]
pub struct R(crate::R<IOMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOMODE` writer"]
pub struct W(crate::W<IOMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMODE_SPEC>;
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
impl From<crate::W<IOMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMODE_SPEC>) -> Self {
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
#[doc = "15:14\\]
Select mode for AUXIO\\[8i+7\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO7_A {
    #[doc = "3: Open-Source Mode: \n\nWhen GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: \n\nWhen GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is driven low.  \n\nWhen GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode:\n\nWhen GPIODIE bit 7 is 0: AUXIO\\[8i+7\\]
is enabled for analog signal transfer.\n\nWhen GPIODIE bit 7 is 1: AUXIO\\[8i+7\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode:\n\nGPIODOUT bit 7 drives AUXIO\\[8i+7\\]."]
    OUT = 0,
}
impl From<IO7_A> for u8 {
    #[inline(always)]
    fn from(variant: IO7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO7` reader - 15:14\\]
Select mode for AUXIO\\[8i+7\\]."]
pub struct IO7_R(crate::FieldReader<u8, IO7_A>);
impl IO7_R {
    pub(crate) fn new(bits: u8) -> Self {
        IO7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO7_A {
        match self.bits {
            3 => IO7_A::OPEN_SOURCE,
            2 => IO7_A::OPEN_DRAIN,
            1 => IO7_A::IN,
            0 => IO7_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        **self == IO7_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        **self == IO7_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        **self == IO7_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        **self == IO7_A::OUT
    }
}
impl core::ops::Deref for IO7_R {
    type Target = crate::FieldReader<u8, IO7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO7` writer - 15:14\\]
Select mode for AUXIO\\[8i+7\\]."]
pub struct IO7_W<'a> {
    w: &'a mut W,
}
impl<'a> IO7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO7_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Open-Source Mode: When GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO7_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\]
is driven low. When GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO7_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 7 is 0: AUXIO\\[8i+7\\]
is enabled for analog signal transfer. When GPIODIE bit 7 is 1: AUXIO\\[8i+7\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO7_A::IN)
    }
    #[doc = "Output Mode: GPIODOUT bit 7 drives AUXIO\\[8i+7\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO7_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "13:12\\]
Select mode for AUXIO\\[8i+6\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO6_A {
    #[doc = "3: Open-Source Mode: \n\nWhen GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: \n\nWhen GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is driven low.  \n\nWhen GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode:\n\nWhen GPIODIE bit 6 is 0: AUXIO\\[8i+6\\]
is enabled for analog signal transfer.\n\nWhen GPIODIE bit 6 is 1: AUXIO\\[8i+6\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode:\n\nGPIODOUT bit 6 drives AUXIO\\[8i+6\\]."]
    OUT = 0,
}
impl From<IO6_A> for u8 {
    #[inline(always)]
    fn from(variant: IO6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO6` reader - 13:12\\]
Select mode for AUXIO\\[8i+6\\]."]
pub struct IO6_R(crate::FieldReader<u8, IO6_A>);
impl IO6_R {
    pub(crate) fn new(bits: u8) -> Self {
        IO6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO6_A {
        match self.bits {
            3 => IO6_A::OPEN_SOURCE,
            2 => IO6_A::OPEN_DRAIN,
            1 => IO6_A::IN,
            0 => IO6_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        **self == IO6_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        **self == IO6_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        **self == IO6_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        **self == IO6_A::OUT
    }
}
impl core::ops::Deref for IO6_R {
    type Target = crate::FieldReader<u8, IO6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO6` writer - 13:12\\]
Select mode for AUXIO\\[8i+6\\]."]
pub struct IO6_W<'a> {
    w: &'a mut W,
}
impl<'a> IO6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Open-Source Mode: When GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO6_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\]
is driven low. When GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO6_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 6 is 0: AUXIO\\[8i+6\\]
is enabled for analog signal transfer. When GPIODIE bit 6 is 1: AUXIO\\[8i+6\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO6_A::IN)
    }
    #[doc = "Output Mode: GPIODOUT bit 6 drives AUXIO\\[8i+6\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO6_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "11:10\\]
Select mode for AUXIO\\[8i+5\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO5_A {
    #[doc = "3: Open-Source Mode: \n\nWhen GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: \n\nWhen GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is driven low.  \n\nWhen GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode:\n\nWhen GPIODIE bit 5 is 0: AUXIO\\[8i+5\\]
is enabled for analog signal transfer.\n\nWhen GPIODIE bit 5 is 1: AUXIO\\[8i+5\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode:\n\nGPIODOUT bit 5 drives AUXIO\\[8i+5\\]."]
    OUT = 0,
}
impl From<IO5_A> for u8 {
    #[inline(always)]
    fn from(variant: IO5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO5` reader - 11:10\\]
Select mode for AUXIO\\[8i+5\\]."]
pub struct IO5_R(crate::FieldReader<u8, IO5_A>);
impl IO5_R {
    pub(crate) fn new(bits: u8) -> Self {
        IO5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO5_A {
        match self.bits {
            3 => IO5_A::OPEN_SOURCE,
            2 => IO5_A::OPEN_DRAIN,
            1 => IO5_A::IN,
            0 => IO5_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        **self == IO5_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        **self == IO5_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        **self == IO5_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        **self == IO5_A::OUT
    }
}
impl core::ops::Deref for IO5_R {
    type Target = crate::FieldReader<u8, IO5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO5` writer - 11:10\\]
Select mode for AUXIO\\[8i+5\\]."]
pub struct IO5_W<'a> {
    w: &'a mut W,
}
impl<'a> IO5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Open-Source Mode: When GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO5_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\]
is driven low. When GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO5_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 5 is 0: AUXIO\\[8i+5\\]
is enabled for analog signal transfer. When GPIODIE bit 5 is 1: AUXIO\\[8i+5\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO5_A::IN)
    }
    #[doc = "Output Mode: GPIODOUT bit 5 drives AUXIO\\[8i+5\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO5_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "9:8\\]
Select mode for AUXIO\\[8i+4\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO4_A {
    #[doc = "3: Open-Source Mode: \n\nWhen GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: \n\nWhen GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is driven low.  \n\nWhen GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode:\n\nWhen GPIODIE bit 4 is 0: AUXIO\\[8i+4\\]
is enabled for analog signal transfer.\n\nWhen GPIODIE bit 4 is 1: AUXIO\\[8i+4\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode:\n\nGPIODOUT bit 4 drives AUXIO\\[8i+4\\]."]
    OUT = 0,
}
impl From<IO4_A> for u8 {
    #[inline(always)]
    fn from(variant: IO4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO4` reader - 9:8\\]
Select mode for AUXIO\\[8i+4\\]."]
pub struct IO4_R(crate::FieldReader<u8, IO4_A>);
impl IO4_R {
    pub(crate) fn new(bits: u8) -> Self {
        IO4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO4_A {
        match self.bits {
            3 => IO4_A::OPEN_SOURCE,
            2 => IO4_A::OPEN_DRAIN,
            1 => IO4_A::IN,
            0 => IO4_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        **self == IO4_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        **self == IO4_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        **self == IO4_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        **self == IO4_A::OUT
    }
}
impl core::ops::Deref for IO4_R {
    type Target = crate::FieldReader<u8, IO4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO4` writer - 9:8\\]
Select mode for AUXIO\\[8i+4\\]."]
pub struct IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> IO4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Open-Source Mode: When GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO4_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\]
is driven low. When GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO4_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 4 is 0: AUXIO\\[8i+4\\]
is enabled for analog signal transfer. When GPIODIE bit 4 is 1: AUXIO\\[8i+4\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO4_A::IN)
    }
    #[doc = "Output Mode: GPIODOUT bit 4 drives AUXIO\\[8i+4\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO4_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "7:6\\]
Select mode for AUXIO\\[8i+3\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO3_A {
    #[doc = "3: Open-Source Mode: \n\nWhen GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: \n\nWhen GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is driven low.  \n\nWhen GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode:\n\nWhen GPIODIE bit 3 is 0: AUXIO\\[8i+3\\]
is enabled for analog signal transfer.\n\nWhen GPIODIE bit 3 is 1: AUXIO\\[8i+3\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode:\n\nGPIODOUT bit 3 drives AUXIO\\[8i+3\\]."]
    OUT = 0,
}
impl From<IO3_A> for u8 {
    #[inline(always)]
    fn from(variant: IO3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO3` reader - 7:6\\]
Select mode for AUXIO\\[8i+3\\]."]
pub struct IO3_R(crate::FieldReader<u8, IO3_A>);
impl IO3_R {
    pub(crate) fn new(bits: u8) -> Self {
        IO3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO3_A {
        match self.bits {
            3 => IO3_A::OPEN_SOURCE,
            2 => IO3_A::OPEN_DRAIN,
            1 => IO3_A::IN,
            0 => IO3_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        **self == IO3_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        **self == IO3_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        **self == IO3_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        **self == IO3_A::OUT
    }
}
impl core::ops::Deref for IO3_R {
    type Target = crate::FieldReader<u8, IO3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO3` writer - 7:6\\]
Select mode for AUXIO\\[8i+3\\]."]
pub struct IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> IO3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Open-Source Mode: When GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO3_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\]
is driven low. When GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO3_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 3 is 0: AUXIO\\[8i+3\\]
is enabled for analog signal transfer. When GPIODIE bit 3 is 1: AUXIO\\[8i+3\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO3_A::IN)
    }
    #[doc = "Output Mode: GPIODOUT bit 3 drives AUXIO\\[8i+3\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO3_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "5:4\\]
Select mode for AUXIO\\[8i+2\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO2_A {
    #[doc = "3: Open-Source Mode: \n\nWhen GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: \n\nWhen GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is driven low.  \n\nWhen GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode:\n\nWhen GPIODIE bit 2 is 0: AUXIO\\[8i+2\\]
is enabled for analog signal transfer.\n\nWhen GPIODIE bit 2 is 1: AUXIO\\[8i+2\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode:\n\nGPIODOUT bit 2 drives AUXIO\\[8i+2\\]."]
    OUT = 0,
}
impl From<IO2_A> for u8 {
    #[inline(always)]
    fn from(variant: IO2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO2` reader - 5:4\\]
Select mode for AUXIO\\[8i+2\\]."]
pub struct IO2_R(crate::FieldReader<u8, IO2_A>);
impl IO2_R {
    pub(crate) fn new(bits: u8) -> Self {
        IO2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO2_A {
        match self.bits {
            3 => IO2_A::OPEN_SOURCE,
            2 => IO2_A::OPEN_DRAIN,
            1 => IO2_A::IN,
            0 => IO2_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        **self == IO2_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        **self == IO2_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        **self == IO2_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        **self == IO2_A::OUT
    }
}
impl core::ops::Deref for IO2_R {
    type Target = crate::FieldReader<u8, IO2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO2` writer - 5:4\\]
Select mode for AUXIO\\[8i+2\\]."]
pub struct IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> IO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Open-Source Mode: When GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO2_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\]
is driven low. When GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO2_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 2 is 0: AUXIO\\[8i+2\\]
is enabled for analog signal transfer. When GPIODIE bit 2 is 1: AUXIO\\[8i+2\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO2_A::IN)
    }
    #[doc = "Output Mode: GPIODOUT bit 2 drives AUXIO\\[8i+2\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO2_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "3:2\\]
Select mode for AUXIO\\[8i+1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO1_A {
    #[doc = "3: Open-Source Mode: \n\nWhen GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: \n\nWhen GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is driven low.  \n\nWhen GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode:\n\nWhen GPIODIE bit 1 is 0: AUXIO\\[8i+1\\]
is enabled for analog signal transfer.\n\nWhen GPIODIE bit 1 is 1: AUXIO\\[8i+1\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode:\n\nGPIODOUT bit 1 drives AUXIO\\[8i+1\\]."]
    OUT = 0,
}
impl From<IO1_A> for u8 {
    #[inline(always)]
    fn from(variant: IO1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO1` reader - 3:2\\]
Select mode for AUXIO\\[8i+1\\]."]
pub struct IO1_R(crate::FieldReader<u8, IO1_A>);
impl IO1_R {
    pub(crate) fn new(bits: u8) -> Self {
        IO1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO1_A {
        match self.bits {
            3 => IO1_A::OPEN_SOURCE,
            2 => IO1_A::OPEN_DRAIN,
            1 => IO1_A::IN,
            0 => IO1_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        **self == IO1_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        **self == IO1_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        **self == IO1_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        **self == IO1_A::OUT
    }
}
impl core::ops::Deref for IO1_R {
    type Target = crate::FieldReader<u8, IO1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO1` writer - 3:2\\]
Select mode for AUXIO\\[8i+1\\]."]
pub struct IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> IO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Open-Source Mode: When GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO1_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\]
is driven low. When GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO1_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 1 is 0: AUXIO\\[8i+1\\]
is enabled for analog signal transfer. When GPIODIE bit 1 is 1: AUXIO\\[8i+1\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO1_A::IN)
    }
    #[doc = "Output Mode: GPIODOUT bit 1 drives AUXIO\\[8i+1\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO1_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "1:0\\]
Select mode for AUXIO\\[8i+0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO0_A {
    #[doc = "3: Open-Source Mode: \n\nWhen GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is driven high."]
    OPEN_SOURCE = 3,
    #[doc = "2: Open-Drain Mode: \n\nWhen GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is driven low.  \n\nWhen GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 2,
    #[doc = "1: Input Mode:\n\nWhen GPIODIE bit 0 is 0: AUXIO\\[8i+0\\]
is enabled for analog signal transfer.\n\nWhen GPIODIE bit 0 is 1: AUXIO\\[8i+0\\]
is enabled for digital input."]
    IN = 1,
    #[doc = "0: Output Mode:\n\nGPIODOUT bit 0 drives AUXIO\\[8i+0\\]."]
    OUT = 0,
}
impl From<IO0_A> for u8 {
    #[inline(always)]
    fn from(variant: IO0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO0` reader - 1:0\\]
Select mode for AUXIO\\[8i+0\\]."]
pub struct IO0_R(crate::FieldReader<u8, IO0_A>);
impl IO0_R {
    pub(crate) fn new(bits: u8) -> Self {
        IO0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO0_A {
        match self.bits {
            3 => IO0_A::OPEN_SOURCE,
            2 => IO0_A::OPEN_DRAIN,
            1 => IO0_A::IN,
            0 => IO0_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline(always)]
    pub fn is_open_source(&self) -> bool {
        **self == IO0_A::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        **self == IO0_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        **self == IO0_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        **self == IO0_A::OUT
    }
}
impl core::ops::Deref for IO0_R {
    type Target = crate::FieldReader<u8, IO0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO0` writer - 1:0\\]
Select mode for AUXIO\\[8i+0\\]."]
pub struct IO0_W<'a> {
    w: &'a mut W,
}
impl<'a> IO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Open-Source Mode: When GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is driven high."]
    #[inline(always)]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO0_A::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\]
is driven low. When GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\]
is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO0_A::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 0 is 0: AUXIO\\[8i+0\\]
is enabled for analog signal transfer. When GPIODIE bit 0 is 1: AUXIO\\[8i+0\\]
is enabled for digital input."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO0_A::IN)
    }
    #[doc = "Output Mode: GPIODOUT bit 0 drives AUXIO\\[8i+0\\]."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO0_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
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
    #[doc = "Bits 14:15 - 15:14\\]
Select mode for AUXIO\\[8i+7\\]."]
    #[inline(always)]
    pub fn io7(&self) -> IO7_R {
        IO7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Select mode for AUXIO\\[8i+6\\]."]
    #[inline(always)]
    pub fn io6(&self) -> IO6_R {
        IO6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Select mode for AUXIO\\[8i+5\\]."]
    #[inline(always)]
    pub fn io5(&self) -> IO5_R {
        IO5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Select mode for AUXIO\\[8i+4\\]."]
    #[inline(always)]
    pub fn io4(&self) -> IO4_R {
        IO4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Select mode for AUXIO\\[8i+3\\]."]
    #[inline(always)]
    pub fn io3(&self) -> IO3_R {
        IO3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select mode for AUXIO\\[8i+2\\]."]
    #[inline(always)]
    pub fn io2(&self) -> IO2_R {
        IO2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Select mode for AUXIO\\[8i+1\\]."]
    #[inline(always)]
    pub fn io1(&self) -> IO1_R {
        IO1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Select mode for AUXIO\\[8i+0\\]."]
    #[inline(always)]
    pub fn io0(&self) -> IO0_R {
        IO0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 14:15 - 15:14\\]
Select mode for AUXIO\\[8i+7\\]."]
    #[inline(always)]
    pub fn io7(&mut self) -> IO7_W {
        IO7_W { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\]
Select mode for AUXIO\\[8i+6\\]."]
    #[inline(always)]
    pub fn io6(&mut self) -> IO6_W {
        IO6_W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\]
Select mode for AUXIO\\[8i+5\\]."]
    #[inline(always)]
    pub fn io5(&mut self) -> IO5_W {
        IO5_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
Select mode for AUXIO\\[8i+4\\]."]
    #[inline(always)]
    pub fn io4(&mut self) -> IO4_W {
        IO4_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Select mode for AUXIO\\[8i+3\\]."]
    #[inline(always)]
    pub fn io3(&mut self) -> IO3_W {
        IO3_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select mode for AUXIO\\[8i+2\\]."]
    #[inline(always)]
    pub fn io2(&mut self) -> IO2_W {
        IO2_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
Select mode for AUXIO\\[8i+1\\]."]
    #[inline(always)]
    pub fn io1(&mut self) -> IO1_W {
        IO1_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Select mode for AUXIO\\[8i+0\\]."]
    #[inline(always)]
    pub fn io0(&mut self) -> IO0_W {
        IO0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iomode](index.html) module"]
pub struct IOMODE_SPEC;
impl crate::RegisterSpec for IOMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iomode::R](R) reader structure"]
impl crate::Readable for IOMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iomode::W](W) writer structure"]
impl crate::Writable for IOMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOMODE to value 0"]
impl crate::Resettable for IOMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
