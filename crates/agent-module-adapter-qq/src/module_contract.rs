//! QQ 模块与 EchoAgent 宿主之间的领域 DTO。
//!
//! `echo-fabric-arch` v0.14 起只提供 `Contribution::Custom`；这里显式定义
//! 分类名与载荷，模块与宿主按同一 JSON 形状交换，不再借用框架的领域类型。

use echo_module_sdk::Contribution;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 聊天平台实例的宿主自定义贡献分类。
pub const INSTANCE_KIND: &str = "echo-agent.instance";
/// 技能（SKILL.md 能力包）的宿主自定义贡献分类。
///
/// 与 `INSTANCE_KIND` 并列而不是嵌在实例里：技能是**模块**级能力（这个模块
/// 带哪些操作知识），不是某个实例的运行时能力位。
pub const SKILL_KIND: &str = "echo-agent.skill";

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

/// 模块贡献的技能（SKILL.md 形态）。
///
/// 正文来自 `include_str!`——技能内容随模块二进制走，宿主不需要能读到本机
/// 文件路径（联邦/容器部署下读不到）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillContribution {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub always: bool,
    #[serde(default)]
    pub system: bool,
    #[serde(default)]
    pub category: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 所属包 id：包级门控按它聚合工具与技能。
    pub package: Option<String>,
    /// SKILL.md 全文（含 frontmatter）。
    pub body: String,
}

impl SkillContribution {
    /// 编码为框架通用贡献；`name` 与载荷同名（宿主按它做键，不一致会被拒）。
    pub fn into_custom(self) -> Contribution {
        Contribution::Custom {
            kind: SKILL_KIND.to_string(),
            name: self.name.clone(),
            payload: serde_json::to_value(self).expect("skill contribution is serializable"),
        }
    }
}
