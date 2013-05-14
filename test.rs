// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This is not a typical test harness, because alerts require user input.

use Alert;
use AlertMethods;

use cocoa::appkit;
use core::task::PlatformThread;
use core::task;

#[main]
pub fn main() {
    let mut builder = task::task();
    builder.sched_mode(PlatformThread);
    do builder.spawn {
        init();

        let mut alert: Alert = AlertMethods::new("All units destroyed.");
        alert.add_prompt();
        alert.run()
    }
}

#[cfg(target_os="macos")]
fn init() {
    let _ = appkit::NSApp();
}

