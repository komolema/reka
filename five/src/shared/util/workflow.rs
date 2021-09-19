use crate::shared::util::config::WorkflowConfig;

pub struct WorkflowService {
    workflow_config: WorkflowConfig,
}

impl WorkflowService {
    pub fn new(workflow_config: WorkflowConfig) -> WorkflowService {
        WorkflowService { workflow_config }
    }

    pub fn start_workflow(workflow_info: WorkflowInfo){}
    pub fn workflow_status(workflow_info: WorkflowInfo)-> WorkflowStatus{
        WorkflowStatus::NotStarted
    }
}

pub struct WorkflowDeployer{}
pub struct WorkflowInfo{}
pub enum WorkflowStatus{
    NotStarted
}

