//! agent-module-adapter-qq 模块进程入口（P1.7b，2026-10 适配器模块化）。
//!
//! 宿主以 stdio 承载协议帧（4 字节 LE 长度前缀 + JSON），进程内不再有
//! 独立配置来源——实例名 / 端口 / 适配器配置全部经 `Hello.config` 下发
//! （见 [`agent_module_adapter_qq::module`]）。
//!
//! 手动调试：`ECHO_LOG=debug ./agent-module-adapter-qq`（日志走 stderr；stdout
//! 是协议通道，绝不能混入日志字节）。

use agent_module_adapter_qq::module::QqModule;
use echo_module_sdk::serve_stdio;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .init();

    if let Err(error) = serve_stdio(QqModule::new()).await {
        eprintln!("qq adapter module exited: {error}");
        std::process::exit(1);
    }
}
