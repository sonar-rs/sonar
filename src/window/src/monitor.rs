// Copyright 2016 James Chapman
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub struct VideoMode {}

impl VideoMode {
    pub fn size(&self) -> (u32, u32) {
        unimplemented!()
    }

    pub fn refresh_rate(&self) -> u32 {
        unimplemented!()
    }
}

pub struct Monitor {}

impl Monitor {
    pub fn pos(&self) -> (u32, u32) {
        unimplemented!()
    }

    pub fn size(&self) -> (u32, u32) {
        unimplemented!()
    }

    pub fn name(&self) -> &str {
        unimplemented!()
    }

    pub fn modes(&self) -> &[VideoMode] {
        unimplemented!()
    }

    pub fn current_mode(&self) -> &VideoMode {
        unimplemented!()
    }

    pub fn preferred_mode(&self) -> &VideoMode {
        unimplemented!()
    }
}

pub fn monitors() -> Vec<Monitor> {
    unimplemented!()
}

pub fn primary_monitor() -> Option<Monitor> {
    unimplemented!()
}
