#[derive(Debug)]
pub struct AuthTokenConf {
    pub exr: u32,
    pub secret_key: Vec<u8>,
}

#[derive(Debug)]
pub struct DataBaseConf {
    pub connect: bool,
    pub db_addr: String,
    pub db_user: String,
    pub db_password: String,
    pub db_ns: String,
    pub db_db: String,
}

impl Default for DataBaseConf {
    fn default() -> Self {
        Self {
            connect: false,
            db_addr: "127.0.0.1:8000".to_string(),
            db_user: "root".to_string(),
            db_password: "root".to_string(),
            db_ns: "nero".to_string(),
            db_db: "nero".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct CorsConf {
    pub is_allow_cors: bool,
    pub allow_origin: String,
    pub allow_headers: Vec<String>,
    pub allow_methods: Vec<String>,
}

impl Default for CorsConf {
    fn default() -> Self {
        Self {
            is_allow_cors: false,
            allow_origin: "*".to_string(),
            allow_headers: vec!["*".to_string()],
            allow_methods: vec!["GET".to_string(), "POST".to_string(), "OPTIONS".to_string()],
        }
    }
}

#[derive(Debug)]
pub struct ServerConf {
    pub addr: String,
    pub max_head_size: usize,
    pub max_body_size: usize,
}

impl Default for ServerConf {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:8080".to_string(),
            max_head_size: 4096,      // 4 KB
            max_body_size: 4_194_304, // 4 MB
        }
    }
}
