#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "message")]
pub struct CQMessage<T>{
    header: CQHeader,
    body: CQBody<T>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "header")]
pub struct CQHeader{
    pub domain_info: DomainInfo,
    pub trace_info: TraceInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "body")]
pub struct CQBody<T>{
    contents: T
}

#[derive(Serialize, Deserialize, Debug, Clone,)]
pub struct DomainInfo {
    pub domain: String,
    pub routing_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraceInfo {
    pub trace_id: String,
    pub timestamp: String,
}