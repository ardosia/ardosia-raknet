use raknet_rust::low_level::transport::{
    ShardedRuntimeConfig, TransportConfig, TransportServer, spawn_sharded_runtime,
};

#[tokio::test]
#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
)))]
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
