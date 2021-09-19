#[macro_use]
extern crate log;
extern crate config;
#[macro_use]
extern crate enum_map;
#[macro_use]
extern crate crossbeam_channel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

pub mod api;
pub mod domains;
pub mod shared;

use crate::shared::{
    domain::backend::DomainName as BackEndDomainName,
    util::config::{DatabaseConfig, RabbitMQConfig, RedisConfig, WorkflowConfig},
};
use config::ConfigError;
use enum_map::EnumMap;

pub struct FiveConfig {
    pub backend_rabbitmq_config: RabbitMQConfig,
    pub frontend_rabbitmq_config: RabbitMQConfig,
    pub redis_config: RedisConfig,
    pub domain_database_config: EnumMap<BackEndDomainName, DatabaseConfig>,
    pub camunda_workflow_config: WorkflowConfig,
}

impl FiveConfig {
    pub fn new(settings_file: &str) -> Result<Self, ConfigError> {
        let mut settings = config::Config::new();
        settings.merge(config::File::with_name(settings_file));

        let backend_rabbitmq_config = settings
            .get::<RabbitMQConfig>("backend_rabbitmq_config")
            .unwrap();
        let frontend_rabbitmq_config = settings
            .get::<RabbitMQConfig>("frontend_rabbitmq_config")
            .unwrap();
        let redis_config = settings.get::<RedisConfig>("redis_config").unwrap();
        let camunda_workflow_config = settings
            .get::<WorkflowConfig>("camunda_workflow_config")
            .unwrap();
        let account_database_config = settings
            .get::<DatabaseConfig>("account_database_config")
            .unwrap();
        let courier_database_config = settings
            .get::<DatabaseConfig>("courier_database_config")
            .unwrap();
        let workflows_database_config = settings
            .get::<DatabaseConfig>("workflows_database_config")
            .unwrap();

        let mut domain_database_config = enum_map! {
            BackEndDomainName::ACCOUNTS => account_database_config.clone(),
            BackEndDomainName::COURIER => courier_database_config.clone(),
            BackEndDomainName::WORKFLOWS => workflows_database_config.clone(),
        };

        Ok(FiveConfig {
            backend_rabbitmq_config,
            frontend_rabbitmq_config,
            redis_config,
            domain_database_config,
            camunda_workflow_config,
        })
    }
}
