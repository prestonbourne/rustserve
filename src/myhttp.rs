pub const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";
pub const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

pub struct HTTPRequest {
    method: HTTPMethod,
    pub path: String,
}

impl HTTPRequest {
    pub fn new(method: HTTPMethod, path: String) -> Self {
        Self { method, path }
    }
}

#[derive(Debug)]
pub enum HTTPMethod {
    GET,
    POST,
    INVALID,
}

impl std::fmt::Display for HTTPMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            HTTPMethod::GET => write!(f, "GET"),
            HTTPMethod::POST => write!(f, "POST"),
            HTTPMethod::INVALID => write!(f, "INVALID"),
        }
    }
}
