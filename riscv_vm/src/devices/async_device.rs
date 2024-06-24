use std::{
    any::Any,
    fmt::Debug,
    future::Future,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, RwLock,
    },
    thread::sleep,
    time::{Duration, Instant},
};

use crate::memory::{memory_buffer::MemoryBuffer, Memory};

use super::{DeviceError, DeviceInitError, DeviceMemHandle, DeviceObject};

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

/// Part three of an async device, this trait defines the behaviour of the device.
/// The update function is called once initially and then at the request of the device or
/// on an event, see [`AsyncDeviceUpdate`] on update reasons, and [`AsyncDeviceUpdateResult`] on
/// possible options for requesting the next event.
pub trait AsyncDevice: Debug + DeviceObject + Send {
    // TODO: events, maybe handle the eventloop more externally
    fn update(&mut self, update: AsyncDeviceUpdate)
        -> Result<AsyncDeviceUpdateResult, DeviceError>;
}

#[derive(Debug)]
pub(crate) struct AsyncDeviceHolder {
    device: Box<dyn AsyncDevice>,
}

impl AsyncDeviceHolder {
    pub fn new(device: Box<dyn AsyncDevice>) -> (Sender<()>, Self) {
        let (s, r) = mpsc::channel();
        (s, Self { device })
    }

    pub fn init_device(&mut self, mem: &mut Memory) -> Result<(), DeviceInitError> {
        DeviceObject::init(self.device.as_mut(), DeviceMemHandle::new(mem))?;
        Ok(())
    }

    pub fn run(mut self) {
        std::thread::spawn(move || {
            let mut result = self.device.update(AsyncDeviceUpdate::Initial);
            loop {
                match result {
                    Ok(AsyncDeviceUpdateResult::TimeOut(d)) => {
                        sleep(d);
                        result = self.device.update(AsyncDeviceUpdate::TimeOut)
                    }
                    Ok(AsyncDeviceUpdateResult::TimeoutUntil(i)) => {
                        let duration = i.saturating_duration_since(Instant::now());
                        sleep(duration);
                        result = self.device.update(AsyncDeviceUpdate::TimeOut)
                    }
                    Ok(AsyncDeviceUpdateResult::Continue) => {
                        self.device.update(AsyncDeviceUpdate::Continue);
                    }
                    Err(e) => {
                        eprintln!("Device {:#?} has errored with error {:#?}", self.device, e);
                        break;
                    }
                }
            }
        });
    }
}
