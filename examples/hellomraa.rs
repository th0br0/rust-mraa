/*
 * Author: Andreas C. Osowski <andreas@osowski.de>
 * Copyright (c) 2015 Andreas C. Osowski.
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
 * LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 * OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
 * WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

extern crate mraa;

use std::ffi::CStr;
use std::str;
use mraa::bindings::common::*;

pub fn main() {
    let board_name = str::from_utf8(unsafe { CStr::from_ptr(mraa_get_platform_name()) }.to_bytes()).unwrap();
    let version = str::from_utf8(unsafe { CStr::from_ptr(mraa_get_version())}.to_bytes()).unwrap();

    println!("hello mraa\n Version: {}\n Running on: {}", version, board_name);

    unsafe { mraa_deinit() };
}
