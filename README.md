# `robius-android-env`

[![Project Robius Matrix Chat](https://img.shields.io/matrix/robius-general%3Amatrix.org?server_fqdn=matrix.org&style=flat&logo=matrix&label=Project%20Robius%20Matrix%20Chat&color=B7410E)](https://matrix.to/#/#robius:matrix.org)

This crate provides easy Rust access to Android state (native Java objects) managed by UI toolkits.

## Usage of this crate
This crate exists for two kinds of downstream users:
1. The UI toolkit that exposes its key internal states that hold
   the current Android activity being displayed and the Java VM / JNI environment.
   Either the UI toolkit or the app itself should set these states on startup,
   specifically using [`set_vm`] and [`set_activity_getter`] functions.
2. The Rust platform feature crates that need to access the current activity
   and JNI environment in order to interact with the Android platform.

### Supported UI toolkits
* Makepad: enable the `makepad` Cargo feature.
* Others coming soon! (in the meantime, see below)

### Usage of this crate for other UI toolkits
For any other UI toolkits not listed above, your application code must
manually provide two key states:
* a reference to the current Android activity.
* a reference to the current Java VM instance (and JNI environment).

This can be achieved by doing the following:
* Enable the `set` Cargo feature, and
* Provide an [ActivityGetterFn] callback by calling [`set_activity_getter()`].
* If the [ActivityGetterFn] cannot provide a [`JNIEnv`] object,
the application must also call the [`set_vm()`] function. 
