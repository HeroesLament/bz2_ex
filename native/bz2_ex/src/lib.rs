//! Rustler NIF bindings for bzip2 compression using libbz2-rs-sys

use rustler::{Atom, Binary, Env, NewBinary, NifResult, ResourceArc};
use std::sync::Mutex;

mod atoms {
    rustler::atoms! {
        ok,
        error,
        config_error,
        param_error,
        mem_error,
        data_error,
        data_error_magic,
        io_error,
        unexpected_eof,
        outbuff_full,
        sequence_error,
        unknown_error,
        ready,
        finished,
    }
}

fn bz_error_to_atom(code: i32) -> Atom {
    match code {
        libbz2_rs_sys::BZ_CONFIG_ERROR => atoms::config_error(),
        libbz2_rs_sys::BZ_PARAM_ERROR => atoms::param_error(),
        libbz2_rs_sys::BZ_MEM_ERROR => atoms::mem_error(),
        libbz2_rs_sys::BZ_DATA_ERROR => atoms::data_error(),
        libbz2_rs_sys::BZ_DATA_ERROR_MAGIC => atoms::data_error_magic(),
        libbz2_rs_sys::BZ_IO_ERROR => atoms::io_error(),
        libbz2_rs_sys::BZ_UNEXPECTED_EOF => atoms::unexpected_eof(),
        libbz2_rs_sys::BZ_OUTBUFF_FULL => atoms::outbuff_full(),
        libbz2_rs_sys::BZ_SEQUENCE_ERROR => atoms::sequence_error(),
        _ => atoms::unknown_error(),
    }
}

// =============================================================================
// One-shot API
// =============================================================================

// Apple (all architectures) - uses i8
#[cfg(target_vendor = "apple")]
#[rustler::nif(schedule = "DirtyCpu")]
fn compress<'a>(
    env: Env<'a>,
    input: Binary<'a>,
    block_size: i32,
    work_factor: i32,
) -> NifResult<(Atom, Binary<'a>)> {
    let input_slice = input.as_slice();
    let max_output_size = input_slice.len() + (input_slice.len() / 100) + 600;
    let mut output_vec = vec![0u8; max_output_size];
    let mut dest_len = max_output_size as u32;

    let result = unsafe {
        libbz2_rs_sys::BZ2_bzBuffToBuffCompress(
            output_vec.as_mut_ptr() as *mut i8,
            &mut dest_len,
            input_slice.as_ptr() as *mut i8,
            input_slice.len() as u32,
            block_size,
            0,
            work_factor,
        )
    };

    if result == libbz2_rs_sys::BZ_OK {
        output_vec.truncate(dest_len as usize);
        let mut binary = NewBinary::new(env, dest_len as usize);
        binary.as_mut_slice().copy_from_slice(&output_vec);
        Ok((atoms::ok(), binary.into()))
    } else {
        let binary = NewBinary::new(env, 0);
        Ok((bz_error_to_atom(result), binary.into()))
    }
}

// aarch64 Linux - uses u8
#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
#[rustler::nif(schedule = "DirtyCpu")]
fn compress<'a>(
    env: Env<'a>,
    input: Binary<'a>,
    block_size: i32,
    work_factor: i32,
) -> NifResult<(Atom, Binary<'a>)> {
    let input_slice = input.as_slice();
    let max_output_size = input_slice.len() + (input_slice.len() / 100) + 600;
    let mut output_vec = vec![0u8; max_output_size];
    let mut dest_len = max_output_size as u32;

    let result = unsafe {
        libbz2_rs_sys::BZ2_bzBuffToBuffCompress(
            output_vec.as_mut_ptr(),
            &mut dest_len,
            input_slice.as_ptr() as *mut u8,
            input_slice.len() as u32,
            block_size,
            0,
            work_factor,
        )
    };

    if result == libbz2_rs_sys::BZ_OK {
        output_vec.truncate(dest_len as usize);
        let mut binary = NewBinary::new(env, dest_len as usize);
        binary.as_mut_slice().copy_from_slice(&output_vec);
        Ok((atoms::ok(), binary.into()))
    } else {
        let binary = NewBinary::new(env, 0);
        Ok((bz_error_to_atom(result), binary.into()))
    }
}

// x86_64 Linux - uses i8
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
#[rustler::nif(schedule = "DirtyCpu")]
fn compress<'a>(
    env: Env<'a>,
    input: Binary<'a>,
    block_size: i32,
    work_factor: i32,
) -> NifResult<(Atom, Binary<'a>)> {
    let input_slice = input.as_slice();
    let max_output_size = input_slice.len() + (input_slice.len() / 100) + 600;
    let mut output_vec = vec![0u8; max_output_size];
    let mut dest_len = max_output_size as u32;

    let result = unsafe {
        libbz2_rs_sys::BZ2_bzBuffToBuffCompress(
            output_vec.as_mut_ptr() as *mut i8,
            &mut dest_len,
            input_slice.as_ptr() as *mut i8,
            input_slice.len() as u32,
            block_size,
            0,
            work_factor,
        )
    };

    if result == libbz2_rs_sys::BZ_OK {
        output_vec.truncate(dest_len as usize);
        let mut binary = NewBinary::new(env, dest_len as usize);
        binary.as_mut_slice().copy_from_slice(&output_vec);
        Ok((atoms::ok(), binary.into()))
    } else {
        let binary = NewBinary::new(env, 0);
        Ok((bz_error_to_atom(result), binary.into()))
    }
}

// Apple (all architectures) - uses i8
#[cfg(target_vendor = "apple")]
#[rustler::nif(schedule = "DirtyCpu")]
fn decompress<'a>(
    env: Env<'a>,
    input: Binary<'a>,
    small: bool,
) -> NifResult<(Atom, Binary<'a>)> {
    let input_slice = input.as_slice();
    let mut current_size = input_slice.len() * 4;
    let mut output_vec = vec![0u8; current_size];

    loop {
        let mut dest_len = current_size as u32;

        let result = unsafe {
            libbz2_rs_sys::BZ2_bzBuffToBuffDecompress(
                output_vec.as_mut_ptr() as *mut i8,
                &mut dest_len,
                input_slice.as_ptr() as *mut i8,
                input_slice.len() as u32,
                if small { 1 } else { 0 },
                0,
            )
        };

        match result {
            libbz2_rs_sys::BZ_OK => {
                output_vec.truncate(dest_len as usize);
                let mut binary = NewBinary::new(env, dest_len as usize);
                binary.as_mut_slice().copy_from_slice(&output_vec);
                return Ok((atoms::ok(), binary.into()));
            }
            libbz2_rs_sys::BZ_OUTBUFF_FULL => {
                current_size *= 2;
                if current_size > 1024 * 1024 * 1024 {
                    let binary = NewBinary::new(env, 0);
                    return Ok((atoms::outbuff_full(), binary.into()));
                }
                output_vec.resize(current_size, 0u8);
            }
            _ => {
                let binary = NewBinary::new(env, 0);
                return Ok((bz_error_to_atom(result), binary.into()));
            }
        }
    }
}

// aarch64 Linux - uses u8
#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
#[rustler::nif(schedule = "DirtyCpu")]
fn decompress<'a>(
    env: Env<'a>,
    input: Binary<'a>,
    small: bool,
) -> NifResult<(Atom, Binary<'a>)> {
    let input_slice = input.as_slice();
    let mut current_size = input_slice.len() * 4;
    let mut output_vec = vec![0u8; current_size];

    loop {
        let mut dest_len = current_size as u32;

        let result = unsafe {
            libbz2_rs_sys::BZ2_bzBuffToBuffDecompress(
                output_vec.as_mut_ptr(),
                &mut dest_len,
                input_slice.as_ptr() as *mut u8,
                input_slice.len() as u32,
                if small { 1 } else { 0 },
                0,
            )
        };

        match result {
            libbz2_rs_sys::BZ_OK => {
                output_vec.truncate(dest_len as usize);
                let mut binary = NewBinary::new(env, dest_len as usize);
                binary.as_mut_slice().copy_from_slice(&output_vec);
                return Ok((atoms::ok(), binary.into()));
            }
            libbz2_rs_sys::BZ_OUTBUFF_FULL => {
                current_size *= 2;
                if current_size > 1024 * 1024 * 1024 {
                    let binary = NewBinary::new(env, 0);
                    return Ok((atoms::outbuff_full(), binary.into()));
                }
                output_vec.resize(current_size, 0u8);
            }
            _ => {
                let binary = NewBinary::new(env, 0);
                return Ok((bz_error_to_atom(result), binary.into()));
            }
        }
    }
}

// x86_64 Linux - uses i8
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
#[rustler::nif(schedule = "DirtyCpu")]
fn decompress<'a>(
    env: Env<'a>,
    input: Binary<'a>,
    small: bool,
) -> NifResult<(Atom, Binary<'a>)> {
    let input_slice = input.as_slice();
    let mut current_size = input_slice.len() * 4;
    let mut output_vec = vec![0u8; current_size];

    loop {
        let mut dest_len = current_size as u32;

        let result = unsafe {
            libbz2_rs_sys::BZ2_bzBuffToBuffDecompress(
                output_vec.as_mut_ptr() as *mut i8,
                &mut dest_len,
                input_slice.as_ptr() as *mut i8,
                input_slice.len() as u32,
                if small { 1 } else { 0 },
                0,
            )
        };

        match result {
            libbz2_rs_sys::BZ_OK => {
                output_vec.truncate(dest_len as usize);
                let mut binary = NewBinary::new(env, dest_len as usize);
                binary.as_mut_slice().copy_from_slice(&output_vec);
                return Ok((atoms::ok(), binary.into()));
            }
            libbz2_rs_sys::BZ_OUTBUFF_FULL => {
                current_size *= 2;
                if current_size > 1024 * 1024 * 1024 {
                    let binary = NewBinary::new(env, 0);
                    return Ok((atoms::outbuff_full(), binary.into()));
                }
                output_vec.resize(current_size, 0u8);
            }
            _ => {
                let binary = NewBinary::new(env, 0);
                return Ok((bz_error_to_atom(result), binary.into()));
            }
        }
    }
}

// =============================================================================
// Streaming API - Resources
// =============================================================================

struct CompressStreamInner {
    stream: Box<libbz2_rs_sys::bz_stream>,
    initialized: bool,
}

unsafe impl Send for CompressStreamInner {}
unsafe impl Sync for CompressStreamInner {}

pub struct CompressStream {
    inner: Mutex<CompressStreamInner>,
}

#[rustler::resource_impl]
impl rustler::Resource for CompressStream {}

impl CompressStream {
    fn new(block_size: i32, work_factor: i32) -> Result<Self, i32> {
        let mut stream = Box::new(libbz2_rs_sys::bz_stream {
            next_in: std::ptr::null_mut(),
            avail_in: 0,
            total_in_lo32: 0,
            total_in_hi32: 0,
            next_out: std::ptr::null_mut(),
            avail_out: 0,
            total_out_lo32: 0,
            total_out_hi32: 0,
            state: std::ptr::null_mut(),
            bzalloc: None,
            bzfree: None,
            opaque: std::ptr::null_mut(),
        });

        let result = unsafe {
            libbz2_rs_sys::BZ2_bzCompressInit(&mut *stream, block_size, 0, work_factor)
        };

        if result == libbz2_rs_sys::BZ_OK {
            Ok(Self {
                inner: Mutex::new(CompressStreamInner {
                    stream,
                    initialized: true,
                }),
            })
        } else {
            Err(result)
        }
    }
}

impl Drop for CompressStream {
    fn drop(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        if inner.initialized {
            unsafe {
                libbz2_rs_sys::BZ2_bzCompressEnd(&mut *inner.stream);
            }
            inner.initialized = false;
        }
    }
}

struct DecompressStreamInner {
    stream: Box<libbz2_rs_sys::bz_stream>,
    initialized: bool,
}

unsafe impl Send for DecompressStreamInner {}
unsafe impl Sync for DecompressStreamInner {}

pub struct DecompressStream {
    inner: Mutex<DecompressStreamInner>,
}

#[rustler::resource_impl]
impl rustler::Resource for DecompressStream {}

impl DecompressStream {
    fn new(small: bool) -> Result<Self, i32> {
        let mut stream = Box::new(libbz2_rs_sys::bz_stream {
            next_in: std::ptr::null_mut(),
            avail_in: 0,
            total_in_lo32: 0,
            total_in_hi32: 0,
            next_out: std::ptr::null_mut(),
            avail_out: 0,
            total_out_lo32: 0,
            total_out_hi32: 0,
            state: std::ptr::null_mut(),
            bzalloc: None,
            bzfree: None,
            opaque: std::ptr::null_mut(),
        });

        let result = unsafe {
            libbz2_rs_sys::BZ2_bzDecompressInit(&mut *stream, 0, if small { 1 } else { 0 })
        };

        if result == libbz2_rs_sys::BZ_OK {
            Ok(Self {
                inner: Mutex::new(DecompressStreamInner {
                    stream,
                    initialized: true,
                }),
            })
        } else {
            Err(result)
        }
    }
}

impl Drop for DecompressStream {
    fn drop(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        if inner.initialized {
            unsafe {
                libbz2_rs_sys::BZ2_bzDecompressEnd(&mut *inner.stream);
            }
            inner.initialized = false;
        }
    }
}

// =============================================================================
// Streaming NIFs
// =============================================================================

#[rustler::nif]
fn compress_stream_init(
    block_size: i32,
    work_factor: i32,
) -> NifResult<(Atom, ResourceArc<CompressStream>)> {
    match CompressStream::new(block_size, work_factor) {
        Ok(stream) => Ok((atoms::ok(), ResourceArc::new(stream))),
        Err(code) => Err(rustler::Error::Term(Box::new(bz_error_to_atom(code)))),
    }
}

// Apple (all architectures) - uses i8
#[cfg(target_vendor = "apple")]
#[rustler::nif(schedule = "DirtyCpu")]
fn compress_stream_deflate<'a>(
    env: Env<'a>,
    stream: ResourceArc<CompressStream>,
    input: Binary<'a>,
) -> NifResult<(Atom, Binary<'a>)> {
    let mut inner = stream.inner.lock().unwrap();
    if !inner.initialized {
        return Err(rustler::Error::Term(Box::new(atoms::sequence_error())));
    }

    let input_slice = input.as_slice();
    let mut output_vec = vec![0u8; input_slice.len() + 600];

    inner.stream.next_in = input_slice.as_ptr() as *const i8;
    inner.stream.avail_in = input_slice.len() as u32;
    inner.stream.next_out = output_vec.as_mut_ptr() as *mut i8;
    inner.stream.avail_out = output_vec.len() as u32;

    let result = unsafe { libbz2_rs_sys::BZ2_bzCompress(&mut *inner.stream, libbz2_rs_sys::BZ_RUN) };

    if result == libbz2_rs_sys::BZ_RUN_OK {
        let bytes_written = output_vec.len() - inner.stream.avail_out as usize;
        output_vec.truncate(bytes_written);

        let mut binary = NewBinary::new(env, bytes_written);
        binary.as_mut_slice().copy_from_slice(&output_vec);
        Ok((atoms::ok(), binary.into()))
    } else {
        let binary = NewBinary::new(env, 0);
        Ok((bz_error_to_atom(result), binary.into()))
    }
}

// aarch64 Linux - uses u8
#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
#[rustler::nif(schedule = "DirtyCpu")]
fn compress_stream_deflate<'a>(
    env: Env<'a>,
    stream: ResourceArc<CompressStream>,
    input: Binary<'a>,
) -> NifResult<(Atom, Binary<'a>)> {
    let mut inner = stream.inner.lock().unwrap();
    if !inner.initialized {
        return Err(rustler::Error::Term(Box::new(atoms::sequence_error())));
    }

    let input_slice = input.as_slice();
    let mut output_vec = vec![0u8; input_slice.len() + 600];

    inner.stream.next_in = input_slice.as_ptr();
    inner.stream.avail_in = input_slice.len() as u32;
    inner.stream.next_out = output_vec.as_mut_ptr();
    inner.stream.avail_out = output_vec.len() as u32;

    let result = unsafe { libbz2_rs_sys::BZ2_bzCompress(&mut *inner.stream, libbz2_rs_sys::BZ_RUN) };

    if result == libbz2_rs_sys::BZ_RUN_OK {
        let bytes_written = output_vec.len() - inner.stream.avail_out as usize;
        output_vec.truncate(bytes_written);

        let mut binary = NewBinary::new(env, bytes_written);
        binary.as_mut_slice().copy_from_slice(&output_vec);
        Ok((atoms::ok(), binary.into()))
    } else {
        let binary = NewBinary::new(env, 0);
        Ok((bz_error_to_atom(result), binary.into()))
    }
}

// x86_64 Linux - uses i8
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
#[rustler::nif(schedule = "DirtyCpu")]
fn compress_stream_deflate<'a>(
    env: Env<'a>,
    stream: ResourceArc<CompressStream>,
    input: Binary<'a>,
) -> NifResult<(Atom, Binary<'a>)> {
    let mut inner = stream.inner.lock().unwrap();
    if !inner.initialized {
        return Err(rustler::Error::Term(Box::new(atoms::sequence_error())));
    }

    let input_slice = input.as_slice();
    let mut output_vec = vec![0u8; input_slice.len() + 600];

    inner.stream.next_in = input_slice.as_ptr() as *const i8;
    inner.stream.avail_in = input_slice.len() as u32;
    inner.stream.next_out = output_vec.as_mut_ptr() as *mut i8;
    inner.stream.avail_out = output_vec.len() as u32;

    let result = unsafe { libbz2_rs_sys::BZ2_bzCompress(&mut *inner.stream, libbz2_rs_sys::BZ_RUN) };

    if result == libbz2_rs_sys::BZ_RUN_OK {
        let bytes_written = output_vec.len() - inner.stream.avail_out as usize;
        output_vec.truncate(bytes_written);

        let mut binary = NewBinary::new(env, bytes_written);
        binary.as_mut_slice().copy_from_slice(&output_vec);
        Ok((atoms::ok(), binary.into()))
    } else {
        let binary = NewBinary::new(env, 0);
        Ok((bz_error_to_atom(result), binary.into()))
    }
}

// Apple (all architectures) - uses i8
#[cfg(target_vendor = "apple")]
#[rustler::nif(schedule = "DirtyCpu")]
fn compress_stream_finish<'a>(
    env: Env<'a>,
    stream: ResourceArc<CompressStream>,
) -> NifResult<(Atom, Binary<'a>)> {
    let mut inner = stream.inner.lock().unwrap();
    if !inner.initialized {
        return Err(rustler::Error::Term(Box::new(atoms::sequence_error())));
    }

    let mut output_chunks: Vec<u8> = Vec::new();
    let mut buffer = vec![0u8; 4096];

    loop {
        inner.stream.next_in = std::ptr::null();
        inner.stream.avail_in = 0;
        inner.stream.next_out = buffer.as_mut_ptr() as *mut i8;
        inner.stream.avail_out = buffer.len() as u32;

        let result =
            unsafe { libbz2_rs_sys::BZ2_bzCompress(&mut *inner.stream, libbz2_rs_sys::BZ_FINISH) };

        let bytes_written = buffer.len() - inner.stream.avail_out as usize;
        output_chunks.extend_from_slice(&buffer[..bytes_written]);

        match result {
            libbz2_rs_sys::BZ_STREAM_END => {
                unsafe {
                    libbz2_rs_sys::BZ2_bzCompressEnd(&mut *inner.stream);
                }
                inner.initialized = false;

                let mut binary = NewBinary::new(env, output_chunks.len());
                binary.as_mut_slice().copy_from_slice(&output_chunks);
                return Ok((atoms::ok(), binary.into()));
            }
            libbz2_rs_sys::BZ_FINISH_OK => {}
            _ => {
                let binary = NewBinary::new(env, 0);
                return Ok((bz_error_to_atom(result), binary.into()));
            }
        }
    }
}

// aarch64 Linux - uses u8
#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
#[rustler::nif(schedule = "DirtyCpu")]
fn compress_stream_finish<'a>(
    env: Env<'a>,
    stream: ResourceArc<CompressStream>,
) -> NifResult<(Atom, Binary<'a>)> {
    let mut inner = stream.inner.lock().unwrap();
    if !inner.initialized {
        return Err(rustler::Error::Term(Box::new(atoms::sequence_error())));
    }

    let mut output_chunks: Vec<u8> = Vec::new();
    let mut buffer = vec![0u8; 4096];

    loop {
        inner.stream.next_in = std::ptr::null();
        inner.stream.avail_in = 0;
        inner.stream.next_out = buffer.as_mut_ptr();
        inner.stream.avail_out = buffer.len() as u32;

        let result =
            unsafe { libbz2_rs_sys::BZ2_bzCompress(&mut *inner.stream, libbz2_rs_sys::BZ_FINISH) };

        let bytes_written = buffer.len() - inner.stream.avail_out as usize;
        output_chunks.extend_from_slice(&buffer[..bytes_written]);

        match result {
            libbz2_rs_sys::BZ_STREAM_END => {
                unsafe {
                    libbz2_rs_sys::BZ2_bzCompressEnd(&mut *inner.stream);
                }
                inner.initialized = false;

                let mut binary = NewBinary::new(env, output_chunks.len());
                binary.as_mut_slice().copy_from_slice(&output_chunks);
                return Ok((atoms::ok(), binary.into()));
            }
            libbz2_rs_sys::BZ_FINISH_OK => {}
            _ => {
                let binary = NewBinary::new(env, 0);
                return Ok((bz_error_to_atom(result), binary.into()));
            }
        }
    }
}

// x86_64 Linux - uses i8
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
#[rustler::nif(schedule = "DirtyCpu")]
fn compress_stream_finish<'a>(
    env: Env<'a>,
    stream: ResourceArc<CompressStream>,
) -> NifResult<(Atom, Binary<'a>)> {
    let mut inner = stream.inner.lock().unwrap();
    if !inner.initialized {
        return Err(rustler::Error::Term(Box::new(atoms::sequence_error())));
    }

    let mut output_chunks: Vec<u8> = Vec::new();
    let mut buffer = vec![0u8; 4096];

    loop {
        inner.stream.next_in = std::ptr::null();
        inner.stream.avail_in = 0;
        inner.stream.next_out = buffer.as_mut_ptr() as *mut i8;
        inner.stream.avail_out = buffer.len() as u32;

        let result =
            unsafe { libbz2_rs_sys::BZ2_bzCompress(&mut *inner.stream, libbz2_rs_sys::BZ_FINISH) };

        let bytes_written = buffer.len() - inner.stream.avail_out as usize;
        output_chunks.extend_from_slice(&buffer[..bytes_written]);

        match result {
            libbz2_rs_sys::BZ_STREAM_END => {
                unsafe {
                    libbz2_rs_sys::BZ2_bzCompressEnd(&mut *inner.stream);
                }
                inner.initialized = false;

                let mut binary = NewBinary::new(env, output_chunks.len());
                binary.as_mut_slice().copy_from_slice(&output_chunks);
                return Ok((atoms::ok(), binary.into()));
            }
            libbz2_rs_sys::BZ_FINISH_OK => {}
            _ => {
                let binary = NewBinary::new(env, 0);
                return Ok((bz_error_to_atom(result), binary.into()));
            }
        }
    }
}

#[rustler::nif]
fn decompress_stream_init(small: bool) -> NifResult<(Atom, ResourceArc<DecompressStream>)> {
    match DecompressStream::new(small) {
        Ok(stream) => Ok((atoms::ok(), ResourceArc::new(stream))),
        Err(code) => Err(rustler::Error::Term(Box::new(bz_error_to_atom(code)))),
    }
}

// Apple (all architectures) - uses i8
#[cfg(target_vendor = "apple")]
#[rustler::nif(schedule = "DirtyCpu")]
fn decompress_stream_inflate<'a>(
    env: Env<'a>,
    stream: ResourceArc<DecompressStream>,
    input: Binary<'a>,
) -> NifResult<(Atom, Binary<'a>, Atom)> {
    let mut inner = stream.inner.lock().unwrap();
    if !inner.initialized {
        return Err(rustler::Error::Term(Box::new(atoms::sequence_error())));
    }

    let input_slice = input.as_slice();
    let mut output_chunks: Vec<u8> = Vec::new();
    let mut buffer = vec![0u8; input_slice.len() * 4 + 4096];

    inner.stream.next_in = input_slice.as_ptr() as *const i8;
    inner.stream.avail_in = input_slice.len() as u32;

    loop {
        inner.stream.next_out = buffer.as_mut_ptr() as *mut i8;
        inner.stream.avail_out = buffer.len() as u32;

        let result = unsafe { libbz2_rs_sys::BZ2_bzDecompress(&mut *inner.stream) };

        let bytes_written = buffer.len() - inner.stream.avail_out as usize;
        output_chunks.extend_from_slice(&buffer[..bytes_written]);

        match result {
            libbz2_rs_sys::BZ_OK => {
                if inner.stream.avail_in == 0 {
                    let mut binary = NewBinary::new(env, output_chunks.len());
                    binary.as_mut_slice().copy_from_slice(&output_chunks);
                    return Ok((atoms::ok(), binary.into(), atoms::ready()));
                }
            }
            libbz2_rs_sys::BZ_STREAM_END => {
                unsafe {
                    libbz2_rs_sys::BZ2_bzDecompressEnd(&mut *inner.stream);
                }
                inner.initialized = false;

                let mut binary = NewBinary::new(env, output_chunks.len());
                binary.as_mut_slice().copy_from_slice(&output_chunks);
                return Ok((atoms::ok(), binary.into(), atoms::finished()));
            }
            _ => {
                let binary = NewBinary::new(env, 0);
                return Ok((bz_error_to_atom(result), binary.into(), atoms::error()));
            }
        }
    }
}

// aarch64 Linux - uses u8
#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
#[rustler::nif(schedule = "DirtyCpu")]
fn decompress_stream_inflate<'a>(
    env: Env<'a>,
    stream: ResourceArc<DecompressStream>,
    input: Binary<'a>,
) -> NifResult<(Atom, Binary<'a>, Atom)> {
    let mut inner = stream.inner.lock().unwrap();
    if !inner.initialized {
        return Err(rustler::Error::Term(Box::new(atoms::sequence_error())));
    }

    let input_slice = input.as_slice();
    let mut output_chunks: Vec<u8> = Vec::new();
    let mut buffer = vec![0u8; input_slice.len() * 4 + 4096];

    inner.stream.next_in = input_slice.as_ptr();
    inner.stream.avail_in = input_slice.len() as u32;

    loop {
        inner.stream.next_out = buffer.as_mut_ptr();
        inner.stream.avail_out = buffer.len() as u32;

        let result = unsafe { libbz2_rs_sys::BZ2_bzDecompress(&mut *inner.stream) };

        let bytes_written = buffer.len() - inner.stream.avail_out as usize;
        output_chunks.extend_from_slice(&buffer[..bytes_written]);

        match result {
            libbz2_rs_sys::BZ_OK => {
                if inner.stream.avail_in == 0 {
                    let mut binary = NewBinary::new(env, output_chunks.len());
                    binary.as_mut_slice().copy_from_slice(&output_chunks);
                    return Ok((atoms::ok(), binary.into(), atoms::ready()));
                }
            }
            libbz2_rs_sys::BZ_STREAM_END => {
                unsafe {
                    libbz2_rs_sys::BZ2_bzDecompressEnd(&mut *inner.stream);
                }
                inner.initialized = false;

                let mut binary = NewBinary::new(env, output_chunks.len());
                binary.as_mut_slice().copy_from_slice(&output_chunks);
                return Ok((atoms::ok(), binary.into(), atoms::finished()));
            }
            _ => {
                let binary = NewBinary::new(env, 0);
                return Ok((bz_error_to_atom(result), binary.into(), atoms::error()));
            }
        }
    }
}

// x86_64 Linux - uses i8
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
#[rustler::nif(schedule = "DirtyCpu")]
fn decompress_stream_inflate<'a>(
    env: Env<'a>,
    stream: ResourceArc<DecompressStream>,
    input: Binary<'a>,
) -> NifResult<(Atom, Binary<'a>, Atom)> {
    let mut inner = stream.inner.lock().unwrap();
    if !inner.initialized {
        return Err(rustler::Error::Term(Box::new(atoms::sequence_error())));
    }

    let input_slice = input.as_slice();
    let mut output_chunks: Vec<u8> = Vec::new();
    let mut buffer = vec![0u8; input_slice.len() * 4 + 4096];

    inner.stream.next_in = input_slice.as_ptr() as *const i8;
    inner.stream.avail_in = input_slice.len() as u32;

    loop {
        inner.stream.next_out = buffer.as_mut_ptr() as *mut i8;
        inner.stream.avail_out = buffer.len() as u32;

        let result = unsafe { libbz2_rs_sys::BZ2_bzDecompress(&mut *inner.stream) };

        let bytes_written = buffer.len() - inner.stream.avail_out as usize;
        output_chunks.extend_from_slice(&buffer[..bytes_written]);

        match result {
            libbz2_rs_sys::BZ_OK => {
                if inner.stream.avail_in == 0 {
                    let mut binary = NewBinary::new(env, output_chunks.len());
                    binary.as_mut_slice().copy_from_slice(&output_chunks);
                    return Ok((atoms::ok(), binary.into(), atoms::ready()));
                }
            }
            libbz2_rs_sys::BZ_STREAM_END => {
                unsafe {
                    libbz2_rs_sys::BZ2_bzDecompressEnd(&mut *inner.stream);
                }
                inner.initialized = false;

                let mut binary = NewBinary::new(env, output_chunks.len());
                binary.as_mut_slice().copy_from_slice(&output_chunks);
                return Ok((atoms::ok(), binary.into(), atoms::finished()));
            }
            _ => {
                let binary = NewBinary::new(env, 0);
                return Ok((bz_error_to_atom(result), binary.into(), atoms::error()));
            }
        }
    }
}

// =============================================================================
// NIF Registration
// =============================================================================

rustler::init!("Elixir.Bz2Ex.Native");