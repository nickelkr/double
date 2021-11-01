use std::path::PathBuf;
use std::fs;

pub struct Endpoint {
    pub payload: String,
}

impl Endpoint {
    pub fn from_file(path: PathBuf) -> Result<Endpoint, std::io::Error>  {
        let payload = match fs::read_to_string(path) {
            Ok(payload) => payload,
            Err(e) => return Err(e),
        };

        Ok(Endpoint { payload })
    }
}
