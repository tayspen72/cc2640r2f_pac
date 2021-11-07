#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED15` reader - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED15_R(crate::FieldReader<u32, u32>);
impl RESERVED15_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED15_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED15` writer - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED15_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 15)) | ((value as u32 & 0x0001_ffff) << 15);
        self.w
    }
}
#[doc = "14:14\\]
GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBPWML_A {
    #[doc = "1: Inverted"]
    INVERTED = 1,
    #[doc = "0: Not inverted"]
    NORMAL = 0,
}
impl From<TBPWML_A> for bool {
    #[inline(always)]
    fn from(variant: TBPWML_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBPWML` reader - 14:14\\]
GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted."]
pub struct TBPWML_R(crate::FieldReader<bool, TBPWML_A>);
impl TBPWML_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBPWML_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBPWML_A {
        match self.bits {
            true => TBPWML_A::INVERTED,
            false => TBPWML_A::NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == TBPWML_A::INVERTED
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == TBPWML_A::NORMAL
    }
}
impl core::ops::Deref for TBPWML_R {
    type Target = crate::FieldReader<bool, TBPWML_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBPWML` writer - 14:14\\]
GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted."]
pub struct TBPWML_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPWML_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBPWML_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TBPWML_A::INVERTED)
    }
    #[doc = "Not inverted"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TBPWML_A::NORMAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `RESERVED12` reader - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED12_R(crate::FieldReader<u8, u8>);
impl RESERVED12_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED12` writer - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "11:10\\]
GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TBEVENT_A {
    #[doc = "3: Both edges"]
    BOTH = 3,
    #[doc = "1: Negative edge"]
    NEG = 1,
    #[doc = "0: Positive edge"]
    POS = 0,
}
impl From<TBEVENT_A> for u8 {
    #[inline(always)]
    fn from(variant: TBEVENT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TBEVENT` reader - 11:10\\]
GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
pub struct TBEVENT_R(crate::FieldReader<u8, TBEVENT_A>);
impl TBEVENT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TBEVENT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TBEVENT_A> {
        match self.bits {
            3 => Some(TBEVENT_A::BOTH),
            1 => Some(TBEVENT_A::NEG),
            0 => Some(TBEVENT_A::POS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == TBEVENT_A::BOTH
    }
    #[doc = "Checks if the value of the field is `NEG`"]
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        **self == TBEVENT_A::NEG
    }
    #[doc = "Checks if the value of the field is `POS`"]
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        **self == TBEVENT_A::POS
    }
}
impl core::ops::Deref for TBEVENT_R {
    type Target = crate::FieldReader<u8, TBEVENT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBEVENT` writer - 11:10\\]
GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
pub struct TBEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBEVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBEVENT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(TBEVENT_A::BOTH)
    }
    #[doc = "Negative edge"]
    #[inline(always)]
    pub fn neg(self) -> &'a mut W {
        self.variant(TBEVENT_A::NEG)
    }
    #[doc = "Positive edge"]
    #[inline(always)]
    pub fn pos(self) -> &'a mut W {
        self.variant(TBEVENT_A::POS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "9:9\\]
GPT Timer B Stall Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBSTALL_A {
    #[doc = "1: Timer B freezes counting while the processor is halted by the debugger."]
    EN = 1,
    #[doc = "0: Timer B continues counting while the processor is halted by the debugger."]
    DIS = 0,
}
impl From<TBSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: TBSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBSTALL` reader - 9:9\\]
GPT Timer B Stall Enable"]
pub struct TBSTALL_R(crate::FieldReader<bool, TBSTALL_A>);
impl TBSTALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBSTALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBSTALL_A {
        match self.bits {
            true => TBSTALL_A::EN,
            false => TBSTALL_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TBSTALL_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TBSTALL_A::DIS
    }
}
impl core::ops::Deref for TBSTALL_R {
    type Target = crate::FieldReader<bool, TBSTALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBSTALL` writer - 9:9\\]
GPT Timer B Stall Enable"]
pub struct TBSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TBSTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBSTALL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer B freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TBSTALL_A::EN)
    }
    #[doc = "Timer B continues counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBSTALL_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "8:8\\]
GPT Timer B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBEN_A {
    #[doc = "1: Timer B is enabled and begins counting or the capture logic is enabled based on CFG register."]
    EN = 1,
    #[doc = "0: Timer B is disabled."]
    DIS = 0,
}
impl From<TBEN_A> for bool {
    #[inline(always)]
    fn from(variant: TBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBEN` reader - 8:8\\]
GPT Timer B Enable"]
pub struct TBEN_R(crate::FieldReader<bool, TBEN_A>);
impl TBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBEN_A {
        match self.bits {
            true => TBEN_A::EN,
            false => TBEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TBEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TBEN_A::DIS
    }
}
impl core::ops::Deref for TBEN_R {
    type Target = crate::FieldReader<bool, TBEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBEN` writer - 8:8\\]
GPT Timer B Enable"]
pub struct TBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer B is enabled and begins counting or the capture logic is enabled based on CFG register."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TBEN_A::EN)
    }
    #[doc = "Timer B is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBEN_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED7_R(crate::FieldReader<bool, bool>);
impl RESERVED7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
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
GPT Timer A PWM Output Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAPWML_A {
    #[doc = "1: Inverted"]
    INVERTED = 1,
    #[doc = "0: Not inverted"]
    NORMAL = 0,
}
impl From<TAPWML_A> for bool {
    #[inline(always)]
    fn from(variant: TAPWML_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAPWML` reader - 6:6\\]
GPT Timer A PWM Output Level"]
pub struct TAPWML_R(crate::FieldReader<bool, TAPWML_A>);
impl TAPWML_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAPWML_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAPWML_A {
        match self.bits {
            true => TAPWML_A::INVERTED,
            false => TAPWML_A::NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == TAPWML_A::INVERTED
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == TAPWML_A::NORMAL
    }
}
impl core::ops::Deref for TAPWML_R {
    type Target = crate::FieldReader<bool, TAPWML_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAPWML` writer - 6:6\\]
GPT Timer A PWM Output Level"]
pub struct TAPWML_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPWML_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAPWML_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TAPWML_A::INVERTED)
    }
    #[doc = "Not inverted"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TAPWML_A::NORMAL)
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
#[doc = "Field `RESERVED4` reader - 5:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_R(crate::FieldReader<u8, u8>);
impl RESERVED4_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED4` writer - 5:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "3:2\\]
GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAEVENT_A {
    #[doc = "3: Both edges"]
    BOTH = 3,
    #[doc = "1: Negative edge"]
    NEG = 1,
    #[doc = "0: Positive edge"]
    POS = 0,
}
impl From<TAEVENT_A> for u8 {
    #[inline(always)]
    fn from(variant: TAEVENT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TAEVENT` reader - 3:2\\]
GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
pub struct TAEVENT_R(crate::FieldReader<u8, TAEVENT_A>);
impl TAEVENT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAEVENT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TAEVENT_A> {
        match self.bits {
            3 => Some(TAEVENT_A::BOTH),
            1 => Some(TAEVENT_A::NEG),
            0 => Some(TAEVENT_A::POS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == TAEVENT_A::BOTH
    }
    #[doc = "Checks if the value of the field is `NEG`"]
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        **self == TAEVENT_A::NEG
    }
    #[doc = "Checks if the value of the field is `POS`"]
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        **self == TAEVENT_A::POS
    }
}
impl core::ops::Deref for TAEVENT_R {
    type Target = crate::FieldReader<u8, TAEVENT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAEVENT` writer - 3:2\\]
GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
pub struct TAEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAEVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAEVENT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(TAEVENT_A::BOTH)
    }
    #[doc = "Negative edge"]
    #[inline(always)]
    pub fn neg(self) -> &'a mut W {
        self.variant(TAEVENT_A::NEG)
    }
    #[doc = "Positive edge"]
    #[inline(always)]
    pub fn pos(self) -> &'a mut W {
        self.variant(TAEVENT_A::POS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "1:1\\]
GPT Timer A Stall Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASTALL_A {
    #[doc = "1: Timer A freezes counting while the processor is halted by the debugger."]
    EN = 1,
    #[doc = "0: Timer A continues counting while the processor is halted by the debugger."]
    DIS = 0,
}
impl From<TASTALL_A> for bool {
    #[inline(always)]
    fn from(variant: TASTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASTALL` reader - 1:1\\]
GPT Timer A Stall Enable"]
pub struct TASTALL_R(crate::FieldReader<bool, TASTALL_A>);
impl TASTALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TASTALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TASTALL_A {
        match self.bits {
            true => TASTALL_A::EN,
            false => TASTALL_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TASTALL_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TASTALL_A::DIS
    }
}
impl core::ops::Deref for TASTALL_R {
    type Target = crate::FieldReader<bool, TASTALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TASTALL` writer - 1:1\\]
GPT Timer A Stall Enable"]
pub struct TASTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TASTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASTALL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer A freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TASTALL_A::EN)
    }
    #[doc = "Timer A continues counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TASTALL_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "0:0\\]
GPT Timer A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAEN_A {
    #[doc = "1: Timer A is enabled and begins counting or the capture logic is enabled based on the CFG register."]
    EN = 1,
    #[doc = "0: Timer A is disabled."]
    DIS = 0,
}
impl From<TAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAEN` reader - 0:0\\]
GPT Timer A Enable"]
pub struct TAEN_R(crate::FieldReader<bool, TAEN_A>);
impl TAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAEN_A {
        match self.bits {
            true => TAEN_A::EN,
            false => TAEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TAEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TAEN_A::DIS
    }
}
impl core::ops::Deref for TAEN_R {
    type Target = crate::FieldReader<bool, TAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAEN` writer - 0:0\\]
GPT Timer A Enable"]
pub struct TAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer A is enabled and begins counting or the capture logic is enabled based on the CFG register."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TAEN_A::EN)
    }
    #[doc = "Timer A is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TAEN_A::DIS)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new(((self.bits >> 15) & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 14 - 14:14\\]
GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tbpwml(&self) -> TBPWML_R {
        TBPWML_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline(always)]
    pub fn tbevent(&self) -> TBEVENT_R {
        TBEVENT_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Stall Enable"]
    #[inline(always)]
    pub fn tbstall(&self) -> TBSTALL_R {
        TBSTALL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Enable"]
    #[inline(always)]
    pub fn tben(&self) -> TBEN_R {
        TBEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
GPT Timer A PWM Output Level"]
    #[inline(always)]
    pub fn tapwml(&self) -> TAPWML_R {
        TAPWML_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline(always)]
    pub fn taevent(&self) -> TAEVENT_R {
        TAEVENT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Stall Enable"]
    #[inline(always)]
    pub fn tastall(&self) -> TASTALL_R {
        TASTALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
GPT Timer A Enable"]
    #[inline(always)]
    pub fn taen(&self) -> TAEN_R {
        TAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&mut self) -> RESERVED15_W {
        RESERVED15_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\]
GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tbpwml(&mut self) -> TBPWML_W {
        TBPWML_W { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\]
GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline(always)]
    pub fn tbevent(&mut self) -> TBEVENT_W {
        TBEVENT_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Stall Enable"]
    #[inline(always)]
    pub fn tbstall(&mut self) -> TBSTALL_W {
        TBSTALL_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Enable"]
    #[inline(always)]
    pub fn tben(&mut self) -> TBEN_W {
        TBEN_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
GPT Timer A PWM Output Level"]
    #[inline(always)]
    pub fn tapwml(&mut self) -> TAPWML_W {
        TAPWML_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline(always)]
    pub fn taevent(&mut self) -> TAEVENT_W {
        TAEVENT_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Stall Enable"]
    #[inline(always)]
    pub fn tastall(&mut self) -> TASTALL_W {
        TASTALL_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
GPT Timer A Enable"]
    #[inline(always)]
    pub fn taen(&mut self) -> TAEN_W {
        TAEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
