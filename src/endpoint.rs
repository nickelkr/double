use std::fs;
use std::path::PathBuf;

pub struct Endpoint {
    pub path: String,
    pub payload: String,
}

impl Endpoint {
    pub fn from_file(path: String, payload_path: PathBuf) -> Result<Endpoint, std::io::Error> {
        let payload = match fs::read_to_string(payload_path) {
            Ok(payload) => payload,
            Err(e) => return Err(e),
        };

        Ok(Endpoint { path, payload })
    }
}
