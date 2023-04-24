use std::{
    ffi::{c_char, CStr, CString},
    mem,
};

use tokenizers::{Encoding, Tokenizer};

#[repr(C)]
pub enum ExtResult<T> {
    OK(*mut T),
    Err(*mut c_char),
}

#[no_mangle]
pub extern "C" fn from_pretrained(identifier: *const c_char) -> ExtResult<Tokenizer> {
    let identifier_result = unsafe {
        assert!(!identifier.is_null());
        CStr::from_ptr(identifier)
    }
    .to_str()
    .map_err(|_e| String::from("Utf8 error"));

    let tokenizer_result = identifier_result.and_then(|identifier| {
        Tokenizer::from_pretrained(identifier, None).map_err(|e| e.to_string())
    });

    match tokenizer_result {
        Ok(tokenizer) => ExtResult::OK(Box::into_raw(Box::new(tokenizer))),
        Err(e) => ExtResult::Err(CString::new(e.to_string()).unwrap().into_raw()),
    }
}

#[no_mangle]
pub extern "C" fn tokenizer_encode(
    tokenizer_ptr: *mut Tokenizer,
    input: *const c_char,
) -> ExtResult<Encoding> {
    let tokenizer = unsafe { &mut *tokenizer_ptr };
    let input_result = unsafe {
        assert!(!input.is_null());
        CStr::from_ptr(input)
    }
    .to_str()
    .map_err(|_e| String::from("Utf8 error"));
    let encoding =
        input_result.and_then(|input| tokenizer.encode(input, true).map_err(|e| e.to_string()));
    match encoding {
        Ok(encoding) => ExtResult::OK(Box::into_raw(Box::new(encoding))),
        Err(e) => ExtResult::Err(CString::new(e.to_string()).unwrap().into_raw()),
    }
}

#[no_mangle]
pub extern "C" fn tokenizer_free(tokenizer_ptr: *mut Tokenizer) {
    unsafe { Box::from_raw(tokenizer_ptr as *mut Tokenizer) };
}

#[no_mangle]
pub extern "C" fn encoding_len(encoding_ptr: *mut Encoding) -> i32 {
    let encoding = unsafe {
        assert!(!encoding_ptr.is_null());
        &mut *encoding_ptr
    };
    encoding.len() as i32
}

#[no_mangle]
pub extern "C" fn encoding_n_sequences(encoding_ptr: *mut Encoding) -> i32 {
    let encoding = unsafe {
        assert!(!encoding_ptr.is_null());
        &mut *encoding_ptr
    };
    encoding.n_sequences() as i32
}

/// helper to convert int arrays from an encoding
fn convert_encoding_ints<F>(encoding_ptr: *mut Encoding, getter: F) -> *mut i64
where
    F: Fn(&Encoding) -> &[u32],
{
    let encoding = unsafe {
        assert!(!encoding_ptr.is_null());
        &mut *encoding_ptr
    };
    let mut ids: Vec<_> = getter(&encoding).iter().map(|&e| e as i64).collect();
    ids.shrink_to_fit();
    assert!(ids.len() == ids.capacity());

    let ptr = ids.as_mut_ptr();
    mem::forget(ids); // TODO ensure deallocation
    ptr
}

#[no_mangle]
pub extern "C" fn encoding_ids(encoding_ptr: *mut Encoding) -> *mut i64 {
    convert_encoding_ints(encoding_ptr, |e| e.get_ids())
}

#[no_mangle]
pub extern "C" fn encoding_type_ids(encoding_ptr: *mut Encoding) -> *mut i64 {
    convert_encoding_ints(encoding_ptr, |e| e.get_type_ids())
}

#[no_mangle]
pub extern "C" fn encoding_attention_mask(encoding_ptr: *mut Encoding) -> *mut i64 {
    convert_encoding_ints(encoding_ptr, |e| e.get_attention_mask())
}

#[no_mangle]
pub extern "C" fn encoding_special_tokens_mask(encoding_ptr: *mut Encoding) -> *mut i64 {
    convert_encoding_ints(encoding_ptr, |e| e.get_special_tokens_mask())
}

#[no_mangle]
pub extern "C" fn encoding_tokens(encoding_ptr: *mut Encoding) -> *mut *mut c_char {
    let encoding = unsafe {
        assert!(!encoding_ptr.is_null());
        &mut *encoding_ptr
    };
    let tokens = encoding.get_tokens();

    // Turn each null-terminated string into a pointer.
    // `into_raw` takes ownership, gives us the pointer and does NOT drop the data.
    let mut c_tokens: Vec<*mut i8> = tokens
        .iter()
        .map(|token| CString::new(String::from(token)).unwrap())
        .map(|s| s.into_raw()).collect::<Vec<_>>();

    // Make sure we're not wasting space.
    c_tokens.shrink_to_fit();
    assert!(c_tokens.len() == c_tokens.capacity());

    // Get the pointer to our vector.
    let ptr = c_tokens.as_mut_ptr();
    mem::forget(c_tokens); // TODO ensure deallocation
    ptr
}

#[no_mangle]
pub extern "C" fn encoding_word_ids(encoding_ptr: *mut Encoding) -> *mut i64 {
    let encoding = unsafe {
        assert!(!encoding_ptr.is_null());
        &mut *encoding_ptr
    };
    let word_ids = encoding.get_word_ids();

    // Encode missing values as -1 for the FFI
    let mut ids: Vec<_> = word_ids
        .iter()
        .map(|e| e.map(|e| e as i64).unwrap_or(-1))
        .collect();
    ids.shrink_to_fit();
    assert!(ids.len() == ids.capacity());

    let ptr = ids.as_mut_ptr();
    mem::forget(ids); // TODO ensure deallocation
    ptr
}
