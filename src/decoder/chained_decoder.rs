use bytes::BytesMut;
use tokio_util::codec::Decoder;

pub struct ChainedDecoder<D1, D2> {
    first: D1,
    second: D2,
    intermediate_buffer: BytesMut,
}

impl<D1, D2> ChainedDecoder<D1, D2> {
    pub fn new(first: D1, second: D2) -> Self {
        Self {
            first,
            second,
            intermediate_buffer: BytesMut::new(),
        }
    }
}

impl<D1, D2> Decoder for ChainedDecoder<D1, D2>
where
    D1: Decoder<Item = BytesMut>,
    D2: Decoder,
{
    type Item = D2::Item;
    type Error = Error<D1, D2>;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // First, check if we can decode using our buffer
        if let Some(result) = self
            .second
            .decode(&mut self.intermediate_buffer)
            .map_err(Error::D2)?
        {
            return Ok(Some(result));
        }

        // Then, try to load more data into the second decoder's buffer using the first decoder
        if let Some(intermediate) = self.first.decode(src).map_err(Error::D1)? {
            // Add the intermediate result to our buffer
            self.intermediate_buffer.extend_from_slice(&intermediate);
        }

        // Now retry to decode using the second decoder
        self.second
            .decode(&mut self.intermediate_buffer)
            .map_err(Error::D2)
    }
}

#[derive(Debug)]
pub enum Error<D1, D2>
where
    D1: Decoder,
    D2: Decoder,
{
    Io(std::io::Error),
    D1(D1::Error),
    D2(D2::Error),
}

impl<D1, D2> From<std::io::Error> for Error<D1, D2>
where
    D1: Decoder,
    D2: Decoder,
{
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
