//! Abstractions for Rust access to Android state (native Java objects) managed by UI toolkits.
//!
//! The states of interest are:
//! * a reference to the current Android activity.
//! * a reference to the current Java VM instance (and JNI environment).
//!
//! This crate exists for two kinds of downstream users:
//! 1. The UI toolkit that exposes its key internal states that hold
//!    the current Android activity being displayed and the Java VM / JNI environment.
//!    Either the UI toolkit or the app itself should set these states on startup,
//!    specifically using [`set_vm`] and [`set_activity_getter`] functions.
//! 2. The Rust platform feature crates that need to access the current activity
//!    and JNI environment in order to interact with the Android platform.
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
///
/// ## Safety
/// The caller must ensure that the provided function `f` provides correct pointers
/// to the current JNI environment (if used) and the current Android Activity.
#[inline]
#[cfg(feature = "set")]
pub unsafe fn set_activity_getter(f: ActivityGetterFn) -> Result<(), ActivityGetterFn> {
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
    // SAFETY: we have no option but to trust the pointers received from the getter fn.
    let activity = unsafe { JObject::from_raw(activity) };
    let mut env = jni_env_opt
        // SAFETY: we have no option but to trust the pointers received from the getter fn.
        .and_then(|raw| unsafe { JNIEnv::from_raw(raw) }.ok())
        .or_else(|| VM.get()?.get_env().ok())?;
    Some(f(&mut env, &activity))
}
