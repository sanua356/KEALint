use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEACtrlAgentControlSockets {
    pub d2: Option<KEACtrlAgentContolSocket>,
    pub dhcp4: Option<KEACtrlAgentContolSocket>,
    pub dhcp6: Option<KEACtrlAgentContolSocket>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KEACtrlAgentContolSocket {
    pub socket_name: Option<String>,
    pub socket_type: Option<String>,
    pub comment: Option<String>,

    pub user_context: Option<Value>,
}
