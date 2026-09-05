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
