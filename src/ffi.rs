#![allow(non_camel_case_types)]

use std::os::raw::{c_int, c_uint, c_char, c_void};
use std::error;
use std::fmt;
use std::error::Error;

pub enum liq_attr {}
pub enum liq_image {}
pub enum liq_result {}
pub enum liq_histogram {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum liq_error {
    LIQ_OK = 0,
    LIQ_QUALITY_TOO_LOW = 99,
    LIQ_VALUE_OUT_OF_RANGE = 100,
    LIQ_OUT_OF_MEMORY,
    LIQ_ABORTED,
    LIQ_BITMAP_NOT_AVAILABLE,
    LIQ_BUFFER_TOO_SMALL,
    LIQ_INVALID_POINTER,
    LIQ_UNSUPPORTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum liq_ownership {
    LIQ_OWN_ROWS = 4,
    LIQ_OWN_PIXELS = 8,
}

#[repr(C)]
pub struct liq_palette {
    pub count: c_int,
    pub entries: [super::Color; 256],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct liq_histogram_entry {
    pub color: super::Color,
    pub count: c_uint,
}

impl error::Error for liq_error {
    fn description(&self) -> &str {
        match *self {
            liq_error::LIQ_OK => "OK",
            liq_error::LIQ_QUALITY_TOO_LOW => "LIQ_QUALITY_TOO_LOW",
            liq_error::LIQ_VALUE_OUT_OF_RANGE => "VALUE_OUT_OF_RANGE",
            liq_error::LIQ_OUT_OF_MEMORY => "OUT_OF_MEMORY",
            liq_error::LIQ_ABORTED => "LIQ_ABORTED",
            liq_error::LIQ_BITMAP_NOT_AVAILABLE => "BITMAP_NOT_AVAILABLE",
            liq_error::LIQ_BUFFER_TOO_SMALL => "BUFFER_TOO_SMALL",
            liq_error::LIQ_INVALID_POINTER => "INVALID_POINTER",
            liq_error::LIQ_UNSUPPORTED => "LIQ_UNSUPPORTED",
        }
    }
}

impl fmt::Display for liq_error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[link(name="imagequant", kind="static")]
extern "C" {

    pub fn liq_attr_create() -> *mut liq_attr;
    pub fn liq_attr_copy(orig: &liq_attr) -> *mut liq_attr;
    pub fn liq_attr_destroy(attr: &mut liq_attr);

    pub fn liq_set_max_colors(attr: &mut liq_attr, colors: c_int) -> liq_error;
    pub fn liq_get_max_colors(attr: &liq_attr) -> c_int;
    pub fn liq_set_speed(attr: &mut liq_attr, speed: c_int) -> liq_error;
    pub fn liq_get_speed(attr: &liq_attr) -> c_int;
    pub fn liq_set_min_posterization(attr: &mut liq_attr, bits: c_int) -> liq_error;
    pub fn liq_get_min_posterization(attr: &liq_attr) -> c_int;
    pub fn liq_set_quality(attr: &mut liq_attr, minimum: c_int, maximum: c_int) -> liq_error;
    pub fn liq_get_min_quality(attr: &liq_attr) -> c_int;
    pub fn liq_get_max_quality(attr: &liq_attr) -> c_int;
    pub fn liq_set_last_index_transparent(attr: &mut liq_attr, is_last: c_int);

    pub fn liq_image_create_rgba_rows(attr: &liq_attr, rows: *const *const u8, width: c_int, height: c_int, gamma: f64) -> *mut liq_image;
    pub fn liq_image_create_rgba(attr: &liq_attr, bitmap: *const u8, width: c_int, height: c_int, gamma: f64) -> *mut liq_image;

    pub fn liq_image_get_width(img: &liq_image) -> c_int;
    pub fn liq_image_get_height(img: &liq_image) -> c_int;
    pub fn liq_image_destroy(img: &mut liq_image);

    pub fn liq_histogram_create(attr: &liq_attr) -> *mut liq_histogram;
    pub fn liq_histogram_add_image(hist: &mut liq_histogram, attr: &liq_attr, image: &liq_image) -> liq_error;
    pub fn liq_histogram_add_colors(hist: &mut liq_histogram, attr: &liq_attr, entries: *const liq_histogram_entry, num_entries: c_int, gamma: f64) -> liq_error;
    pub fn liq_histogram_destroy(hist: &mut liq_histogram);

    pub fn liq_quantize_image(options: &liq_attr, input_image: &liq_image) -> *mut liq_result;
    pub fn liq_histogram_quantize(input_hist: &liq_histogram, options: &liq_attr, result_output: &mut *mut liq_result) -> liq_error;
    pub fn liq_image_quantize(input_image: &liq_image, options: &liq_attr, result_output: &mut *mut liq_result) -> liq_error;

    pub fn liq_set_dithering_level(res: &mut liq_result, dither_level: f32) -> liq_error;
    pub fn liq_set_output_gamma(res: &mut liq_result, gamma: f64) -> liq_error;
    pub fn liq_get_output_gamma(result: &liq_result) -> f64;


    pub fn liq_write_remapped_image(result: &mut liq_result, input_image: &liq_image, buffer: *mut u8, buffer_size: usize) -> liq_error;
    pub fn liq_write_remapped_image_rows(result: &mut liq_result, input_image: &liq_image, row_pointers: *const *mut u8) -> liq_error;

    pub fn liq_get_palette(result: &mut liq_result) -> &liq_palette;
    pub fn liq_get_quantization_error(result: &liq_result) -> f64;
    pub fn liq_get_quantization_quality(result: &liq_result) -> c_int;

    pub fn liq_result_destroy(res: &mut liq_result);
    pub fn liq_get_remapping_error(result: &liq_result) -> f64;
    pub fn liq_get_remapping_quality(result: &liq_result) -> c_int;
    pub fn liq_version() -> c_int;
}
