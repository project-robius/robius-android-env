use std::sync::OnceLock;

use jni::{objects::JObject, sys, JNIEnv, JavaVM};

static VM: OnceLock<JavaVM> = OnceLock::new();

#[inline]
#[cfg(feature = "set")]
pub fn set_vm(vm: JavaVM) -> Result<(), JavaVM> {
    VM.set(vm)
}

// NOTE: These lifetimes are wrong, but that's fine because we don't expose them
// to the user.
static ACTIVITY_GETTER: OnceLock<fn() -> (Option<*mut sys::JNIEnv>, sys::jobject)> =
    OnceLock::new();

#[inline]
#[cfg(feature = "set")]
pub fn set_current_activity_getter(
    f: fn() -> (Option<*mut sys::JNIEnv>, sys::jobject),
) -> Result<(), fn() -> (Option<*mut sys::JNIEnv>, sys::jobject)> {
    ACTIVITY_GETTER.set(f)
}

pub fn with_activity<T, R>(f: T) -> Option<R>
where
    T: for<'a, 'b, 'c, 'd> Fn(&'a mut JNIEnv<'b>, &'c JObject<'d>) -> R,
{
    let getter = ACTIVITY_GETTER.get()?;
    let (maybe_env, object) = getter();
    let object = unsafe { JObject::from_raw(object) };
    let mut env = maybe_env
        .and_then(|raw| unsafe { JNIEnv::from_raw(raw) }.ok())
        .or_else(|| VM.get()?.get_env().ok())?;
    Some(f(&mut env, &object))
}
