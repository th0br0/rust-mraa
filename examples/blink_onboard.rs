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

use std::thread::sleep_ms;
use std::ffi::CStr;
use std::{env, str};
use std::ptr;
use mraa::bindings::common::*;
use mraa::bindings::types::*;
use mraa::bindings::gpio::*;

const DEFAULT_IOPIN : i32 = 8;
// XXX - Signal handling is not yet implemented in rust, see rust-lang/rust/issues/11203

pub fn main() {

    let gpio = unsafe { match mraa_get_platform_type() {
        MRAA_INTEL_GALILEO_GEN1 => mraa_gpio_init_raw(3),
        MRAA_INTEL_MINNOWBOARD_MAX => mraa_gpio_init(21),
        _ => mraa_gpio_init(13)
    }};
    let mut gpio_in : mraa_gpio_context = ptr::null_mut();

    let board_name = str::from_utf8(unsafe { CStr::from_ptr(mraa_get_platform_name()) }.to_bytes()).unwrap();
    let version = str::from_utf8(unsafe { CStr::from_ptr(mraa_get_version())}.to_bytes()).unwrap();

    println!("Welcome to libmraa\n Version: {}\n Running on: {}", version, board_name);

    if gpio.is_null() {
        panic!("Could not initialize gpio");
    }

    unsafe {
        if MRAA_INTEL_MINNOWBOARD_MAX == mraa_get_platform_type() {
            gpio_in = mraa_gpio_init(14);
            if ! gpio_in.is_null() {
                mraa_gpio_dir(gpio_in, MRAA_GPIO_IN);
                println!("Press and hold S1 to stop, Press SW1 to shutdown!");
            }
        }

        mraa_gpio_dir(gpio, MRAA_GPIO_OUT);
    }

    let mut ledstate = false;
    loop {
        if (!(gpio_in.is_null()) && unsafe {mraa_gpio_read(gpio_in) != 0}) {break;}

        ledstate = unsafe { match ledstate {
            true => {mraa_gpio_write(gpio, 0); false},
            false => {mraa_gpio_write(gpio, 1); true}
        }};
        sleep_ms(1000);
    }


}
