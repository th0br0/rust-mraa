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

use mraa::bindings::*;
use mraa::bindings::aio::*;

pub fn main() {
    let adc_a0: mraa_aio_context = unsafe { mraa_aio_init(0) };
    if adc_a0.is_null() {
        panic!("Failed to initialise aio context.");
    }

    loop {
        let adc_value : u32 = unsafe { mraa_aio_read(adc_a0) };
        let adc_value_float : f32 = unsafe { mraa_aio_read_float(adc_a0) };

        println!("ADC A0 read {}", adc_value);
        println!("ADC A0 read float - {:.5}", adc_value_float);
    }

    unsafe { mraa_aio_close(adc_a0) };
}
