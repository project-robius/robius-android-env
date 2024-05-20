use makepad_android_state::{get_activity, get_java_vm};

pub fn with_activity_inner<F, R>(f: F) -> Option<R>
where
    F: for<'a, 'b, 'c, 'd> Fn(&'a mut crate::JNIEnv<'b>, &'c crate::JObject<'d>) -> R,
{
    let jvm = unsafe { crate::JavaVM::from_raw(get_java_vm().cast()) }.ok()?;
    let mut jni_env = jvm.get_env().ok()?;

    let activity = unsafe { crate::JObject::from_raw(get_activity().cast()) };
    Some(f(&mut jni_env, &activity))
}
