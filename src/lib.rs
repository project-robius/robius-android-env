use std::sync::OnceLock;

pub use jni::{objects::JObject, JNIEnv, JavaVM};

static VM: OnceLock<JavaVM> = OnceLock::new();

#[inline]
#[cfg(feature = "set")]
pub fn set_vm(vm: JavaVM) -> Result<(), JavaVM> {
    VM.set(vm)
}

// NOTE: These lifetimes are wrong, but that's fine because we don't expose them
// to the user.
static ACTIVITY_GETTER: OnceLock<fn() -> (Option<JNIEnv<'static>>, &'static JObject<'static>)> =
    OnceLock::new();

#[inline]
#[cfg(feature = "set")]
pub fn set_current_activity_getter(
    f: fn() -> (Option<JNIEnv<'static>>, &'static JObject<'static>),
) -> Result<(), fn() -> (Option<JNIEnv<'static>>, &'static JObject<'static>)> {
    ACTIVITY_GETTER.set(f)
}

pub fn with_activity<T, R>(f: T) -> Option<R>
where
    T: for<'a, 'b, 'c, 'd> Fn(&'a mut JNIEnv<'b>, &'c JObject<'d>) -> R,
{
    let getter = ACTIVITY_GETTER.get()?;
    let (maybe_env, object) = getter();
    let mut env = maybe_env.or_else(|| VM.get()?.get_env().ok())?;
    Some(f(&mut env, object))
}
