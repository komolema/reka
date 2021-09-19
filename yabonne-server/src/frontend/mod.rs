use crate::{
    frontend::api::ApiController,
    shared::{
        cq::CQMessage,
        util::{config::RedisConfig, rabbitmq::RabbitMQContext, redis::RedisRepositoryImpl},
    },
};
use crossbeam_channel::{unbounded, Receiver};
use futures::future::join_all;
use std::sync::Arc;
use std::ops::Deref;

pub mod api;
pub mod domains;

pub struct FrontEndController {
    front_end_receiver_channel: Receiver<CQMessage>,
}

impl FrontEndController {
    pub fn new(front_end_receiver_channel: Receiver<CQMessage>) -> FrontEndController {
        FrontEndController {
            front_end_receiver_channel,
        }
    }

    pub async fn start(
        &self,
        rabbitmq_context: RabbitMQContext,
        redis_config: RedisConfig,
    ) {
        info!("Starting FrontEndController");

        let (api_manager_sending_channel, api_manager_receiving_channel) = unbounded();
        let redis_repository = RedisRepositoryImpl::new(redis_config);

        let api_controller = Arc::new(ApiController::new(
            api_manager_receiving_channel,
            rabbitmq_context,
            Arc::new(redis_repository),
        ));
        let api_blocking_handle = tokio::spawn(async move {
            api_controller.deref().start().await;
        });

        join_all(vec![api_blocking_handle]).await;
    }
}
