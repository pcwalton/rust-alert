// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use AlertMethods;

use cocoa::appkit::{NSPoint, NSRect, NSSize};
use cocoa::base::{ObjCMethodCall, id, nil};
use core_foundation::base::TCFType;
use core_foundation::string::{CFString, CFStringRef};
use std::cast::transmute;

/// An alert.
pub struct Alert {
    nsalert: id,
    nstextfield: Option<id>,
}

impl AlertMethods for Alert {
    /// Creates a new alert with an OK and Cancel button.
    #[fixed_stack_segment]
    fn new(message_text: &str) -> Alert {
        unsafe {
            let alert_string: CFString = from_str(message_text).unwrap();
            let cancel_string = CFString::from_static_string("Cancel");
            let empty_string = CFString::from_static_string("");
            let nsalert = "NSAlert".send("alertWithMessageText:defaultButton:alternateButton:\
                                          otherButton:informativeTextWithFormat:",
                                         (transmute::<CFStringRef,id>(
                                                 alert_string.as_concrete_TypeRef()),
                                          nil,
                                          transmute::<CFStringRef,id>(
                                              cancel_string.as_concrete_TypeRef()),
                                          nil,
                                          transmute::<CFStringRef,id>(
                                              empty_string.as_concrete_TypeRef())));
            Alert {
                nsalert: transmute(nsalert),
                nstextfield: None,
            }
        }
    }

    #[fixed_stack_segment]
    fn add_prompt(&mut self) {
        unsafe {
            // [NSTextField alloc]
            let nstextfield = "NSTextField".send("alloc", ());

            // [nstextfield initWithFrame: NSMakeRect(0, 0, 200, 24)]
            let frame = NSRect {
                origin: NSPoint::new(0.0, 0.0),
                size: NSSize::new(200.0, 24.0),
            };
            let nstextfield = nstextfield.send("initWithFrame:", frame);

            // [nsalert setAccessoryView: nstextfield];
            self.nsalert.send_void("setAccessoryView:", nstextfield);

            // [nsalert layout];
            self.nsalert.send_void("layout", ());

            self.nstextfield = Some(transmute(nstextfield))
        }
    }

    #[fixed_stack_segment]
    fn run(&self) {
        unsafe {
            self.nsalert.send_void("runModal", ())
        }
    }

    #[fixed_stack_segment]
    fn prompt_value(&self) -> ~str {
        unsafe {
            // [nstextfield stringValue]
            match self.nstextfield {
                None => fail!("No prompt!"),
                Some(nstextfield) => {
                    let string = nstextfield.send("stringValue", ());
                    let string: CFString = TCFType::wrap_under_get_rule(transmute(string));
                    string.to_str()
                }
            }
        }
    }
}

