#[doc = "Register `KEYWRITEAREA` reader"]
pub struct R(crate::R<KEYWRITEAREA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYWRITEAREA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYWRITEAREA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYWRITEAREA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYWRITEAREA` writer"]
pub struct W(crate::W<KEYWRITEAREA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYWRITEAREA_SPEC>;
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
impl From<crate::W<KEYWRITEAREA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYWRITEAREA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED8_R(crate::FieldReader<u32, u32>);
impl RESERVED8_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED8_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "7:7\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA7_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA7_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA7` reader - 7:7\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub struct RAM_AREA7_R(crate::FieldReader<bool, RAM_AREA7_A>);
impl RAM_AREA7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA7_A {
        match self.bits {
            true => RAM_AREA7_A::SEL,
            false => RAM_AREA7_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        **self == RAM_AREA7_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        **self == RAM_AREA7_A::NOT_SEL
    }
}
impl core::ops::Deref for RAM_AREA7_R {
    type Target = crate::FieldReader<bool, RAM_AREA7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA7` writer - 7:7\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub struct RAM_AREA7_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA7_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA7_A::NOT_SEL)
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
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA6_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA6_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA6` reader - 6:6\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub struct RAM_AREA6_R(crate::FieldReader<bool, RAM_AREA6_A>);
impl RAM_AREA6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA6_A {
        match self.bits {
            true => RAM_AREA6_A::SEL,
            false => RAM_AREA6_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        **self == RAM_AREA6_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        **self == RAM_AREA6_A::NOT_SEL
    }
}
impl core::ops::Deref for RAM_AREA6_R {
    type Target = crate::FieldReader<bool, RAM_AREA6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA6` writer - 6:6\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub struct RAM_AREA6_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA6_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA6_A::NOT_SEL)
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
#[doc = "5:5\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA5_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA5_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA5` reader - 5:5\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub struct RAM_AREA5_R(crate::FieldReader<bool, RAM_AREA5_A>);
impl RAM_AREA5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA5_A {
        match self.bits {
            true => RAM_AREA5_A::SEL,
            false => RAM_AREA5_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        **self == RAM_AREA5_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        **self == RAM_AREA5_A::NOT_SEL
    }
}
impl core::ops::Deref for RAM_AREA5_R {
    type Target = crate::FieldReader<bool, RAM_AREA5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA5` writer - 5:5\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub struct RAM_AREA5_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA5_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA5_A::NOT_SEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "4:4\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA4_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA4_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA4` reader - 4:4\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub struct RAM_AREA4_R(crate::FieldReader<bool, RAM_AREA4_A>);
impl RAM_AREA4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA4_A {
        match self.bits {
            true => RAM_AREA4_A::SEL,
            false => RAM_AREA4_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        **self == RAM_AREA4_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        **self == RAM_AREA4_A::NOT_SEL
    }
}
impl core::ops::Deref for RAM_AREA4_R {
    type Target = crate::FieldReader<bool, RAM_AREA4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA4` writer - 4:4\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub struct RAM_AREA4_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA4_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA4_A::NOT_SEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "3:3\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA3_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA3_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA3` reader - 3:3\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub struct RAM_AREA3_R(crate::FieldReader<bool, RAM_AREA3_A>);
impl RAM_AREA3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA3_A {
        match self.bits {
            true => RAM_AREA3_A::SEL,
            false => RAM_AREA3_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        **self == RAM_AREA3_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        **self == RAM_AREA3_A::NOT_SEL
    }
}
impl core::ops::Deref for RAM_AREA3_R {
    type Target = crate::FieldReader<bool, RAM_AREA3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA3` writer - 3:3\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub struct RAM_AREA3_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA3_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA3_A::NOT_SEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "2:2\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA2_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA2_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA2` reader - 2:2\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub struct RAM_AREA2_R(crate::FieldReader<bool, RAM_AREA2_A>);
impl RAM_AREA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA2_A {
        match self.bits {
            true => RAM_AREA2_A::SEL,
            false => RAM_AREA2_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        **self == RAM_AREA2_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        **self == RAM_AREA2_A::NOT_SEL
    }
}
impl core::ops::Deref for RAM_AREA2_R {
    type Target = crate::FieldReader<bool, RAM_AREA2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA2` writer - 2:2\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub struct RAM_AREA2_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA2_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA2_A::NOT_SEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "1:1\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA1_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA1_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA1` reader - 1:1\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub struct RAM_AREA1_R(crate::FieldReader<bool, RAM_AREA1_A>);
impl RAM_AREA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA1_A {
        match self.bits {
            true => RAM_AREA1_A::SEL,
            false => RAM_AREA1_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        **self == RAM_AREA1_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        **self == RAM_AREA1_A::NOT_SEL
    }
}
impl core::ops::Deref for RAM_AREA1_R {
    type Target = crate::FieldReader<bool, RAM_AREA1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA1` writer - 1:1\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub struct RAM_AREA1_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA1_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA1_A::NOT_SEL)
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
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA0_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA0_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA0` reader - 0:0\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub struct RAM_AREA0_R(crate::FieldReader<bool, RAM_AREA0_A>);
impl RAM_AREA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA0_A {
        match self.bits {
            true => RAM_AREA0_A::SEL,
            false => RAM_AREA0_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        **self == RAM_AREA0_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        **self == RAM_AREA0_A::NOT_SEL
    }
}
impl core::ops::Deref for RAM_AREA0_R {
    type Target = crate::FieldReader<bool, RAM_AREA0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA0` writer - 0:0\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub struct RAM_AREA0_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_AREA0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA0_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA0_A::NOT_SEL)
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
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 7 - 7:7\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area7(&self) -> RAM_AREA7_R {
        RAM_AREA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area6(&self) -> RAM_AREA6_R {
        RAM_AREA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area5(&self) -> RAM_AREA5_R {
        RAM_AREA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area4(&self) -> RAM_AREA4_R {
        RAM_AREA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area3(&self) -> RAM_AREA3_R {
        RAM_AREA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area2(&self) -> RAM_AREA2_R {
        RAM_AREA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area1(&self) -> RAM_AREA1_R {
        RAM_AREA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area0(&self) -> RAM_AREA0_R {
        RAM_AREA0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area7(&mut self) -> RAM_AREA7_W {
        RAM_AREA7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area6(&mut self) -> RAM_AREA6_W {
        RAM_AREA6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area5(&mut self) -> RAM_AREA5_W {
        RAM_AREA5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area4(&mut self) -> RAM_AREA4_W {
        RAM_AREA4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area3(&mut self) -> RAM_AREA3_W {
        RAM_AREA3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area2(&mut self) -> RAM_AREA2_W {
        RAM_AREA2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area1(&mut self) -> RAM_AREA1_W {
        RAM_AREA1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area0(&mut self) -> RAM_AREA0_W {
        RAM_AREA0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Write Area\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keywritearea](index.html) module"]
pub struct KEYWRITEAREA_SPEC;
impl crate::RegisterSpec for KEYWRITEAREA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keywritearea::R](R) reader structure"]
impl crate::Readable for KEYWRITEAREA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keywritearea::W](W) writer structure"]
impl crate::Writable for KEYWRITEAREA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYWRITEAREA to value 0"]
impl crate::Resettable for KEYWRITEAREA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
