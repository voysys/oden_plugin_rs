//! Async com channels can be used to send com channels messages from threads

#![allow(clippy::upper_case_acronyms)]
#![allow(missing_docs)]

use crate::{
    plugin_h::OdenPluginGlobalFunctions, AsyncComChannelMessageId, AsyncComChannelReceiveError,
    AsyncComChannelVarSizeMsgId, ODEN_GLOBAL,
};
use assert_into::AssertInto;
use std::{ffi::CString, mem, time::Duration};
use zerocopy::{AsBytes, FromBytes};

pub fn async_com_channel_add_message_to_send_queue(message_id: &str, data: &impl AsBytes) {
    if unsafe { ODEN_GLOBAL.is_null() } {
        return;
    }

    let globals: &OdenPluginGlobalFunctions = unsafe { &*ODEN_GLOBAL };

    let c_message_id = CString::new(message_id).unwrap();

    if let Some(async_com_channel_add_message_to_send_queue) =
        globals.asyncComChannelAddMessageToSendQueue
    {
        let data = data.as_bytes();

        unsafe {
            async_com_channel_add_message_to_send_queue(
                c_message_id.as_ptr(),
                data.as_ptr(),
                data.len() as i32,
            )
        };
    }
}

pub fn async_com_channel_add_message_to_send_queue_raw(message_id: &str, data: &[u8]) {
    if unsafe { ODEN_GLOBAL.is_null() } {
        return;
    }

    let globals: &OdenPluginGlobalFunctions = unsafe { &*ODEN_GLOBAL };

    let c_message_id = CString::new(message_id).unwrap();

    if let Some(async_com_channel_add_message_to_send_queue) =
        globals.asyncComChannelAddMessageToSendQueue
    {
        unsafe {
            async_com_channel_add_message_to_send_queue(
                c_message_id.as_ptr(),
                data.as_ptr(),
                data.len() as i32,
            )
        };
    }
}

pub fn async_com_channel_flush_message_queue() {
    if unsafe { ODEN_GLOBAL.is_null() } {
        return;
    }

    let globals: &OdenPluginGlobalFunctions = unsafe { &*ODEN_GLOBAL };

    if let Some(async_com_channel_flush_message_queue) = globals.asyncComChannelFlushMessageQueue {
        unsafe { async_com_channel_flush_message_queue() };
    }
}

/// Receive any of the ids messages into buffer, return the index of the message received and bytes written to buffer
pub fn async_com_channel_receive_var_size(
    ids: &[&str],
    buffer: &mut [u8],
    timeout: Duration,
) -> Result<(i32, usize), AsyncComChannelReceiveError> {
    if unsafe { ODEN_GLOBAL.is_null() } {
        return Err(AsyncComChannelReceiveError::OdenAsyncComChannelReceiveErrorUnknown);
    }

    let globals: &OdenPluginGlobalFunctions = unsafe { &*ODEN_GLOBAL };

    if let Some(async_com_channel_receive_var_size_msg) = globals.asyncComChannelReceiveVarSizeMsg {
        let mut c_message_ids_strs: Vec<CString> = Vec::new();
        let mut c_message_ids: Vec<AsyncComChannelVarSizeMsgId> = Vec::new();

        for id in ids {
            c_message_ids_strs.push(CString::new(*id).unwrap());

            c_message_ids.push(AsyncComChannelVarSizeMsgId {
                messageId: c_message_ids_strs.last().unwrap().as_ptr(),
            });
        }

        let mut bytes_written = 0;

        let res = unsafe {
            async_com_channel_receive_var_size_msg(
                c_message_ids.as_ptr(),
                c_message_ids.len() as i32,
                buffer.as_mut_ptr() as *mut _,
                buffer.len() as i32,
                &mut bytes_written as *mut _,
                timeout.as_millis() as i32,
            )
        };

        match res {
            0_i32..=i32::MAX => Ok((res, bytes_written as usize)),
            -1 => Err(AsyncComChannelReceiveError::OdenAsyncComChannelReceiveErrorTimeout),
            -2 => Err(
                AsyncComChannelReceiveError::OdenAsyncComChannelReceiveErrorBufferTooSmallForMessage,
            ),
            -4 => {
                Err(AsyncComChannelReceiveError::OdenAsyncComChannelReceiveErrorGotMessageWithWrongSize)
            }
            i32::MIN..=-3_i32 => {
                Err(AsyncComChannelReceiveError::OdenAsyncComChannelReceiveErrorUnknown)
            }
        }
    } else {
        Err(AsyncComChannelReceiveError::OdenAsyncComChannelReceiveErrorUnknown)
    }
}

macro_rules! impl_async_com_channel_receive {
    ($($t:ident),*) => {
        paste::paste! {

            pub enum [<AsyncReceive$($t)*>]<$($t),*> {
                $(
                    $t($t)
                ),*
            }

            impl<$($t),*> [<AsyncReceive$($t)*>]<$($t),*> {
                #[allow(clippy::too_many_arguments)]
                pub fn async_com_channel_receive(
                    $(
                        [<message_id_$t:lower>]: &str,
                    )*
                    timeout: Duration,
                ) -> Result<Self, AsyncComChannelReceiveError>
                where
                    $(
                        $t: FromBytes + AsBytes + Default,
                    )*
                {
                    if unsafe { ODEN_GLOBAL.is_null() } {
                        return Err(AsyncComChannelReceiveError::OdenAsyncComChannelReceiveErrorUnknown);
                    }

                    let globals: &OdenPluginGlobalFunctions = unsafe { &*ODEN_GLOBAL };

                    if let Some(async_com_channel_receive) = globals.asyncComChannelReceive {
                        let mut c_message_ids_strs: Vec<CString> = Vec::new();
                        let mut c_message_ids: Vec<AsyncComChannelMessageId> = Vec::new();

                        let mut max_size = 0;

                        $(
                            {
                                let c_str = CString::new([<message_id_$t:lower>]).unwrap();
                                c_message_ids_strs.push(c_str);

                                let message_size = mem::size_of::<$t>();

                                c_message_ids.push(AsyncComChannelMessageId {
                                    messageId: c_message_ids_strs.last().unwrap().as_ptr(),
                                    dataSize: message_size.assert_into(),
                                });

                                max_size = max_size.max(message_size)
                            }
                        )*

                        let mut buf: Vec<u8> = vec![0; max_size];

                        let res = unsafe {
                            async_com_channel_receive(
                                c_message_ids.as_ptr(),
                                c_message_ids.len() as i32,
                                buf.as_mut_ptr() as *mut _,
                                buf.len() as i32,
                                timeout.as_millis() as i32,
                            )
                        };

                        match res {
                            0_i32..=i32::MAX => {
                                let res = res as usize;

                                let mut count = -1i32;

                                $(
                                    count += 1;

                                    let count_usize: usize = count.assert_into();
                                    if res == count_usize {
                                        let mut dest_message = $t::default();
                                        let dest = dest_message.as_bytes_mut();

                                        let src = &buf[..dest.len()];

                                        dest.copy_from_slice(src);

                                        return Ok(Self::$t(dest_message))
                                    }
                                )*

                                panic!("Internal error");
                            }
                            -1 => {
                                Err(AsyncComChannelReceiveError::OdenAsyncComChannelReceiveErrorTimeout)
                            }
                            -2 => {
                                Err(AsyncComChannelReceiveError::OdenAsyncComChannelReceiveErrorBufferTooSmallForMessage)
                            }
                            -4 => {
                                Err(AsyncComChannelReceiveError::OdenAsyncComChannelReceiveErrorGotMessageWithWrongSize)
                            }
                            i32::MIN..=-3_i32 => {
                                Err(AsyncComChannelReceiveError::OdenAsyncComChannelReceiveErrorUnknown)
                            }
                        }
                    } else {
                        Err(AsyncComChannelReceiveError::OdenAsyncComChannelReceiveErrorUnknown)
                    }
                }
            }
        }
    };
}

impl_async_com_channel_receive!(A);
impl_async_com_channel_receive!(A, B);
impl_async_com_channel_receive!(A, B, C);
impl_async_com_channel_receive!(A, B, C, D);
impl_async_com_channel_receive!(A, B, C, D, E);
impl_async_com_channel_receive!(A, B, C, D, E, F);
impl_async_com_channel_receive!(A, B, C, D, E, F, G);
impl_async_com_channel_receive!(A, B, C, D, E, F, G, H);
impl_async_com_channel_receive!(A, B, C, D, E, F, G, H, I);
impl_async_com_channel_receive!(A, B, C, D, E, F, G, H, I, J);
impl_async_com_channel_receive!(A, B, C, D, E, F, G, H, I, J, K);
impl_async_com_channel_receive!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_async_com_channel_receive!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_async_com_channel_receive!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_async_com_channel_receive!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_async_com_channel_receive!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
