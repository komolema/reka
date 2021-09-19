use crate::{
    backend::domains::accounts::AccountHandlerFunction,
    shared::{
        cq::{backend::MessageType, CQMessage, CQMessageType},
        domain::{DomainConfig, DomainState, DomainStatus},
        util::{database::DatabaseContext, rabbitmq::RabbitMQContext},
    },
};
use crossbeam_channel::{Receiver, Sender};
use futures::{future::join_all, StreamExt};
use lapin::{options::*, types::FieldTable, Channel, Consumer};
use std::ops::Add;

pub struct AccountDomainController {
    rabbitmq_context: RabbitMQContext,
    database_context: DatabaseContext,
    domain_config: DomainConfig<MessageType, AccountHandlerFunction>,
    account_domain_receiver_channel: Receiver<CQMessage>,
    account_domain_sender_channel: Sender<CQMessage>,
}

impl AccountDomainController {
    pub fn new(
        rabbitmq_context: RabbitMQContext,
        database_context: DatabaseContext,
        domain_config: DomainConfig<MessageType, AccountHandlerFunction>,
        account_domain_receiver_channel: Receiver<CQMessage>,
        account_domain_sender_channel: Sender<CQMessage>,
    ) -> AccountDomainController {
        AccountDomainController {
            rabbitmq_context,
            database_context,
            domain_config,
            account_domain_receiver_channel,
            account_domain_sender_channel,
        }
    }
    pub async fn start(&self) {
        info!("Starting AccountDomainController");
        let mut handlers = vec![];
        let queues = self.domain_config.queues.clone();
        let backend_rmq = self.rabbitmq_context.motsamaisi_rmq_connection.clone();
        let domain_sender_channel_clone = self.account_domain_sender_channel.clone();
        handlers = queues
            .into_iter()
            .map(|queue| {
                let q_clone = queue.clone();

                let backend_rmq_clone= backend_rmq.clone();

                let domain_sender = domain_sender_channel_clone.clone();
                tokio::spawn(async move {
                    let channel_promise = backend_rmq_clone.create_channel();

                    let channel: Channel = channel_promise.await.unwrap();
                    let consumer_options = BasicConsumeOptions {
                        no_local: false,
                        no_ack: true,
                        exclusive: false,
                        nowait: false,
                    };

                    let mut consumer: Consumer = channel
                        .basic_consume(
                            q_clone.clone().as_str(),
                            q_clone.clone().add("-tag").as_str(),
                            consumer_options,
                            FieldTable::default(),
                        )
                        .await
                        .unwrap();
                    info!("consumer created for queue:{}", q_clone);

                    loop {
                        while let Some(delivery) = consumer.next().await {
                            let delivery = delivery.unwrap();
                            let message_data_str =
                                std::str::from_utf8(&delivery.data).unwrap();

                            let message: CQMessage =
                                serde_json::from_str(message_data_str.clone()).unwrap();
                            domain_sender.send(message);
                            info!("Message received with values {}", message_data_str);
                        }
                    }
                })
            })
            .collect();

        let domain_receiver = self.account_domain_receiver_channel.clone();

        let receiving_handle = tokio::spawn(async move {
            select! {
                 recv(domain_receiver) -> msg => {
                 //call the correct methods in here dynamically that is
                 handle_message(msg.unwrap());
                 },
                 default => {},
            }
        });

        handlers.append(&mut vec![receiving_handle]);
        join_all(handlers).await;
    }
    pub async fn pause(&self) {}
    pub async fn stop(&self) {}
    pub async fn status(&self) -> DomainStatus {
        DomainStatus::new(DomainState::UNKNOWN, 0, 0, 0)
    }
}

fn handle_message(msg: CQMessage) {
    let msg_header = msg.header.clone();
    match msg_header.message_type {
        CQMessageType::FrontEndMessageType(front_end_msg) => {}
        CQMessageType::BackEndMessageType(back_end_msg) => match back_end_msg {
            MessageType::REGISTERINDIVIDUALSELLERACCOUNT => {
                register_individual_seller_account(msg);
            }
            MessageType::REGISTERINDIVIDUALBUYERACCOUNT => {
                register_individual_buyer_account(msg);
            }
            MessageType::REGISTERINDIVIDUALCOURIERACCOUNT => {
                register_individual_courier_account(msg);
            }
            MessageType::REGISTERBUSINESSSELLERACCOUNT => {
                register_business_seller_account(msg);
            }
            MessageType::REGISTERBUSINESSCOURIERACCOUNT => {
                register_business_courier_account(msg);
            }
            MessageType::REGISTERBUSINESSBUYERACCOUNT => {
                register_business_seller_account(msg);
            }
        },
    };
}

fn register_individual_seller_account(msg: CQMessage) {}

fn register_individual_buyer_account(msg: CQMessage) {}

fn register_individual_courier_account(msg: CQMessage) {}

fn register_business_seller_account(msg: CQMessage) {}

fn register_business_buyer_account(msg: CQMessage) {}

fn register_business_courier_account(msg: CQMessage) {}
