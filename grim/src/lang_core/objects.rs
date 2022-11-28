use crate::lang_core::Type;
use std::fmt::{self, Display};
pub trait Pointable {
    type Obj;
    fn get_ref(self) -> Option<&'static Self::Obj>;
    fn as_raw(self) -> *const Self::Obj;
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq)]
pub enum ObjectPointer {
    String(StringPointer),
}

impl Display for ObjectPointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ObjectPointer::String(s) => format!("{}", s),
            },
        )
    }
}
impl From<&'static Object> for ObjectPointer {
    fn from(o: &'static Object) -> Self {
        match o {
            Object::String(s) => ObjectPointer::String(StringPointer::from(s)),
        }
    }
}
#[derive(Debug)]
pub enum Object {
    String(ObjString),
}
unsafe impl Send for Object {}
unsafe impl Sync for Object {}
impl Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq)]
pub struct ObjString {
    chars: Vec<u8>,
}

unsafe impl Send for ObjString {}
unsafe impl Sync for ObjString {}
impl ObjString {
    pub fn new(message: &str) -> Self {
        Self {
            chars: message.chars().map(|x| x as u8).collect(),
            ..Default::default()
        }
    }
    pub fn len(&self) -> usize {
        self.chars.len()
    }
}
impl From<&'static ObjString> for StringPointer {
    fn from(s: &'static ObjString) -> Self {
        Self(s)
    }
}
impl From<ObjString> for Object {
    fn from(s: ObjString) -> Self {
        Self::String(s)
    }
}
impl Display for ObjString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = if self.chars[0] as char == '"' {
            unsafe { std::str::from_utf8_unchecked(&self.chars[1..self.len() - 1]) }
        } else {
            unsafe { std::str::from_utf8_unchecked(&self.chars[..]) }
        };
        write!(f, "{}", string)
    }
}

impl From<StringPointer> for Type {
    fn from(s: StringPointer) -> Self {
        Type::Object(ObjectPointer::String(s))
    }
}
impl Default for ObjString {
    fn default() -> Self {
        Self {
            chars: Vec::default(),
        }
    }
}
impl std::ops::Deref for ObjString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        unsafe { std::str::from_utf8_unchecked(&self.chars[..]) }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct StringPointer(*const ObjString);
impl StringPointer {
    pub fn new(ptr: *const ObjString) -> Self {
        Self(ptr)
    }
}
unsafe impl Send for StringPointer {}
unsafe impl Sync for StringPointer {}
impl Display for StringPointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = unsafe { self.0.as_ref().expect("valid pointer") };
        write!(f, "{}", s)
    }
}
impl Default for StringPointer {
    fn default() -> Self {
        Self(std::ptr::null())
    }
}
impl Pointable for StringPointer {
    type Obj = ObjString;

    fn get_ref(self) -> Option<&'static Self::Obj> {
        unsafe { self.0.as_ref() }
    }
    fn as_raw(self) -> *const Self::Obj {
        self.0
    }
}
impl From<StringPointer> for ObjectPointer {
    fn from(s: StringPointer) -> Self {
        Self::String(s)
    }
}
