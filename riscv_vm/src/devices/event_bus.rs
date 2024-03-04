use std::sync::mpsc::{self, Receiver, Sender};

use enumflags2::bitflags;
use nohash_hasher::IntMap;

use crate::{
    hart::trap::{Interrupt, InterruptTarget},
    memory::address::Address,
};

use super::DeviceId;

pub enum DeviceEventType {
    RegisterWrite(Address),
}

pub struct DeviceEvent(pub DeviceId, pub DeviceEventType);

pub struct DeviceEventBus {
    receiver: Receiver<DeviceEvent>,
    distributor: IntMap<DeviceId, Sender<DeviceEvent>>,
    interrupter: Sender<(InterruptTarget, Interrupt)>,
    interrupt_receiver: Receiver<(InterruptTarget, Interrupt)>,
}

#[repr(u8)]
#[bitflags]
#[derive(Clone, Copy)]
pub enum InterruptPermission {
    Normal,
    InterruptController,
}

pub enum EventBusError {
    PermissionDenied,
}

pub struct DeviceEventBusHandle {
    permission: InterruptPermission,
    interrupter: Sender<(InterruptTarget, Interrupt)>,
}

impl DeviceEventBus {
    pub fn new() -> (Sender<DeviceEvent>, Self) {
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

    pub fn add_device(&mut self, device: DeviceId, sender: Sender<DeviceEvent>) {
        self.distributor.insert(device, sender);
    }

    pub fn distribute(&self) {
        for e in self.receiver.try_iter() {
            self.distributor.get(&e.0).unwrap().send(e);
        }
    }

    pub fn interrupts(&self) -> Vec<(InterruptTarget, Interrupt)> {
        self.interrupt_receiver.try_iter().collect()
    }

    pub fn get_handle(&self, permission: InterruptPermission) -> DeviceEventBusHandle {
        DeviceEventBusHandle {
            permission,
            interrupter: self.interrupter.clone(),
        }
    }
}

impl DeviceEventBusHandle {
    pub fn send_interrupt(
        &self,
        target_hart: InterruptTarget,
        interrupt: Interrupt,
    ) -> Result<(), EventBusError> {
        match self.permission {
            InterruptPermission::Normal => {
                if interrupt == Interrupt::External {
                    self.interrupter.send((target_hart, interrupt));
                    Ok(())
                } else {
                    Err(EventBusError::PermissionDenied)
                }
            }
            InterruptPermission::InterruptController => {
                self.interrupter.send((target_hart, interrupt));
                Ok(())
            }
        }
    }
}
