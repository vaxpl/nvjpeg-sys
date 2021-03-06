#![allow(deref_nullptr)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::op_ref)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_cuda_stream() {
        unsafe {
            let mut count = 0;
            let status = cudaGetDeviceCount(&mut count);
            assert_eq!(status, cudaError::cudaSuccess);
            assert!(count >= 1);
            let status = cudaSetDevice(0);
            assert_eq!(status, cudaError::cudaSuccess);
            let mut handle: cudaStream_t = std::ptr::null_mut();
            let status = cudaStreamCreate(&mut handle);
            assert_eq!(status, cudaError::cudaSuccess);
            let status = cudaStreamDestroy(handle);
            assert_eq!(status, cudaError::cudaSuccess);
        }
    }

    #[test]
    fn test_nvjpeg_create_simple() {
        use nvjpegChromaSubsampling_t::*;
        use nvjpegOutputFormat_t::*;
        use nvjpegStatus_t::*;

        const LENA_JPG: &[u8] = &[
            0xff, 0xd8, 0xff, 0xe0, 0x00, 0x10, 0x4a, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xff, 0xdb, 0x00, 0x84, 0x00, 0x06, 0x06, 0x06,
            0x06, 0x07, 0x06, 0x07, 0x08, 0x08, 0x07, 0x0a, 0x0b, 0x0a, 0x0b, 0x0a, 0x0f, 0x0e,
            0x0c, 0x0c, 0x0e, 0x0f, 0x16, 0x10, 0x11, 0x10, 0x11, 0x10, 0x16, 0x22, 0x15, 0x19,
            0x15, 0x15, 0x19, 0x15, 0x22, 0x1e, 0x24, 0x1e, 0x1c, 0x1e, 0x24, 0x1e, 0x36, 0x2a,
            0x26, 0x26, 0x2a, 0x36, 0x3e, 0x34, 0x32, 0x34, 0x3e, 0x4c, 0x44, 0x44, 0x4c, 0x5f,
            0x5a, 0x5f, 0x7c, 0x7c, 0xa7, 0x01, 0x06, 0x06, 0x06, 0x06, 0x07, 0x06, 0x07, 0x08,
            0x08, 0x07, 0x0a, 0x0b, 0x0a, 0x0b, 0x0a, 0x0f, 0x0e, 0x0c, 0x0c, 0x0e, 0x0f, 0x16,
            0x10, 0x11, 0x10, 0x11, 0x10, 0x16, 0x22, 0x15, 0x19, 0x15, 0x15, 0x19, 0x15, 0x22,
            0x1e, 0x24, 0x1e, 0x1c, 0x1e, 0x24, 0x1e, 0x36, 0x2a, 0x26, 0x26, 0x2a, 0x36, 0x3e,
            0x34, 0x32, 0x34, 0x3e, 0x4c, 0x44, 0x44, 0x4c, 0x5f, 0x5a, 0x5f, 0x7c, 0x7c, 0xa7,
            0xff, 0xc2, 0x00, 0x11, 0x08, 0x00, 0x6e, 0x00, 0x6e, 0x03, 0x01, 0x22, 0x00, 0x02,
            0x11, 0x01, 0x03, 0x11, 0x01, 0xff, 0xc4, 0x00, 0x1c, 0x00, 0x00, 0x03, 0x01, 0x00,
            0x03, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x06,
            0x07, 0x04, 0x00, 0x01, 0x03, 0x02, 0x08, 0xff, 0xda, 0x00, 0x08, 0x01, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x68, 0x00, 0xb0, 0x6f, 0xdb, 0xae, 0x7c, 0x65, 0x6f, 0xe6, 0xb4,
            0x74, 0x9a, 0xe8, 0x65, 0x83, 0x3e, 0xc4, 0x58, 0x57, 0x57, 0xda, 0xf8, 0x6e, 0x18,
            0xa9, 0x57, 0x2e, 0xae, 0x5f, 0x86, 0x1f, 0x0f, 0x48, 0x3d, 0x45, 0x0d, 0x9a, 0x58,
            0x46, 0x1f, 0x54, 0x39, 0xd3, 0x3b, 0x41, 0x0e, 0x26, 0x30, 0x42, 0xb4, 0x54, 0x91,
            0x4f, 0x26, 0x3e, 0x07, 0x2d, 0xdf, 0xd3, 0x99, 0x35, 0x4f, 0xcf, 0x57, 0x3d, 0x09,
            0x07, 0x92, 0x5f, 0x41, 0x28, 0x0c, 0xb3, 0x3a, 0x4c, 0x27, 0x86, 0xea, 0xb9, 0x91,
            0x19, 0xd3, 0x0e, 0xa2, 0x83, 0xb7, 0x7d, 0xbd, 0xc4, 0x67, 0x77, 0x32, 0x59, 0xe7,
            0xcf, 0x89, 0xc0, 0xf0, 0x98, 0x7a, 0xe8, 0x94, 0xa3, 0x2d, 0x8f, 0xc4, 0xf4, 0x42,
            0x98, 0x80, 0x23, 0xa2, 0xae, 0xe4, 0xbd, 0xe2, 0x54, 0x06, 0x53, 0x8e, 0x5f, 0x98,
            0x69, 0xd2, 0x92, 0xe4, 0xa7, 0x15, 0x02, 0xd8, 0x64, 0xf6, 0xcf, 0x67, 0xb3, 0xbf,
            0x96, 0xaa, 0x51, 0x07, 0x30, 0x4b, 0x2d, 0x94, 0xf1, 0xb3, 0x1a, 0x6e, 0x7a, 0xd4,
            0x8c, 0x53, 0xac, 0x1f, 0x6b, 0x9a, 0x05, 0x05, 0xbd, 0xbd, 0x6b, 0x27, 0xae, 0xc9,
            0xa6, 0xa7, 0x58, 0x1e, 0xa3, 0x3e, 0x4f, 0xc5, 0xcf, 0x39, 0xc1, 0xde, 0xa7, 0xa1,
            0x9c, 0x8a, 0x46, 0xf6, 0xfc, 0x37, 0x34, 0x6d, 0xd6, 0xeb, 0x27, 0x1e, 0x04, 0x57,
            0xff, 0xc4, 0x00, 0x1b, 0x01, 0x00, 0x03, 0x01, 0x00, 0x03, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x04, 0x05, 0x02, 0x00, 0x01, 0x06,
            0x07, 0xff, 0xda, 0x00, 0x0a, 0x02, 0x02, 0x10, 0x03, 0x10, 0x00, 0x00, 0x00, 0xf3,
            0xbf, 0x43, 0x70, 0xbd, 0x63, 0x80, 0xc9, 0x96, 0x5e, 0xf0, 0x25, 0x50, 0x36, 0xba,
            0x5f, 0x7e, 0x77, 0x35, 0x18, 0xe8, 0x9c, 0x97, 0x57, 0x79, 0x78, 0x0c, 0xcb, 0x44,
            0x0e, 0xb5, 0x2c, 0xf9, 0xa2, 0xfa, 0xf5, 0x26, 0xc5, 0x74, 0x98, 0xd8, 0xd1, 0xce,
            0xe8, 0xfa, 0x38, 0xf3, 0xcf, 0x1e, 0xfd, 0x19, 0x6a, 0x2f, 0x3d, 0x1a, 0x74, 0xbd,
            0x0c, 0xd0, 0x12, 0x0d, 0xe3, 0x46, 0x16, 0xc5, 0xdb, 0x1c, 0xbe, 0xa2, 0x0f, 0xc1,
            0xb4, 0x48, 0x46, 0xd3, 0xba, 0xff, 0xc4, 0x00, 0x27, 0x10, 0x00, 0x02, 0x03, 0x00,
            0x02, 0x02, 0x01, 0x03, 0x05, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x03,
            0x01, 0x04, 0x05, 0x00, 0x06, 0x11, 0x12, 0x13, 0x07, 0x15, 0x21, 0x14, 0x22, 0x25,
            0x31, 0x32, 0x35, 0x34, 0xff, 0xda, 0x00, 0x08, 0x01, 0x01, 0x00, 0x01, 0x08, 0x00,
            0x81, 0x22, 0x28, 0xf3, 0xd9, 0xd8, 0x45, 0x55, 0x5c, 0x02, 0x9f, 0x61, 0xe2, 0xca,
            0x7c, 0x72, 0x0e, 0x78, 0x4c, 0xf1, 0xf8, 0xe0, 0x9f, 0x9f, 0x3c, 0x92, 0x9f, 0x3c,
            0x23, 0x35, 0xd8, 0x51, 0x88, 0x48, 0x90, 0xc1, 0x72, 0x0d, 0x8b, 0x28, 0x30, 0xec,
            0x75, 0x7e, 0xe5, 0x9a, 0x8d, 0x74, 0x09, 0x0c, 0x09, 0x19, 0xe8, 0x76, 0x7b, 0xa6,
            0xf9, 0x8c, 0xeb, 0x77, 0x01, 0x57, 0x6b, 0x54, 0x1e, 0xd5, 0xe0, 0x6a, 0x2b, 0x8a,
            0x28, 0x92, 0x19, 0x90, 0x28, 0xe7, 0xb8, 0xc4, 0xcc, 0x72, 0xb2, 0xaa, 0x99, 0xc4,
            0x18, 0xe2, 0x67, 0xba, 0x3f, 0x1a, 0x99, 0xb6, 0x73, 0x62, 0x4d, 0x8d, 0x64, 0x11,
            0x8f, 0x85, 0x9f, 0xc6, 0x03, 0xec, 0x6d, 0x2f, 0xcc, 0x47, 0x5d, 0xbd, 0xf0, 0x59,
            0x24, 0x3b, 0xb5, 0x68, 0xd6, 0x6e, 0x95, 0xba, 0x59, 0xcc, 0x38, 0x8f, 0x04, 0x59,
            0xf6, 0x9b, 0x6b, 0x74, 0x6c, 0x33, 0xb7, 0x7f, 0xe4, 0x57, 0x15, 0xfd, 0xc7, 0x22,
            0x64, 0x55, 0x27, 0xc4, 0xc1, 0x99, 0xf0, 0x24, 0x55, 0x03, 0x25, 0x40, 0x0e, 0x56,
            0x12, 0xc5, 0x48, 0x8f, 0x3b, 0x5f, 0x5e, 0x66, 0x45, 0x91, 0xb7, 0x5f, 0x5a, 0x67,
            0xed, 0x16, 0x08, 0x7a, 0xfe, 0xc0, 0x68, 0xd0, 0x0f, 0x6e, 0xdf, 0xb3, 0x35, 0xeb,
            0xc6, 0x6a, 0x20, 0x7c, 0x90, 0x88, 0xf5, 0xbe, 0xb8, 0xaa, 0x81, 0x2d, 0x66, 0x42,
            0xfd, 0x74, 0x91, 0x3c, 0xed, 0xe1, 0x31, 0x45, 0x5c, 0x54, 0x44, 0x48, 0xf2, 0xd0,
            0x7a, 0x64, 0x66, 0x94, 0x56, 0x22, 0xf0, 0x3e, 0xb9, 0x68, 0x05, 0xb2, 0x18, 0x6b,
            0x7c, 0x0c, 0xfe, 0x02, 0xc8, 0xc0, 0xf2, 0xe0, 0xd6, 0xbb, 0x42, 0xcd, 0x3b, 0x36,
            0xb0, 0xaf, 0xda, 0xac, 0xea, 0x4a, 0xea, 0xfd, 0x5f, 0x2f, 0xae, 0xd6, 0x95, 0x54,
            0xfa, 0x8d, 0xf0, 0x87, 0x76, 0xdd, 0xf8, 0x7a, 0x66, 0x2c, 0xb9, 0xd3, 0xa0, 0xf1,
            0x98, 0x1f, 0x11, 0x18, 0xc1, 0x33, 0xa2, 0xa9, 0xe7, 0x71, 0xff, 0x00, 0x9e, 0xa9,
            0xe4, 0x4c, 0xfe, 0x27, 0x93, 0x58, 0x5b, 0xd6, 0x32, 0x0c, 0xa9, 0xc4, 0x02, 0xff,
            0x00, 0x0a, 0xb3, 0x0b, 0x8f, 0x69, 0x0e, 0xc3, 0x5b, 0xd1, 0x93, 0x28, 0xed, 0x99,
            0x67, 0x6d, 0x35, 0x10, 0x8a, 0xe7, 0x23, 0x06, 0xef, 0x20, 0xb5, 0x13, 0x19, 0x67,
            0xb3, 0xd1, 0x2b, 0x46, 0xd4, 0x07, 0xea, 0xb7, 0x36, 0x64, 0x99, 0x52, 0xb2, 0xaa,
            0x56, 0x52, 0x54, 0x25, 0xf9, 0xe6, 0x59, 0x40, 0xe9, 0xa2, 0x39, 0xdc, 0x63, 0xf8,
            0xc0, 0x9e, 0x04, 0x4c, 0xfa, 0xf3, 0x15, 0xd0, 0x7d, 0x7f, 0x42, 0xbb, 0x25, 0xaa,
            0x4a, 0x65, 0x87, 0x7f, 0x6d, 0xb6, 0x7c, 0x8a, 0xe8, 0x15, 0x98, 0xb5, 0x15, 0x07,
            0xad, 0x60, 0x54, 0xc8, 0xaa, 0x2c, 0x3a, 0xd0, 0xc6, 0x78, 0x2e, 0x7d, 0xcd, 0xb6,
            0xf5, 0xae, 0xd5, 0xd8, 0xdf, 0xec, 0x6f, 0xd9, 0x90, 0x10, 0xe8, 0x94, 0x3c, 0xb4,
            0xed, 0x99, 0x47, 0x26, 0x3c, 0x4f, 0x33, 0x67, 0xc6, 0xa5, 0x5e, 0x77, 0x08, 0xfe,
            0x2c, 0x38, 0xb9, 0x88, 0xf4, 0xf2, 0x66, 0xca, 0x69, 0x89, 0x66, 0xd5, 0xc6, 0x5a,
            0x67, 0x90, 0x09, 0xf0, 0xff, 0x00, 0xd9, 0xd7, 0xb1, 0x2b, 0xe1, 0xd1, 0x9b, 0x97,
            0xd3, 0xd9, 0x33, 0x9d, 0x61, 0x68, 0xe2, 0x6f, 0xd7, 0xaf, 0x56, 0x6c, 0xbb, 0x5b,
            0x42, 0x8d, 0xfe, 0xf0, 0x9b, 0x14, 0x59, 0x12, 0x20, 0x2b, 0x1c, 0x4a, 0x2b, 0xa3,
            0x9a, 0x84, 0x70, 0xa2, 0x62, 0x63, 0xd0, 0x9c, 0x41, 0x3e, 0x0b, 0x3d, 0xe3, 0xf7,
            0xca, 0x11, 0x1d, 0xc4, 0x62, 0x72, 0xc3, 0x90, 0x2b, 0xab, 0x09, 0xba, 0xcd, 0xbd,
            0x27, 0xe8, 0x2e, 0x6e, 0xd8, 0xd1, 0x58, 0xc5, 0x0a, 0xcc, 0x4f, 0x46, 0xcb, 0x53,
            0x5e, 0xfd, 0x6b, 0xd7, 0x93, 0x1d, 0x94, 0x57, 0xf3, 0x16, 0x15, 0x6a, 0x99, 0xf5,
            0x14, 0x3a, 0x99, 0xe9, 0xd2, 0xeb, 0x99, 0x9f, 0x2c, 0xa6, 0x51, 0xdb, 0x69, 0xb4,
            0xf3, 0xe9, 0x45, 0x8e, 0xcd, 0x08, 0x99, 0x9f, 0x58, 0x81, 0xe7, 0xb3, 0x1a, 0xc0,
            0x52, 0xab, 0x60, 0x2f, 0xcf, 0x97, 0xe4, 0x2d, 0x87, 0xbd, 0x9f, 0xe9, 0xdb, 0x7f,
            0xe4, 0x8c, 0xf2, 0xfb, 0xc6, 0x66, 0x16, 0x3a, 0x55, 0x49, 0xb9, 0xf5, 0x49, 0x54,
            0x81, 0x85, 0x54, 0xea, 0xd4, 0xdb, 0xb2, 0xba, 0xc1, 0x47, 0xac, 0x52, 0xcb, 0x69,
            0x00, 0x87, 0x34, 0x5c, 0xd3, 0x50, 0x94, 0x57, 0x79, 0xc6, 0x0c, 0x28, 0xf4, 0x74,
            0x21, 0xba, 0x6f, 0xb8, 0x19, 0x19, 0xf0, 0xbe, 0xc7, 0xbf, 0x6b, 0x8c, 0x9f, 0x3c,
            0xeb, 0xd5, 0x3d, 0xc5, 0x96, 0x65, 0x4b, 0x88, 0xfe, 0xba, 0xa3, 0x3e, 0x4d, 0x74,
            0x9c, 0xf6, 0xc8, 0x0f, 0xb4, 0x44, 0x16, 0xdb, 0xa6, 0x1e, 0x40, 0x23, 0x4d, 0x88,
            0x69, 0xd7, 0x8d, 0xd7, 0xd6, 0xc2, 0xc7, 0xa7, 0x4a, 0x8d, 0x42, 0x80, 0x72, 0xd9,
            0xcc, 0xab, 0x80, 0xd5, 0x2f, 0x90, 0xcb, 0x70, 0xc9, 0x18, 0xed, 0xdb, 0xf6, 0x91,
            0x9b, 0xe8, 0x60, 0xa9, 0x3a, 0x2e, 0x1e, 0x66, 0x8f, 0x9a, 0x69, 0x6f, 0x1b, 0xfb,
            0x62, 0x7c, 0xf5, 0xd5, 0x7f, 0x0f, 0x50, 0xe0, 0x26, 0x22, 0x7c, 0x46, 0x1a, 0xac,
            0xe4, 0xec, 0x29, 0x16, 0x7b, 0x22, 0xa1, 0xf9, 0xb0, 0x12, 0xc9, 0x65, 0xe9, 0x69,
            0x26, 0xa5, 0xa4, 0xc9, 0xc3, 0x79, 0xbf, 0x48, 0x82, 0xc0, 0xd8, 0x9f, 0x12, 0x27,
            0xce, 0xbe, 0x96, 0x1b, 0x4c, 0xe2, 0x8b, 0x88, 0x22, 0x06, 0x7b, 0x27, 0x47, 0xd1,
            0xd6, 0x60, 0x5d, 0xa7, 0x08, 0x6d, 0x35, 0xd9, 0xad, 0x6e, 0xbb, 0x2d, 0x22, 0x9d,
            0x41, 0x0b, 0x1a, 0xa3, 0xe8, 0x70, 0x75, 0x6c, 0x55, 0xce, 0xc3, 0xac, 0xd7, 0x69,
            0xfd, 0x41, 0xdf, 0xb7, 0x68, 0xfe, 0xd8, 0x2f, 0x9b, 0x98, 0x5d, 0x5a, 0xf3, 0x3b,
            0x63, 0x65, 0x58, 0x57, 0x48, 0x69, 0x47, 0xb1, 0x17, 0x9a, 0x16, 0x02, 0x1a, 0x22,
            0xdd, 0x25, 0xb1, 0xf4, 0x0b, 0xde, 0x56, 0xd2, 0x7c, 0x2c, 0x72, 0xeb, 0x7c, 0x5a,
            0x37, 0x03, 0x95, 0xcb, 0xc1, 0xc7, 0x33, 0x9d, 0xe6, 0x46, 0x39, 0xf5, 0x4b, 0x0c,
            0x2e, 0x60, 0xc6, 0xa2, 0x0a, 0x20, 0x02, 0x06, 0x32, 0x32, 0x87, 0x52, 0xe3, 0x40,
            0xfe, 0xa1, 0x5c, 0x82, 0x1a, 0x99, 0x41, 0xe5, 0x51, 0xfd, 0xe3, 0xd6, 0xb0, 0x3d,
            0x42, 0xa4, 0x37, 0xb6, 0xc8, 0xce, 0x05, 0x9f, 0x64, 0xcf, 0xf9, 0x89, 0xa6, 0x4a,
            0x1b, 0x3e, 0xae, 0xda, 0x6b, 0x92, 0x8f, 0x8e, 0xce, 0x08, 0x7b, 0x11, 0xdf, 0xb5,
            0x90, 0x0c, 0x95, 0xb2, 0xcb, 0xbf, 0xc9, 0xc4, 0xf2, 0x83, 0xfc, 0x18, 0xcf, 0x25,
            0x0a, 0xbf, 0x49, 0xd5, 0x58, 0x7b, 0x77, 0xeb, 0x41, 0xcd, 0x9e, 0x9b, 0x64, 0x6e,
            0x2d, 0x1a, 0x6b, 0xd8, 0xda, 0x8d, 0x7d, 0x3b, 0xb6, 0x42, 0xed, 0xa2, 0x40, 0x2e,
            0x47, 0xaa, 0xc1, 0xdf, 0xea, 0x9a, 0xa6, 0xce, 0xf8, 0x64, 0xbe, 0xb3, 0xca, 0x95,
            0xfd, 0xcd, 0x3c, 0xf8, 0x86, 0x6a, 0xa9, 0xd1, 0x79, 0x05, 0x70, 0x6b, 0x10, 0xe7,
            0x63, 0xd6, 0x50, 0x28, 0x89, 0x51, 0x31, 0x1c, 0x10, 0xf6, 0x88, 0xe5, 0x65, 0xc8,
            0x94, 0x78, 0xcd, 0x33, 0xf5, 0x8e, 0x76, 0x5a, 0x60, 0x9e, 0xcd, 0xa2, 0x9e, 0x5c,
            0x1b, 0x11, 0x96, 0x75, 0x39, 0x4e, 0xa7, 0xea, 0x1a, 0xb5, 0xc3, 0x65, 0xba, 0x57,
            0x5d, 0x09, 0xff, 0xc4, 0x00, 0x39, 0x10, 0x00, 0x02, 0x01, 0x02, 0x04, 0x04, 0x03,
            0x04, 0x08, 0x05, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x00, 0x11,
            0x12, 0x21, 0x31, 0x41, 0x04, 0x13, 0x51, 0x61, 0x10, 0x22, 0x71, 0x32, 0x42, 0x91,
            0xa1, 0x05, 0x23, 0x52, 0x53, 0x72, 0x81, 0xb1, 0xc1, 0x14, 0x33, 0x43, 0x82, 0xd1,
            0x62, 0x73, 0x92, 0xa2, 0xe2, 0xff, 0xda, 0x00, 0x08, 0x01, 0x01, 0x00, 0x09, 0x3f,
            0x00, 0x26, 0x89, 0xb0, 0x90, 0x51, 0xa3, 0xe0, 0x4d, 0x13, 0xe0, 0x73, 0x0d, 0x47,
            0x51, 0x44, 0x82, 0xa6, 0xe0, 0xf7, 0x14, 0x3e, 0xb2, 0x15, 0xc1, 0x3a, 0x8f, 0xb1,
            0xff, 0x00, 0x9a, 0x70, 0xa8, 0xab, 0x89, 0x9c, 0x9b, 0x00, 0x06, 0xe6, 0xa7, 0x97,
            0x87, 0x88, 0x68, 0xea, 0x4a, 0xbc, 0x9d, 0xcd, 0x37, 0x9d, 0xee, 0xc7, 0xd0, 0x57,
            0xde, 0xad, 0x0f, 0x03, 0xe6, 0xe9, 0x48, 0x4f, 0xa9, 0xd6, 0xa3, 0x68, 0xc9, 0xdd,
            0x18, 0xdc, 0x55, 0x9e, 0x0b, 0xd8, 0x4c, 0x36, 0xec, 0xc3, 0x6a, 0xea, 0x28, 0xd8,
            0x11, 0xe1, 0x63, 0x04, 0xe0, 0x87, 0xbe, 0x80, 0xf5, 0x3d, 0xaa, 0x7c, 0x7f, 0x47,
            0xc3, 0x31, 0x08, 0xe3, 0xfa, 0xb6, 0xdf, 0xd0, 0x6d, 0x46, 0xc3, 0x4a, 0xd5, 0xc9,
            0xcb, 0xa0, 0xd8, 0x57, 0xde, 0x2f, 0x86, 0x81, 0x82, 0xdf, 0xb9, 0xa3, 0x76, 0x26,
            0xe4, 0x9a, 0x6d, 0xec, 0x2a, 0xe8, 0x2d, 0x90, 0xf7, 0x8f, 0xf8, 0xa4, 0x5b, 0x6e,
            0x08, 0x06, 0xe3, 0xbd, 0xe8, 0x5f, 0xe8, 0xf9, 0x67, 0x55, 0x51, 0xbc, 0x2c, 0xde,
            0xe3, 0x76, 0xe8, 0x68, 0x90, 0xcb, 0x09, 0x2a, 0x7b, 0x81, 0x4c, 0x39, 0xd1, 0xad,
            0x9d, 0x77, 0xcb, 0x7a, 0x7b, 0x4b, 0x3a, 0xde, 0x76, 0x1e, 0xe4, 0x67, 0x45, 0xf5,
            0x6a, 0x04, 0x92, 0x40, 0x50, 0x33, 0x24, 0x9c, 0xa9, 0x10, 0xce, 0xc9, 0x84, 0xe2,
            0x18, 0x82, 0x0d, 0x70, 0x8a, 0xea, 0x6b, 0x79, 0x56, 0x8d, 0x2f, 0xf3, 0x66, 0x9e,
            0x52, 0x7a, 0x8b, 0xe0, 0x5f, 0x92, 0xd2, 0x92, 0xc7, 0x61, 0xa9, 0xa2, 0x1a, 0x6f,
            0xb7, 0xb2, 0x76, 0x5a, 0x37, 0x3d, 0x4e, 0xd4, 0x49, 0x36, 0xc8, 0x57, 0xf2, 0xa7,
            0x8f, 0x0b, 0xdb, 0x51, 0xb8, 0x61, 0xdc, 0x11, 0x7a, 0x74, 0x17, 0x5c, 0x07, 0x88,
            0x60, 0x70, 0xd8, 0xfb, 0xc1, 0x77, 0xa4, 0xc7, 0x2c, 0x96, 0xe7, 0x4e, 0xf6, 0x2f,
            0x21, 0xef, 0xfb, 0x0a, 0x00, 0x0e, 0x6c, 0x61, 0x80, 0xfb, 0x7c, 0xb5, 0xc7, 0x49,
            0x92, 0x12, 0xb0, 0x5f, 0xae, 0xef, 0xfb, 0x0a, 0x19, 0x01, 0x5d, 0x6b, 0xef, 0x57,
            0xc1, 0x80, 0x07, 0x85, 0xb0, 0x3d, 0x1a, 0xe4, 0xda, 0x8e, 0x76, 0xf3, 0x37, 0xec,
            0x2a, 0xe2, 0xd7, 0xc2, 0x06, 0xb4, 0x70, 0x08, 0xc8, 0x57, 0x04, 0xd9, 0xbc, 0xda,
            0x10, 0x37, 0x15, 0x33, 0xcb, 0x3c, 0x87, 0x24, 0x54, 0x26, 0xc3, 0xab, 0x74, 0x15,
            0xaf, 0xd9, 0xa7, 0x48, 0xe3, 0x51, 0x76, 0x66, 0x60, 0xaa, 0x3d, 0x49, 0xa9, 0x99,
            0xb8, 0x1e, 0x05, 0x39, 0x9c, 0x47, 0x12, 0x32, 0x8f, 0x2f, 0x71, 0x49, 0xf6, 0x9d,
            0xf4, 0x51, 0x4c, 0x79, 0xfc, 0x67, 0x12, 0xd2, 0x48, 0x7a, 0x62, 0x25, 0x98, 0xfe,
            0x42, 0x93, 0x0a, 0x22, 0x85, 0x51, 0xd0, 0x0f, 0x0d, 0xcd, 0x7d, 0xea, 0x78, 0x10,
            0x07, 0x02, 0x79, 0xb1, 0xb9, 0x3b, 0x4d, 0x7b, 0x0f, 0xc9, 0xa9, 0xb0, 0x44, 0x9a,
            0x93, 0xb7, 0x7f, 0x5a, 0x94, 0xa4, 0x42, 0x5c, 0x2d, 0x6c, 0x88, 0x50, 0x77, 0x3b,
            0xd4, 0x72, 0x19, 0x54, 0xb3, 0x38, 0x00, 0x96, 0x2d, 0xa6, 0x23, 0x6e, 0x9a, 0x2d,
            0x25, 0xf8, 0xa9, 0x46, 0x6e, 0x75, 0xb0, 0xad, 0x09, 0xca, 0xa5, 0xe7, 0x7f, 0x0d,
            0x8d, 0xde, 0x2c, 0x36, 0x8d, 0x51, 0x33, 0xba, 0x25, 0x44, 0x38, 0x7e, 0x06, 0x02,
            0x4c, 0x1c, 0x32, 0xf5, 0xfb, 0x6f, 0xd5, 0xe9, 0x72, 0x37, 0x8d, 0x0f, 0xcd, 0x8f,
            0x8e, 0xec, 0x6b, 0xef, 0x52, 0xba, 0x8a, 0x56, 0x58, 0x78, 0x9f, 0x24, 0xce, 0x72,
            0x58, 0xd5, 0x2c, 0xea, 0x5e, 0xfb, 0x93, 0x90, 0xa0, 0x63, 0x89, 0x55, 0x79, 0x4b,
            0x6b, 0x13, 0x98, 0xa5, 0xf3, 0x1c, 0x3c, 0xa4, 0xd4, 0xbe, 0x2d, 0x00, 0xeb, 0x7a,
            0x68, 0xa3, 0x79, 0x45, 0xe7, 0x94, 0xe4, 0x2f, 0x6b, 0x00, 0x3f, 0x61, 0x4e, 0x19,
            0x40, 0x0a, 0xa6, 0x37, 0x59, 0x6c, 0x01, 0xb5, 0xdc, 0x2e, 0x62, 0xa4, 0x54, 0x85,
            0x05, 0xdd, 0xce, 0x82, 0x9a, 0xf0, 0xf1, 0x91, 0xbc, 0x2c, 0xd7, 0x19, 0x97, 0x42,
            0xb4, 0x2e, 0xc7, 0x6e, 0xfa, 0x01, 0x43, 0xd9, 0x40, 0xbe, 0xa4, 0x6a, 0x7f, 0x33,
            0x4c, 0x45, 0x2d, 0xfd, 0x2b, 0x79, 0x0f, 0xe9, 0x5f, 0x78, 0xb5, 0x29, 0x8d, 0x61,
            0x65, 0x75, 0x40, 0xca, 0x8c, 0xec, 0xa6, 0xe3, 0xcc, 0x6f, 0x60, 0x3d, 0x29, 0x04,
            0x09, 0xcc, 0x0b, 0xc3, 0x70, 0xeb, 0x9a, 0xa8, 0xdd, 0xb3, 0xcf, 0x1f, 0x52, 0x69,
            0xae, 0xf0, 0xb9, 0x76, 0xb3, 0x06, 0xb2, 0xb9, 0xc8, 0x9f, 0xcc, 0x5a, 0xa5, 0xe5,
            0xf0, 0x5c, 0x0a, 0x17, 0x0c, 0x74, 0x5d, 0x89, 0x1f, 0xa0, 0x14, 0xa5, 0x23, 0x8d,
            0xaf, 0x04, 0x37, 0xf6, 0x3f, 0x17, 0x56, 0x3b, 0xd7, 0x0d, 0x12, 0x3f, 0x0e, 0xf2,
            0x32, 0x48, 0xb7, 0x2e, 0x79, 0x99, 0xb5, 0xc9, 0xda, 0xae, 0xd1, 0xc5, 0x3c, 0x8e,
            0xd1, 0x86, 0x2a, 0x19, 0xc1, 0x16, 0x26, 0x95, 0x14, 0xc9, 0xc6, 0x44, 0xe4, 0x20,
            0xb2, 0x27, 0x32, 0x40, 0x2c, 0x29, 0x6e, 0x21, 0x99, 0xcb, 0x0f, 0xf6, 0x8f, 0xf9,
            0xad, 0x14, 0x52, 0xe2, 0x76, 0x36, 0x02, 0x99, 0xa4, 0x72, 0x36, 0x36, 0x51, 0x42,
            0xe1, 0x5c, 0x96, 0xf8, 0x51, 0xfe, 0xaa, 0x77, 0xde, 0x80, 0x4f, 0xa8, 0x5b, 0x0f,
            0xc4, 0xc6, 0xed, 0x73, 0xd0, 0x54, 0x76, 0xb4, 0xc5, 0x2f, 0xa9, 0xbc, 0xad, 0xe5,
            0xf8, 0x9a, 0xe1, 0x09, 0x9b, 0x88, 0xc3, 0x17, 0x13, 0x23, 0x82, 0xcb, 0x02, 0xa9,
            0xc2, 0x59, 0x87, 0xa9, 0xa9, 0x03, 0x41, 0xc2, 0xca, 0xcd, 0xc5, 0x48, 0xba, 0x4b,
            0x3a, 0x03, 0x61, 0xe8, 0x9f, 0xad, 0x6b, 0x61, 0x40, 0x91, 0xbd, 0xb5, 0xa8, 0xa4,
            0x46, 0xfe, 0x24, 0x94, 0xc6, 0x2d, 0x75, 0xb5, 0x8d, 0x0b, 0xa4, 0x33, 0x45, 0x80,
            0xf6, 0x89, 0x81, 0xbf, 0xca, 0xbd, 0x9f, 0xe2, 0x2d, 0x1f, 0xf7, 0x0c, 0x6d, 0xf3,
            0x3e, 0x03, 0xcc, 0xec, 0x51, 0x7b, 0x28, 0xa1, 0x44, 0xf9, 0x72, 0x3f, 0x9d, 0x5c,
            0x8e, 0x6a, 0x64, 0x35, 0x24, 0x9c, 0x85, 0x3a, 0x95, 0xc2, 0x11, 0x8f, 0x78, 0x96,
            0xff, 0x00, 0xf1, 0xbb, 0x53, 0x15, 0x56, 0x5c, 0x0c, 0x5b, 0xd9, 0x1a, 0x06, 0x61,
            0xdd, 0x4e, 0x94, 0x39, 0x32, 0x4d, 0x1a, 0xca, 0xa2, 0xf7, 0x76, 0x73, 0xa4, 0xcd,
            0xd0, 0x01, 0xec, 0xf5, 0x6a, 0xc8, 0xac, 0x80, 0xfc, 0x45, 0xa8, 0xfb, 0xa2, 0xf4,
            0xea, 0x56, 0xf9, 0x1e, 0x59, 0x6b, 0x0e, 0xe0, 0x1a, 0x9d, 0x5a, 0x49, 0x81, 0x48,
            0x40, 0x8c, 0x46, 0x45, 0xc5, 0x9d, 0xcd, 0x0b, 0x6a, 0x3e, 0x55, 0xac, 0xb1, 0x47,
            0x21, 0xf5, 0x65, 0x1e, 0x1e, 0xfa, 0x93, 0xf1, 0x3e, 0x11, 0xd8, 0x4b, 0x22, 0xaa,
            0x9a, 0xc4, 0x47, 0x31, 0x4e, 0x15, 0xb0, 0x26, 0xd9, 0xe6, 0x4e, 0x4a, 0xbd, 0x4d,
            0x5c, 0x8e, 0x64, 0xa1, 0x5c, 0xed, 0xef, 0x66, 0x6a, 0xcd, 0x14, 0x8c, 0xac, 0x91,
            0xb7, 0xb6, 0x1c, 0xe4, 0xea, 0x3a, 0x0a, 0x91, 0x9b, 0xea, 0xac, 0xe8, 0xc7, 0x11,
            0x8c, 0x28, 0xc2, 0x99, 0xf4, 0x23, 0x6d, 0xab, 0x40, 0x45, 0xea, 0x77, 0x8f, 0x01,
            0xb1, 0x50, 0x01, 0x07, 0xe3, 0x5e, 0x62, 0x74, 0xc8, 0x0a, 0xe2, 0xe2, 0x79, 0x16,
            0x30, 0x07, 0x0e, 0xfe, 0x4e, 0xfe, 0x56, 0xa8, 0x5e, 0x19, 0xe3, 0x95, 0x84, 0x91,
            0xb8, 0xb3, 0x29, 0xb5, 0x22, 0x94, 0x5e, 0x1a, 0x15, 0xb6, 0xf7, 0x08, 0x2a, 0x27,
            0x56, 0x54, 0x66, 0x39, 0x5c, 0x58, 0x54, 0xab, 0x14, 0x10, 0x70, 0xa8, 0xcf, 0x23,
            0xe4, 0x00, 0xb5, 0xe8, 0xc7, 0xc2, 0x70, 0xaa, 0x7c, 0x8c, 0xf1, 0x07, 0x96, 0x4e,
            0xed, 0x8b, 0x4a, 0x00, 0xcc, 0x7e, 0x91, 0x8d, 0x1d, 0xfe, 0x22, 0x98, 0x82, 0x60,
            0x65, 0x36, 0xef, 0x46, 0xc8, 0x90, 0xc8, 0xdf, 0x8d, 0x82, 0x9b, 0x0a, 0x62, 0x30,
            0xdc, 0xf3, 0x09, 0xd2, 0xfd, 0x7f, 0x7a, 0x09, 0x8d, 0xbe, 0x8e, 0x90, 0xca, 0x84,
            0xf9, 0xf9, 0xcd, 0x61, 0x16, 0x42, 0xe4, 0x85, 0xa5, 0x6c, 0x4e, 0x54, 0x28, 0x22,
            0xc6, 0xf4, 0x05, 0x84, 0x70, 0x7c, 0x70, 0xd6, 0xb4, 0x69, 0x6d, 0xc4, 0xf0, 0x8c,
            0x88, 0xed, 0xbb, 0xc1, 0x2b, 0x04, 0xff, 0x00, 0xa9, 0x35, 0xb0, 0xa3, 0x68, 0xe3,
            0x4b, 0xbf, 0x7c, 0x46, 0xc0, 0x0a, 0xf6, 0x71, 0x09, 0x26, 0xfc, 0xbd, 0x95, 0xa0,
            0x30, 0xe6, 0x17, 0xfc, 0xd2, 0x10, 0xdc, 0x3f, 0xd2, 0xd1, 0x9b, 0x76, 0x2f, 0x43,
            0x26, 0x52, 0x3e, 0x58, 0xaa, 0xc3, 0x6f, 0x8e, 0xe6, 0xae, 0x23, 0xb8, 0x0c, 0x54,
            0x5e, 0xc2, 0x84, 0x32, 0xbb, 0x85, 0xe5, 0x4a, 0x9e, 0xca, 0x85, 0x22, 0xec, 0xbf,
            0x88, 0x54, 0x85, 0x61, 0x81, 0x72, 0x66, 0xa4, 0x28, 0xf3, 0xbe, 0x32, 0x87, 0x55,
            0x5d, 0x15, 0x4f, 0xa0, 0xf0, 0x34, 0x03, 0x24, 0xd1, 0x94, 0x20, 0xf7, 0xd2, 0xb8,
            0x65, 0x92, 0xc4, 0x86, 0x0b, 0x91, 0x04, 0x1b, 0x11, 0x4e, 0x60, 0xe1, 0x49, 0x97,
            0x9f, 0x8c, 0xdb, 0x28, 0xba, 0xd1, 0x22, 0x23, 0x21, 0x21, 0xbf, 0xd3, 0xa5, 0x2d,
            0xcb, 0x6d, 0xd0, 0x53, 0x9f, 0xa8, 0x9d, 0x30, 0xff, 0x00, 0x6d, 0x9a, 0xbd, 0xe9,
            0x51, 0x49, 0xec, 0x41, 0xab, 0x7b, 0x6b, 0xf2, 0xcc, 0xd2, 0x8c, 0x4b, 0x0a, 0xe3,
            0xee, 0x56, 0xcd, 0xfa, 0x12, 0x28, 0xaa, 0x85, 0x3c, 0x93, 0x7d, 0x4d, 0xb4, 0x6f,
            0x80, 0xab, 0xc8, 0x50, 0x5d, 0x71, 0x9b, 0x85, 0x3d, 0x86, 0x83, 0xc4, 0xf8, 0x01,
            0x82, 0x57, 0x12, 0x81, 0xd3, 0x9a, 0x03, 0x1a, 0x98, 0xaf, 0x0a, 0xb2, 0x17, 0xe4,
            0xae, 0x40, 0xbb, 0x6e, 0x68, 0x85, 0x89, 0x3c, 0xec, 0x06, 0xac, 0x47, 0x5a, 0x22,
            0x30, 0x99, 0x0b, 0xd7, 0xff, 0xc4, 0x00, 0x2a, 0x11, 0x00, 0x02, 0x01, 0x03, 0x03,
            0x02, 0x06, 0x01, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00,
            0x03, 0x04, 0x11, 0x12, 0x21, 0x31, 0x41, 0x71, 0x13, 0x14, 0x22, 0x51, 0x61, 0xb1,
            0x32, 0x10, 0x24, 0x42, 0x62, 0x91, 0xff, 0xda, 0x00, 0x08, 0x01, 0x02, 0x01, 0x01,
            0x3f, 0x00, 0xa3, 0x9f, 0x16, 0x9f, 0xc9, 0xda, 0x54, 0x06, 0x04, 0x26, 0x68, 0x32,
            0xaa, 0xe5, 0x57, 0xde, 0x0c, 0xa9, 0x0c, 0x39, 0x12, 0xda, 0xd9, 0x40, 0x0e, 0xe3,
            0x24, 0x8d, 0x81, 0xe9, 0x18, 0x28, 0xb9, 0xa4, 0x04, 0x7c, 0x6a, 0x1f, 0x27, 0xf4,
            0x52, 0x25, 0x5f, 0xca, 0x91, 0xc7, 0x59, 0x46, 0xde, 0x93, 0x5c, 0x96, 0x1f, 0x88,
            0xdc, 0x09, 0x7b, 0x5f, 0x9a, 0x6a, 0x7b, 0x98, 0xf9, 0xf3, 0x54, 0xbb, 0x89, 0x70,
            0x0e, 0x14, 0x8e, 0x8c, 0x25, 0x0b, 0x66, 0xaa, 0xdf, 0x1d, 0x65, 0x7b, 0x7b, 0x7a,
            0x49, 0xab, 0x72, 0xdc, 0x63, 0x31, 0x14, 0xb6, 0xaf, 0x81, 0x99, 0x4d, 0x7c, 0x0a,
            0x4e, 0xec, 0x3d, 0x44, 0x42, 0x75, 0x64, 0x93, 0xcc, 0xab, 0xb5, 0xc5, 0x03, 0xda,
            0x51, 0xb4, 0xa9, 0x59, 0x95, 0x88, 0xc2, 0x75, 0x26, 0x39, 0x4b, 0x74, 0xdb, 0xf8,
            0xfd, 0xc2, 0xe1, 0xc3, 0x97, 0x63, 0xa8, 0x9e, 0x25, 0xaa, 0xa8, 0x6c, 0x96, 0xdf,
            0x49, 0xda, 0x5e, 0xbe, 0x42, 0x20, 0xee, 0x61, 0x7a, 0x68, 0x3d, 0x67, 0x73, 0xc0,
            0x1c, 0xcb, 0xa0, 0x7c, 0xd5, 0x01, 0xdb, 0xee, 0x51, 0x07, 0xcb, 0x81, 0xfd, 0x60,
            0xf5, 0xe5, 0xf0, 0x71, 0xbe, 0x3b, 0xc2, 0x0e, 0x48, 0x32, 0xd5, 0x30, 0xa5, 0xcf,
            0x61, 0x1d, 0x8b, 0xd4, 0x24, 0xf4, 0xdb, 0xfc, 0x8d, 0xbd, 0x57, 0x66, 0xf7, 0xc0,
            0x97, 0x43, 0xf7, 0x94, 0x33, 0xc9, 0x60, 0x22, 0x1c, 0x6d, 0x1c, 0x05, 0x63, 0x2b,
            0xd4, 0x05, 0xf2, 0x3a, 0x81, 0x2d, 0xee, 0x18, 0xe2, 0x9b, 0x01, 0x8e, 0x86, 0x1a,
            0x8a, 0xaa, 0xce, 0x44, 0xb4, 0xa6, 0xae, 0x0b, 0xb2, 0x86, 0x24, 0x9c, 0x03, 0x2e,
            0x32, 0xd7, 0xb6, 0xe3, 0xae, 0xb1, 0x07, 0xbc, 0xbf, 0xac, 0xa8, 0x0a, 0x92, 0x72,
            0xdf, 0x51, 0x9f, 0x51, 0x26, 0x52, 0x3b, 0xca, 0xe8, 0xaf, 0x4d, 0x50, 0x6c, 0x58,
            0x8d, 0xfe, 0x25, 0x2a, 0x49, 0x4c, 0x05, 0x1c, 0x28, 0x9f, 0xff, 0xc4, 0x00, 0x28,
            0x11, 0x00, 0x02, 0x02, 0x02, 0x01, 0x03, 0x02, 0x07, 0x01, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x03, 0x11, 0x21, 0x12, 0x04, 0x31, 0x41, 0x51,
            0x61, 0x05, 0x13, 0x22, 0x32, 0x71, 0x81, 0xa1, 0x42, 0x91, 0xff, 0xda, 0x00, 0x08,
            0x01, 0x03, 0x01, 0x01, 0x3f, 0x00, 0x0b, 0xe7, 0xd8, 0xc5, 0x19, 0x96, 0x5a, 0x95,
            0xf7, 0x04, 0xca, 0xba, 0xca, 0x2c, 0x6e, 0x39, 0xe2, 0x7d, 0x0c, 0x65, 0xf4, 0x97,
            0xd6, 0xc0, 0x0b, 0x17, 0xee, 0x5d, 0xfe, 0x44, 0xb3, 0xa8, 0xe5, 0x80, 0xb9, 0x02,
            0x6f, 0xf8, 0x66, 0x78, 0xe3, 0xdc, 0xcb, 0xce, 0x01, 0x27, 0x72, 0xf4, 0x62, 0x49,
            0xc4, 0xe8, 0x7a, 0xcc, 0xd4, 0xeb, 0x73, 0x60, 0xa7, 0x93, 0xe4, 0x41, 0xf1, 0x23,
            0x68, 0x35, 0x84, 0xc1, 0xc7, 0xdd, 0x98, 0x89, 0xb0, 0x63, 0x77, 0x5f, 0xc1, 0x96,
            0x12, 0xbf, 0x2d, 0xbd, 0x1b, 0x7f, 0xbd, 0x4b, 0xec, 0x55, 0x04, 0x9e, 0xf1, 0xec,
            0xde, 0xd4, 0x62, 0x0a, 0x5a, 0xc6, 0xde, 0x97, 0x20, 0x18, 0xb5, 0xaa, 0x91, 0x8e,
            0xc0, 0x40, 0xd0, 0xed, 0x97, 0xf7, 0x3a, 0x8e, 0xa9, 0x6b, 0xca, 0x01, 0x93, 0x39,
            0xb5, 0x84, 0xb3, 0x19, 0x57, 0x4f, 0xc5, 0xeb, 0xe6, 0xbb, 0x3a, 0xc6, 0x7e, 0xd1,
            0x0d, 0x5c, 0x45, 0xcc, 0x01, 0xc7, 0x22, 0xbf, 0xf0, 0xc5, 0x10, 0xb7, 0x13, 0xa3,
            0x10, 0x8d, 0x7a, 0x6e, 0x75, 0x2c, 0x3e, 0x79, 0x39, 0xff, 0x00, 0x51, 0x46, 0x58,
            0x67, 0xc1, 0x8c, 0xa1, 0x99, 0x5c, 0x62, 0x5c, 0xc7, 0x2c, 0x9a, 0xdb, 0x16, 0x6c,
            0x42, 0x40, 0x00, 0x7b, 0x4b, 0xdf, 0x2e, 0x40, 0xf1, 0x28, 0xb0, 0xb5, 0x04, 0xf6,
            0x2b, 0xc8, 0x4b, 0x06, 0x77, 0xe4, 0x4a, 0x18, 0xb2, 0xfb, 0xe6, 0x2d, 0x64, 0x53,
            0x83, 0xe3, 0x31, 0x9d, 0xab, 0xbf, 0xea, 0x39, 0xc9, 0x8c, 0x17, 0x3b, 0xd0, 0xf5,
            0x94, 0xd0, 0xbc, 0x45, 0x8e, 0x3b, 0xce, 0x9c, 0xe2, 0xbb, 0xbd, 0x98, 0xff, 0x00,
            0x44, 0x69, 0xf0, 0xfa, 0x19, 0x8f, 0x2c, 0x0c, 0x2c, 0xe3, 0xc5, 0x67, 0xc4, 0x2b,
            0xc1, 0x0e, 0x22, 0x56, 0x6d, 0x52, 0xcb, 0x81, 0xf4, 0xef, 0xf7, 0x19, 0xcb, 0x71,
            0x51, 0x3f, 0xff, 0xd9,
        ];
        unsafe {
            // Initialize the CUDA
            let status = cudaSetDevice(0);
            assert_eq!(status, cudaError::cudaSuccess);
            let mut stream: cudaStream_t = std::ptr::null_mut();
            let status = cudaStreamCreate(&mut stream);
            assert_eq!(status, cudaError::cudaSuccess);

            // Create nvJPEG library handle
            let mut handle: nvjpegHandle_t = std::ptr::null_mut();
            let status = nvjpegCreateSimple(&mut handle);
            assert_eq!(status, NVJPEG_STATUS_SUCCESS);

            // Retrieve the width and height information from the JPEG-encoded image
            let mut components = 0;
            let mut subsampling = NVJPEG_CSS_UNKNOWN;
            let mut widths: [i32; 4] = [0, 0, 0, 0];
            let mut heights: [i32; 4] = [0, 0, 0, 0];
            let status = nvjpegGetImageInfo(
                handle,
                LENA_JPG.as_ptr(),
                LENA_JPG.len() as u64,
                &mut components,
                &mut subsampling,
                widths.as_mut_ptr(),
                heights.as_mut_ptr(),
            );
            assert_eq!(status, NVJPEG_STATUS_SUCCESS);
            assert_eq!(components, 3);
            assert_eq!(subsampling, NVJPEG_CSS_420);
            assert_eq!(widths[0], 110);
            assert_eq!(heights[0], 110);

            // Allocate the decoded picture buffer
            let picth0 = widths[0] * 3;
            let image_size = picth0 * heights[0];
            let mut ptr: *mut std::os::raw::c_void = std::ptr::null_mut();
            let status = cudaMallocHost(&mut ptr, image_size.try_into().unwrap());
            assert_eq!(status, cudaError::cudaSuccess);

            // Clear the decoded picture buffer
            cudaMemset(ptr, 0, image_size.try_into().unwrap());

            // Fill image information for decoding
            let pixels = ptr as *mut u8;
            let mut dest = nvjpegImage_t {
                channel: [
                    pixels,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ],
                pitch: [picth0.try_into().unwrap(), 0, 0, 0],
            };

            // Create JPEG decoding temporary state handle
            let mut jpeg_handle: nvjpegJpegState_t = std::ptr::null_mut();
            let status = nvjpegJpegStateCreate(handle, &mut jpeg_handle);
            assert_eq!(status, NVJPEG_STATUS_SUCCESS);

            // Decoding the JPEG encoded bytestream
            let status = nvjpegDecode(
                handle,
                jpeg_handle,
                LENA_JPG.as_ptr(),
                LENA_JPG.len().try_into().unwrap(),
                NVJPEG_OUTPUT_RGBI,
                &mut dest,
                stream,
            );
            assert_eq!(status, NVJPEG_STATUS_SUCCESS);

            // Release
            let status = cudaFreeHost(pixels as *mut std::os::raw::c_void);
            assert_eq!(status, cudaError::cudaSuccess);

            // Destroy JPEG decoding temporary state handle
            let status = nvjpegJpegStateDestroy(jpeg_handle);
            assert_eq!(status, NVJPEG_STATUS_SUCCESS);

            // Destroy nvJPEG library handle
            let status = nvjpegDestroy(handle);
            assert_eq!(status, NVJPEG_STATUS_SUCCESS);

            // Destroy CUDA stream
            let status = cudaStreamDestroy(stream);
            assert_eq!(status, cudaError::cudaSuccess);
        }
    }
}
