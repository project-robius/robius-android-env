use std::sync::OnceLock;

use jni::{objects::JObject, JavaVM};

static VM: OnceLock<JavaVM> = OnceLock::new();

#[inline]
#[cfg(feature = "set")]
pub fn set_vm(vm: JavaVM) -> Result<(), JavaVM> {
    VM.set(vm)
}

#[inline]
pub fn vm() -> Option<&'static JavaVM> {
    VM.get()
}

static ACTIVITY_GETTER: OnceLock<fn() -> &'static JObject<'static>> = OnceLock::new();

#[inline]
#[cfg(feature = "set")]
pub fn set_current_activity_getter(
    f: fn() -> &'static JObject<'static>,
) -> Result<(), fn() -> &'static JObject<'static>> {
    ACTIVITY_GETTER.set(f)
}

#[inline]
pub fn current_activity() -> Option<&'static JObject<'static>> {
    ACTIVITY_GETTER.get().map(|f| f())
}
