use crate::{
    backend::{
        domains::accounts::{
            controller::AccountDomainController, create_domain_config_for_accounts,
        },
        util::workflow::WorkflowService,
    },
    shared::{
        cq::CQMessage,
        util::rabbitmq::RabbitMQContext,
    },
};
use crossbeam_channel::{Receiver, unbounded};
use enum_map::EnumMap;
use crate::shared::util::config::DatabaseConfig;
use futures::future::join_all;
use crate::shared::domain::backend::DomainName as BackEndDomainName;
use crate::shared::util::database::DatabaseContext;

pub mod accounts;

pub mod constants {}

pub struct DomainManager {
    domain_manager_receiving_channel: Receiver<CQMessage>,
    rabbitmq_context: RabbitMQContext,
    database_config_map: EnumMap<BackEndDomainName, DatabaseConfig>,
    workflow_service: WorkflowService,
}

impl DomainManager {
    pub fn new(
        domain_manager_receiving_channel: Receiver<CQMessage>,
        rabbitmq_context: RabbitMQContext,
        database_config_map: EnumMap<BackEndDomainName, DatabaseConfig>,
        workflow_service: WorkflowService,
    ) -> DomainManager {
        DomainManager {
            domain_manager_receiving_channel,
            rabbitmq_context,
            database_config_map,
            workflow_service,
        }
    }

    pub async fn start_domains(&self) {

        //accounts domain
        let account_database_context = DatabaseContext::new(self.database_config_map[BackEndDomainName::ACCOUNTS].clone()).await.unwrap();
        let account_domain_config = create_domain_config_for_accounts();
        let (account_sending_channel, account_receiving_channel) = unbounded();
        let account_domain_controller = AccountDomainController::new(
            self.rabbitmq_context.clone(),
            account_database_context,
            account_domain_config,
            account_receiving_channel,
            account_sending_channel,
        );

        let account_handle = tokio::spawn(async move {
            account_domain_controller.start().await;
        });

        join_all(vec![account_handle]).await;
    }
}
