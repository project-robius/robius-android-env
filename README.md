# `robius-android-env`

[![Latest Version](https://img.shields.io/crates/v/robius-android-env.svg)](https://crates.io/crates/robius-android-env)
[![Docs](https://docs.rs/robius-android-env/badge.svg)](https://docs.rs/robius-android-env/latest/robius_android_env/)
[![Project Robius Matrix Chat](https://img.shields.io/matrix/robius-general%3Amatrix.org?server_fqdn=matrix.org&style=flat&logo=matrix&label=Project%20Robius%20Matrix%20Chat&color=B7410E)](https://matrix.to/#/#robius:matrix.org)

This crate provides easy Rust access to Android state (native Java objects) managed by UI toolkits.

## Usage of this crate
This crate exists for two kinds of downstream users:
1. The UI toolkit that exposes its key internal states that hold
   the current Android activity being displayed and the Java VM / JNI environment.
   Either the UI toolkit or the app itself should set these states on startup,
   either by using [ndk-context] or by manually calling the [`set_vm()`] and [`set_activity_getter()`] functions.
2. The platform feature "middleware" crates that need to access the current activity
   and JNI environment from Rust code in order to interact with the Android platform.

### Supported UI toolkits
* Makepad: enable the `makepad` Cargo feature. 
* UI toolkits compatible with [ndk-context]: enable the `ndk_context` Cargo feature.
* Others coming soon! (in the meantime, see below)

### Usage of this crate for other UI toolkits
> ⚠️ Note: for wider compatibility, you should prefer the `ndk-context` feature instead of what is described below.

For any other UI toolkits not listed above, you don't need to enable any cargo features. 
Instead, your application code must manually provide two key states:
* a reference to the current Android activity.
* a reference to the current Java VM instance (and JNI environment).

This can be achieved by doing the following:
* Provide an [ActivityGetterFn] callback by calling [`set_activity_getter()`].
* If the [ActivityGetterFn] cannot provide a low-level [`JNIEnv`] object,
the application must also call the [`set_vm()`] function. 

[`set_vm()`]: https://docs.rs/robius-android-env/latest/robius_android_env/fn.set_vm.html
[`set_activity_getter()`]: https://docs.rs/robius-android-env/latest/robius_android_env/fn.set_activity_getter.html
[ActivityGetterFn]: https://docs.rs/robius-android-env/latest/robius_android_env/type.ActivityGetterFn.html
[`JNIEnv`]: https://docs.rs/jni/latest/jni/sys/type.JNIEnv.html
[ndk-context]: https://docs.rs/ndk-context/latest/ndk_context/
