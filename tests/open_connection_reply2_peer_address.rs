use bytes::BytesMut;
use raknet_rust::handshake::{
    OfflinePacket, OpenConnectionRequest1, OpenConnectionRequest2, Request2ParsePath,
};
use raknet_rust::low_level::protocol::constants::DEFAULT_UNCONNECTED_MAGIC;
use raknet_rust::low_level::transport::{
    HandshakeHeuristicsConfig, TransportConfig, TransportEvent, TransportServer,
};
use tokio::net::UdpSocket;

async fn exchange_offline(
    server: &mut TransportServer,
    client: &UdpSocket,
    server_addr: std::net::SocketAddr,
    packet: OfflinePacket,
) -> OfflinePacket {
    let mut encoded = BytesMut::new();
    packet.encode(&mut encoded).expect("offline packet must encode");
    client
        .send_to(&encoded, server_addr)
        .await
        .expect("client must send offline packet");

    let event = server
        .recv_and_process()
        .await
        .expect("server must process offline packet");
    assert!(matches!(event, TransportEvent::OfflinePacket { .. }));

    let mut response = [0u8; 2048];
    let (len, from) = client
        .recv_from(&mut response)
        .await
        .expect("client must receive offline reply");
    assert_eq!(from, server_addr);

    let mut source = &response[..len];
    OfflinePacket::decode(&mut source).expect("offline reply must decode")
}

#[tokio::test]
async fn open_connection_reply2_reports_client_socket_address() {
    let config = TransportConfig {
        bind_addr: "127.0.0.1:0".parse().unwrap(),
        send_cookie: false,
        supported_protocols: vec![8],
        handshake_heuristics: HandshakeHeuristicsConfig {
            enabled: false,
            ..HandshakeHeuristicsConfig::default()
        },
        ..TransportConfig::default()
    };
    let mut server = TransportServer::bind(config)
        .await
        .expect("transport server must bind");
    let server_addr = server.local_addr().expect("server address must exist");

    let client = UdpSocket::bind("127.0.0.1:0")
        .await
        .expect("client socket must bind");
    let client_addr = client.local_addr().expect("client address must exist");

    let reply1 = exchange_offline(
        &mut server,
        &client,
        server_addr,
        OfflinePacket::OpenConnectionRequest1(OpenConnectionRequest1 {
            protocol_version: 8,
            mtu: 1200,
            magic: DEFAULT_UNCONNECTED_MAGIC,
        }),
    )
    .await;

    let negotiated_mtu = match reply1 {
        OfflinePacket::OpenConnectionReply1(reply) => reply.mtu,
        other => panic!("expected OpenConnectionReply1, got {other:?}"),
    };

    let reply2 = exchange_offline(
        &mut server,
        &client,
        server_addr,
        OfflinePacket::OpenConnectionRequest2(OpenConnectionRequest2 {
            server_addr,
            mtu: negotiated_mtu,
            client_guid: 0x1122_3344_5566_7788,
            cookie: None,
            client_proof: false,
            parse_path: Request2ParsePath::StrictNoCookie,
            magic: DEFAULT_UNCONNECTED_MAGIC,
        }),
    )
    .await;

    let reply2 = match reply2 {
        OfflinePacket::OpenConnectionReply2(reply) => reply,
        other => panic!("expected OpenConnectionReply2, got {other:?}"),
    };

    assert_eq!(
        reply2.server_addr, client_addr,
        "RakNet OpenConnectionReply2 must report the connecting client's socket address"
    );
}
