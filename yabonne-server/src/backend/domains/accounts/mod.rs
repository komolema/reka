use crate::shared::{
    cq::backend::MessageType,
    domain::{
        backend::queues::{
            ACCOUNT_BUYER_QUEUE, ACCOUNT_COURIER_QUEUE, ACCOUNT_EVENTS_QUEUE,
            ACCOUNT_GENERAL_QUEUE, ACCOUNT_SELLER_QUEUE,
        },
        DomainConfig,
    },
};
use std::collections::HashMap;

pub mod controller;

pub struct DomainQueueMessage {
    queue_name: String,
    where_to_get_data: String,
    stored_procedure_to_call: String,
    cq_message: String,
}

#[derive(Clone, Copy)]
pub enum AccountHandlerFunction {
    RegisterIndividualSellerAccountHandler,
    RegisterIndividualBuyerAccountHandler,
    RegisterIndividualCourierAccountHandler,
    RegisterBusinessSellerAccountHandler,
    RegisterBusinessCourierAccountHandler,
}

pub fn create_domain_config_for_accounts(
) -> DomainConfig<MessageType, AccountHandlerFunction> {
    let queues = vec![
        String::from(ACCOUNT_BUYER_QUEUE),
        String::from(ACCOUNT_COURIER_QUEUE),
        String::from(ACCOUNT_GENERAL_QUEUE),
        String::from(ACCOUNT_SELLER_QUEUE),
        String::from(ACCOUNT_EVENTS_QUEUE),
    ];

    let mut message_to_function_map: HashMap<MessageType, AccountHandlerFunction> =
        HashMap::new();

    message_to_function_map.insert(
        MessageType::REGISTERBUSINESSCOURIERACCOUNT,
        AccountHandlerFunction::RegisterBusinessCourierAccountHandler,
    );

    DomainConfig {
        queues,
        message_to_function_map,
    }
}
