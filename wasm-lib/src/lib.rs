use wasm_bindgen::prelude::*;
use xelis_hash::v2::{self, ScratchPad};
use hex;
use js_sys;
use std::sync::Mutex;

// Enable console error panic hook for better error messages
#[wasm_bindgen]
pub fn init_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// Thread-local scratchpad for hashing
thread_local! {
    static SCRATCH_PAD: Mutex<ScratchPad> = Mutex::new(ScratchPad::default());
}

// Hash function that returns bytes as Vec<u8>
#[wasm_bindgen]
pub fn xelis_hash(data: &[u8]) -> Result<Vec<u8>, JsValue> {
    SCRATCH_PAD.with(|scratch_pad| {
        let mut scratch_pad = scratch_pad.lock().unwrap();
        v2::xelis_hash(data, &mut scratch_pad)
            .map(|hash| hash.to_vec())
            .map_err(|e| JsValue::from_str(&format!("Hashing error: {:?}", e)))
    })
}

// Hash function that returns a hex string
#[wasm_bindgen]
pub fn xelis_hash_hex(data: &[u8]) -> Result<String, JsValue> {
    xelis_hash(data).map(|hash| hex::encode(hash))
}

// Helper function to convert JavaScript string to bytes and hash
#[wasm_bindgen]
pub fn hash_string(input: &str) -> Result<String, JsValue> {
    xelis_hash_hex(input.as_bytes())
}

// Helper function to hash multiple times
#[wasm_bindgen]
pub fn xelis_hash_multiple(data: &[u8], iterations: u32) -> Result<Vec<u8>, JsValue> {
    if iterations == 0 {
        return Ok(data.to_vec());
    }

    let mut result = xelis_hash(data)?;
    
    for _ in 1..iterations {
        result = xelis_hash(&result)?;
    }
    
    Ok(result)
}

// Convert a hash from bytes back to hex string
#[wasm_bindgen]
pub fn bytes_to_hex(data: &[u8]) -> String {
    hex::encode(data)
}

// Convert hex string to bytes
#[wasm_bindgen]
pub fn hex_to_bytes(hex_string: &str) -> Result<Vec<u8>, JsValue> {
    hex::decode(hex_string)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))
}

// Verify if two hashes (in hex format) are equal
#[wasm_bindgen]
pub fn verify_hash(hex_hash1: &str, hex_hash2: &str) -> Result<bool, JsValue> {
    let bytes1 = hex::decode(hex_hash1)
        .map_err(|e| JsValue::from_str(&format!("Invalid first hash: {}", e)))?;
    
    let bytes2 = hex::decode(hex_hash2)
        .map_err(|e| JsValue::from_str(&format!("Invalid second hash: {}", e)))?;
    
    Ok(bytes1 == bytes2)
}

// Get the size of the hash in bytes
#[wasm_bindgen]
pub fn get_hash_size() -> usize {
    32
}

// Advanced function that returns both bytes and hex
#[wasm_bindgen]
pub fn xelis_hash_detailed(data: &[u8]) -> Result<JsValue, JsValue> {
    let hash_bytes = xelis_hash(data)?;
    let hex_string = hex::encode(&hash_bytes);
    
    let result = js_sys::Object::new();
    
    js_sys::Reflect::set(
        &result,
        &"bytes".into(),
        &js_sys::Uint8Array::from(&hash_bytes[..]).into()
    ).map_err(|_| JsValue::from_str("Failed to set bytes property"))?;
    
    js_sys::Reflect::set(
        &result,
        &"hex".into(),
        &hex_string.into()
    ).map_err(|_| JsValue::from_str("Failed to set hex property"))?;
    
    js_sys::Reflect::set(
        &result,
        &"size".into(),
        &(hash_bytes.len() as u32).into()
    ).map_err(|_| JsValue::from_str("Failed to set size property"))?;
    
    Ok(result.into())
}

// Batch hashing multiple inputs
#[wasm_bindgen]
pub fn batch_hash(data_slices: js_sys::Array) -> Result<js_sys::Array, JsValue> {
    let results = js_sys::Array::new();
    
    for i in 0..data_slices.length() {
        let data = data_slices.get(i).dyn_into::<js_sys::Uint8Array>()
            .map_err(|_| JsValue::from_str("Failed to convert input to Uint8Array"))?;
            
        let bytes = data.to_vec();
        let hash_result = xelis_hash(&bytes)?;
        results.push(&js_sys::Uint8Array::from(&hash_result[..]));
    }
    
    Ok(results)
}

// Hash and return as both hex and bytes for a single input
#[wasm_bindgen]
pub fn hash_with_metadata(data: &[u8]) -> Result<JsValue, JsValue> {
    let hash_bytes = xelis_hash(data)?;
    let hash_hex = hex::encode(&hash_bytes);
    
    let obj = js_sys::Object::new();
    
    js_sys::Reflect::set(
        &obj,
        &"input_length".into(),
        &(data.len() as u32).into()
    ).map_err(|_| JsValue::from_str("Failed to set input_length property"))?;
    
    js_sys::Reflect::set(
        &obj,
        &"hash_length".into(),
        &(hash_bytes.len() as u32).into()
    ).map_err(|_| JsValue::from_str("Failed to set hash_length property"))?;
    
    js_sys::Reflect::set(
        &obj,
        &"hash_bytes".into(),
        &js_sys::Uint8Array::from(&hash_bytes[..])
    ).map_err(|_| JsValue::from_str("Failed to set hash_bytes property"))?;
    
    js_sys::Reflect::set(
        &obj,
        &"hash_hex".into(),
        &hash_hex.into()
    ).map_err(|_| JsValue::from_str("Failed to set hash_hex property"))?;
    
    Ok(obj.into())
}
