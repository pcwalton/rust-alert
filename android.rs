// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use AlertMethods;

/// An alert.
pub struct Alert;

impl AlertMethods for Alert {
    fn new(_: &str) -> Alert {
        // TODO
        Alert
    }

    fn add_prompt(&mut self) {
        // TODO
    }

    fn run(&self) {
        // TODO
    }

    fn prompt_value(&self) -> ~str {
        // TODO
        ~""
    }
}

