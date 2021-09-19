use crate::shared::{
    cq::{
        backend::{
            accounts::cq::RegisterIndividualBuyerAccount,
            Body::REGISTERINDIVIDUALBUYERACCOUNT,
        },
        CQBody
    },
    domain::{
        backend::{routing_keys::ACCOUNT_BUYER_ROUTE, DomainName as BackEndDomainName},
        types::PersonalInformation,
        DomainName,
    },
};
use crossbeam_channel::Sender;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
use crate::shared::cq::backend::MessageType;
use crate::shared::cq::{CQMessageType, OriginSystem, CQMessage};
use crate::shared::util::redis::{RedisRepository, RedisRepositoryImpl};
use std::ops::Deref;
use std::sync::Arc;
use crate::shared::api::{success_api_result, account_already_exists, create_backend_header};

#[derive(Serialize, Deserialize)]
pub struct NewBuyer {
    personal_information: PersonalInformation,
}

#[derive(Serialize, Deserialize)]
pub struct NewSeller {
    personal_information: PersonalInformation,
}

#[derive(Serialize, Deserialize)]
pub struct NewCourier {
    personal_information: PersonalInformation,
}

#[post("/buyer", format = "json", data = "<new_buyer>")]
pub fn register_buyer(
    producer_sending_channel: State<Sender<CQMessage>>,
    redis_repository: State<Arc<dyn RedisRepository>>,
    new_buyer: Json<NewBuyer>,
) -> JsonValue {

    let account_exists = redis_repository.deref().account_exists(new_buyer.personal_information.email.clone());

    if account_exists {
        json!(account_already_exists())
    } else {
        let header = create_backend_header(BackEndDomainName::ACCOUNTS,
                                           ACCOUNT_BUYER_ROUTE.to_string(),
                                           OriginSystem::BackEnd,
                                           OriginSystem::FrontEnd,
                                           CQMessageType::BackEndMessageType(MessageType::REGISTERINDIVIDUALBUYERACCOUNT));

        let body = RegisterIndividualBuyerAccount {
            personal_information: new_buyer.0.personal_information,
        };

        let message: CQMessage = CQMessage {
            header,
            body: (CQBody::BackBody(REGISTERINDIVIDUALBUYERACCOUNT(body))),
        };

        producer_sending_channel.send(message);
        json!(success_api_result())
    }
}

#[post("/seller", format = "json", data = "<new_seller>")]
pub fn register_seller(
    producer_sending_channel: State<Sender<CQMessage>>,
    redis_repository: State<Arc<dyn RedisRepository>>,
    new_seller: Json<NewSeller>,
) -> JsonValue {


    let account_exists = redis_repository.deref().account_exists(new_seller.personal_information.email.clone());

    if account_exists {
        json!(account_already_exists())
    } else {
        let header = create_backend_header(BackEndDomainName::ACCOUNTS,
                                           ACCOUNT_BUYER_ROUTE.to_string(),
                                           OriginSystem::BackEnd,
                                           OriginSystem::FrontEnd,
                                           CQMessageType::BackEndMessageType(MessageType::REGISTERINDIVIDUALBUYERACCOUNT));

        let body = RegisterIndividualBuyerAccount {
            personal_information: new_seller.0.personal_information,
        };

        let message: CQMessage = CQMessage {
            header,
            body: (CQBody::BackBody(REGISTERINDIVIDUALBUYERACCOUNT(body))),
        };

        producer_sending_channel.send(message);
        json!(success_api_result())
    }
}

#[post("/courier", format = "json", data = "<new_courier>")]
pub fn register_courier(
    producer_sending_channel: State<Sender<CQMessage>>,
    redis_repository: State<Arc<dyn RedisRepository>>,
    new_courier: Json<NewCourier>,
) -> JsonValue {


    let account_exists = redis_repository.deref().account_exists(new_courier.personal_information.email.clone());

    if account_exists {
        json!(account_already_exists())
    } else {
        let header = create_backend_header(BackEndDomainName::ACCOUNTS,
                                           ACCOUNT_BUYER_ROUTE.to_string(),
                                           OriginSystem::BackEnd,
                                           OriginSystem::FrontEnd,
                                           CQMessageType::BackEndMessageType(MessageType::REGISTERINDIVIDUALBUYERACCOUNT));

        let body = RegisterIndividualBuyerAccount {
            personal_information: new_courier.0.personal_information,
        };

        let message: CQMessage = CQMessage {
            header,
            body: (CQBody::BackBody(REGISTERINDIVIDUALBUYERACCOUNT(body))),
        };

        producer_sending_channel.send(message);
        json!(success_api_result())
    }
}


