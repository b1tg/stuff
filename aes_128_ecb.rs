#![no_std]
#![feature(type_ascription)]
#![feature(stdarch)]
#![feature(min_const_generics)]

/**
 * b1tg @ 2021.05.30 17:38
 *
 * refs links:
 *  - https://github.com/gamozolabs/chocolate_milk/tree/master/shared/falkhash
 *  - https://gist.github.com/acapola/d5b940da024080dfaf5f
 *
 * more:
 *  - https://github.com/amiralis/libaesni/blob/master/intel_aes.c
 *  - https://github.com/jackmott/simdeez
 */
extern crate alloc;

use core::arch::x86_64::*;
// use crate::alloc::borrow::ToOwned;
// use alloc::string::String;

macro_rules! AES_128_key_exp {
    ($x:expr, $y:expr) => {
        aes_128_key_expansion($x, _mm_aeskeygenassist_si128($x, $y))
    };
}

pub struct Cryptor {
    key_schedule: [__m128i; 20],
}

impl Cryptor {
    pub unsafe fn new(key: &[u8]) -> Self {
        let features = cpu::get_cpu_features();
        assert!(features.aesni, "AES-NI required for this");
        let k0 = _mm_loadu_si128(key.as_ptr() as *const __m128i);
        let k1 = AES_128_key_exp!(k0, 0x01);
        let k2 = AES_128_key_exp!(k1, 0x02);
        let k3 = AES_128_key_exp!(k2, 0x04);
        let k4 = AES_128_key_exp!(k3, 0x08);
        let k5 = AES_128_key_exp!(k4, 0x10);
        let k6 = AES_128_key_exp!(k5, 0x20);
        let k7 = AES_128_key_exp!(k6, 0x40);
        let k8 = AES_128_key_exp!(k7, 0x80);
        let k9 = AES_128_key_exp!(k8, 0x1B);
        let k10 = AES_128_key_exp!(k9, 0x36);
        let k11 = _mm_aesimc_si128(k9);
        let k12 = _mm_aesimc_si128(k8);
        let k13 = _mm_aesimc_si128(k7);
        let k14 = _mm_aesimc_si128(k6);
        let k15 = _mm_aesimc_si128(k5);
        let k16 = _mm_aesimc_si128(k4);
        let k17 = _mm_aesimc_si128(k3);
        let k18 = _mm_aesimc_si128(k2);
        let k19 = _mm_aesimc_si128(k1);
        let key_schedule = [
            k0, k1, k2, k3, k4, k5, k6, k7, k8, k9, k10, k11, k12, k13, k14, k15, k16, k17, k18,
            k19,
        ];
        Self {
            key_schedule: key_schedule,
        }
    }
}

impl Cryptor {
    pub fn aes_128_ecb_enc(&self, buffer: &[u8], result: &[u8]) {
        unsafe { self.aes_128_ecb_enc_(buffer: &[u8], result) }
    }
    pub fn aes_128_ecb_dec(&self, cipher_text: &[u8], plain_text: &[u8]) {
        unsafe { self.aes_128_ecb_dec_(cipher_text, plain_text) }
    }
    unsafe fn aes_128_ecb_dec_(&self, cipher_text: &[u8], plain_text: &[u8]) {
        let mut m = _mm_loadu_si128(cipher_text.as_ptr() as *const __m128i);
        m = _mm_xor_si128(m, self.key_schedule[10]);
        for i in 11..=19 {
            m = _mm_aesdec_si128(m, self.key_schedule[i]);
        }
        m = _mm_aesdeclast_si128(m, self.key_schedule[0]);
        _mm_storeu_si128(plain_text.as_ptr() as *mut __m128i, m);
    }
    unsafe fn aes_128_ecb_enc_(&self, buffer: &[u8], result: &[u8]) {
        let mut m = _mm_loadu_si128(buffer.as_ptr() as *const __m128i);
        m = _mm_xor_si128(m, self.key_schedule[0]);
        for i in 1..=9 {
            m = _mm_aesenc_si128(m, self.key_schedule[i]);
        }
        m = _mm_aesenclast_si128(m, self.key_schedule[10]);
        _mm_storeu_si128(result.as_ptr() as *mut __m128i, m);
    }
}

unsafe fn aes_128_key_expansion(key: __m128i, mut keygened: __m128i) -> __m128i {
    keygened = _mm_shuffle_epi32(keygened, _MM_SHUFFLE(3, 3, 3, 3));
    let mut key = _mm_xor_si128(key, _mm_slli_si128(key, 4));
    key = _mm_xor_si128(key, _mm_slli_si128(key, 4));
    key = _mm_xor_si128(key, _mm_slli_si128(key, 4));
    return _mm_xor_si128(key, keygened);
}

#[test]
fn validate_correctness() {
    // let test_data = [0x41u8; 128];
    let plain = [
        0x32u8, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07,
        0x34,
    ];
    let enc_key = [
        0x2bu8, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f,
        0x3c,
    ];
    let cipher = [
        0x39u8, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a, 0x0b,
        0x32,
    ];
    let fh = unsafe { Cryptor::new(&enc_key) };
    let mut result = [0x0u8; 16];
    fh.aes_128_ecb_enc(&plain, &mut result);
    debug_assert_eq!(result, cipher);

    let mut plain1 = [0x0u8; 16];
    fh.aes_128_ecb_dec(&cipher, &mut plain1);
    debug_assert_eq!(plain, plain1)
}
