/// A request from the client to the server
#[derive(Debug, PartialEq)]
pub enum Request {
    /// Add the document `doc` to the archive
    Publish { doc: String },
    /// Search for the word `word` in the archive
    Search { word: String },
    /// Retrieve the document with the index `id` from the archive
    Retrieve { id: usize },
}
impl Request {
    // Convert the request `self` into a byte vector. 
    // One byte tag at beginning encodes which of the three requests is sent
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            // To publish, encode tag of 1, length of input doc, and then input doc
            Request::Publish { doc } => {
                bytes.push(1 as u8); 
                let len = doc.len() as usize; 
                bytes.extend(len.to_be_bytes().iter());
                bytes.extend(doc.as_bytes().iter())
            }
            // To search, encode tag of 2, length of query word, and then query word
            Request::Search { word } => {
                bytes.push(2 as u8); 
                let len = word.len() as usize;
                bytes.extend(len.to_be_bytes().iter());
                bytes.extend(word.as_bytes().iter())
            }
            // To retrieve, encode tag of 3 and id
            Request::Retrieve { id } => {
                bytes.push(3 as u8);
                bytes.extend(id.to_be_bytes().iter());
            }
        }
        bytes
    }
    // Read a request from `reader` and return it. Calling `to_bytes` from above and then calling
    // `from_bytes` should return the original request. If the request is invalid, return `None`.
    // Convert back using convention set above
    pub fn from_bytes<R: std::io::Read>(mut reader: R) -> Option<Self> {
        let mut tag_buffer = [0 as u8; 1];
        reader.read_exact(&mut tag_buffer).ok()?;
        let tag = tag_buffer[0];
        match tag {
            1 => {
                let mut len_buffer = [0 as u8; std::mem::size_of::<usize>()];
                reader.read_exact(&mut len_buffer).ok()?;
                let len = u64::from_be_bytes(len_buffer) as usize;
                let mut doc_buffer = vec![0 as u8; len];
                reader.read_exact(&mut doc_buffer).ok()?;
                let doc = String::from_utf8(doc_buffer).ok()?;
                return Some (Request::Publish { doc })
            }
            2 => {
                let mut len_buffer = [0 as u8; std::mem::size_of::<usize>()];
                reader.read_exact(&mut len_buffer).ok()?;
                let len = u64::from_be_bytes(len_buffer) as usize;
                let mut word_buffer = vec![0 as u8; len];
                reader.read_exact(&mut word_buffer).ok()?;
                let word = String::from_utf8(word_buffer).ok()?;
                return Some (Request::Search { word })
            }
            3 => {
                let mut id_buffer = [0 as u8; std::mem::size_of::<usize>()];
                reader.read_exact(&mut id_buffer).ok()?;
                let id = u64::from_be_bytes(id_buffer) as usize;
                return Some (Request::Retrieve { id })
            }
            // If doesn't matc any of the tags, return none for invalid request
            _ => return None,
        }
    }
}

/// A response from the server to the client
#[derive(Debug, PartialEq)]
pub enum Response {
    /// The document was successfully added to the archive with the given index
    PublishSuccess(usize),
    /// The search for the word was successful, and the indices of the documents containing the
    /// word are returned
    SearchSuccess(Vec<usize>),
    /// The retrieval of the document was successful, and the document is returned
    RetrieveSuccess(String),
    /// The request failed
    Failure,
}
impl Response {
    // Convert the request `self` into a byte vector. 
    // One byte tag at beginning encodes which of the three requests is sent
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            Response::PublishSuccess(index) => {
                bytes.push(1 as u8);
                bytes.extend(index.to_be_bytes().iter());
            }
            Response::SearchSuccess(indices) => {
                bytes.push(2 as u8);
                let len = indices.len() as usize;
                bytes.extend(len.to_be_bytes().iter());
                for index in indices {
                    bytes.extend(index.to_be_bytes().iter());
                }
            }
            Response::RetrieveSuccess(doc) => {
                bytes.push(3 as u8);
                let len = doc.len() as usize;
                bytes.extend(len.to_be_bytes().iter());
                bytes.extend(doc.as_bytes().iter())
            }
            Response::Failure => {
                bytes.push(4 as u8);
            }
        }
        bytes
    }

    // Read a request from `reader` and return it. Calling `to_bytes` from above and then calling
    // `from_bytes` should return the original request. If the request is invalid, return `None`.
    pub fn from_bytes<R: std::io::Read>(mut reader: R) -> Option<Self> {
        let mut tag_buffer = [0 as u8; 1];
        reader.read_exact(&mut tag_buffer).ok()?; //should not panic here
        let tag = tag_buffer[0];
        match tag {
            // For publish response, encode tag of 1 and index of newly published doc
            1 => {
                let mut id_buffer = [0 as u8; std::mem::size_of::<usize>()];
                reader.read_exact(&mut id_buffer).unwrap();
                let id = u64::from_be_bytes(id_buffer) as usize;
                return Some (Response::PublishSuccess(id))
            }
            // For search response, encode tag of 2 and index of docs that contain word
            2 => {
                let mut len_buffer = [0 as u8; std::mem::size_of::<usize>()];
                reader.read_exact(&mut len_buffer).unwrap();
                let len = u64::from_be_bytes(len_buffer) as usize;
                let mut indices = Vec::with_capacity(len);
                for _ in 0..len {
                    let mut index_buffer = [0 as u8; std::mem::size_of::<usize>()];
                    reader.read_exact(&mut index_buffer).unwrap();
                    let index = u64::from_be_bytes(index_buffer) as usize;
                    indices.push(index);
                }
                return Some (Response::SearchSuccess(indices))
            }
            // For retrieve response, encode tag of 3, length of docm and then output doc
            3 => {
                let mut len_buffer = [0 as u8; std::mem::size_of::<usize>()];
                reader.read_exact(&mut len_buffer).unwrap();
                let len = u64::from_be_bytes(len_buffer) as usize;
                let mut doc_buffer = vec![0 as u8; len];
                reader.read_exact(&mut doc_buffer).unwrap();
                let doc = String::from_utf8(doc_buffer).ok()?;
                return Some (Response::RetrieveSuccess(doc))
            }
            4 => {
                return Some (Response::Failure)
            }
            _ => return None,
        }
    }
}

