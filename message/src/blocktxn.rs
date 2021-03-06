// Copyright 2019 Conflux Foundation. All rights reserved.
// Conflux is free software and distributed under GNU General Public License.
// See http://www.gnu.org/licenses/

use crate::{Message, MsgId, RequestId};
use cfx_types::H256;
use primitives::TransactionWithSignature;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Default)]
pub struct GetBlockTxnResponce {
    pub request_id: RequestId,
    pub block_hash: H256,
    pub block_txn: Vec<TransactionWithSignature>,
}

impl Message for GetBlockTxnResponce {
    fn msg_id(&self) -> MsgId { MsgId::GET_BLOCK_TXN_RESPONSE }

    fn is_size_sensitive(&self) -> bool { self.block_txn.len() > 1 }
}

impl Deref for GetBlockTxnResponce {
    type Target = RequestId;

    fn deref(&self) -> &Self::Target { &self.request_id }
}

impl DerefMut for GetBlockTxnResponce {
    fn deref_mut(&mut self) -> &mut RequestId { &mut self.request_id }
}

impl Encodable for GetBlockTxnResponce {
    fn rlp_append(&self, stream: &mut RlpStream) {
        stream
            .begin_list(3)
            .append(&self.request_id)
            .append(&self.block_hash)
            .append_list(&self.block_txn);
    }
}

impl Decodable for GetBlockTxnResponce {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(GetBlockTxnResponce {
            request_id: rlp.val_at(0)?,
            block_hash: rlp.val_at(1)?,
            block_txn: rlp.list_at(2)?,
        })
    }
}
