use crate::{
    rocket::config::Config,
    shared::{
        cq::CQMessage,
        util::{rabbitmq::RabbitMQContext},
    },
};
use crossbeam_channel::{Receiver, unbounded, Sender};
use crate::shared::util::redis::RedisRepository;
use std::sync::Arc;
use lapin::BasicProperties;
use lapin::options::BasicPublishOptions;
use futures::future::join_all;
use crate::shared::cq::{OriginSystem, DomainInfo, TraceInfo, CQHeader, CQMessageType};
use crate::shared::domain::DomainName;
use uuid::Uuid;
use chrono::Utc;
use crate::shared::domain::backend::DomainName as BackEndDomainName;
use crate::shared::domain::DomainName::BackEnd;
use crate::domains as domains;



pub struct ApiController {
    api_controller_receiving_channel: Receiver<CQMessage>,
    rabbitmq_context: RabbitMQContext,
    redis_repository: Arc<dyn RedisRepository>,
}

impl ApiController {
    pub fn new(
        api_controller_receiving_channel: Receiver<CQMessage>,
        rabbitmq_context: RabbitMQContext,
        redis_repository: Arc<dyn RedisRepository>,
    ) -> ApiController {
        ApiController {
            api_controller_receiving_channel,
            rabbitmq_context,
            redis_repository,
        }
    }

    pub async fn start(&self) {
        info!("Starting ApiController");
        let (producer_sending_channel, producer_receiving_channel) = unbounded();

        let rocket = self.start_rocket(producer_sending_channel);
        let api_handle = tokio::spawn(async move {
            if let Err(e) = rocket.launch().await {
                info!("Whoops Rocket did not launch");
                info!("Error: {:?}", e);
            }
        });

        let producer_rabbitmq_channel = self.rabbitmq_context.motsamaisi_rmq_connection.create_channel().await.unwrap();
        let producer_handle = tokio::spawn(async move {
            loop {
                select! {

                recv(producer_receiving_channel) -> msg => {
                    let prc_clone = producer_rabbitmq_channel.clone();
                    ApiController::publish_message(prc_clone, msg.unwrap()).await;
                },
                default => {},
               }
            }
        });

        join_all(vec![api_handle, producer_handle]).await;
    }

    async fn publish_message(producer_rabbitmq_channel: lapin::Channel, message: CQMessage) {
        let exchange = back_end_domain_name(message.header.domain_info.domain.clone());
        let routing_key = message.header.domain_info.routing_key.clone();
        let payload = serde_json::to_string(&message).unwrap();
        producer_rabbitmq_channel.basic_publish(exchange.as_ref().to_lowercase().as_str(),
                                                routing_key.as_str(),
                                                BasicPublishOptions::default(),
                                                payload.into_bytes(),
                                                BasicProperties::default()).await;
    }

    fn start_rocket(&self, producer_sending_channel: Sender<CQMessage>) -> rocket::Rocket {
        let config = Config::debug_default();

        rocket::custom(config)
            .manage(self.redis_repository.clone())
            .manage(producer_sending_channel)
            .mount("/", routes![domains::accounts::rest::registration::register_buyer])
    }
}

fn back_end_domain_name(domain_name: DomainName) -> crate::shared::domain::backend::DomainName {
    match domain_name   {
        BackEnd(BackEndDomainName::ACCOUNTS)=>
            BackEndDomainName::ACCOUNTS,
        _ => { BackEndDomainName::ACCOUNTS}
    }
}

