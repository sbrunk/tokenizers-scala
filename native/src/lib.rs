//! Expose the tokenizers API to the JVM via J4RS/JNI
//!

pub mod jvm_unwrapper;

use jni::objects::{JLongArray, JObject, JObjectArray, JString, JValue};
use jni::sys::{jboolean, jint, jlong, jlongArray, jobject, jobjectArray};
use jni::JNIEnv;
use jvm_unwrapper::{wrap_errors, JvmUnwrapper};
use tokenizers::{Encoding, Tokenizer};

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Tokenizer_00024_fromPretrainedNative(
    mut env: JNIEnv,
    _object: JObject,
    identifier: JString,
) -> jlong {
    wrap_errors(|| {
        let identifier: String = env.get_string(&identifier)?.into();
        Tokenizer::from_pretrained(identifier, None).map(to_boxed_ptr)
    })
    .jvm_unwrap(&mut env, -1)
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Tokenizer_encode(
    mut env: JNIEnv,
    _object: JObject,
    tokenizer_ptr: jlong,
    input: JString,
    add_special_tokens: jboolean,
) -> jlong {
    wrap_errors(|| {
        let tokenizer = from_boxed_ptr::<Tokenizer>(tokenizer_ptr);
        let input: String = env.get_string(&input)?.into();
        let encoding = tokenizer.encode_char_offsets(input, add_special_tokens != 0);
        encoding.map(to_boxed_ptr)
    })
    .jvm_unwrap(&mut env, -1)
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Tokenizer_decode<'a>(
    mut env: JNIEnv<'a>,
    _object: JObject,
    tokenizer_ptr: jlong,
    ids: JLongArray,
    skip_special_tokens: jboolean,
) -> JString<'a> {
    wrap_errors(|| {
        let tokenizer = from_boxed_ptr::<Tokenizer>(tokenizer_ptr);
        // All elements can be initialized to the same value.
        let len = env.get_array_length(&ids)? as usize;
        let mut buf: Vec<jlong> = vec![0; len];
        env.get_long_array_region(&ids, 0, &mut buf)?;
        let decoded = tokenizer.decode(
            buf.iter().map(|&e| e as u32).collect(),
            skip_special_tokens != 0,
        )?;
        env.new_string(decoded).map_err(|e| e.into())
    })
    .jvm_unwrap(&mut env, JString::default())
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Tokenizer_encodeBatch<'local>(
    mut env: JNIEnv<'local>,
    _object: JObject<'local>,
    tokenizer_ptr: jlong,
    inputs: JObjectArray<'local>,
    add_special_tokens: jboolean,
) -> JLongArray<'local> {
    let tokenizer = from_boxed_ptr::<Tokenizer>(tokenizer_ptr);
    wrap_errors(|| {
        let inputs = (0..env.get_array_length(&inputs)?)
            .map(|i| {
                let input = env.get_object_array_element(&inputs, i)?;
                let identifier: Result<String, jni::errors::Error> =
                    unsafe { env.get_string_unchecked((&input).into()).map(|i| i.into()) };
                identifier
            })
            .collect::<Result<Vec<_>, _>>()?;
        let len = inputs.len();
        let encodings: Vec<_> = tokenizer
            .encode_batch_char_offsets(inputs, add_special_tokens != 0)?
            .into_iter() // it is important to move the ecodings out of the vector or they will be deallocated
            .map(|encoding| Box::into_raw(Box::new(encoding)) as jlong)
            .collect();
        let encodings_java = env.new_long_array(len as i32)?;
        env.set_long_array_region(&encodings_java, 0, &encodings)?;
        Ok(encodings_java)
    })
    .jvm_unwrap(&mut env, JLongArray::default())
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Tokenizer_00024_free(
    _env: JNIEnv,
    _object: JObject,
    tokenizer_ptr: jlong,
) {
    free::<Tokenizer>(tokenizer_ptr)
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Encoding_length(
    _env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jint {
    let encoding = from_boxed_ptr::<Encoding>(encoding_ptr);
    encoding.len() as i32
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Encoding_nSequences(
    _env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jint {
    let encoding = from_boxed_ptr::<Encoding>(encoding_ptr);
    encoding.n_sequences() as i32
}

/// helper to convert int arrays from an encoding
fn vector_to_java<F>(mut env: JNIEnv, encoding_ptr: jlong, extractor: F) -> jlongArray
where
    F: Fn(&Encoding) -> Vec<u32>,
{
    wrap_errors(|| {
        let encoding = from_boxed_ptr::<Encoding>(encoding_ptr);
        let ids: Vec<i64> = extractor(encoding).iter().map(|&e| e as i64).collect();
        let ids_java = env.new_long_array(ids.len() as i32)?;
        env.set_long_array_region(&ids_java, 0, &ids)?;
        Ok(ids_java.into_raw())
    })
    .jvm_unwrap(&mut env, JObject::null().into_raw())
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Encoding_ids(
    env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jlongArray {
    vector_to_java(env, encoding_ptr, |e| e.get_ids().to_vec())
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Encoding_typeIds(
    env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jlongArray {
    vector_to_java(env, encoding_ptr, |e| e.get_type_ids().to_vec())
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Encoding_attentionMask(
    env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jlongArray {
    vector_to_java(env, encoding_ptr, |e| e.get_attention_mask().to_vec())
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Encoding_specialTokensMask(
    env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jlongArray {
    vector_to_java(env, encoding_ptr, |e| e.get_special_tokens_mask().to_vec())
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Encoding_tokens(
    mut env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jobjectArray {
    wrap_errors(|| {
        let encoding = from_boxed_ptr::<Encoding>(encoding_ptr);
        let tokens = encoding.get_tokens().to_vec();
        let string_class = env.find_class("java/lang/String")?;
        let empty_string = env.new_string("")?;
        let tokens_java = env.new_object_array(tokens.len() as i32, string_class, empty_string)?;
        for (i, type_id) in tokens.iter().enumerate() {
            env.set_object_array_element(&tokens_java, i as i32, env.new_string(type_id)?)?;
        }
        Ok(tokens_java.into_raw())
    })
    .jvm_unwrap(&mut env, JObject::null().into_raw())
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Encoding_wordIds(
    mut env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jlongArray {
    wrap_errors(|| {
        let encoding = from_boxed_ptr::<Encoding>(encoding_ptr);
        let ids: Vec<_> = encoding
            .get_word_ids()
            .iter()
            .map(|e| e.map(|e| e as i64).unwrap_or(-1))
            .collect();
        let ids_java = env.new_long_array(ids.len() as i32)?;
        env.set_long_array_region(&ids_java, 0, &ids)?;
        Ok(ids_java.into_raw())
    })
    .jvm_unwrap(&mut env, JObject::null().into_raw())
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Encoding_offsets(
    mut env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jobject {
    let encoding = from_boxed_ptr::<Encoding>(encoding_ptr);
    let offsets = encoding.get_offsets();
    let start_offsets: Vec<_> = offsets.iter().map(|o| o.0 as i64).collect();
    let end_offsets: Vec<_> = offsets.iter().map(|o| o.1 as i64).collect();
    wrap_errors(|| {
        let start_offsets_java = env.new_long_array(start_offsets.len() as i32)?;
        env.set_long_array_region(&start_offsets_java, 0, &start_offsets)?;
        let end_offsets_java = env.new_long_array(end_offsets.len() as i32)?;
        env.set_long_array_region(&end_offsets_java, 0, &end_offsets)?;
        let tokenizer_java = env.new_object(
            "io/brunk/tokenizers/NativeOffsets",
            "([J[J)V",
            &[
                JValue::Object(&start_offsets_java),
                JValue::Object(&end_offsets_java),
            ],
        )?;
        Ok(tokenizer_java.into_raw())
    })
    .jvm_unwrap(&mut env, JObject::null().into_raw())
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Encoding_00024_free(
    _env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) {
    free::<Encoding>(encoding_ptr);
}

fn free<T>(ptr: i64) {
    drop(unsafe { Box::from_raw(ptr as *mut T) })
}

fn to_boxed_ptr<T>(value: T) -> i64 {
    Box::into_raw(Box::new(value)) as jlong
}

fn from_boxed_ptr<T>(ptr: i64) -> &'static mut T {
    unsafe { &mut *(ptr as *mut T) }
}
