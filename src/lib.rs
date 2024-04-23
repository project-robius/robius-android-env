//! Abstractions for Rust access to Android state (native Java objects) managed by UI toolkits.
//!
//! The states of interest are:
//! * a reference to the current Android activity
//! * a reference to the current Java VM instance (and JNI environment)
//!

use std::sync::OnceLock;

// Re-export all types that downstream users might need to instantiate.
pub use jni::{JavaVM, JNIEnv, objects::JObject};

/// Re-exports of low-level pointer types from the `jni-sys` crate.
pub mod sys {
    pub use jni::sys::{jobject, JNIEnv};
}

/// The Java VM instance.
static VM: OnceLock<JavaVM> = OnceLock::new();

/// Sets the current Java VM.
#[inline]
#[cfg(feature = "set")]
pub fn set_vm(vm: JavaVM) -> Result<(), JavaVM> {
    VM.set(vm)
}

/// The function signature registered by the UI toolkit that provides pointers to
/// current JNI environment (optional) and the current activity (as a jobect).
pub type ActivityGetterFn = fn() -> (Option<*mut sys::JNIEnv>, sys::jobject);

/// The function registered by the UI toolkit that provides raw pointers
/// to the current Android Activity object and JNI environment.
static ACTIVITY_GETTER: OnceLock<ActivityGetterFn> = OnceLock::new();

/// Registers the function that retrieves pointers to the
/// current JNI environment (optional) and the current Android Activity (as a jobect).
#[inline]
#[cfg(feature = "set")]
pub fn set_current_activity_getter(f: ActivityGetterFn) -> Result<(), ActivityGetterFn> {
    ACTIVITY_GETTER.set(f)
}

/// Invokes the given closure `f` with a reference to the current JNI environment
/// and the current activity.
///
/// Returns `None` upon error, including:
/// * If the function that gets the current activity and JNI environment
///   has not been set.
/// * If the current JNI environment cannot be obtained.
pub fn with_activity<F, R>(f: F) -> Option<R>
where
    F: for<'a, 'b, 'c, 'd> Fn(&'a mut JNIEnv<'b>, &'c JObject<'d>) -> R,
{
    let getter = ACTIVITY_GETTER.get()?;
    let (jni_env_opt, activity) = getter();
    let activity = unsafe { JObject::from_raw(activity) };
    let mut env = jni_env_opt
        .and_then(|raw| unsafe { JNIEnv::from_raw(raw) }.ok())
        .or_else(|| VM.get()?.get_env().ok())?;
    Some(f(&mut env, &activity))
}
