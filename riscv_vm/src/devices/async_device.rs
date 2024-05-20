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

use crate::memory::{registers::MemoryRegisterHandle, DeviceMemory};

use super::{
    event_bus::{self, DeviceEvent, DeviceEventBusHandle},
    DeviceData, DeviceError, DeviceInitError, DeviceObject,
};

/// Indicates the reason an `AsyncDevice`'s update function was called
pub enum AsyncDeviceUpdate {
    /// Initial device update, used to force the device to return an `AsyncDeviceUpdateResult`
    Initial,
    /// Requested timeout has ended
    TimeOut,
    /// Immediate continue from last event
    Continue,
    /// Device recieved an event
    DeviceEvent(DeviceEvent),
}

/// Allows a async device to indicate when it wants its next update.
pub enum AsyncDeviceUpdateResult {
    /// Wait for duration or an event whichever is earlier
    TimeOut(Duration),
    /// Wait until instant or an event whichever is earlier
    TimeoutUntil(Instant),
    /// Wait for event
    WaitForEvent,
    /// Immedtiately update
    Continue,
}

/// Part three of an async device, this trait defines the behaviour of the device.
/// The update function is called once initially and then at the request of the device or
/// on an event, see `AsyncDeviceUpdate` on update reasons, and `AsyncDeviceUpdateResult` on
/// possible options for requesting the next event.
pub trait AsyncDevice: Debug + DeviceObject + Send {
    // TODO: events, maybe handle the eventloop more externally
    fn update(
        &mut self,
        mem: Arc<RwLock<DeviceMemory>>,
        update: AsyncDeviceUpdate,
        event_bus: &DeviceEventBusHandle,
        data: DeviceData,
    ) -> Result<AsyncDeviceUpdateResult, DeviceError>;
}

#[derive(Debug)]
pub(crate) struct AsyncDeviceHolder {
    device: Box<dyn AsyncDevice>,
    event_bus: Receiver<DeviceEvent>,
    data: Option<DeviceData>,
}

impl AsyncDeviceHolder {
    pub fn new(device: Box<dyn AsyncDevice>) -> (Sender<DeviceEvent>, Self) {
        let (s, r) = mpsc::channel();
        (
            s,
            Self {
                device,
                event_bus: r,
                data: None,
            },
        )
    }

    pub fn init_device(
        &mut self,
        mem: &mut DeviceMemory,
        registers: MemoryRegisterHandle<'_>,
    ) -> Result<(), DeviceInitError> {
        let data = DeviceObject::init(self.device.as_mut(), mem, registers)?;
        self.data = Some(data);
        Ok(())
    }

    pub fn run(mut self, mem: Arc<RwLock<DeviceMemory>>, event_bus: DeviceEventBusHandle) {
        std::thread::spawn(move || {
            let data = self.data.expect("Device run before initialising");
            let mut result = self.device.update(
                mem.clone(),
                AsyncDeviceUpdate::Initial,
                &event_bus,
                data.clone(),
            );
            loop {
                match result {
                    Ok(AsyncDeviceUpdateResult::TimeOut(d)) => match self.event_bus.recv_timeout(d)
                    {
                        Ok(e) => {
                            result = self.device.update(
                                mem.clone(),
                                AsyncDeviceUpdate::DeviceEvent(e),
                                &event_bus,
                                data.clone(),
                            )
                        }
                        Err(e) => match e {
                            mpsc::RecvTimeoutError::Timeout => {
                                result = self.device.update(
                                    mem.clone(),
                                    AsyncDeviceUpdate::TimeOut,
                                    &event_bus,
                                    data.clone(),
                                )
                            }
                            mpsc::RecvTimeoutError::Disconnected => break,
                        },
                    },
                    Ok(AsyncDeviceUpdateResult::TimeoutUntil(i)) => {
                        let duration = i.saturating_duration_since(Instant::now());
                        match self.event_bus.recv_timeout(duration) {
                            Ok(e) => {
                                result = self.device.update(
                                    mem.clone(),
                                    AsyncDeviceUpdate::DeviceEvent(e),
                                    &event_bus,
                                    data.clone(),
                                )
                            }
                            Err(e) => match e {
                                mpsc::RecvTimeoutError::Timeout => {
                                    result = self.device.update(
                                        mem.clone(),
                                        AsyncDeviceUpdate::TimeOut,
                                        &event_bus,
                                        data.clone(),
                                    )
                                }
                                mpsc::RecvTimeoutError::Disconnected => break,
                            },
                        }
                    }
                    Ok(AsyncDeviceUpdateResult::WaitForEvent) => match self.event_bus.recv() {
                        Ok(e) => {
                            result = self.device.update(
                                mem.clone(),
                                AsyncDeviceUpdate::DeviceEvent(e),
                                &event_bus,
                                data.clone(),
                            )
                        }
                        Err(e) => {
                            eprintln!(
                                "Device {:#?} has errored with error {:#?} on event_bus",
                                self.device, e
                            );
                            break;
                        }
                    },
                    Ok(AsyncDeviceUpdateResult::Continue) => {
                        self.device.update(
                            mem.clone(),
                            AsyncDeviceUpdate::Continue,
                            &event_bus,
                            data.clone(),
                        );
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
