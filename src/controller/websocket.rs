#![allow(dead_code)]
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, tungstenite, MaybeTlsStream, WebSocketStream};
use tokio_util::sync::CancellationToken;

use std::net::ToSocketAddrs;

use clap::Parser;

use tokio_util::task::task_tracker::TaskTracker;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 要连接的客户端地址列表
    #[arg(short, long, value_delimiter = ',', value_parser = parse_peer)]
    peers: Vec<String>,

    /// 绑定服务器的地址
    #[arg(short, long, value_parser = parse_bind)]
    bind: String,
}

/// 解析并验证客户端url
fn parse_peer(s: &str) -> Result<String, String> {
    // 验证以ws://或wss://开头的URL
    if s.starts_with("ws://") {
        let ip_port = &s[5..];
        if let Ok(_socket_addr) = ip_port.to_socket_addrs() {
            return Ok(s.to_string());
        }
    }
    Err(format!("Invalid client URL: {}", s))
}

/// 解析并验证绑定地址
fn parse_bind(s: &str) -> Result<String, String> {
    if let Ok(_socket_addr) = s.to_socket_addrs() {
        return Ok(s.to_string());
    }
    Err(format!("Invalid bind address: {}", s))
}

// 管理消息的发送和接收
struct WebSocketActor {
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebSocketActor {
    async fn connect(url: &str) -> Option<Self> {
        match connect_async(url).await {
            Ok((ws_stream, _)) => {
                println!("Connected successfully to {}", url);
                Some(WebSocketActor { ws_stream })
            }
            Err(err) => {
                println!("Error during websocket connection: {}", err);
                None
            }
        }
    }
}

// 维护网络状态
struct P2PWebsocketNetwork {
    addresses: Arc<Mutex<HashMap<SocketAddr, UnboundedSender<P2PInnerMessage>>>>,
    master: Arc<Mutex<UnboundedSender<P2PInnerMessage>>>,
}

#[derive(Debug)]
struct P2PInnerMessage {
    message: Message,
    tx_handler: UnboundedSender<P2PInnerMessage>,
}

async fn handle_server_connection(
    state: Arc<P2PWebsocketNetwork>,
    raw_stream: TcpStream,
    addr: SocketAddr,
    token: CancellationToken,
) {
    let (tx, mut rx) = unbounded_channel::<P2PInnerMessage>();
    {
        let mut list = state.addresses.lock().unwrap();
        list.insert(addr, tx.clone());
    }

    log::info!("Incoming TCP connection from: {}", addr);

    let ws_stream = match tokio_tungstenite::accept_async(raw_stream).await {
        Ok(ws) => ws,
        Err(e) => {
            log::error!("WebSocket handshake error: {:?}", e);
            return;
        }
    };

    log::info!("WebSocket connection established: {}", addr);

    let (mut ws_tx, mut ws_rx) = ws_stream.split();
    loop {
        tokio::select! {
            Some(msg) = ws_rx.next() => {
                log::debug!("Received: {:?}", msg);
                match msg {
                    Ok(msg) => {
                        if let Err(e) = state.master.lock().unwrap().send(P2PInnerMessage {
                            message: msg,
                            tx_handler: tx.clone(),
                        }) {
                            log::error!("Failed to send message to master: {:?}", e);
                        }
                    },
                    Err(e) => {
                        log::error!("Error receiving message or connection closed: {:?}", e);
                        break
                    }
                }
            }
            Some(msg) = rx.recv() => {
                log::debug!("Sending: {:?}", msg);
                if let Err(e) = ws_tx.send(msg.message).await {
                    log::error!("Failed to send message on socket: {:?}", e);
                }
            }
            _ = token.cancelled() => {
                log::warn!("task cancelled");
                break
            }
        }
    }
    {
        // 从列表中删除客户端
        let mut list = state.addresses.lock().unwrap();
        list.remove(&addr);
    }
}

async fn broadcast(
    state: Arc<P2PWebsocketNetwork>,
    tx: UnboundedSender<P2PInnerMessage>,
    bind: String,
) {
    log::debug!("Broadcast start");

    // 广播到已连接的客户端
    let list = state.addresses.lock().unwrap();

    for (i, cl) in list.iter().enumerate() {
        log::debug!("Broadcasting to {} ", cl.0);
        if let Err(e) = cl.1.send(P2PInnerMessage {
            message: tungstenite::protocol::Message::text(format!(
                "Message to client {} from {}",
                i, bind
            )),
            tx_handler: tx.clone(),
        }) {
            log::error!("Failed to send broadcast message: {:?}", e);
        }
    }
    log::debug!("Broadcast end");
}

pub async fn test_websocket() {
    let args = Args::parse();
    env_logger::init();

    let cancelation_token = CancellationToken::new();
    let tracker = TaskTracker::new();
    let (tx, mut rx) = unbounded_channel::<P2PInnerMessage>();
    let network_state: Arc<P2PWebsocketNetwork> = Arc::new(P2PWebsocketNetwork {
        addresses: Arc::new(Mutex::new(HashMap::new())),
        master: Arc::new(Mutex::new(tx.clone())),
    });

    for url in &args.peers {
        log::info!("connecting to {} ...", url);
        if let Some(conn) = WebSocketActor::connect(url).await {
            tracker.spawn(handle_connection(
                network_state.clone(),
                conn,
                cancelation_token.clone(),
            ));
        } else {
            log::warn!("could not connect to server: {url}");
        }
    }

    let listener = TcpListener::bind(&args.bind).await.expect("Failed to bind");

    loop {
        tokio::select! {
            Ok((stream, addr)) = listener.accept() => {
                tracker.spawn(handle_server_connection(
                    network_state.clone(),
                    stream, addr, cancelation_token.clone()));
            }
            Some(msg) = rx.recv() => {
                log::debug!("consuming ->{msg:?}");
            }
            _ = tokio::signal::ctrl_c() => {
                log::warn!("Received Ctrl+C, shutting down...");
                tracker.close();
                cancelation_token.cancel();
                break
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(10)) => {
                tracker.spawn(broadcast(network_state.clone(), tx.clone(), args.bind.clone()));
            }
        }
    }
    log::info!("waiting for all tasks");
    tracker.wait().await;
    log::debug!("tasks all are stoped");
}

async fn handle_connection(
    state: Arc<P2PWebsocketNetwork>,
    conn: WebSocketActor,
    token: CancellationToken,
) {
    // 提取套接字地址作为客户端列表的键
    let addr = match conn.ws_stream.get_ref() {
        MaybeTlsStream::Plain(f) => f.peer_addr().unwrap(),
        _ => {
            panic!("tls is not supported yet");
        }
    };

    // 这个tx应该在网络状态下共享
    let (tx, mut rx) = unbounded_channel::<P2PInnerMessage>();
    {
        let mut list = state.addresses.lock().unwrap();
        list.insert(addr, tx.clone());
    }

    let (mut ws_tx, mut ws_rx) = conn.ws_stream.split();

    loop {
        tokio::select! {
            Some(msg) = ws_rx.next() => {
                log::debug!("Received: {:?}", msg);
                match msg {
                    Ok(msg) => {
                        if let Err(e) = state.master.lock().unwrap().send(P2PInnerMessage {
                            message: msg,
                            tx_handler: tx.clone(),
                        }) {
                            log::error!("Failed to send message to master: {:?}", e);
                        }
                    },
                    Err(e) => {
                        log::error!("Error receiving message or connection closed: {:?}", e);
                        break
                    }
                }
            }
            Some(msg) = rx.recv() => {
                log::debug!("Sending: {:?}", msg);
                if let Err(e) = ws_tx.send(msg.message).await {
                    log::error!("Failed to send message on socket: {:?}", e);
                }
            }
            _ = token.cancelled() => {
                log::warn!("task cancelled");
                break
            }
        }
    }

    {
        // 从列表中删除客户端
        let mut list = state.addresses.lock().unwrap();
        list.remove(&addr);
    }
}
