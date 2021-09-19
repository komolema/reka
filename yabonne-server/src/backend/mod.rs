use crate::shared::cq::CQMessage;
use crossbeam_channel::{Receiver, unbounded};
use crate::backend::domains::DomainManager;
use crate::shared::util::rabbitmq::RabbitMQContext;
use crate::shared::util::config::{WorkflowConfig, DatabaseConfig};
use crate::backend::util::workflow::WorkflowService;
use enum_map::EnumMap;
use crate::shared::domain::backend::DomainName as BackEndDomainName;



pub mod domains;
pub mod util;

pub struct BackEndController {
    back_end_receiver_channel: Receiver<CQMessage>,
}

impl BackEndController {
    pub fn new(back_end_receiver_channel: Receiver<CQMessage>) -> BackEndController {
        BackEndController {
            back_end_receiver_channel,
        }
    }

    pub async fn start(&self, rabbitmq_context: RabbitMQContext, database_config_map: EnumMap<BackEndDomainName, DatabaseConfig>, workflow_config: WorkflowConfig) {
        info!("Starting BackEndController");
        let workflow_service = WorkflowService::new(workflow_config);
        let (domain_sending_channel, domain_receiving_channel) = unbounded();
        let domain_manager = DomainManager::new(domain_receiving_channel, rabbitmq_context, database_config_map, workflow_service);
         domain_manager.start_domains().await;
    }
}
