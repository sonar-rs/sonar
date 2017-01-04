// Copyright 2016 James Chapman
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Event;
use Monitor;

#[derive(Copy, Clone)]
pub struct WindowAttributes<'a, 'b> {
    pub title: &'a str,
    pub size: Option<(u32, u32)>,
    pub monitor: Option<&'b Monitor>
}

impl<'a, 'b> Default for WindowAttributes<'a, 'b> {
    fn default() -> Self {
        WindowAttributes {
            title: "Sonar Application",
            size: None,
            monitor: None
        }
    }
}

pub struct PollEvents<'a> {
    window: &'a Window
}

pub struct WaitEvents<'a> {
    window: &'a Window
}

impl<'a> Iterator for PollEvents<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

impl<'a> Iterator for WaitEvents<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

pub struct Window {
}

impl Window {
    pub fn new(mut attributes: WindowAttributes) -> Result<Self, ()> {
        if attributes.size.is_none() {
            attributes.size = attributes.monitor.map_or(Some((1024, 768)), |m| Some(m.size()));
        }

        Ok(Window {})
    }

    pub fn poll_events<'a>(&'a self) -> PollEvents<'a> {
        PollEvents { window: self }
    }

    pub fn wait_events<'a>(&'a self) -> WaitEvents<'a> {
        WaitEvents { window: self }
    }
}

#[derive(Copy, Clone)]
pub struct WindowBuilder<'a, 'b> {
    pub attributes: WindowAttributes<'a, 'b>
}

impl<'a, 'b> WindowBuilder<'a, 'b> {
    pub fn new() -> Self {
        WindowBuilder { attributes: WindowAttributes::default() }
    }

    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.attributes.title = title;
        self
    }

    pub fn size(&mut self, width: u32, height: u32) -> &mut Self {
        self.attributes.size = Some((width, height));
        self
    }

    pub fn monitor(&mut self, monitor: &'b Monitor) -> &mut Self {
        self.attributes.monitor = Some(monitor);
        self
    }

    pub fn build(&self) -> Result<Window, ()> {
        Window::new(self.attributes)
    }
}
