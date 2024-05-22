use core::panic;
use std::sync::mpsc::{self, Receiver, Sender};

use enumflags2::bitflags;
use nohash_hasher::IntMap;

use crate::{
    hart::trap::{Interrupt, InterruptTarget},
    memory::address::Address,
};

use super::DeviceId;

#[derive(Debug)]
pub enum DeviceEventType {
    RegisterWrite(Address),
}

#[derive(Debug)]
pub(crate) enum InterruptSignal {
    Set(InterruptTarget, Interrupt),
    Clear(InterruptTarget, Interrupt),
}

#[derive(Debug)]
pub struct DeviceEvent(pub DeviceId, pub DeviceEventType);

pub(crate) struct DeviceEventBus {
    receiver: Receiver<DeviceEvent>,
    distributor: IntMap<DeviceId, Sender<DeviceEvent>>,
    interrupter: Sender<InterruptSignal>,
    interrupt_receiver: Receiver<InterruptSignal>,
}

#[repr(u8)]
#[bitflags]
#[derive(Clone, Copy)]
pub(crate) enum InterruptPermission {
    Normal,
    InterruptController,
}

#[derive(Debug)]
pub enum EventBusError {
    PermissionDenied,
}

/// Device's connection to the event bus, allows a device to send interrupts
pub struct DeviceEventBusHandle {
    permission: InterruptPermission,
    interrupter: Sender<InterruptSignal>,
}

impl DeviceEventBus {
    pub(crate) fn new() -> (Sender<DeviceEvent>, Self) {
        let (se, re) = mpsc::channel();
        let (si, ri) = mpsc::channel();
        (
            se,
            Self {
                receiver: re,
                distributor: IntMap::default(),
                interrupter: si,
                interrupt_receiver: ri,
            },
        )
    }

    pub(crate) fn add_device(&mut self, device: DeviceId, sender: Sender<DeviceEvent>) {
        self.distributor.insert(device, sender);
    }

    pub(crate) fn distribute(&self) {
        for e in self.receiver.try_iter() {
            if let Some(dev) = self.distributor.get(&e.0) {
                dev.send(e);
            }
        }
    }

    pub(crate) fn interrupts(&self) -> Vec<InterruptSignal> {
        self.interrupt_receiver.try_iter().collect()
    }

    pub(crate) fn get_handle(&self, permission: InterruptPermission) -> DeviceEventBusHandle {
        DeviceEventBusHandle {
            permission,
            interrupter: self.interrupter.clone(),
        }
    }
}

impl DeviceEventBusHandle {
    /// Send an interrupt to the device, may target a single hart of all harts, see [`InterruptTarget`] for details.
    /// Errors is the device does not have permission to send the type of interrupt, unless stated
    /// otherwise, devices should assume that they only have permission to send External
    /// interrupts.
    pub fn send_interrupt(
        &self,
        target_hart: InterruptTarget,
        interrupt: Interrupt,
    ) -> Result<(), EventBusError> {
        match self.permission {
            InterruptPermission::Normal => {
                if interrupt == Interrupt::External {
                    self.interrupter
                        .send(InterruptSignal::Set(target_hart, interrupt));
                    Ok(())
                } else {
                    Err(EventBusError::PermissionDenied)
                }
            }
            InterruptPermission::InterruptController => {
                self.interrupter
                    .send(InterruptSignal::Set(target_hart, interrupt));
                Ok(())
            }
        }
    }

    /// Clean an interrupt, clears the interrupt for all values.
    pub fn clear_interrupt(
        &self,
        target_hart: InterruptTarget,
        interrupt: Interrupt,
    ) -> Result<(), EventBusError> {
        match self.permission {
            InterruptPermission::Normal => {
                if interrupt == Interrupt::External {
                    self.interrupter
                        .send(InterruptSignal::Clear(target_hart, interrupt));
                    Ok(())
                } else {
                    Err(EventBusError::PermissionDenied)
                }
            }
            InterruptPermission::InterruptController => {
                self.interrupter
                    .send(InterruptSignal::Clear(target_hart, interrupt));
                Ok(())
            }
        }
    }
}
