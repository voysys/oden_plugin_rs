use layout_id::layout_id;
use oden_plugin_rs::{SettingsApi, webview_user_message::WebviewUserMessage};
use std::{
    collections::{HashMap, HashSet},
    mem,
    sync::{
        Arc, Mutex,
        mpsc::{self, RecvTimeoutError, TryRecvError},
    },
    time::Duration,
};

mod layout_id;
#[derive(Debug)]
struct RegisterReceiver {
    sender: mpsc::SyncSender<WebviewUserMessage>,
    wanted_messages: HashSet<String>,
}

pub struct Sender {
    senders: Arc<Mutex<HashMap<String, RegisterReceiver>>>,
}

impl Clone for Sender {
    fn clone(&self) -> Self {
        Self {
            senders: self.senders.clone(),
        }
    }
}

impl Sender {
    pub fn try_send(&self, message: WebviewUserMessage) {
        if let Ok(lock) = self.senders.lock() {
            for registered_receiver in (*lock).values() {
                if registered_receiver.wanted_messages.contains(&message.name) {
                    registered_receiver.sender.try_send(message.clone()).ok();
                }
            }
        }
    }
}

pub struct Receiver {
    receiver: mpsc::Receiver<WebviewUserMessage>,
}

impl Receiver {
    pub fn try_recv(&self) -> Result<WebviewUserMessage, TryRecvError> {
        self.receiver.try_recv()
    }

    pub fn recv_timeout(&self, timeout: Duration) -> Result<WebviewUserMessage, RecvTimeoutError> {
        self.receiver.recv_timeout(timeout)
    }
}

pub fn create_sender(api: &dyn SettingsApi) -> Sender {
    let holder = get_or_create_webview_holder(api);
    unsafe {
        Sender {
            senders: (*holder).senders.clone(),
        }
    }
}

pub fn create_receiver(api: &dyn SettingsApi, wanted_messages: HashSet<String>) -> Receiver {
    let holder = get_or_create_webview_holder(api);

    let (sender, receiver) = mpsc::sync_channel(16);

    unsafe {
        let mut lock = (*holder).senders.lock().unwrap();
        lock.insert(
            api.instance_uuid().unwrap().to_string(),
            RegisterReceiver {
                sender,
                wanted_messages,
            },
        );
    }

    Receiver { receiver }
}

fn get_or_create_webview_holder(api: &dyn SettingsApi) -> *mut Sender {
    let name = format!("webview_userdata_channel_{}", layout_id::<Sender>().0);

    if let Ok((addr_data, _)) = api.query_data(&name, mem::size_of::<*const u8>() as i32) {
        let addr = usize::from_le_bytes(addr_data.try_into().unwrap());
        addr as *mut Sender
    } else {
        let holder = Box::new(Sender {
            senders: Arc::new(Mutex::new(HashMap::new())),
        });

        let p = Box::into_raw(holder);
        let addr = p as usize;
        let addr = addr.to_le_bytes();
        api.publish_data(&name, &addr);

        p
    }
}
