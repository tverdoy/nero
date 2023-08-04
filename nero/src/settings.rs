use once_cell::sync::OnceCell;
use serde::Serialize;

static SETTINGS: Settings = Settings::init();

pub struct Settings {
    pub server: OnceCell<ServerConf>,
    pub db: OnceCell<DataBaseConf>,
    pub cors: OnceCell<CorsConf>,
    pub admin_auth: OnceCell<AuthTokenConf>,
}

impl Settings {
    const fn init() -> Self {
        Self {
            server: OnceCell::new(),
            db: OnceCell::new(),
            cors: OnceCell::new(),
            admin_auth: OnceCell::new(),
        }
    }

    pub fn set_server(server: ServerConf) {
        SETTINGS.server.set(server).unwrap();
    }

    pub fn set_db(db: DataBaseConf) {
        SETTINGS.db.set(db).unwrap();
    }

    pub fn set_cors(cors: CorsConf) {
        SETTINGS.cors.set(cors).unwrap();
    }

    pub fn set_admin_auth(auth: AuthTokenConf) {
        SETTINGS.admin_auth.set(auth).unwrap();
    }

    pub fn server() -> &'static ServerConf {
        SETTINGS.server.get_or_init(ServerConf::default)
    }

    pub fn db() -> &'static DataBaseConf {
        SETTINGS.db.get_or_init(DataBaseConf::default)
    }

    pub fn cors() -> &'static CorsConf {
        SETTINGS.cors.get_or_init(CorsConf::default)
    }

    pub fn admin_auth() -> &'static AuthTokenConf {
        SETTINGS
            .admin_auth
            .get()
            .expect("Admin auth settings is not set")
    }
}

#[derive(Debug)]
pub struct AuthTokenConf {
    pub exr: u32,
    pub secret_key: Vec<u8>,
}

#[derive(Debug, Serialize)]
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
            connect: true,
            db_addr: "127.0.0.1:8000".to_string(),
            db_user: "root".to_string(),
            db_password: "root".to_string(),
            db_ns: "nero".to_string(),
            db_db: "nero".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CorsConf {
    pub is_allow_cors: bool,
    pub allow_origin: String,
    pub allow_headers: Vec<String>,
    pub allow_methods: Vec<String>,
}

impl Default for CorsConf {
    fn default() -> Self {
        Self {
            is_allow_cors: true,
            allow_origin: "*".to_string(),
            allow_headers: vec!["*".to_string()],
            allow_methods: vec!["GET".to_string(), "POST".to_string(), "OPTIONS".to_string()],
        }
    }
}

#[derive(Debug, Serialize)]
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
