//! Expose the tokenizers API via FFI functions
//! 
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
    add_special_tokens: bool
) -> ExtResult<Encoding> {
    let tokenizer = unsafe { &mut *tokenizer_ptr };
    let input_result = unsafe {
        assert!(!input.is_null());
        CStr::from_ptr(input)
    }
    .to_str()
    .map_err(|_e| String::from("Utf8 error"));
    let encoding =
        input_result.and_then(|input| tokenizer.encode_char_offsets(input, add_special_tokens).map_err(|e| e.to_string()));
    match encoding {
        Ok(encoding) => ExtResult::OK(Box::into_raw(Box::new(encoding))),
        Err(e) => ExtResult::Err(CString::new(e.to_string()).unwrap().into_raw()),
    }
}

#[no_mangle]
pub extern "C" fn tokenizer_free(tokenizer_ptr: *mut Tokenizer) {
    free(tokenizer_ptr)
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
fn convert_encoding_ints<F>(encoding_ptr: *mut Encoding, getter: F) -> *const i64
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
    mem::forget(ids);
    ptr
}

#[no_mangle]
pub extern "C" fn encoding_ids(encoding_ptr: *mut Encoding) -> *const i64 {
    convert_encoding_ints(encoding_ptr, |e| e.get_ids())
}

#[no_mangle]
pub extern "C" fn encoding_type_ids(encoding_ptr: *mut Encoding) -> *const i64 {
    convert_encoding_ints(encoding_ptr, |e| e.get_type_ids())
}

#[no_mangle]
pub extern "C" fn encoding_attention_mask(encoding_ptr: *mut Encoding) -> *const i64 {
    convert_encoding_ints(encoding_ptr, |e| e.get_attention_mask())
}

#[no_mangle]
pub extern "C" fn encoding_special_tokens_mask(encoding_ptr: *mut Encoding) -> *const i64 {
    convert_encoding_ints(encoding_ptr, |e| e.get_special_tokens_mask())
}

#[no_mangle]
pub extern "C" fn encoding_tokens(encoding_ptr: *mut Encoding) -> *const *mut c_char {
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
        .map(|s| s.into_raw())
        .collect::<Vec<_>>();

    // Make sure we're not wasting space.
    c_tokens.shrink_to_fit();
    assert!(c_tokens.len() == c_tokens.capacity());

    // Get the pointer to our vector.
    let ptr = c_tokens.as_ptr();
    mem::forget(c_tokens);
    ptr
}

#[no_mangle]
pub extern "C" fn encoding_word_ids(encoding_ptr: *mut Encoding) -> *const i64 {
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
    mem::forget(ids);
    ptr
}

#[repr(C)]
pub struct Offset {
    start: usize,
    end: usize,
}

#[no_mangle]
pub extern "C" fn encoding_offsets(encoding_ptr: *mut Encoding) -> *const Offset {
    let encoding = unsafe {
        assert!(!encoding_ptr.is_null());
        &mut *encoding_ptr
    };

    let mut offsets: Vec<_> = encoding.get_offsets().iter().map(|&(start, end)| {
        Offset {
            start,
            end
        }
    }).collect();
    offsets.shrink_to_fit();
    assert!(offsets.len() == offsets.capacity());
    let ptr = offsets.as_ptr();
    mem::forget(offsets);
    ptr
}




fn free<T>(ptr: *mut T) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
/// Deallocate a Rust-allocated string from FFI code.
pub extern "C" fn string_free(str_ptr: *mut c_char) {
    free(str_ptr)
}


#[no_mangle]
/// Deallocate a Rust-allocated encoding from FFI code.
pub extern "C" fn encoding_free(encoding_ptr: *mut Encoding) {
    free(encoding_ptr)
}

#[no_mangle]
/// Deallocate a Rust-allocated encoding from FFI code.
pub extern "C" fn offset_free(offsetr_ptr: *mut Offset) {
    free(offsetr_ptr)
}

#[no_mangle]
/// Deallocate a Rust-allocated i64 array from FFI code.
pub unsafe extern "C" fn vec_free(ptr: *mut i64, len: usize) {
    if ptr.is_null() {
        return;
    }
    drop(Vec::from_raw_parts(ptr, len, len));
}