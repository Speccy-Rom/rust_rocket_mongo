#![allow(unused_must_use)]
use crate::{error::Error, secure::cert::generate_cert};
use serde::{de, Deserialize, Deserializer};
use std::{collections::HashMap, fs::File, io::Read, net::IpAddr, path::Path};

const SRV_ADDR: &str = "127.0.0.1";
const SRV_PORT: usize = 8080;
const SRV_KEEP_ALIVE: usize = 60;
const SRV_FORMS_LIMIT: usize = 1024 * 256;
const SRV_JSON_LIMIT: usize = 1024 * 256;
const SRV_SECRET_KEY: &str = "t/xZkYvxfC8CSfTSH9ANiIR9t1SvLHqOYZ7vH4fp11s=";

const SSL_ENABLED: bool = false;
const SSL_GENERATE_SELF_SIGNED: bool = true;
const SSL_KEY_FILE: &str = "./private/key";
const SSL_CERT_FILE: &str = "./private/cert";

const MONGO_HOST: &str = "127.0.0.1";
const MONGO_PORT: usize = 27017;
const MONGO_USER: &str = "";
const MONGO_PASS: &str = "";

/// Rocket API Server parameters
#[derive(Deserialize, Clone, Debug, Default)]
pub struct Settings {
    /// Server config related parameters
    #[serde(default)]
    pub server: ServerConfig,

    /// SSL Configuration
    #[serde(default, deserialize_with = "configure_ssl")]
    pub ssl: Option<SslConfig>,

    /// Mongo DB configuration
    #[serde(default, deserialize_with = "configure_mongodb")]
    pub mongo_db: MongoDbConfig,
}

impl Settings {
    pub fn from_file<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        //! Read configuration settings from yaml file
        //!
        //! ## Example usage
        //! ```ignore
        //! Settings::from_file("config.yml");
        //! ```
        //!
        let mut cfg = config::Config::default();
        cfg.merge(config::File::with_name(path.as_ref().to_str().unwrap()))?;
        match cfg.try_into() {
            Ok(c) => Ok(c),
            Err(e) => {
                println!("err: {:?}", e);
                Err(Error::ConfigurationError)
            }
        }
    }
}

/// Rocket Server params
#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    /// Server Ip Address to start Rocket API Server
    #[serde(default = "default_server_host")]
    pub host: IpAddr,
    /// Server port to listen Rocket API Server
    #[serde(default = "default_server_port")]
    pub port: usize,
    /// Server Keep Alive
    #[serde(default = "default_server_keep_alive")]
    pub keep_alive: usize,
    /// Forms limitation
    #[serde(default = "default_server_forms_limit")]
    pub forms_limit: usize,
    /// JSON transfer limitation
    #[serde(default = "default_server_json_limit")]
    pub json_limit: usize,
    /// Api Server Secret key
    #[serde(default = "default_server_secret_key")]
    pub secret_key: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: SRV_ADDR.parse().unwrap(),
            port: SRV_PORT,
            keep_alive: SRV_KEEP_ALIVE,
            forms_limit: SRV_FORMS_LIMIT,
            json_limit: SRV_JSON_LIMIT,
            secret_key: SRV_SECRET_KEY.into(),
        }
    }
}

/// Server SSL params
#[derive(Deserialize, Clone, Debug)]
pub struct SslConfig {
    /// Enabled: yes/no
    #[serde(default = "default_ssl_enabled")]
    pub enabled: bool,
    /// Let the server generate a self-signed pair: yes/no
    #[serde(default = "default_ssl_self_signed")]
    generate_self_signed: bool,
    /// key file (if generate_self_signed is `NO`)
    #[serde(default = "default_ssl_key_file")]
    key_file: String,
    /// certificate pem file (if generate_self_signed is `NO`)
    #[serde(default = "default_ssl_cert_file")]
    cert_file: String,

    // Not to be included in config file
    // hidden and for use with rocket app
    pub pem_certificate: Option<Vec<u8>>,
    pub pem_private_key: Option<Vec<u8>>,
}

impl Default for SslConfig {
    fn default() -> Self {
        Self {
            enabled: SSL_ENABLED,
            generate_self_signed: SSL_GENERATE_SELF_SIGNED,
            key_file: SSL_KEY_FILE.into(),
            cert_file: SSL_CERT_FILE.into(),
            pem_certificate: None,
            pem_private_key: None,
        }
    }
}

/// Mongo Databse parameters
#[derive(Deserialize, Clone, Debug)]
pub struct MongoDbConfig {
    #[serde(default = "default_mongo_host")]
    pub mongo_host: String,
    #[serde(default = "default_mongo_port")]
    pub mongo_port: usize,
    #[serde(default = "default_mongo_user")]
    pub auth_user: String,
    #[serde(default = "default_mongo_pass")]
    pub auth_pass: String,
    #[serde(default = "default_mongo_db")]
    pub db: HashMap<String, String>,

    // hidden setting to deserialize from host & port
    pub db_uri: Option<String>,
}

impl Default for MongoDbConfig {
    fn default() -> Self {
        Self {
            mongo_host: MONGO_HOST.into(),
            mongo_port: MONGO_PORT,
            auth_user: MONGO_USER.into(),
            auth_pass: MONGO_PASS.into(),
            db: HashMap::default(),
            db_uri: None,
        }
    }
}

// All Server defaults
fn default_server_host() -> IpAddr {
    SRV_ADDR.parse().unwrap()
}

fn default_server_port() -> usize {
    SRV_PORT
}

fn default_server_keep_alive() -> usize {
    SRV_KEEP_ALIVE
}

fn default_server_forms_limit() -> usize {
    SRV_FORMS_LIMIT
}

fn default_server_json_limit() -> usize {
    SRV_JSON_LIMIT
}

fn default_server_secret_key() -> String {
    SRV_SECRET_KEY.into()
}

// All SSL config defaults
fn default_ssl_enabled() -> bool {
    SSL_ENABLED
}

fn default_ssl_self_signed() -> bool {
    SSL_GENERATE_SELF_SIGNED
}

fn default_ssl_key_file() -> String {
    SSL_KEY_FILE.into()
}

fn default_ssl_cert_file() -> String {
    SSL_CERT_FILE.into()
}

// All Mongo Databse defaults
fn default_mongo_host() -> String {
    MONGO_HOST.into()
}

fn default_mongo_port() -> usize {
    MONGO_PORT
}

fn default_mongo_user() -> String {
    MONGO_USER.into()
}

fn default_mongo_pass() -> String {
    MONGO_PASS.into()
}

fn default_mongo_db() -> HashMap<String, String> {
    HashMap::default()
}

/// SSL configuration deserializer
fn configure_ssl<'de, D>(deserializer: D) -> Result<Option<SslConfig>, D::Error>
where
    D: Deserializer<'de>,
{
    let ssl_config: Option<SslConfig> = Option::deserialize(deserializer)?;
    match ssl_config {
        Some(mut s) => {
            if s.enabled && s.generate_self_signed {
                // SSL is enabled, and generate self signed certificate is enabled
                let certs = generate_cert();
                s.pem_certificate = Some(certs.x509_certificate.to_pem().unwrap());
                s.pem_private_key = Some(certs.private_key.private_key_to_pem_pkcs8().unwrap());
            } else if s.enabled && !s.generate_self_signed {
                // SSL is enabled, and generate self signed certificate is disabled
                if s.key_file.is_empty() || s.cert_file.is_empty() {
                    return Err(de::Error::custom("key_file and/or cert_file is empty"));
                } else if !Path::new(&s.key_file).is_file() || !Path::new(&s.cert_file).is_file() {
                    return Err(de::Error::custom("key_file and/or cert_file not available"));
                } else {
                    // read key
                    let mut key = Vec::new();
                    {
                        let mut kf = File::open(&s.key_file).unwrap();
                        kf.read_to_end(&mut key);
                    }
                    // read certificate
                    let mut cert = Vec::new();
                    {
                        let mut cf = File::open(&s.cert_file).unwrap();
                        cf.read_to_end(&mut cert);
                    }
                    s.pem_certificate = Some(cert);
                    s.pem_private_key = Some(key);
                }
            }
            Ok(Some(s))
        }
        None => Ok(None),
    }
}

/// Mongo DB configuration deserializer
fn configure_mongodb<'de, D>(deserializer: D) -> Result<MongoDbConfig, D::Error>
where
    D: Deserializer<'de>,
{
    let mut mongodb_config = MongoDbConfig::deserialize(deserializer)?;
    if mongodb_config.mongo_host.is_empty() {
        return Err(de::Error::custom("Database host not configured"));
    }
    if mongodb_config.auth_user.is_empty() || mongodb_config.auth_pass.is_empty() {
        mongodb_config.db_uri = Some(format!(
            "mongodb://{}:{}",
            mongodb_config.mongo_host, mongodb_config.mongo_port
        ))
    } else {
        mongodb_config.db_uri = Some(format!(
            "mongodb://{}:{}@{}:{}",
            mongodb_config.auth_user,
            mongodb_config.auth_pass,
            mongodb_config.mongo_host,
            mongodb_config.mongo_port
        ))
    };

    Ok(mongodb_config)
}