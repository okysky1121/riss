pub struct Chunk {
    fourcc: [u8; 4],
    size: (u32, bool),
    data: Vec<u8>,
}

impl Chunk {
    pub fn fourcc(&self) -> &[u8; 4] {
        &self.fourcc
    }

    pub fn size(&self) -> &(u32, bool) {
        &self.size
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidHeaderSize,
    InvalidDataSize,
}

impl std::error::Error for ParseError {}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidHeaderSize => "expected at least 8",
            ParseError::InvalidDataSize => "chunk data is shorter than the declared size",
        }
        .fmt(f)
    }
}

pub struct Parser;

impl Parser {
    fn parse_chunk(data: &[u8]) -> Result<Chunk, ParseError> {
        if data.len() < 8 {
            return Err(ParseError::InvalidHeaderSize);
        }

        let mut fourcc = [0; 4];
        let mut size = 0u32;
        for i in 0..4 {
            fourcc[i] = data[i];
            size |= (data[i + 4] as u32) << (8 * i); // i know `u32::from_le_bytes` but i don't want to use `.unwrap`, `.expect` or ... for cast &[u8] to [u8; const]
        }

        let size = (size, (size & 1) == 1);
        let real_size = size.0 as usize + usize::from(size.1);

        if data.len() - 8 < real_size {
            return Err(ParseError::InvalidDataSize);
        }

        let data = Vec::from(&data[8..8 + size.0 as usize]); // without padding

        Ok(Chunk { fourcc, size, data })
    }

    pub fn parse(mut data: &[u8]) -> Result<Vec<Chunk>, ParseError> {
        let mut chunks = vec![];

        while !data.is_empty() {
            let chunk = Self::parse_chunk(data)?;
            let (size, padding) = chunk.size();
            let size = 8 + *size as usize + usize::from(*padding);
            data = &data[size..];
            chunks.push(chunk);
        }

        Ok(chunks)
    }
}
