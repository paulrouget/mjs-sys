//! # mJS Rust bindings
//!
//! mJS documentation: <https://github.com/cesanta/mjs>
//!
//! ## Life time considerations:
//!
//! [Val] lifetime hasn't been properly solidified yet.
//! Calls to [`VM::exec()`] or [`Val::call()`] might make invalidate
//! some previously acquired values.
//! Use [`Val::own()`] to make sure a JS value is not garbage collected.

#![no_std]
#![deny(missing_docs, trivial_numeric_casts, unused_extern_crates)]
#![warn(unused_import_braces)]
#![cfg_attr(
  feature = "cargo-clippy",
  allow(clippy::new_without_default, clippy::new_without_default)
)]
#![cfg_attr(
  feature = "cargo-clippy",
  warn(
    clippy::float_arithmetic,
    clippy::mut_mut,
    clippy::nonminimal_bool,
    clippy::map_unwrap_or,
    clippy::print_stdout,
  )
)]

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(missing_docs)]
#[allow(clippy::all)]
mod sys {
  include!(concat!(env!("OUT_DIR"), "/mjs.rs"));
}

use cstr_core::CStr;
// Inner mJS object
pub use sys::mjs;
use sys::*;

/// mJS virtual machine
#[derive(Clone)]
pub struct VM {
  inner: *mut mjs,
}

/// JS value
pub struct Val {
  vm: VM,
  inner: mjs_val_t,
}

/// Execution error
#[derive(Debug)]
pub enum JSError<'a> {
  /// String does't end with `\0`
  NonNullTerminatedString,
  /// Trying to exectute a Val that's not a function
  NotAFunction,
  /// Too many arguments
  TooManyArgs,
  /// VM execution error
  VMError(&'a str),
}

impl<'a> core::fmt::Display for JSError<'a> {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl VM {
  fn get_error<'a>(&mut self, err: mjs_err_t) -> JSError<'a> {
    unsafe {
      let msg = mjs_strerror(self.inner, err);
      JSError::VMError(CStr::from_ptr(msg).to_str().unwrap())
    }
  }

  /// Create new VM
  pub fn create() -> VM {
    VM {
      inner: unsafe { mjs_create() },
    }
  }

  /// Destroy VM
  pub fn destroy(self) {
    unsafe { mjs_destroy(self.inner) }
  }

  /// Create a VM from an existing instance
  pub fn from_inner(mjs: *mut mjs) -> VM {
    VM { inner: mjs }
  }

  fn get_inner(&self) -> *mut mjs {
    self.inner
  }

  /// Execute code. `source` should be null terminated.
  pub fn exec(&mut self, source: &[u8]) -> Result<Val, JSError> {
    if !matches!(source.last(), Some(0)) {
      return Err(JSError::NonNullTerminatedString);
    }
    let mut ret: mjs_val_t = 0;
    let err = unsafe { mjs_exec(self.inner, source.as_ptr() as _, &mut ret) };
    if err != mjs_err_MJS_OK {
      Err(self.get_error(err))
    } else {
      Ok(self.val(ret))
    }
  }

  /// Return the VM global
  pub fn global(&mut self) -> Val {
    self.val(unsafe { mjs_get_global(self.inner) })
  }

  /// Return scope's `this`
  pub fn this(&mut self) -> Val {
    self.val(unsafe { mjs_get_this(self.inner) })
  }

  /// In a function, return the number of arguments.
  pub fn nargs(&mut self) -> i32 {
    unsafe { mjs_nargs(self.inner) }
  }

  /// Return argument `i`
  pub fn arg(&mut self, i: i32) -> Option<Val> {
    unsafe {
      let val = mjs_arg(self.inner, i);
      if mjs_is_undefined(val) > 0 {
        None
      } else {
        Some(self.val(val))
      }
    }
  }

  /// Create a JS `undefined` value
  pub fn make_undefined(&mut self) -> Val {
    self.val(unsafe { mjs_mk_undefined() })
  }

  /// Create a JS value that wraps a native pointer
  #[allow(clippy::not_unsafe_ptr_arg_deref)]
  pub fn make_foreign(&mut self, f: *mut cty::c_void) -> Val {
    self.val(unsafe { mjs_mk_foreign(self.inner, f) })
  }

  /// Create a JS number
  pub fn make_number(&mut self, number: f64) -> Val {
    self.val(unsafe { mjs_mk_number(self.inner, number) })
  }

  /// Create a JS bool
  pub fn make_boolean(&mut self, b: bool) -> Val {
    self.val(unsafe { mjs_mk_boolean(self.inner, if b { 1 } else { 0 }) })
  }

  /// Create a JS object
  pub fn make_object(&mut self) -> Val {
    self.val(unsafe { mjs_mk_object(self.inner) })
  }

  /// Create a JS string. Bytes don't need to be null terminated.
  /// Content of the string **is not** copied.
  pub fn make_string(&mut self, bytes: &'static [u8]) -> Result<Val, JSError> {
    let s = unsafe { mjs_mk_string(self.inner, bytes.as_ptr() as _, bytes.len() as _, 0) };
    Ok(self.val(s))
  }

  /// Create a JS string. Bytes don't need to be null terminated.
  /// Content of the string **is** copied.
  pub fn make_string_copy(&mut self, bytes: &[u8]) -> Result<Val, JSError> {
    let s = unsafe { mjs_mk_string(self.inner, bytes.as_ptr() as _, bytes.len() as _, 1) };
    Ok(self.val(s))
  }

  fn val(&self, val: mjs_val_t) -> Val {
    Val {
      vm: self.clone(),
      inner: val,
    }
  }
}

impl Val {
  /// Make sure the value don't get garbage collected next time exec or call is
  /// used
  pub fn own(&self) {
    unsafe {
      mjs_own(self.vm.get_inner(), &self.inner as *const mjs_val_t as _);
    }
  }

  /// Disown value
  pub fn disown(&self) {
    unsafe {
      mjs_disown(self.vm.get_inner(), &self.inner as *const mjs_val_t as _);
    }
  }

  /// Is value a number
  pub fn is_number(&self) -> bool {
    unsafe { mjs_is_number(self.inner) > 0 }
  }

  /// Is value an object
  pub fn is_object(&self) -> bool {
    unsafe { mjs_is_object(self.inner) > 0 }
  }

  /// Is value a string
  pub fn is_string(&self) -> bool {
    unsafe { mjs_is_string(self.inner) > 0 }
  }

  /// Is value a function
  pub fn is_function(&self) -> bool {
    unsafe { mjs_is_function(self.inner) > 0 }
  }

  /// Is value a wrapper pointer
  pub fn is_foreign(&self) -> bool {
    unsafe { mjs_is_foreign(self.inner) > 0 }
  }

  /// Get number as int
  pub fn as_int(&self) -> Option<i32> {
    if self.is_number() {
      Some(unsafe { mjs_get_int(self.vm.get_inner(), self.inner) })
    } else {
      None
    }
  }

  /// Get number as double
  pub fn as_double(&self) -> Option<f64> {
    if self.is_number() {
      Some(unsafe { mjs_get_double(self.vm.get_inner(), self.inner) })
    } else {
      None
    }
  }

  /// Get string as bytes
  pub fn as_bytes(&self) -> Option<&[u8]> {
    if self.is_string() {
      let mut len = 0;
      let ptr: *const mjs_val_t = &self.inner;
      unsafe {
        let ptr = mjs_get_string(self.vm.get_inner(), ptr as _, &mut len);
        let slice = core::slice::from_raw_parts(ptr, len as _);
        let slice = &*(slice as *const _ as *const [u8]);
        Some(slice)
      }
    } else {
      None
    }
  }

  /// Get string as str
  pub fn as_str(&self) -> Option<Result<&str, core::str::Utf8Error>> {
    self.as_bytes().map(|b| core::str::from_utf8(b))
  }

  /// Get wrapper pointer
  pub fn as_ptr(&self) -> Option<*const cty::c_void> {
    if self.is_foreign() {
      Some(unsafe { mjs_get_ptr(self.vm.get_inner(), self.inner) as _ })
    } else {
      None
    }
  }

  /// Delete property
  pub fn delete(&self, name: &[u8]) {
    unsafe {
      mjs_del(
        self.vm.get_inner(),
        self.inner,
        name.as_ptr() as _,
        name.len() as _,
      )
    };
  }

  /// Set property
  pub fn set(&mut self, name: &[u8], val: Val) -> Result<(), JSError> {
    let err = unsafe {
      mjs_set(
        self.vm.get_inner(),
        self.inner,
        name.as_ptr() as _,
        name.len() as _,
        val.inner,
      )
    };
    if err != mjs_err_MJS_OK {
      Err(self.vm.get_error(err))
    } else {
      Ok(())
    }
  }

  /// Get property
  pub fn get(&self, name: &[u8]) -> Option<Val> {
    let val = unsafe {
      mjs_get(
        self.vm.get_inner(),
        self.inner,
        name.as_ptr() as _,
        name.len() as _,
      )
    };
    if unsafe { mjs_is_undefined(val) } > 0 {
      None
    } else {
      Some(Val {
        vm: self.vm.clone(),
        inner: val,
      })
    }
  }

  /// Execute function
  pub fn call(&mut self, this: Option<Val>, args: &[&Val]) -> Result<Val, JSError> {
    if self.is_function() {
      let mut ret: mjs_val_t = 0;

      let this = this.unwrap_or_else(|| self.vm.make_undefined());

      let mut mjs_args: [mjs_val_t; 8] = [0; 8];

      if args.len() > mjs_args.len() {
        return Err(JSError::TooManyArgs);
      }

      for (i, val) in args.iter().enumerate() {
        mjs_args[i] = val.inner;
      }

      let err = unsafe {
        mjs_apply(
          self.vm.get_inner(),
          &mut ret,
          self.inner,
          this.inner,
          args.len() as _,
          mjs_args.as_mut_ptr(),
        )
      };

      if err != mjs_err_MJS_OK {
        Err(self.vm.get_error(err))
      } else {
        Ok(Val {
          vm: self.vm.clone(),
          inner: ret,
        })
      }
    } else {
      Err(JSError::NotAFunction)
    }
  }
}
