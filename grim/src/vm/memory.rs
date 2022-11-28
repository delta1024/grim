use crate::{
    err::VmError,
    lang_core::{
        objects::{ObjString, Object},
        prelude::{ObjectPointer, StringPointer},
        Type,
    },
};
use std::{
    collections::{HashMap, HashSet, LinkedList},
    pin::Pin,
};

use super::Result;
macro_rules! error {
    ($string: tt, $($var: expr),*) => {
      VmError::new(format!($string, $($var,)*))
    };
    ($string: tt) => {
        VmError::new($string.into())
    }
}

pub struct Memory {
    globals: Option<HashMap<StringPointer, Type>>,
    strings: Option<HashSet<ObjString>>,
    objects: LinkedList<Pin<Box<Object>>>,
}
impl Memory {
    pub const fn new() -> Self {
        Self {
            globals: None,
            strings: None,
            objects: LinkedList::new(),
        }
    }
    pub fn allocate_string(&mut self, string: &str) -> StringPointer {
        let key = ObjString::new(string);
        match self
            .strings
            .as_ref()
            .expect("could not get table")
            .get(&key)
        {
            None => {
                self.strings
                    .as_mut()
                    .expect("could not get table")
                    .insert(key.clone());
                StringPointer::new(
                    self.strings
                        .as_ref()
                        .expect("Could not get table")
                        .get(&key)
                        .unwrap(),
                )
            }
            Some(s) => StringPointer::new(s),
        }
    }
    pub fn set_global(&mut self, key: StringPointer, value: Type) -> Option<Type> {
        self.globals
            .as_mut()
            .expect("initialized vm")
            .insert(key, value)
    }
    pub fn assign_global(&mut self, key: StringPointer, value: Type) -> Result<()> {
        let Some(old) = self.set_global(key, value) else {
                        self.remove_global(key);
                        return error!("Undefined variable '{}'", key);
        };
        if !old.types_equal(&value) {
            self.set_global(key, old);
            error!("Type mismatch.")
        } else {
            Ok(())
        }
    }
    pub fn remove_global(&mut self, key: StringPointer) {
        self.globals.as_mut().expect("initialized vm").remove(&key);
    }
    pub fn get_global(&self, key: StringPointer) -> Option<Type> {
        match self.globals.as_ref().expect("uninitialized vm").get(&key) {
            Some(t) => Some(*t),
            None => None,
        }
    }
    pub fn allocate_object<T: Into<Object>>(&'static mut self, obj: T) -> ObjectPointer {
        self.objects.push_back(Box::pin(obj.into()));
        self.objects.back().expect("could not allocate object");
        ObjectPointer::from(
            self.objects
                .back()
                .expect("could not get object")
                .as_ref()
                .get_ref(),
        )
    }
    pub fn initialize_memory(&mut self) {
        _ = self.globals.insert(HashMap::new());
        _ = self.strings.insert(HashSet::new());
    }
}
#[macro_export]
macro_rules! allocate_object {
    ($obj: expr) => {
        crate::vm::VM.lock().memory.allocate_object($obj)
    };
}

#[macro_export]
macro_rules! allocate_string {
    ($str: expr) => {
        crate::vm::VM.lock().memory.allocate_string($str)
    };
}
