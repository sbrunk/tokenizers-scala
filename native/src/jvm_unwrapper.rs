use std::error::Error;

use jni::JNIEnv;

/// Convienence function to allow using error handling via Result and ? inside our JNI functions.
/// JNI functions have to return a value, even if we throw a Java exception
pub fn wrap_errors<T, F>(mut f: F) -> tokenizers::Result<T>
where
    F: FnMut() -> tokenizers::Result<T>,
{
    f()
}

// ensure that we always throw a JVM exception instead of `panic`ing
pub trait JvmUnwrapper<T> {
    fn jvm_unwrap(self, env: &mut JNIEnv, default: T) -> T;
}

fn throw(e: Box<dyn Error + Send + Sync>, env: &mut JNIEnv) {
    let description = e.to_string();
    // don't `unwrap` `throw_new`, another JVM exception might have already been thrown, in which case the `Result` is `Err`
    let _ = env.throw_new("java/lang/RuntimeException", description);
}

impl<T> JvmUnwrapper<T> for Result<T, Box<dyn Error + Send + Sync>> {
    fn jvm_unwrap(self, env: &mut JNIEnv, default: T) -> T {
        self.unwrap_or_else(|e| {
            throw(e, env);
            default
        })
    }
}
