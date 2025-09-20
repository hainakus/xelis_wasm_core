// Import the generated wasm module
import * as wasm from '../pkg/wasm_lib.js';
import { Buffer } from 'buffer';

async function main() {
    console.log('Initializing XELIS WASM module...');
    
    try {
        // Initialize panic hook for better error messages
        wasm.init_panic_hook();

        // Test 1: Basic hashing
        await testBasicHashing();
        
        // Test 2: String hashing
        await testStringHashing();
        
        // Test 3: Multiple hashing
        await testMultipleHashing();
        
        // Test 4: Batch hashing
        await testBatchHashing();
        
        // Test 5: Hash with metadata
        await testHashWithMetadata();
        
    } catch (error) {
        console.error('Test failed:', error);
        process.exit(1);
    }
}

async function testBasicHashing() {
    console.log('\n=== Testing Basic Hashing ===');
    const input = 'Hello, XELIS!';
    const data = new TextEncoder().encode(input);
    
    // Hash as bytes
    const hashBytes = wasm.xelis_hash(data);
    console.log('Input:', input);
    console.log('Hash (bytes):', Buffer.from(hashBytes).toString('hex'));
    
    // Hash as hex string
    const hashHex = wasm.xelis_hash_hex(data);
    console.log('Hash (hex):', hashHex);
    
    // Verify hash size
    const hashSize = wasm.get_hash_size();
    console.log('Hash size:', hashSize, 'bytes');
    console.assert(hashBytes.length === hashSize, 'Hash size mismatch');
    
    // Test bytes to hex conversion
    const bytesToHex = wasm.bytes_to_hex(hashBytes);
    console.log('Bytes to hex:', bytesToHex);
    console.assert(bytesToHex === hashHex, 'Hex conversion mismatch');
    
    // Test hex to bytes conversion
    const hexToBytes = wasm.hex_to_bytes(hashHex);
    console.log('Hex to bytes:', Buffer.from(hexToBytes).toString('hex'));
    console.assert(Buffer.from(hexToBytes).equals(Buffer.from(hashBytes)), 'Bytes conversion mismatch');
    
    // Test verify hash
    const isValid = wasm.verify_hash(hashHex, hashHex);
    console.log('Hash verification (should be true):', isValid);
    console.assert(isValid === true, 'Hash verification failed');
}

async function testStringHashing() {
    console.log('\n=== Testing String Hashing ===');
    const input = 'XELIS is awesome!';
    
    const hashHex = wasm.hash_string(input);
    console.log('Input string:', input);
    console.log('Hashed string:', hashHex);
    
    // Verify it matches the expected hash from xelis_hash_hex
    const expectedHash = wasm.xelis_hash_hex(new TextEncoder().encode(input));
    console.assert(hashHex === expectedHash, 'String hashing mismatch');
}

async function testMultipleHashing() {
    console.log('\n=== Testing Multiple Hashing ===');
    const input = new TextEncoder().encode('XELIS');
    const iterations = 3;
    
    const multiHash = wasm.xelis_hash_multiple(input, iterations);
    console.log(`Hash after ${iterations} iterations:`, Buffer.from(multiHash).toString('hex'));
    
    // Manually verify by hashing multiple times
    let expected = input;
    for (let i = 0; i < iterations; i++) {
        expected = wasm.xelis_hash(expected);
    }
    console.assert(Buffer.from(multiHash).equals(Buffer.from(expected)), 'Multiple hashing mismatch');
}

async function testBatchHashing() {
    console.log('\n=== Testing Batch Hashing ===');
    const inputs = [
        'XELIS',
        'Blockchain',
        'WASM',
        'Rust'
    ];
    
    // Convert strings to Uint8Array
    const data = inputs.map(str => new TextEncoder().encode(str));
    
    // Create JS array of Uint8Array for wasm
    const jsArray = data.map(buf => {
        const arr = new Uint8Array(buf.length);
        arr.set(buf);
        return arr;
    });
    
    const batchResults = wasm.batch_hash(jsArray);
    console.log(`Processed ${batchResults.length} hashes:`);
    
    // Verify each hash
    for (let i = 0; i < inputs.length; i++) {
        const expectedHash = wasm.xelis_hash_hex(data[i]);
        const actualHash = Buffer.from(batchResults[i]).toString('hex');
        console.log(`  ${inputs[i]}: ${actualHash}`);
        console.assert(actualHash === expectedHash, `Batch hash mismatch for input: ${inputs[i]}`);
    }
}

async function testHashWithMetadata() {
    console.log('\n=== Testing Hash with Metadata ===');
    const input = 'XELIS with metadata';
    const data = new TextEncoder().encode(input);
    
    const metadata = wasm.hash_with_metadata(data);
    console.log('Input:', input);
    console.log('Metadata:', {
        inputLength: metadata.input_length,
        hashLength: metadata.hash_length,
        hashHex: metadata.hash_hex,
        hashBytes: Buffer.from(metadata.hash_bytes).toString('hex')
    });
    
    // Verify metadata
    console.assert(metadata.input_length === input.length, 'Input length mismatch');
    console.assert(metadata.hash_length === wasm.get_hash_size(), 'Hash length mismatch');
    console.assert(metadata.hash_hex === wasm.xelis_hash_hex(data), 'Hash hex mismatch');
    console.assert(Buffer.from(metadata.hash_bytes).equals(Buffer.from(wasm.xelis_hash(data))), 'Hash bytes mismatch');
}

// Run the tests
main().catch(console.error);
