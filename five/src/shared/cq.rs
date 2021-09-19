use crate::shared::{ domain::DomainName};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "message")]
pub struct CQMessage {
    pub header: CQHeader,
    pub body: CQBody,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
#[serde(rename = "body")]
pub enum CQBody {
    FrontBody(frontend::Body),
    BackBody(backend::Body),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "header")]
pub struct CQHeader {
    pub domain_info: DomainInfo,
    pub trace_info: TraceInfo,
    pub event_route_back: Option<EventRouteBack>,
    pub message_type: CQMessageType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CQMessageType {
    FrontEndMessageType(frontend::MessageType),
    BackEndMessageType(backend::MessageType),
}

#[derive(Serialize, Deserialize, Debug, Clone,)]
pub struct DomainInfo {
    pub domain: DomainName,
    pub routing_key: String,
    pub to_where: OriginSystem,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraceInfo {
    pub trace_id: String,
    pub timestamp: String,
    pub from_where: OriginSystem,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OriginSystem {
    FrontEnd,
    BackEnd,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventRouteBack {
    pub to_where: OriginSystem,
    pub destination_domain: DomainInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "event")]
pub struct CQEvent {
    pub header: CQEventHeader,
    pub body: CQEventBody,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
#[serde(rename = "body")]
pub enum CQEventBody {
    FrontEventBody(frontend::EventBody),
    BackEventBody(backend::EventBody),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "header")]
pub struct CQEventHeader {
    pub domain_info: DomainInfo,
    pub trace_info: TraceInfo,
}

pub mod backend {
    use crate::shared::cq::backend::accounts::{
        cq::{
            RegisterBusinessCourierAccount, RegisterBusinessSellerAccount,
            RegisterIndividualBuyerAccount, RegisterIndividualCourierAccount,
            RegisterIndividualSellerAccount,
        },
        events::{
            BusinessCourierAccountRegistered, BusinessSellerAccountRegistered,
            IndividualBuyerAccountRegistered, IndividualCourierAccountRegistered,
            IndividualSellerAccountRegistered,
        },
    };

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(tag = "type", content = "content")]
    #[serde(rename = "m")]
    pub enum Body {
        EMPTY,
        REGISTERINDIVIDUALSELLERACCOUNT(RegisterIndividualSellerAccount),
        REGISTERINDIVIDUALBUYERACCOUNT(RegisterIndividualBuyerAccount),
        REGISTERINDIVIDUALCOURIERACCOUNT(RegisterIndividualCourierAccount),
        REGISTERBUSINESSSELLERACCOUNT(RegisterBusinessSellerAccount),
        REGISTERBUSINESSCOURIERACCOUNT(RegisterBusinessCourierAccount),
    }

    #[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug, Clone)]
    pub enum MessageType {
        REGISTERINDIVIDUALSELLERACCOUNT,
        REGISTERINDIVIDUALBUYERACCOUNT,
        REGISTERINDIVIDUALCOURIERACCOUNT,
        REGISTERBUSINESSSELLERACCOUNT,
        REGISTERBUSINESSCOURIERACCOUNT,
        REGISTERBUSINESSBUYERACCOUNT,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(tag = "type", content = "content")]
    #[serde(rename = "m")]
    pub enum EventBody {
        EMPTY,
        INDIVIDUALSELLERACCOUNTREGISTERED(IndividualSellerAccountRegistered),
        INDIVIDUALBUYERACCOUNTREGISTERED(IndividualBuyerAccountRegistered),
        INDIVIDUALCOURIERACCOUNTREGISTERED(IndividualCourierAccountRegistered),
        BUSINESSSELLERACCOUNTREGISTERED(BusinessSellerAccountRegistered),
        BUSINESSCOURIERACCOUNTREGISTERED(BusinessCourierAccountRegistered),
    }

    mod workflow {}

    pub mod accounts {
        pub mod cq {
            use crate::shared::domain::types::PersonalInformation;

            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct RegisterIndividualSellerAccount {
                pub personal_information: PersonalInformation,
            }

            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct RegisterIndividualBuyerAccount {
                pub personal_information: PersonalInformation,
            }

            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct RegisterIndividualCourierAccount {}

            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct RegisterBusinessSellerAccount {}

            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct RegisterBusinessCourierAccount {}
        }

        pub mod events {
            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct IndividualSellerAccountRegistered {}

            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct IndividualBuyerAccountRegistered {}

            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct IndividualCourierAccountRegistered {}

            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct BusinessSellerAccountRegistered {}

            #[derive(Serialize, Deserialize, Debug, Clone)]
            pub struct BusinessCourierAccountRegistered {}
        }
    }

    mod billing {}
}

mod frontend {
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(tag = "type", content = "content")]
    #[serde(rename = "b")]
    pub enum Body {}

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(tag = "type", content = "content")]
    #[serde(rename = "b")]
    pub enum EventBody {
        Empty,
    }

    #[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug, Clone)]
    pub enum MessageType {
        SYNCSTUFF,
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::{
        cq::{
            backend::{
                accounts::cq::RegisterIndividualBuyerAccount,
                Body::REGISTERINDIVIDUALBUYERACCOUNT, MessageType,
            },
            CQBody, CQHeader, CQMessage,
            CQMessageType::BackEndMessageType,
            DomainInfo, OriginSystem, TraceInfo,
        },
        domain::{
            types::{
                location::{
                    AddressType::DeliveryAddress, City, Coordinates, Address, Province,
                },
                Email, FullName, PersonalInformation,
            },
            DomainName,
        },
    };

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn success_from_json_string_register_individual_buyer_account() {
        let data = r#"
{
  "header": {
    "domain_info": {
      "domain": {
        "Motsamaisi": "ACCOUNTS"
      },
      "routing_key": "44"
    },
    "trace_info": {
      "trace_id": "33",
      "start_timestamp": "44",
      "end_timestamp": "44"
    }
  },
  "body": {
    "type": "MBODY",
    "content": {
      "type": "REGISTERINDIVIDUALBUYERACCOUNT",
      "content": {
        "personal_information": {
          "fullname": {
            "name": "karabo",
            "surname": "Molema"
          },
          "address": {
            "type": "DELIVERYADDRESS",
            "content": {
              "address_line_1": "3434",
              "address_line_2": "4343",
              "suburb": "Tableview",
              "city": {
                "name": "Cape Town",
                "short_name": "CPT"
              },
              "province": {
                "name": "Western Cape",
                "short_name": "WC"
              },
              "location": {
                "longitude": 1.0,
                "latitude": 1.0
              }
            }
          },
          "cellphone": "44",
          "email": {
            "value": "k@gmail.com"
          }
        }
      }
    }
  }
}
        "#;

        let register_individual_message: CQMessage = serde_json::from_str(data).unwrap();

        match register_individual_message.body {
            CQBody::FrontBody(REGISTERINDIVIDUALBUYERACCOUNT(body)) => {
                assert_eq!(body.personal_information.fullname.name, "karabo");
            }
            _ => {
                panic!("Deserialization did not work properly");
            }
        }
    }

    #[test]
    fn success_from_register_individual_buyer_account_to_json() {
        let dummy = CQMessage {
            header: CQHeader {
                domain_info: DomainInfo {
                    domain: DomainName::BackEnd(motsamaisi::DomainName::ACCOUNTS),
                    routing_key: String::from("44"),
                    to_where: OriginSystem::BackEnd,
                },
                trace_info: TraceInfo {
                    trace_id: String::from("33"),
                    timestamp: "".to_string(),
                    from_where: OriginSystem::FrontEnd,
                },
                event_route_back: None,
                message_type: BackEndMessageType(
                    MessageType::REGISTERINDIVIDUALBUYERACCOUNT,
                ),
            },
            body: CQBody::BackBody(REGISTERINDIVIDUALBUYERACCOUNT(
                RegisterIndividualBuyerAccount {
                    personal_information: PersonalInformation {
                        fullname: FullName {
                            name: "karabo".to_string(),
                            surname: "Molema".to_string(),
                        },
                        address: (DeliveryAddress(Address {
                            address_line_1: "3434".to_string(),
                            address_line_2: "4343".to_string(),
                            suburb: "Tableview".to_string(),
                            city: City {
                                name: "Cape Town".to_string(),
                                short_name: "CPT".to_string(),
                            },
                            province: Province {
                                name: "Western Cape".to_string(),
                                short_name: "WC".to_string(),
                            },
                            coordinates: Coordinates {
                                longitude: 1.0,
                                latitude: 1.0,
                            },
                        })),
                        cellphone: "".to_string(),
                        email: Email {
                            value: "".to_string(),
                        },
                    },
                },
            )),
        };

        let dummy_string = serde_json::to_string_pretty(&dummy).unwrap();

        println!("{}", dummy_string);
    }
}
