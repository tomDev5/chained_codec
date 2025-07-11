use bytes::BytesMut;
use tokio_util::codec::Decoder;

use crate::decoder::ChainedDecoder;

pub trait DecoderExt: Decoder<Item = BytesMut> {
    fn chain_decoder<D>(self, other: D) -> ChainedDecoder<Self, D>
    where
        Self: Sized,
        D: Decoder,
    {
        ChainedDecoder::new(self, other)
    }
}

impl<D> DecoderExt for D
where
    D: Decoder<Item = BytesMut>,
{
    fn chain_decoder<D2>(self, other: D2) -> ChainedDecoder<Self, D2>
    where
        Self: Sized,
        D2: Decoder,
    {
        ChainedDecoder::new(self, other)
    }
}
