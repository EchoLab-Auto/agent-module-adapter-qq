//! 本模块贡献的**技能**（SKILL.md 能力包）。
//!
//! ## 技能为什么在这里而不在 Core
//!
//! 这两个技能讲的全是 QQ / OneBot 的操作知识（消息从哪来、回复必须走哪个
//! 工具、适配器怎么启停、群历史怎么查）。它们服务的工具由本模块提供，
//! 触发它们的话术也属于本模块的领域——所以内容随模块走：
//!
//! | 好处 | 说明 |
//! |---|---|
//! | 一致性 | 工具改了技能同一提交里改，不会出现「Core 里的技能教的是旧工具」 |
//! | 门控 | 包被禁用 / 模块没装 → 技能自动消失，不再出现"技能在教一个不存在的工具" |
//! | 部署 | 正文 `include_str!` 进二进制，宿主不需要读本机文件路径 |
//!
//! ## 内容怎么走
//!
//! `body` 是 SKILL.md **全文**（含 frontmatter）。宿主解析时以正文为准、
//! DTO 字段只做兜底——这样技能文案只需要维护一份（`skills/*/SKILL.md`），
//! 这里的 DTO 字段只是"宿主不解析也能列清单"的镜像。

use crate::module_contract::SkillContribution;

/// 技能正文（编译期内联）。
const QQ_TRANSPORT: &str = include_str!("../skills/qq-transport/SKILL.md");
const QQ_MANAGEMENT: &str = include_str!("../skills/qq-management/SKILL.md");

/// 包标签：工具与技能共用，包级门控按它聚合。
const PACKAGE: &str = "echo-agent.instance.qq";

/// 本模块贡献的技能清单。
pub fn contributions() -> Vec<SkillContribution> {
    vec![
        SkillContribution {
            name: "qq-transport".to_string(),
            description: "QQ 消息收发闭环".to_string(),
            keywords: Vec::new(),
            // 常驻：任何时候都可能收到 QQ 消息，回复规则必须一直在上下文里。
            always: true,
            system: false,
            category: "qq".to_string(),
            package: Some(PACKAGE.to_string()),
            body: QQ_TRANSPORT.to_string(),
        },
        SkillContribution {
            name: "qq-management".to_string(),
            description: "Manage QQ/OneBot adapter lifecycle".to_string(),
            keywords: Vec::new(),
            always: false,
            system: false,
            category: "qq".to_string(),
            package: Some(PACKAGE.to_string()),
            body: QQ_MANAGEMENT.to_string(),
        },
    ]
}

/// 编码成框架贡献（握手时随 `Register` 上报）。
pub fn custom_contributions() -> Vec<echo_module_sdk::Contribution> {
    contributions()
        .into_iter()
        .map(SkillContribution::into_custom)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_skills_have_frontmatter_with_matching_name() {
        for skill in contributions() {
            assert!(
                skill.body.trim_start().starts_with("---"),
                "{} 缺少 frontmatter",
                skill.name
            );
            let expected = format!("name: {}", skill.name);
            assert!(
                skill.body.contains(&expected),
                "{} 的 frontmatter 名字与 DTO 不一致（宿主会以正文为权威，两边必须同源）",
                skill.name
            );
            assert!(
                skill.body.contains(&format!("package: {PACKAGE}")),
                "{} 缺少 package 标签：包级门控会漏掉它",
                skill.name
            );
        }
    }

    #[test]
    fn contributions_are_domain_neutral_custom_payloads() {
        for contribution in custom_contributions() {
            match contribution {
                echo_module_sdk::Contribution::Custom { kind, name, .. } => {
                    assert_eq!(kind, "echo-agent.skill");
                    assert!(!name.is_empty());
                }
                other => panic!("技能必须是 Custom 贡献，得到 {other:?}"),
            }
        }
    }
}
