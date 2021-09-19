use crate::shared::util::config::RabbitMQConfig;
use futures::{executor::block_on};
use lapin::{ConnectionProperties, Error};
use std::sync::Arc;

#[derive(Clone)]
pub struct RabbitMQContext {
    rabbitmq_config_botho: RabbitMQConfig,
    rabbitmq_config_motsamai: RabbitMQConfig,
    pub botho_rmq_connection: Arc<lapin::Connection>,
    pub motsamaisi_rmq_connection: Arc<lapin::Connection>,
}

impl RabbitMQContext {
    pub async fn new(
        rabbitmq_config_frontend: RabbitMQConfig,
        rabbitmq_config_backend: RabbitMQConfig,
    ) -> Option<RabbitMQContext> {
        let botho_r_result =
            block_on(create_rmq_connection(rabbitmq_config_frontend.clone()));
        match botho_r_result {
            Ok(botho_rmq_connection) => {
                let motsamaisi_r_result =
                    block_on(create_rmq_connection(rabbitmq_config_backend.clone()));
                match motsamaisi_r_result {
                    Ok(motsamaisi_rmq_connection) => {
                        let rmq_context = RabbitMQContext {
                            rabbitmq_config_botho: rabbitmq_config_frontend,
                            rabbitmq_config_motsamai: rabbitmq_config_backend,
                            botho_rmq_connection: Arc::new(
                                botho_rmq_connection),
                            motsamaisi_rmq_connection: Arc::new(
                                motsamaisi_rmq_connection),
                        };

                        Some(rmq_context)
                    }
                    Err(error) => {
                        info!("Error occurred while trying to connect to motsamaisi rabbitmq virtual host: {}", error);
                        None
                    }
                }
            }
            Err(error) => {
                info!(
                    "Error occurred while trying to connect to frontend rabbitmq virtual host: {}",
                    error
                );
                None
            }
        }
    }
}

pub async fn create_rmq_connection(
    rmq_server_config: RabbitMQConfig,
) -> Result<lapin::Connection, Error> {
    let rmq_server_addr =
        generate_rabbit_mq_addr_based_on_config(rmq_server_config.clone());

    lapin::Connection::connect(rmq_server_addr.as_str(), ConnectionProperties::default())
        .await
        .map(|f| f)
}

fn generate_rabbit_mq_addr_based_on_config(rabbitmq_config: RabbitMQConfig) -> String {
    format!(
        "amqp://{}:{}@{}:{}/{}",
        rabbitmq_config.username,
        rabbitmq_config.password,
        rabbitmq_config.hostname,
        rabbitmq_config.port,
        rabbitmq_config.virtual_host
    )
}
