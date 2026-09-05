mod config;
mod proxy;
mod rate_limiter;
mod runtime;
mod server;
mod session_pipeline;

pub use crate::session::tunables::SessionTunables;
pub use config::{HandshakeHeuristicsConfig, TransportConfig};
pub use proxy::{IdentityProxyRouter, InboundProxyRoute, OutboundProxyRoute, ProxyRouter};
pub use runtime::{
    EventOverflowPolicy, ShardedRuntimeCommand, ShardedRuntimeConfig, ShardedRuntimeEvent,
    ShardedRuntimeHandle, ShardedSendPayload,
};
pub use server::{
    ConnectedFrameDelivery, QueueDispatchResult, RemoteDisconnectReason, TransportEvent,
    TransportMetricsSnapshot, TransportRateLimitConfig, TransportServer,
};

pub async fn spawn_sharded_runtime(
    transport_config: TransportConfig,
    mut runtime_config: ShardedRuntimeConfig,
) -> std::io::Result<ShardedRuntimeHandle> {
    if runtime_config.shard_count > 1 && !TransportServer::supports_reuse_port_sharded_bind() {
        runtime_config.shard_count = 1;
    }

    runtime::spawn_sharded_runtime(transport_config, runtime_config).await
}

#[cfg(all(
    test,
    not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))
))]
mod tests {
    use super::{ShardedRuntimeConfig, TransportConfig, TransportServer, spawn_sharded_runtime};

    #[tokio::test]
    async fn unsupported_reuse_port_platform_uses_single_stateful_runtime_worker() {
        assert!(!TransportServer::supports_reuse_port_sharded_bind());

        let transport = TransportConfig {
            bind_addr: "127.0.0.1:0".parse().expect("valid bind address"),
            reuse_port: true,
            ..TransportConfig::default()
        };
        let runtime = ShardedRuntimeConfig {
            shard_count: 4,
            ..ShardedRuntimeConfig::default()
        };

        let handle = spawn_sharded_runtime(transport, runtime)
            .await
            .expect("runtime should start");

        assert_eq!(
            handle.shard_count(),
            1,
            "platforms without reuse-port flow affinity must not split one peer's RakNet session across multiple stateful workers"
        );

        handle.shutdown().await.expect("runtime should shut down");
    }
}
