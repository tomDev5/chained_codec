use bytes::BytesMut;
use tokio_util::codec::Encoder;

pub struct ChainedEncoder<D1, D2> {
    first: D1,
    second: D2,
}

impl<D1, D2> ChainedEncoder<D1, D2> {
    pub fn new(first: D1, second: D2) -> Self {
        Self { first, second }
    }
}

impl<D1, D2, Item> Encoder<Item> for ChainedEncoder<D1, D2>
where
    D1: Encoder<Item>,
    D2: Encoder<BytesMut>,
{
    type Error = Error<D1, D2, Item>;

    fn encode(&mut self, item: Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let mut intermediate = BytesMut::new();

        // First, try to encode using our first encoder
        self.first
            .encode(item, &mut intermediate)
            .map_err(Error::D1)?;

        // Then, try to encode using our second encoder
        self.second.encode(intermediate, dst).map_err(Error::D2)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum Error<D1, D2, Item>
where
    D1: Encoder<Item>,
    D2: Encoder<BytesMut>,
{
    Io(std::io::Error),
    D1(D1::Error),
    D2(D2::Error),
}

impl<D1, D2, Item> From<std::io::Error> for Error<D1, D2, Item>
where
    D1: Encoder<Item>,
    D2: Encoder<BytesMut>,
{
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
