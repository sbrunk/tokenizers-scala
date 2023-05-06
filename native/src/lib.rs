//! Expose the tokenizers API to the JVM via J4RS/JNI
//!

use tokenizers::{Encoding, Tokenizer};
use jni::objects::{JLongArray, JObject, JObjectArray, JString, JValue};
use jni::sys::{jint, jlong, jobject, jboolean, jlongArray};
use jni::JNIEnv;

type JNIResult<T> = jni::errors::Result<T>;


#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Tokenizer_00024_fromPretrainedNative(
    mut env: JNIEnv,
    _object: JObject,
    identifier: JString,
) -> jlong {
    let identifier: String = env
        .get_string(&identifier)
        .expect("Couldn't get java string!")
        .into();
    let tokenizer = Tokenizer::from_pretrained(identifier, None).unwrap();
    Box::into_raw(Box::new(tokenizer)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Tokenizer_encode(
    mut env: JNIEnv,
    _object: JObject,
    tokenizer_ptr: jlong,
    input: JString,
    add_special_tokens: jboolean,
) -> jlong {
    let tokenizer = unsafe { &mut *(tokenizer_ptr as *mut Tokenizer) };
    let input: String = env
        .get_string(&input)
        .expect("Couldn't get java string!")
        .into();
    let encoding = tokenizer
        .encode_char_offsets(input, add_special_tokens != 0)
        .unwrap();
    // .map_err(|e| J4RsError::GeneralError(e.to_string()))
    let encoding_ptr = Box::into_raw(Box::new(encoding)) as jlong;
    encoding_ptr
}

unsafe fn encode_batch<'a>(
    mut env: JNIEnv<'a>,
    tokenizer: &Tokenizer,
    inputs: JObjectArray<'a>,
    add_special_tokens: bool,
) -> JNIResult<JLongArray<'a>> {
    let inputs: Vec<String> = (0..env.get_array_length(&inputs)?)
        .map(|i| {
            let input = env.get_object_array_element(&inputs, i).unwrap();
            let identifier = env.get_string_unchecked((&input).into()).map(|i| i.into());
            identifier.unwrap()
        })
        .collect();

    let len = inputs.len();
    let encodings: Vec<_> = tokenizer
        .encode_batch_char_offsets(inputs, add_special_tokens)
        .unwrap()
        .into_iter() // it is important to move the ecodings out of the vector or they will be deallocated
        .map(|encoding| Box::into_raw(Box::new(encoding)) as jlong)
        .collect();
    let encodings_java = env.new_long_array(len as i32).unwrap();
    env.set_long_array_region(&encodings_java, 0, &encodings)
        .unwrap();
    Ok(encodings_java)
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Tokenizer_encodeBatch<'local>(
    env: JNIEnv<'local>,
    _object: JObject<'local>,
    tokenizer_ptr: jlong,
    inputs: JObjectArray<'local>,
    add_special_tokens: jboolean,
) -> JLongArray<'local> {
    let tokenizer = unsafe { &mut *(tokenizer_ptr as *mut Tokenizer) };
    unsafe { encode_batch(env, tokenizer, inputs, add_special_tokens != 0) }.unwrap()
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Tokenizer_00024_free(
    _env: JNIEnv,
    _object: JObject,
    tokenizer_ptr: jlong,
) {
    unsafe { drop(Box::from_raw(tokenizer_ptr as *mut Tokenizer)) }
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Encoding_length(
    _env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jint {
    let encoding = unsafe { &mut *(encoding_ptr as *mut Encoding) };
    encoding.len() as i32
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Encoding_nSequences(
    _env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jint {
    let encoding = unsafe { &mut *(encoding_ptr as *mut Encoding) };
    encoding.n_sequences() as i32
}

/// helper to convert int arrays from an encoding
fn vector_to_java<F>(env: JNIEnv, encoding_ptr: jlong, extractor: F) -> jlongArray
where
    F: Fn(&Encoding) -> Vec<u32>,
{
    let encoding = unsafe { &mut *(encoding_ptr as *mut Encoding) };
    let ids: Vec<i64> = extractor(encoding).iter().map(|&e| e as i64).collect();
    let ids_java = env.new_long_array(ids.len() as i32).unwrap();
    env.set_long_array_region(&ids_java, 0, &ids).unwrap();
    ids_java.into_raw()
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_brunk_tokenizers_Encoding_ids(
    env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jlongArray {
    vector_to_java(env, encoding_ptr, |e| e.get_ids().to_vec())
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_brunk_tokenizers_Encoding_typeIds(
    env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jlongArray {
    vector_to_java(env, encoding_ptr, |e| e.get_type_ids().to_vec())
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_brunk_tokenizers_Encoding_attentionMask(
    env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jlongArray {
    vector_to_java(env, encoding_ptr, |e| e.get_attention_mask().to_vec())
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_brunk_tokenizers_Encoding_specialTokensMask(
    env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jlongArray {
    vector_to_java(env, encoding_ptr, |e| e.get_special_tokens_mask().to_vec())
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_brunk_tokenizers_Encoding_tokens(
    mut env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jlongArray {
    let encoding = unsafe { &mut *(encoding_ptr as *mut Encoding) };
    let tokens = encoding.get_tokens().to_vec();
    let string_class = env.find_class("java/lang/String").unwrap();
    let empty_string = env.new_string("").unwrap();
    let tokens_java = env
        .new_object_array(tokens.len() as i32, string_class, empty_string)
        .unwrap();
    for (i, type_id) in tokens.iter().enumerate() {
        env.set_object_array_element(&tokens_java, i as i32, env.new_string(type_id).unwrap())
            .unwrap();
    }
    tokens_java.into_raw()
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_brunk_tokenizers_Encoding_wordIds(
    env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jlongArray {
    let encoding = unsafe { &mut *(encoding_ptr as *mut Encoding) };
    let ids: Vec<_> = encoding
        .get_word_ids()
        .iter()
        .map(|e| e.map(|e| e as i64).unwrap_or(-1))
        .collect();
    let ids_java = env.new_long_array(ids.len() as i32).unwrap();
    env.set_long_array_region(&ids_java, 0, &ids).unwrap();
    ids_java.into_raw()
}

#[no_mangle]
pub unsafe extern "system" fn Java_io_brunk_tokenizers_Encoding_offsets(
    mut env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) -> jobject {
    let encoding = unsafe { &mut *(encoding_ptr as *mut Encoding) };
    let offsets = encoding.get_offsets();

    let start_offsets: Vec<_> = offsets.iter().map(|o| o.0 as i64).collect();
    let end_offsets: Vec<_> = offsets.iter().map(|o| o.1 as i64).collect();
    let start_offsets_java = env.new_long_array(start_offsets.len() as i32).unwrap();
    env.set_long_array_region(&start_offsets_java, 0, &start_offsets).unwrap();
    let end_offsets_java = env.new_long_array(end_offsets.len() as i32).unwrap();
    env.set_long_array_region(&end_offsets_java, 0, &end_offsets).unwrap();
    let tokenizer_java = env.new_object(
        "io/brunk/tokenizers/NativeOffsets",
        "([J[J)V",
        &[JValue::Object(&start_offsets_java), JValue::Object(&end_offsets_java)],
    ).unwrap();
    tokenizer_java.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_io_brunk_tokenizers_Encoding_00024_free(
    _env: JNIEnv,
    _object: JObject,
    encoding_ptr: jlong,
) {
    unsafe { drop(Box::from_raw(encoding_ptr as *mut Encoding)) }
}