#![allow(unused)]

use std::{
    convert::Infallible,
    fmt::Debug,
    sync::mpsc::{self, Receiver, Sender},
    thread::sleep,
    time::{Duration, Instant},
};

use crate::memory::Memory;

use super::{Device, DeviceError, DeviceInitError, DeviceMemHandle, DeviceObject};

/// Indicates the reason an [`AsyncDevice`]'s update function was called
pub enum AsyncDeviceUpdate {
    /// Initial device update, used to force the device to return an [`AsyncDeviceUpdateResult`]
    Initial,
    /// Requested timeout has ended
    TimeOut,
    /// Immediate continue from last event
    Continue,
}

/// Allows a async device to indicate when it wants its next update.
pub enum AsyncDeviceUpdateResult {
    /// Wait for duration or an event whichever is earlier
    TimeOut(Duration),
    /// Wait until instant or an event whichever is earlier
    TimeoutUntil(Instant),
    /// Immedtiately update
    Continue,
}

// Part three of an async device, this trait defines the behaviour of the device.
// The update function is called once initially and then at the request of the device or
// on an event, see [`AsyncDeviceUpdate`] on update reasons, and [`AsyncDeviceUpdateResult`] on
// possible options for requesting the next event.
pub trait AsyncDevice: DeviceObject {
    fn run(self) -> Result<Infallible, DeviceError>;
}

#[repr(transparent)]
struct MemPointer(*mut ());

unsafe impl Send for MemPointer {}

pub(crate) struct AsyncDeviceHolder {
    // device: Box<dyn AsyncDevice>,
    notify: Sender<MemPointer>,
    wait: Receiver<()>,
}

impl Debug for AsyncDeviceHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncDeviceHolder").finish_non_exhaustive()
    }
}

impl AsyncDeviceHolder {
    pub fn new<D: Device + AsyncDevice + 'static>() -> Self {
        let (h_notify, d_wait) = mpsc::channel::<MemPointer>();
        let (d_notify, h_wait) = mpsc::channel();
        std::thread::spawn(move || {
            let mut device = Box::new(D::new());
            // SAFETY
            // We get a pointer via channel and call the other notify channel last ting we
            // do before the poiter goes out of scope, because `init_and_run_device` does not
            // return before we are done with the pointer, no one can access mem while we do.
            unsafe {
                let mem = std::mem::transmute::<MemPointer, *mut Memory>(d_wait.recv().unwrap());
                DeviceObject::init(device.as_mut(), DeviceMemHandle::new(&mut *mem)).unwrap();
                d_notify.send(());
            }
            device.run();
        });

        Self {
            notify: h_notify,
            wait: h_wait,
        }
    }

    pub fn init_and_run_device(&mut self, mem: &mut Memory) {
        // SAFETY
        // We give a pointer to the thread on the other side, this thread
        // then notifies us after it is done with the pointer after which we can return.
        self.notify
            .send(MemPointer(unsafe { mem as *mut Memory as *mut () }));
        self.wait.recv();
    }
}
