use crate::shared::util::config::WorkflowConfig;

pub struct WorkflowService {
    workflow_config: WorkflowConfig,
}

impl WorkflowService {
    pub fn new(workflow_config: WorkflowConfig) -> WorkflowService {
        WorkflowService { workflow_config }
    }
}
