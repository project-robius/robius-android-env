//! Abstractions for Rust access to Android state (native Java objects)
//! managed by UI toolkits.
//!
//! # Usage of this crate
//! This crate exists for two kinds of downstream users:
//! 1. The UI toolkit that exposes its key internal states that hold
//!    the current Android activity being displayed and the Java VM / JNI environment.
//!    Either the UI toolkit or the app itself should set these states on startup,
//!    either by using [ndk-context] or by manually calling the [`set_vm`]
//!    and [`set_activity_getter`] functions.
//! 2. The platform feature "middleware" crates that need to access the current activity
//!    and JNI environment from Rust code in order to interact with the Android platform.
//!
//! ## Supported UI toolkits
//! * Makepad: enable the `makepad` Cargo feature.
//! * UI toolkits compatible with [ndk-context]: enable the `ndk_context` Cargo feature.
//! * Others coming soon! (in the meantime, see below)
//!
//! ## Usage of this crate for other UI toolkits
//! ⚠️ **Note: for wider compatibility, you should prefer the `ndk-context` feature instead of what is described below.**
//!
//! For any other UI toolkits not mentioned above, you don't need to enable any cargo features.
//! Instead, your application code must manually provide two key states:
//! * a reference to the current Android activity.
//! * a reference to the current Java VM instance (and JNI environment).
//!
//! This can be achieved by doing the following:
//! * Enable the `set` Cargo feature, and
//! * Provide an [ActivityGetterFn] callback by calling [`set_activity_getter()`].
//! * If the [ActivityGetterFn] cannot provide a low-level [`JNIEnv`] object,
//!   the application must also call the [`set_vm()`] function. 
//!
//! [ndk-context]: https://docs.rs/ndk-context/latest/ndk_context/

#[cfg_attr(feature = "makepad", path = "makepad.rs")]
#[cfg_attr(feature = "ndk_context", path = "ndk_context.rs")]
#[cfg_attr(not(any(feature = "makepad", feature = "ndk_context")), path = "custom.rs")]
mod inner;

use std::sync::OnceLock;

// Re-export all types that downstream users might need to instantiate.
pub use jni::{JavaVM, JNIEnv, objects::JObject};

/// Re-exports of low-level pointer types from the `jni-sys` crate.
pub mod sys {
    pub use jni::sys::{jobject, JNIEnv};
}

/// The Java VM instance.
#[allow(dead_code)]
static VM: OnceLock<JavaVM> = OnceLock::new();

/// Sets the current Java VM.
#[inline]
pub fn set_vm(vm: JavaVM) -> Result<(), JavaVM> {
    VM.set(vm)
}

/// The function signature registered by the UI toolkit that provides pointers to
/// current JNI environment (optional) and the current activity (as a jobect).
pub type ActivityGetterFn = fn() -> (Option<*mut sys::JNIEnv>, sys::jobject);

/// The function registered by the UI toolkit that provides raw pointers
/// to the current Android Activity object and JNI environment.
#[allow(dead_code)]
static ACTIVITY_GETTER: OnceLock<ActivityGetterFn> = OnceLock::new();

/// Registers the function that retrieves pointers to the
/// current JNI environment (optional) and the current Android Activity (as a jobect).
///
/// ## Safety
/// The caller must ensure that the provided function `f` provides correct pointers
/// to the current JNI environment (if used) and the current Android Activity.
#[inline]
pub unsafe fn set_activity_getter(f: ActivityGetterFn) -> Result<(), ActivityGetterFn> {
    ACTIVITY_GETTER.set(f)
}

/// Invokes the given closure `f` with the current JNI environment
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
    inner::with_activity_inner(f)
}
