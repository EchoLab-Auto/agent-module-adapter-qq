//! QQ 模块与 EchoAgent 宿主之间的领域 DTO。
//!
//! `echo-fabric-arch` v0.14 起只提供 `Contribution::Custom`；这里显式定义
//! 分类名与载荷，模块与宿主按同一 JSON 形状交换，不再借用框架的领域类型。

use echo_module_sdk::Contribution;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 聊天平台实例的宿主自定义贡献分类。
pub const INSTANCE_KIND: &str = "echo-agent.instance";

fn default_category() -> String {
    "module".to_string()
}

/// 平台工具的线上声明。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolContribution {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub parameters: Value,
    #[serde(default = "default_category")]
    pub category: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_hint_secs: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(default)]
    pub visible_reply: bool,
}

/// 模块实例能力位；宿主未声明即拒绝调用。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstanceCapabilities {
    #[serde(default)]
    pub send_text: bool,
    #[serde(default)]
    pub send_media: bool,
    #[serde(default)]
    pub groups: bool,
    #[serde(default)]
    pub friends: bool,
    #[serde(default)]
    pub gating: bool,
    #[serde(default)]
    pub login: bool,
}

/// 模块实例的自定义贡献载荷。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceContribution {
    pub name: String,
    pub platform: String,
    pub display_name: String,
    #[serde(default)]
    pub capabilities: InstanceCapabilities,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_schema: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ui: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_labels: Option<Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolContribution>,
}

impl InstanceContribution {
    /// 编码为框架通用贡献；实例 op 继续使用 `name:<op>`。
    pub fn into_custom(self) -> Contribution {
        Contribution::Custom {
            kind: INSTANCE_KIND.to_string(),
            name: self.name.clone(),
            payload: serde_json::to_value(self).expect("instance contribution is serializable"),
        }
    }
}
