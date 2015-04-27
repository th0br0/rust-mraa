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
use mraa::bindings::common::*;
use mraa::bindings::types::*;
use mraa::bindings::gpio::*;

const DEFAULT_IOPIN : i32 = 8;
// XXX - Signal handling is not yet implemented in rust, see rust-lang/rust/issues/11203

pub fn main() {

    let iopin : i32 = match env::args().nth(1) {
        Some(arg) => arg.parse::<i32>().unwrap(),
        _ => {
            println!("Provide an int arg if you want to flash on something other than {}", DEFAULT_IOPIN);
            DEFAULT_IOPIN
        }
    };

    unsafe { mraa_init() };
    let version_slice = unsafe {
        CStr::from_ptr(mraa_get_version())
    };
    let version = str::from_utf8(version_slice.to_bytes()).unwrap();

    println!("MRAA Version: {}\nStarting Blinking on IO {}", version, iopin);

    let gpio : mraa_gpio_context = unsafe { mraa_gpio_init(iopin) };
    if gpio.is_null() {
        panic!("Are you sure the pin({}) you requested is valid on your platform?", iopin);
    }

    println!("Initialised pin{}", iopin);

    match unsafe { mraa_gpio_dir(gpio, MRAA_GPIO_OUT) } {
        r if r != MRAA_SUCCESS => unsafe { mraa_result_print(r); panic!("Could not set GPIO direction"); },
        _ => {}
    };

    loop {
        match unsafe { mraa_gpio_write(gpio, 0) } {
            r if r != MRAA_SUCCESS => unsafe { mraa_result_print(r); panic!("Could not write LOW to GPIO"); },
            _ => {}
        }

        sleep_ms(1000);

        match unsafe { mraa_gpio_write(gpio, 1) } {
            r if r != MRAA_SUCCESS => unsafe { mraa_result_print(r); panic!("Could not write HIGH to GPIO"); },
            _ => {}
        }

        sleep_ms(1000);
    }

    match unsafe { mraa_gpio_close(gpio) } {
        r if r != MRAA_SUCCESS => unsafe { mraa_result_print(r); panic!("Could not close GPIO pin"); },
        _ => {}
    }


}
