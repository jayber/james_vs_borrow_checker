// the struct
#[derive(Debug)]
struct Png {
    chunks: Vec<u8>,
}
/// the implementation
impl TryFrom<&[u8]> for Png {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut bytes = value.iter().peekable();
        let header: Vec<_> = bytes.by_ref().take(8).copied().collect();
        // if header.as_slice() != Self::STANDARD_HEADER {
        //     return Err("Error not PNG".into());
        // }
        let mut chunks = vec![];
        loop {
            if bytes.peek().is_none() {
                break;
            }
            let b = bytes.by_ref();
            let len_b: Vec<_> = b.take(4).copied().collect();
            let len = u32::from_be_bytes(to_xbyte_arr(&len_b));
            let chunk_b: Vec<_> = len_b
                .into_iter()
                .chain(b.take(4 + 4 + len as usize).cloned()) // type + crm + data
                .collect();
            chunks.push(Chunk::try_from(chunk_b.as_ref())?);
        }
        Ok(Self::from_chunks(chunks))
    }
}

fn to_xbyte_arr<const C: usize>(s: &[u8]) -> [u8; C] {
    let mut arr = [0u8; C];
    for i in 0..C {
        arr[i] = s[i]
    }
    arr
}

fn main() {}
