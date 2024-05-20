//! The default implementation for unknown UI toolkits, which accesses
//! the explicitly-set `VM` and `ACTIVITY_GETTER` in the crate root.

use crate::{VM, ACTIVITY_GETTER, JNIEnv, JObject};

pub fn with_activity_inner<F, R>(f: F) -> Option<R>
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
