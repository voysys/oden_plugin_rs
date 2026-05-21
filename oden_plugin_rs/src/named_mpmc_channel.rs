//! Named mpmc channels is a method to communicate with a channel api between different plugins

use crate::{
    layout_id::layout_id, DrawParams, GuiParams, InitParams, SettingsApi, ShutdownParams,
    UpdateParams,
};
use std::{mem, ops::Deref};

/// See flume Sender
#[derive(Clone)]
pub struct Sender<T> {
    inner: flume::Sender<T>,
}

impl<T> Deref for Sender<T> {
    type Target = flume::Sender<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// See flume Receiver
#[derive(Clone)]
pub struct Receiver<T> {
    inner: flume::Receiver<T>,
}

impl<T> Deref for Receiver<T> {
    type Target = flume::Receiver<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

struct NamedMpmcChannel<T> {
    sender: flume::Sender<T>,
    receiver: flume::Receiver<T>,
}

fn create_named_channel<T>(buffer: usize) -> Box<NamedMpmcChannel<T>> {
    let (tx, rx) = flume::bounded(buffer);

    Box::new(NamedMpmcChannel {
        sender: tx,
        receiver: rx,
    })
}

fn get_or_create_named_channel<T>(
    api: &dyn SettingsApi,
    name: &str,
    buffer: usize,
) -> *const NamedMpmcChannel<T> {
    let name = format!("{name}_{}_channel", layout_id::<T>().0);

    if let Ok((addr_data, _)) = api.query_data(&name, mem::size_of::<*const u8>() as i32) {
        let addr = usize::from_le_bytes(addr_data.try_into().unwrap());
        addr as *const NamedMpmcChannel<T>
    } else {
        let p = create_named_channel(buffer);
        let p = Box::into_raw(p);

        let addr = p as usize;
        let addr = addr.to_le_bytes();
        api.publish_data(&name, &addr);

        p
    }
}

fn named_mpmc_channel_tx<T>(api: &dyn SettingsApi, name: &str, buffer: usize) -> Sender<T> {
    let named_channel = get_or_create_named_channel(api, name, buffer);
    let named_channel = unsafe { &*named_channel };

    let tx = named_channel.sender.clone();

    Sender { inner: tx }
}

fn named_mpmc_channel_rx<T>(api: &dyn SettingsApi, name: &str, buffer: usize) -> Receiver<T> {
    let named_channel = get_or_create_named_channel(api, name, buffer);
    let named_channel = unsafe { &*named_channel };

    let rx = named_channel.receiver.clone();

    Receiver { inner: rx }
}

/// Extension trait for named mpmc channel methods
pub trait NamedMpmcChannelExt {
    /// Create or fetch a named mpmc channel with the specified name
    fn named_mpmc_channel_tx<T>(&self, name: &str, buffer: usize) -> Sender<T>;
    /// Create or fetch a named mpmc channel with the specified name
    fn named_mpmc_channel_rx<T>(&self, name: &str, buffer: usize) -> Receiver<T>;
}

impl NamedMpmcChannelExt for InitParams<'_> {
    fn named_mpmc_channel_tx<T>(&self, name: &str, buffer: usize) -> Sender<T> {
        named_mpmc_channel_tx(self, name, buffer)
    }

    fn named_mpmc_channel_rx<T>(&self, name: &str, buffer: usize) -> Receiver<T> {
        named_mpmc_channel_rx(self, name, buffer)
    }
}

impl NamedMpmcChannelExt for ShutdownParams<'_> {
    fn named_mpmc_channel_tx<T>(&self, name: &str, buffer: usize) -> Sender<T> {
        named_mpmc_channel_tx(self, name, buffer)
    }

    fn named_mpmc_channel_rx<T>(&self, name: &str, buffer: usize) -> Receiver<T> {
        named_mpmc_channel_rx(self, name, buffer)
    }
}

impl NamedMpmcChannelExt for UpdateParams<'_> {
    fn named_mpmc_channel_tx<T>(&self, name: &str, buffer: usize) -> Sender<T> {
        named_mpmc_channel_tx(self, name, buffer)
    }

    fn named_mpmc_channel_rx<T>(&self, name: &str, buffer: usize) -> Receiver<T> {
        named_mpmc_channel_rx(self, name, buffer)
    }
}

impl NamedMpmcChannelExt for DrawParams<'_> {
    fn named_mpmc_channel_tx<T>(&self, name: &str, buffer: usize) -> Sender<T> {
        named_mpmc_channel_tx(self, name, buffer)
    }

    fn named_mpmc_channel_rx<T>(&self, name: &str, buffer: usize) -> Receiver<T> {
        named_mpmc_channel_rx(self, name, buffer)
    }
}

impl NamedMpmcChannelExt for GuiParams<'_> {
    fn named_mpmc_channel_tx<T>(&self, name: &str, buffer: usize) -> Sender<T> {
        named_mpmc_channel_tx(self, name, buffer)
    }

    fn named_mpmc_channel_rx<T>(&self, name: &str, buffer: usize) -> Receiver<T> {
        named_mpmc_channel_rx(self, name, buffer)
    }
}

impl NamedMpmcChannelExt for &dyn SettingsApi {
    fn named_mpmc_channel_tx<T>(&self, name: &str, buffer: usize) -> Sender<T> {
        named_mpmc_channel_tx(*self, name, buffer)
    }

    fn named_mpmc_channel_rx<T>(&self, name: &str, buffer: usize) -> Receiver<T> {
        named_mpmc_channel_rx(*self, name, buffer)
    }
}
