use crate::shared::cq::{OriginSystem, CQMessageType, DomainInfo, CQHeader, TraceInfo};
use crate::shared::domain::DomainName;
use crate::shared::domain::backend::DomainName as BackEndDomainName;
use uuid::Uuid;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResult {
    pub code: ResultCode,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResultCode {
    Success,
    AccountAlreadyExist,
}

pub fn success_api_result() -> ApiResult {
    ApiResult {
        code: ResultCode::Success,
        message: "Success".to_string(),
    }
}

pub fn account_already_exists() -> ApiResult {
    ApiResult {
        code: ResultCode::AccountAlreadyExist,
        message: "Account already exists".to_string(),
    }
}

pub fn create_backend_header(back_end_domain_name: BackEndDomainName, route: String, to_origin_system: OriginSystem, from_origin_system: OriginSystem, back_end_message_type: CQMessageType) -> CQHeader {
    let domain_info: DomainInfo = DomainInfo {
        domain: DomainName::BackEnd(back_end_domain_name),
        routing_key: route,
        to_where: to_origin_system,
    };

    let trace_info: TraceInfo = TraceInfo {
        trace_id: Uuid::new_v4().to_string(),
        timestamp: Utc::now().to_rfc3339(),
        from_where: from_origin_system,
    };

    CQHeader {
        domain_info: domain_info,
        trace_info: trace_info,
        event_route_back: None,
        message_type: back_end_message_type,
    }
}
