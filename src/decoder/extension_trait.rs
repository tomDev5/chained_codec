use bytes::BytesMut;
use tokio_util::codec::Decoder;

use crate::decoder::ChainedDecoder;

pub trait DecoderExt: Decoder<Item = BytesMut> {
    fn chain<D>(self, other: D) -> ChainedDecoder<Self, D>
    where
        Self: Sized,
        D: Decoder,
    {
        ChainedDecoder::new(self, other)
    }
}
