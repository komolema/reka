use std::fmt;

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub hostname: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub database_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DataWarehouseConfig {
    pub hostname: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RabbitMQConfig {
    pub hostname: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub virtual_host: String,
}

impl fmt::Display for RabbitMQConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.hostname, self.port)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct WorkflowConfig {
    pub hostname: String,
    pub port: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RedisConfig {
    pub hostname: String,
    pub port: String,
    pub username: String,
    pub password: String,
}
