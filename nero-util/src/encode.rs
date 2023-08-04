use deflate::deflate_bytes;

#[derive(Debug, PartialEq, Clone)]
pub enum EncodeAlgo {
    Gzip,
    Deflate,
    Other(String),
}

impl EncodeAlgo {
    //noinspection RsLiveness
    pub fn encode(&self, data: &[u8]) -> Vec<u8> {
        match self {
            EncodeAlgo::Gzip => todo!(),
            EncodeAlgo::Deflate => deflate_bytes(data),
            EncodeAlgo::Other(algo) => panic!("Nero dont support {algo}"),
        }
    }

    pub fn parse_from_string<T: ToString>(string: T) -> Self {
        match string.to_string().as_str() {
            "gzip" => Self::Gzip,
            "deflate" => Self::Deflate,
            _ => Self::Other(string.to_string()),
        }
    }

    pub fn format_to_string(&self) -> String {
        match &self {
            Self::Gzip => "gzip",
            Self::Deflate => "deflate",
            Self::Other(algo) => algo,
        }
            .to_string()
    }
}
