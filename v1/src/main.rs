use anyhow::{anyhow, Result};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use cityhash::city_hash_64;
use function_name::named;
use futures::StreamExt;
use std::env;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::error;
use v1::default_log_pre;
use v1::utils::router::{ReqContext, ResponseContext};
use v1::{build_routers, get_master_diesel_pool, get_slave_diesel_pool};

const KEY: &str = "F9B14CEC-60B6-810F-1FF7-8BAE688466AC";

#[named]
#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()),
            )
            .finish(),
    )
    .unwrap();

    let api_port = env::var("API_PORT").expect("must set API_PORT env.");

    let mut listener = TcpListener::bind(format!("0.0.0.0:{}", api_port)).await?;

    let router = build_routers();

    let master_diesel_pool = get_master_diesel_pool();
    let slave_diesel_pool = get_slave_diesel_pool();

    let state: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        match stream {
            Err(e) => {
                error!("{}\taccept failed = {:?}", default_log_pre!("", ""), e);
                continue;
            }
            Ok(sock) => {
                let master_diesel_pool = master_diesel_pool.clone();
                let slave_diesel_pool = slave_diesel_pool.clone();
                let router = router.clone();
                let state = state.clone();

                tokio::spawn(async move {
                    let master_diesel_pool = master_diesel_pool.clone();
                    let slave_diesel_pool = slave_diesel_pool.clone();
                    let router = router.clone();
                    let state = state.clone();
                    let (mut recv, sender) = sock.into_split();

                    let sender = Arc::new(Mutex::new(sender));

                    loop {
                        let mut stream_buf = [0; 65535];
                        let master_diesel_pool = master_diesel_pool.clone();
                        let slave_diesel_pool = slave_diesel_pool.clone();
                        let router = router.clone();
                        let state = state.clone();
                        let receiverd = match AsyncReadExt::read(&mut recv, &mut stream_buf).await {
                            Ok(v) if v == 0 => return,
                            Ok(v) => v,
                            Err(ref e) if e.kind() == tokio::io::ErrorKind::ConnectionReset => {
                                return;
                            }
                            Err(e) => {
                                error!(
                                    "{}\tfailed tcp socket recv message:{:?}",
                                    default_log_pre!("", ""),
                                    e
                                );
                                return;
                            }
                        };
                        let mut cursor = std::io::Cursor::new(&stream_buf[..receiverd]);
                        let code = match ReadBytesExt::read_u16::<LittleEndian>(&mut cursor) {
                            Ok(v) => v,
                            Err(e) => {
                                error!(
                                    "{}\tinvalid code formats:{:?}",
                                    default_log_pre!("", ""),
                                    e
                                );
                                let resp = match ResponseContext::get_bincode(
                                    0,
                                    0,
                                    506,
                                    "invalid code.",
                                    "",
                                ) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        error!(
                                            "{}\tfialed encode response:{:?}\t",
                                            default_log_pre!("", ""),
                                            e
                                        );
                                        return;
                                    }
                                };

                                handle_stream(sender.clone(), resp).await;
                                return;
                            }
                        };

                        let version = match ReadBytesExt::read_u8(&mut cursor) {
                            Ok(v) => v,
                            Err(e) => {
                                error!(
                                    "{}\tinvalid version formats:{:?}",
                                    default_log_pre!(code, ""),
                                    e
                                );
                                let resp = match ResponseContext::get_bincode(
                                    code,
                                    0,
                                    506,
                                    "invalid version.",
                                    "",
                                ) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        error!(
                                            "{}\tfialed encode response:{:?}\t",
                                            default_log_pre!(code, ""),
                                            e
                                        );
                                        return;
                                    }
                                };

                                handle_stream(sender.clone(), resp).await;
                                return;
                            }
                        };

                        let session_id = match ReadBytesExt::read_u64::<LittleEndian>(&mut cursor) {
                            Ok(v) => v,
                            Err(e) => {
                                error!(
                                    "{}\tinvalid session id formats:{:?}",
                                    default_log_pre!(code, ""),
                                    e
                                );
                                let resp = match ResponseContext::get_bincode(
                                    code,
                                    0,
                                    506,
                                    "invalid session id.",
                                    "",
                                ) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        error!(
                                            "{}\tfialed encode response:{:?}\t",
                                            default_log_pre!(code, ""),
                                            e
                                        );
                                        return;
                                    }
                                };

                                handle_stream(sender.clone(), resp).await;
                                return;
                            }
                        };

                        let signature = match ReadBytesExt::read_u64::<LittleEndian>(&mut cursor) {
                            Ok(v) => v,
                            Err(e) => {
                                error!(
                                    "{}\tinvalid signature formats:{:?}",
                                    default_log_pre!(code, ""),
                                    e
                                );
                                let resp = match ResponseContext::get_bincode(
                                    code,
                                    session_id,
                                    506,
                                    "invalid signature format.",
                                    "",
                                ) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        error!(
                                            "{}\tfialed encode response:{:?}\t",
                                            default_log_pre!(code, ""),
                                            e
                                        );
                                        return;
                                    }
                                };

                                handle_stream(sender.clone(), resp).await;
                                return;
                            }
                        };

                        let timestamp = match ReadBytesExt::read_u64::<LittleEndian>(&mut cursor) {
                            Ok(v) => v,
                            Err(e) => {
                                error!(
                                    "{}\tinvalid timestamp formats:{:?}",
                                    default_log_pre!(code, ""),
                                    e
                                );
                                let resp = match ResponseContext::get_bincode(
                                    code,
                                    session_id,
                                    506,
                                    "invalid timestamp.",
                                    "",
                                ) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        error!(
                                            "{}\tfialed encode response:{:?}\t",
                                            default_log_pre!(code, ""),
                                            e
                                        );
                                        return;
                                    }
                                };

                                handle_stream(sender.clone(), resp).await;
                                return;
                            }
                        };

                        let len = match ReadBytesExt::read_u32::<LittleEndian>(&mut cursor) {
                            Ok(v) => v,
                            Err(e) => {
                                error!(
                                    "{}\tinvalid length formats:{:?}",
                                    default_log_pre!(code, ""),
                                    e
                                );
                                let resp = match ResponseContext::get_bincode(
                                    code,
                                    session_id,
                                    506,
                                    "invalid body length.",
                                    "",
                                ) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        error!(
                                            "{}\tfialed encode response:{:?}\t",
                                            default_log_pre!(code, ""),
                                            e
                                        );
                                        return;
                                    }
                                };

                                handle_stream(sender.clone(), resp).await;
                                return;
                            }
                        };
                        let position = cursor.position() as usize;
                        let mut new_body = Vec::new();

                        if len > 0 {
                            let body = &stream_buf[position..position + (len as usize)];
                            new_body.extend_from_slice(body);
                        }

                        //signature valid
                        if let Err(e) = signature_valid(signature, timestamp, &new_body) {
                            error!(
                                "{}\tsignature verify invlaid:{:?}",
                                default_log_pre!(code, ""),
                                e
                            );

                            let resp = match ResponseContext::get_bincode(
                                code,
                                session_id,
                                506,
                                "invalid signature.",
                                "",
                            ) {
                                Ok(v) => v,
                                Err(e) => {
                                    error!(
                                        "{}\tfialed encode response:{:?}\t",
                                        default_log_pre!(code, ""),
                                        e
                                    );
                                    return;
                                }
                            };

                            handle_stream(sender.clone(), resp).await;
                            return;
                        }

                        let ctx = ReqContext {
                            socket: sender.clone(),
                            master_db: master_diesel_pool.clone(),
                            slave_db: slave_diesel_pool.clone(),
                            state,
                            code,
                            version,
                            session_id,
                            body_length: len,
                            body: new_body,
                        };

                        match router.call(code) {
                            Ok(f) => {
                                let resp = match f(ctx).await {
                                    Ok(v) => v,
                                    Err(e) => {
                                        error!(
                                            "{}\tfailed to method exec:{:?}",
                                            default_log_pre!(code, ""),
                                            e
                                        );
                                        let resp = match ResponseContext::get_bincode(
                                            code,
                                            session_id,
                                            506,
                                            &e.to_string(),
                                            "",
                                        ) {
                                            Ok(v) => v,
                                            Err(e) => {
                                                error!(
                                                    "{}\tfialed encode response:{:?}\t",
                                                    default_log_pre!(code, ""),
                                                    e
                                                );
                                                return;
                                            }
                                        };
                                        handle_stream(sender.clone(), resp).await;
                                        return;
                                    }
                                };

                                handle_stream(sender.clone(), resp).await;
                            }

                            Err(e) => {
                                error!(
                                    "{}\tresponse error:{}",
                                    default_log_pre!(code, ""),
                                    &e.to_string()
                                );
                                let resp = match ResponseContext::get_bincode(
                                    code,
                                    session_id,
                                    506,
                                    &e.to_string(),
                                    "",
                                ) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        error!(
                                            "{}\tfialed encode response:{:?}\t",
                                            default_log_pre!(code, ""),
                                            e
                                        );
                                        return;
                                    }
                                };
                                handle_stream(sender.clone(), resp).await;
                                return;
                            }
                        }
                    }
                });
            }
        }
    }

    Ok(())
}

//signature valid
fn signature_valid(sign: u64, timestamp: u64, body: &[u8]) -> Result<()> {
    let mut signature = Vec::new();

    signature.extend_from_slice(KEY.as_bytes());

    WriteBytesExt::write_u64::<LittleEndian>(&mut signature, timestamp)?;

    signature.extend_from_slice(body);

    let signature = city_hash_64(&signature);

    if sign != signature {
        return Err(anyhow!("invalid signature."));
    }

    Ok(())
}

#[named]
async fn handle_stream(socket: Arc<Mutex<OwnedWriteHalf>>, body: Vec<u8>) {
    let mut w = socket.lock().await;
    let _written = match w.write_all(&body).await {
        Ok(v) => v,

        Err(e) => {
            error!("{}\tstream send failed {:?}", default_log_pre!("", ""), e);
            return;
        }
    };
}

#[allow(unused)]
fn hex_dump(buf: &[u8]) -> String {
    let vec: Vec<String> = buf.iter().map(|b| format!("{:02x}", b)).collect();

    vec.join("")
}
