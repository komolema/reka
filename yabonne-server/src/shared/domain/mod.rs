use std::collections::HashMap;
use strum_macros::Display;
use strum_macros::{AsRefStr, AsStaticStr, EnumString, IntoStaticStr};

pub mod types;

pub enum DomainState {
    RUNNING,
    PAUSED,
    STOPPING,
    STOPPED,
    UNKNOWN,
}

pub struct DomainStatus {
    pub state: DomainState,
    pub processed_messages: i32,
    pub messages_processing: i32,
    pub erroneous_messages: i32,
}

impl DomainStatus {
    pub fn new(
        state: DomainState,
        processed_messages: i32,
        messages_processing: i32,
        erroneous_messages: i32,
    ) -> DomainStatus {
        DomainStatus {
            state,
            processed_messages,
            messages_processing,
            erroneous_messages,
        }
    }
}

#[derive(Serialize, Deserialize, Display, Debug, Clone, AsRefStr, AsStaticStr, IntoStaticStr)]
pub enum DomainName {
    FrontEnd(frontend::DomainName),
    BackEnd(backend::DomainName),
}

#[derive(Clone)]
pub struct DomainConfig<MK, F> {
    pub queues: Vec<String>,
    pub message_to_function_map: HashMap<MK, F>,
}

pub mod backend {
    use enum_map::Enum;
    use strum_macros::Display;
    use strum_macros::{AsRefStr, AsStaticStr, EnumString, IntoStaticStr};

    #[derive(Serialize, Hash, Eq, PartialEq, Enum, Deserialize, Display, Debug, Clone, AsRefStr, AsStaticStr, IntoStaticStr)]
    pub enum DomainName {
        ACCOUNTS,
        WORKFLOWS,
        COURIER,
    }

    pub mod routing_keys {
        pub const ACCOUNT_SELLER_ROUTE: &str = "account.seller.#";
        pub const ACCOUNT_BUYER_ROUTE: &str = "account.buyer.#";
        pub const ACCOUNT_COURIER_ROUTE: &str = "account.courier.#";
        pub const ACCOUNT_GENERAL_ROUTE: &str = "account.general.#";
        pub const ACCOUNT_EVENT_ROUTE: &str = "account.event.#";
    }

    pub mod queues {
        pub const ACCOUNT_SELLER_QUEUE: &str = "account-seller";
        pub const ACCOUNT_BUYER_QUEUE: &str = "account-buyer";
        pub const ACCOUNT_COURIER_QUEUE: &str = "account-courier";
        pub const ACCOUNT_GENERAL_QUEUE: &str = "account-general";
        pub const ACCOUNT_EVENTS_QUEUE: &str = "account-events";
    }
}

pub mod frontend {
    use enum_map::Enum;
    use strum_macros::Display;
    use strum_macros::{AsRefStr, AsStaticStr, EnumString, IntoStaticStr};

    #[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Enum,Display, Debug, Clone, AsRefStr, AsStaticStr, IntoStaticStr)]
    pub enum DomainName {
        FRONTEND,
    }
}
