//! Expose the tokenizers API to the JVM via J4RS/JNI
//!

use j4rs::{errors::J4RsError, prelude::*};
use j4rs::InvocationArg;
use j4rs_derive::*;

use serde::{Deserialize, Serialize};
use tokenizers::{Encoding, Tokenizer};

type J4RsResult<T> = Result<T, J4RsError>;

#[call_from_java("io.brunk.tokenizers.NativeInterface.fromPretrained")]
fn from_pretrained_j4rs(identifier: Instance) -> J4RsResult<Instance> {
    let jvm: Jvm = Jvm::attach_thread()?;
    let identifier: String = jvm.to_rust(identifier)?;

    Tokenizer::from_pretrained(identifier, None)
        .map_err(|e| J4RsError::GeneralError(e.to_string()))
        .and_then(|tokenizer| {
            let tokenizer_ptr = Box::into_raw(Box::new(tokenizer)) as jlong;
            InvocationArg::try_from(tokenizer_ptr)
        })
        .and_then(|ia| Instance::try_from(ia))
}

#[call_from_java("io.brunk.tokenizers.NativeInterface.tokenizerEncode")]
fn tokenizer_encode(
    tokenizer_ptr: Instance,
    input: Instance,
    add_special_tokens: Instance,
) -> J4RsResult<Instance> {
    let jvm: Jvm = Jvm::attach_thread()?;
    let tokenizer_ptr: jlong = jvm.to_rust(tokenizer_ptr)?;
    let tokenizer = unsafe { &mut *(tokenizer_ptr as *mut Tokenizer) };
    let input: String = jvm.to_rust(input)?;
    let add_special_tokens: bool = jvm.to_rust(add_special_tokens)?;

    tokenizer
        .encode_char_offsets(input, add_special_tokens)
        .map_err(|e| J4RsError::GeneralError(e.to_string()))
        .and_then(|encoding| {
            let encoding_ptr = Box::into_raw(Box::new(encoding)) as jlong;
            InvocationArg::try_from(encoding_ptr)
        })
        .and_then(|ia| Instance::try_from(ia))
}

#[call_from_java("io.brunk.tokenizers.NativeInterface.encodingLength")]
fn encoding_length(encoding_ptr: Instance) -> J4RsResult<Instance> {
    let jvm: Jvm = Jvm::attach_thread()?;
    let encoding_ptr: jlong = jvm.to_rust(encoding_ptr)?;
    let encoding = unsafe { &mut *(encoding_ptr as *mut Encoding) };
    let len = encoding.len() as i32;
    InvocationArg::try_from(len).and_then(|ia| Instance::try_from(ia))
}

/// helper to convert int arrays from an encoding
fn vector_to_java<F>(ptr: Instance, getter: F) -> J4RsResult<Instance>
where
    F: Fn(&Encoding) -> &[u32],
{
    let jvm: Jvm = Jvm::attach_thread()?;
    let encoding_ptr: jlong = jvm.to_rust(ptr)?;
    let encoding = unsafe { &mut *(encoding_ptr as *mut Encoding) };
    let ids: Vec<i64> = getter(encoding).iter().map(|&e| e as i64).collect();
    let jids: Vec<_> = ids
        .into_iter()
        .map(|v| {
            InvocationArg::try_from(v)
                .unwrap()
                .into_primitive()
                .unwrap()
        })
        .collect();
    jvm.create_java_array("long", &jids)
}

#[call_from_java("io.brunk.tokenizers.NativeInterface.encodingIds")]
fn encoding_ids(encoding_ptr: Instance) -> J4RsResult<Instance> {
    vector_to_java(encoding_ptr, |e| e.get_ids())
}

// #[no_mangle]
// pub extern "C" fn tokenizer_free(tokenizer_ptr: *mut Tokenizer) {
//     free(tokenizer_ptr)
// }

#[call_from_java("io.brunk.tokenizers.NativeInterface.encodingNSequences")]
fn encoding_n_sequences(encoding_ptr: Instance) -> J4RsResult<Instance> {
    let jvm: Jvm = Jvm::attach_thread()?;
    let encoding_ptr: jlong = jvm.to_rust(encoding_ptr)?;
    let encoding = unsafe { &mut *(encoding_ptr as *mut Encoding) };
    let n_sequences = encoding.n_sequences() as i32;
    InvocationArg::try_from(n_sequences).and_then(|ia| Instance::try_from(ia))
}

#[call_from_java("io.brunk.tokenizers.NativeInterface.encodingTypeIds")]
fn encoding_type_ids(encoding_ptr: Instance) -> J4RsResult<Instance> {
    vector_to_java(encoding_ptr, |e| e.get_type_ids())
}

#[call_from_java("io.brunk.tokenizers.NativeInterface.encodingAttentionMask")]
fn encoding_attention_mask(encoding_ptr: Instance) -> J4RsResult<Instance> {
    vector_to_java(encoding_ptr, |e| e.get_attention_mask())
}

#[call_from_java("io.brunk.tokenizers.NativeInterface.encodingSpecialTokensMask")]
fn encoding_special_tokens_mask(encoding_ptr: Instance) -> J4RsResult<Instance> {
    vector_to_java(encoding_ptr, |e| e.get_special_tokens_mask())
}

#[call_from_java("io.brunk.tokenizers.NativeInterface.encodingTokens")]
fn encoding_tokens(encoding_ptr: Instance) -> J4RsResult<Instance> {
    let jvm: Jvm = Jvm::attach_thread()?;
    let encoding_ptr: jlong = jvm.to_rust(encoding_ptr)?;
    let encoding = unsafe { &mut *(encoding_ptr as *mut Encoding) };
    let tokens = encoding.get_tokens();

    let jtokens: Vec<_> = tokens
        .into_iter()
        .map(|v| {
            InvocationArg::try_from(v).unwrap()
        })
        .collect();
    jvm.create_java_array("java.lang.String", &jtokens)
}

#[call_from_java("io.brunk.tokenizers.NativeInterface.encodingWordIds")]
fn encoding_word_ids(encoding_ptr: Instance) -> J4RsResult<Instance> {
    let jvm: Jvm = Jvm::attach_thread()?;
    let encoding_ptr: jlong = jvm.to_rust(encoding_ptr)?;
    let encoding = unsafe { &mut *(encoding_ptr as *mut Encoding) };
    let word_ids = encoding.get_word_ids();

    // Encode missing values as -1 for the FFI
    let ids: Vec<_> = word_ids
        .iter()
        .map(|e| e.map(|e| e as i64).unwrap_or(-1))
        .collect();
    let jids: Vec<_> = ids
        .into_iter()
        .map(|v| {
            InvocationArg::try_from(v)
                .unwrap()
                .into_primitive()
                .unwrap()
        })
        .collect();
    jvm.create_java_array("long", &jids)
}

#[derive(Serialize, Deserialize, Debug)]
struct Offset {
    start: i64,
    end: i64,
}

#[call_from_java("io.brunk.tokenizers.NativeInterface.encodingOffsets")]
fn encoding_offsets(encoding_ptr: Instance) -> J4RsResult<Instance> {
    let jvm: Jvm = Jvm::attach_thread()?;
    let encoding_ptr: jlong = jvm.to_rust(encoding_ptr)?;
    let encoding = unsafe { &mut *(encoding_ptr as *mut Encoding) };
    let offsets: Vec<_> = encoding
        .get_offsets()
        .iter()
        .map(|&(start, end)| Offset { start: start as i64, end: end as i64 })
        .map(|v| InvocationArg::new(&v, "io.brunk.tokenizers.Offset"))
        .collect();
    jvm.create_java_array("io.brunk.tokenizers.Offset", &offsets)
}

// fn free<T>(ptr: *mut T) {
//     if ptr.is_null() {
//         return;
//     }
//     unsafe { Box::from_raw(ptr) };
// }

// #[no_mangle]
// /// Deallocate a Rust-allocated string from FFI code.
// pub extern "C" fn string_free(str_ptr: *mut c_char) {
//     free(str_ptr)
// }

// #[no_mangle]
// /// Deallocate a Rust-allocated encoding from FFI code.
// pub extern "C" fn encoding_free(encoding_ptr: *mut Encoding) {
//     free(encoding_ptr)
// }

// #[no_mangle]
// /// Deallocate a Rust-allocated encoding from FFI code.
// pub extern "C" fn offset_free(offsetr_ptr: *mut Offset) {
//     free(offsetr_ptr)
// }

// #[no_mangle]
// /// Deallocate a Rust-allocated i64 array from FFI code.
// pub unsafe extern "C" fn vec_free(ptr: *mut i64, len: usize) {
//     if ptr.is_null() {
//         return;
//     }
//     drop(Vec::from_raw_parts(ptr, len, len));
// }
