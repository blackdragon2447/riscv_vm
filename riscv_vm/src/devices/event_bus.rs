use std::sync::mpsc::{self, Receiver, Sender};

use nohash_hasher::IntMap;

use crate::memory::address::Address;

use super::DeviceId;

pub enum DeviceEventType {
    RegisterWrite(Address),
}

pub struct DeviceEvent(pub DeviceId, pub DeviceEventType);

pub struct DeviceEventBus {
    reciever: Receiver<DeviceEvent>,
    distributor: IntMap<DeviceId, Sender<DeviceEvent>>,
}

impl DeviceEventBus {
    pub fn new() -> (Sender<DeviceEvent>, Self) {
        let (s, r) = mpsc::channel();
        (
            s,
            Self {
                reciever: r,
                distributor: IntMap::default(),
            },
        )
    }

    pub fn add_device(&mut self, device: DeviceId, sender: Sender<DeviceEvent>) {
        self.distributor.insert(device, sender);
    }

    pub fn distribute(&self) {
        for e in self.reciever.try_iter() {
            self.distributor.get(&e.0).unwrap().send(e);
        }
    }
}
