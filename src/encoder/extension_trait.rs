use bytes::BytesMut;
use tokio_util::codec::Encoder;

use crate::encoder::ChainedEncoder;

pub trait EncoderExt<E>: Encoder<E> {
    fn chain_encoder<D>(self, other: D) -> ChainedEncoder<Self, D>
    where
        Self: Sized,
        D: Encoder<BytesMut>,
    {
        ChainedEncoder::new(self, other)
    }
}

impl<D, E> EncoderExt<E> for D
where
    D: Encoder<E>,
{
    fn chain_encoder<D2>(self, other: D2) -> ChainedEncoder<Self, D2>
    where
        Self: Sized,
        D2: Encoder<BytesMut>,
    {
        ChainedEncoder::new(self, other)
    }
}
