
pub struct Configuration {
    pub host: String,
    pub port: u16,
    pub path: String,
    pub index_html: String,
    pub debug: bool
}

impl Configuration {
    fn new(host: String, port: u16, path: String, index_html: String, debug: bool) -> Configuration {
        Configuration {
            host,
            port,
            path,
            index_html,
            debug
        }
    }
}