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
    event_bus::{self, DeviceEvent},
    DeviceError, DeviceInitError, DeviceObject,
};

pub enum AsyncDeviceUpdate {
    Initial,
    TimeOut,
    Continue,
    DeviceEvent(DeviceEvent),
}

pub enum AsyncDeviceUpdateResult {
    /// Wait for duration or an event whichever is earlier
    TimeOut(Duration),
    /// Wait until instant or an event whichever is earlier
    TimeoutUntil(Instant),
    WaitForEvent,
    Continue,
}

pub trait AsyncDevice: Debug + DeviceObject + Send {
    // TODO: events, maybe handle the eventloop more externally
    fn update(
        &mut self,
        mem: Arc<RwLock<DeviceMemory>>,
        update: AsyncDeviceUpdate,
    ) -> Result<AsyncDeviceUpdateResult, DeviceError>;
}

#[derive(Debug)]
pub struct AsyncDeviceHolder {
    device: Box<dyn AsyncDevice>,
    event_bus: Receiver<DeviceEvent>,
}

// pub struct TimeoutReciever<T>(Receiver<T>);
//
// impl<T> Future for TimeoutReciever<T> {
//     type Output = T;
//
//     fn poll(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Self::Output> {
//         todo!()
//     }
// }

impl AsyncDeviceHolder {
    pub fn new(device: Box<dyn AsyncDevice>) -> (Sender<DeviceEvent>, Self) {
        let (s, r) = mpsc::channel();
        (
            s,
            Self {
                device,
                event_bus: r,
            },
        )
    }

    pub fn init_device(
        &mut self,
        mem: &mut DeviceMemory,
        registers: MemoryRegisterHandle<'_>,
    ) -> Result<(), DeviceInitError> {
        DeviceObject::init(self.device.as_mut(), mem, registers)
    }

    pub fn run(mut self, mem: Arc<RwLock<DeviceMemory>>) {
        std::thread::spawn(move || {
            let mut result = self.device.update(mem.clone(), AsyncDeviceUpdate::Initial);
            loop {
                match result {
                    Ok(AsyncDeviceUpdateResult::TimeOut(d)) => match self.event_bus.recv_timeout(d)
                    {
                        Ok(e) => {
                            result = self
                                .device
                                .update(mem.clone(), AsyncDeviceUpdate::DeviceEvent(e))
                        }
                        Err(e) => match e {
                            mpsc::RecvTimeoutError::Timeout => {
                                result = self.device.update(mem.clone(), AsyncDeviceUpdate::TimeOut)
                            }
                            mpsc::RecvTimeoutError::Disconnected => break,
                        },
                    },
                    Ok(AsyncDeviceUpdateResult::TimeoutUntil(i)) => {
                        let duration = i.saturating_duration_since(Instant::now());
                        match self.event_bus.recv_timeout(duration) {
                            Ok(e) => {
                                result = self
                                    .device
                                    .update(mem.clone(), AsyncDeviceUpdate::DeviceEvent(e))
                            }
                            Err(e) => match e {
                                mpsc::RecvTimeoutError::Timeout => {
                                    result =
                                        self.device.update(mem.clone(), AsyncDeviceUpdate::TimeOut)
                                }
                                mpsc::RecvTimeoutError::Disconnected => break,
                            },
                        }
                    }
                    Ok(AsyncDeviceUpdateResult::WaitForEvent) => match self.event_bus.recv() {
                        Ok(e) => {
                            result = self
                                .device
                                .update(mem.clone(), AsyncDeviceUpdate::DeviceEvent(e))
                        }
                        Err(e) => {
                            eprintln!(
                                "Device {:#?} has errored with error {:#?} on event_bus",
                                self, e
                            );
                            break;
                        }
                    },
                    Ok(AsyncDeviceUpdateResult::Continue) => {
                        self.device.update(mem.clone(), AsyncDeviceUpdate::Continue);
                    }
                    Err(e) => {
                        eprintln!("Device {:#?} has errored with error {:#?}", self, e);
                        break;
                    }
                }
            }
        });
    }
}
