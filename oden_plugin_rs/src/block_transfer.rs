//! Block transfer is a system where a BlockTransferServer can be used to serve a block of data and a BlockTransferClient can be used to retrieve the data.

use crate::{SceneApi, SceneApiExt};
use zerocopy::{AsBytes, FromBytes};

#[derive(Copy, Clone, AsBytes, FromBytes, Default)]
#[repr(C, packed)]
struct QueryRequest {
    status: u8,
}

#[derive(Copy, Clone, AsBytes, FromBytes, Default)]
#[repr(C, packed)]
struct QueryResponse {
    block_count: u32,
}

#[derive(Copy, Clone, AsBytes, FromBytes, Default)]
#[repr(C, packed)]
struct BlockRequest {
    block_id: u32,
}

#[derive(Copy, Clone, AsBytes, FromBytes)]
#[repr(C, packed)]
struct BlockResponse {
    block_id: u32,
    data_len: u16,
    data: [u8; 1024],
}

impl Default for BlockResponse {
    fn default() -> Self {
        Self {
            block_id: Default::default(),
            data_len: Default::default(),
            data: [0; 1024],
        }
    }
}

struct BlockTransferIds {
    block_request_id: String,
    block_response_id: String,
    query_request_id: String,
    query_response_id: String,
}

impl BlockTransferIds {
    fn new(id: &str) -> BlockTransferIds {
        BlockTransferIds {
            block_request_id: format!("{id}_BLOCK_REQUEST"),
            block_response_id: format!("{id}_BLOCK_RESPONSE"),
            query_request_id: format!("{id}_QUERY_REQUEST"),
            query_response_id: format!("{id}_QUERY_RESPONSE"),
        }
    }
}

/// A service that allows a client to transfer a specific block of data from the server to the client.
pub struct BlockTransferServer {
    ids: BlockTransferIds,
    blocks: Vec<BlockResponse>,
}

impl BlockTransferServer {
    /// Creates a new block transfer server that serves the data passed in as the data parameter
    /// using the id.
    pub fn new(id: impl AsRef<str>, data: &[u8]) -> BlockTransferServer {
        let id = id.as_ref();

        let blocks = data
            .chunks(1024)
            .enumerate()
            .map(|(i, chunk)| {
                let mut block = BlockResponse {
                    block_id: i.try_into().unwrap(),
                    data_len: chunk.len() as u16,
                    data: [0; 1024],
                };
                block.data[..chunk.len()].copy_from_slice(chunk);
                block
            })
            .collect();

        BlockTransferServer {
            ids: BlockTransferIds::new(id),
            blocks,
        }
    }

    /// Polls the block transfer server allowing it to respond to any
    /// incoming requests.
    pub fn poll(&self, api: &(impl SceneApi + SceneApiExt)) {
        {
            let request_count = api.com_channel_message_count(&self.ids.query_request_id);

            for i in 0..request_count {
                if let Some(_req) =
                    api.com_channel_message::<QueryRequest>(&self.ids.query_request_id, i)
                {
                    api.com_channel_send_message(
                        &self.ids.query_response_id,
                        &QueryResponse {
                            block_count: self.blocks.len() as u32,
                        },
                    )
                }
            }
        }

        {
            let request_count = api.com_channel_message_count(&self.ids.block_request_id);

            for i in 0..request_count {
                if let Some(req) =
                    api.com_channel_message::<BlockRequest>(&self.ids.block_request_id, i)
                {
                    if let Some(block) = self.blocks.get(req.block_id as usize) {
                        api.com_channel_send_message(&self.ids.block_response_id, block)
                    }
                }
            }
        }
    }
}

struct QueryState;

impl QueryState {
    fn poll(
        self,
        api: &(impl SceneApi + SceneApiExt),
        ids: &BlockTransferIds,
    ) -> BlockTransferState {
        api.com_channel_send_message(&ids.query_request_id, &QueryRequest { status: 1 });

        if let Some(response) =
            api.com_channel_last_message::<QueryResponse>(&ids.query_response_id)
        {
            if response.block_count < 102400 {
                return BlockTransferState::Transferring(TransferringState {
                    block_count: response.block_count,
                    next_block_to_request: 0,
                    blocks: vec![None; response.block_count as usize].into_boxed_slice(),
                });
            } else {
                log::warn!("Got unreasonable block count: {}", { response.block_count });
            }
        }

        BlockTransferState::Querying(self)
    }
}

struct TransferringState {
    block_count: u32,
    next_block_to_request: u32,
    blocks: Box<[Option<BlockResponse>]>,
}

impl TransferringState {
    fn poll(
        mut self,
        api: &(impl SceneApi + SceneApiExt),
        ids: &BlockTransferIds,
    ) -> BlockTransferState {
        {
            let response_count = api.com_channel_message_count(&ids.block_response_id);

            for i in 0..response_count {
                if let Some(response) =
                    api.com_channel_message::<BlockResponse>(&ids.block_response_id, i)
                {
                    if let Some(storage) = self.blocks.get_mut(response.block_id as usize) {
                        *storage = Some(response);
                    }
                }
            }
        }

        let remaining_blocks = self.blocks.iter().filter(|b| b.is_none()).count();

        if remaining_blocks == 0 {
            let mut data = Vec::new();
            for block in self.blocks.iter().filter_map(|b| b.as_ref()) {
                data.extend_from_slice(&block.data[..(block.data_len as usize)]);
            }
            return BlockTransferState::Done(DoneState {
                data: data.into_boxed_slice(),
            });
        }

        for _ in 0..remaining_blocks.min(5) {
            while self
                .blocks
                .get(self.next_block_to_request as usize)
                .unwrap()
                .is_some()
            {
                self.next_block_to_request = (self.next_block_to_request + 1) % self.block_count;
            }

            api.com_channel_send_message(
                &ids.block_request_id,
                &BlockRequest {
                    block_id: self.next_block_to_request,
                },
            );

            self.next_block_to_request = (self.next_block_to_request + 1) % self.block_count;
        }

        BlockTransferState::Transferring(self)
    }
}

struct DoneState {
    data: Box<[u8]>,
}

impl DoneState {
    fn poll(self, _api: &impl SceneApi) -> BlockTransferState {
        BlockTransferState::Done(self)
    }
}

enum BlockTransferState {
    Querying(QueryState),
    Transferring(TransferringState),
    Done(DoneState),
}

fn poll<'a>(
    state: &'a mut Option<BlockTransferState>,
    api: &(impl SceneApi + SceneApiExt),
    ids: &BlockTransferIds,
) -> Option<&'a [u8]> {
    *state = Some(match state.take().unwrap() {
        BlockTransferState::Querying(s) => s.poll(api, ids),
        BlockTransferState::Transferring(s) => s.poll(api, ids),
        BlockTransferState::Done(s) => s.poll(api),
    });

    if let Some(state) = state.as_ref() {
        match state {
            BlockTransferState::Done(done) => Some(&done.data),
            _ => None,
        }
    } else {
        None
    }
}

/// A service that allows a client to transfer a specific block of data from the server to the client.
pub struct BlockTransferClient {
    ids: BlockTransferIds,
    state: Option<BlockTransferState>,
}

impl BlockTransferClient {
    /// Creates a new block transfer client that fetches the data that the
    pub fn new(id: impl AsRef<str>) -> BlockTransferClient {
        let id = id.as_ref();
        BlockTransferClient {
            ids: BlockTransferIds::new(id),
            state: Some(BlockTransferState::Querying(QueryState)),
        }
    }

    /// Polls the block transfer client when it has received the block it will start returning the data
    pub fn poll(&mut self, api: &(impl SceneApi + SceneApiExt)) -> Option<&[u8]> {
        poll(&mut self.state, api, &self.ids)
    }
}
