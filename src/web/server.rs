// use std::net::SocketAddr;
use std::sync::LazyLock;
use std::vec::Vec;

use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, Uri, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use colored::Colorize;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;

use super::asset::ASSETS_MAP;
use crate::workflow::editor as workflow;

//https://stackoverflow.com/questions/27840394/how-can-a-rust-program-access-metadata-from-its-cargo-package
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
static VERSION_NUM: LazyLock<u64> = LazyLock::new(|| convert_version(VERSION));

const ASSETS: &[(&[u8], &str)] = &include!("asset.txt");

pub(crate) static IS_EN: LazyLock<bool> = LazyLock::new(|| {
    let language = get_lang();
    // println!("Your OS language is: {}", language);
    language[0..2].eq("en")
});

// https://doc.rust-lang.org/reference/conditional-compilation.html
#[cfg(windows)]
fn get_lang() -> String {
    let mut v = [0u16; windows::Win32::System::SystemServices::LOCALE_NAME_MAX_LENGTH as usize];
    unsafe {
        let l = windows::Win32::Globalization::GetUserDefaultLocaleName(&mut v) as usize;
        String::from_utf16(&v[0..l]).unwrap()
        // windows::Win32::Globalization::GetUserDefaultLangID().to_string()
    }
}

#[cfg(not(windows))]
fn get_lang() -> String {
    std::env::var("LANG").unwrap_or(String::from("en_US"))
}

// fn invalid_ip_msg(addr: &String) -> String {
//     format!("Invalid listening addr {}, please reset the configuration parameters by adding the startup parameter: {}",
//     addr.bright_red(), "-rs".bright_yellow())
// }

pub async fn start_app() {
    let settings = {
        let mut s = crate::db::init().await.expect("Initialize database failed");
        for argument in std::env::args() {
            if argument.eq("-rs") {
                s = settings::GlobalSettings::default();
                settings::save_global_settings(&s).expect("Reset settings failed");
                break;
            }
        }
        s
    };

    let mut listening_ip = String::with_capacity(32);
    let mut port: u16 = 0;
    let mut set_listening_ip = false;
    let mut set_listening_port = false;
    for argument in std::env::args() {
        if set_listening_ip {
            listening_ip.push_str(&argument);
            set_listening_ip = false;
            continue;
        }
        if argument.eq("-ip") {
            set_listening_ip = true;
            continue;
        }
        if set_listening_port {
            port = argument.parse::<u16>().unwrap();
            set_listening_port = false;
            continue;
        }
        if argument.eq("-port") {
            set_listening_port = true;
            continue;
        }
    }
    if listening_ip.is_empty() {
        listening_ip.push_str(&settings.ip);
    }
    if port == 0 {
        port = settings.port;
    }

    let r: Router = gen_router();
    let app = r.fallback(fallback);
    // let socket_addr: SocketAddr = addr.parse().expect(&invalid_ip_msg(&addr));
    let mut bind_res;
    // let mut port = settings.port;
    loop {
        let addr = format!("{}:{}", &listening_ip, port);
        bind_res = tokio::net::TcpListener::bind(&addr).await;
        if bind_res.is_ok() {
            break;
        }
        if !settings.select_random_port_when_conflict {
            log::error!("The listening port is occupied and the program fails to start.");
            log::info!("Tip: You can check the random port in the settings to avoid this problem.");
            std::process::exit(-1);
        }
        port += 1;
        if port == settings.port {
            log::error!("The listening port is occupied and the program fails to start.");
            log::info!("Tip: You can check the random port in the settings to avoid this problem.");
            std::process::exit(-1);
        }
        if port == 65535 {
            port = 1025;
        }
    }
    let listener = bind_res.unwrap();

    #[cfg(target_os = "windows")]
    colored::control::set_virtual_terminal(true).unwrap();

    log::info!(
        "-->  {} {}{}:{}",
        if *IS_EN {
            "The server is running, please open a browser and visit"
        } else {
            "服务已启动，请用浏览器访问"
        },
        "http://".bright_green(),
        if listening_ip.eq("0.0.0.0") {
            "<Your machine IP>".bright_green()
        } else {
            listening_ip.bright_green()
        },
        port.to_string().blue()
    );
    log::info!(
        "Tip: {} {} {} {} {}",
        if *IS_EN {
            "You can use"
        } else {
            "你可以使用"
        },
        "-ip".yellow(),
        if *IS_EN { "and" } else { "和" },
        "-port".yellow(),
        if *IS_EN {
            "to customize the listening IP and port"
        } else {
            "来自定义监听的IP和端口"
        },
    );
    log::info!("---------------------------------------------");
    log::info!("Current version: {VERSION}");
    log::info!("Visiting https://poweragentai.github.io/ for the latest releases");

    log::info!("-->  To close the server, hit {}", "Ctrl+C".bright_red());

    // let addr = format!("{}:{}", settings.ip, settings.port);
    // let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // let addr = SocketAddr::from((settings.ip, settings.port));
    let serve = axum::serve(listener, app);
    // log::info!("{:?}", serve.local_addr().unwrap());
    serve
        // .with_graceful_shutdown(shutdown_signal(sender))
        .await
        .unwrap();
}

fn gen_router() -> Router {
    Router::new()
        .route(
            "/workflow",
            get(workflow::list).post(workflow::save).delete(workflow::delete),
        )
        .route("/version.json", get(version))
        .route("/check-new-version.json", get(check_new_version))
        // .route("/o", get(subflow::output))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024, /* 250mb */
        ))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(
                    |_origin: &HeaderValue, _request_parts| {
                        // println!("{}", String::from_utf8_lossy(origin.as_bytes()));
                        // origin.as_bytes().ends_with(b"localhost")
                        true
                    },
                ))
                .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT]),
        )
}

// https://docs.rs/axum/0.6.18/axum/response/index.html

async fn fallback(uri: Uri) -> Response {
    let v = ASSETS_MAP.get(uri.path());
    if v.is_some() {
        let idx = v.unwrap();
        let d = ASSETS[*idx];
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, d.1.parse().unwrap());
        headers.insert(header::CONTENT_ENCODING, "gzip".parse().unwrap());
        (StatusCode::OK, headers, d.0).into_response()
    } else {
        (StatusCode::NOT_FOUND, format!("Not Found: {}", uri.path())).into_response()
    }
}

fn convert_version(ver: &str) -> u64 {
    let arr: Vec<&str> = ver.split('.').collect();
    let mut v = String::with_capacity(VERSION.len() + 4);
    v.push_str(arr[0]);
    if arr[1].len() == 1 {
        v.push_str("00");
    } else if arr[1].len() == 2 {
        v.push('0');
    }
    v.push_str(arr[1]);
    if arr[2].len() == 1 {
        v.push_str("00");
    } else if arr[2].len() == 2 {
        v.push('0');
    }
    v.push_str(arr[2]);
    // log::info!("vernum={}", &v);
    v.parse().expect("Wrong version")
}

async fn version() -> impl IntoResponse {
    let mut v = String::with_capacity(15);
    v.push('"');
    v.push_str(VERSION);
    v.push('"');
    v
}

async fn check_new_version() -> impl IntoResponse {
    let r = reqwest::get("https://dialogflowai.github.io/check-new-version.json").await;
    if let Err(e) = r {
        return to_res(Err(Error::NetworkConnectTimeout(Box::new(e))));
    }
    r.unwrap().text().await.map_or_else(
        |e| to_res(Err(Error::NetworkReadTimeout(Box::new(e)))),
        |s| {
            #[derive(Debug, Deserialize, Serialize)]
            struct VersionInfo {
                version: String,
                changelog: Vec<String>,
            }
            let obj: core::result::Result<VersionInfo, _> = serde_json::from_str(&s);
            if let Err(e) = obj {
                return to_res(Err(Error::InvalidJsonStructure(Box::new(e))));
            }
            let v = obj.unwrap();
            if convert_version(&v.version) > *VERSION_NUM {
                to_res(Ok(Some(v)))
            } else {
                to_res(Ok(None))
            }
        },
    )
}

async fn shutdown_signal(sender: tokio::sync::oneshot::Sender<()>) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    match sender.send(()) {
        Ok(_) => {}
        Err(_) => log::info!("中断 ctx 失败"),
    };

    crate::intent::phrase::shutdown_db().await;
    crate::kb::qa::shutdown_db().await;
    crate::kb::doc::shutdown_db().await;

    let m = if *IS_EN {
        "This program has been terminated"
    } else {
        "应用已退出"
    };
    log::info!("{m}");
}

#[derive(Serialize)]
struct ResponseData<D> {
    pub(crate) status: u16,
    pub(crate) data: Option<D>,
    pub(crate) err: Option<Error>,
}

pub(crate) fn to_res2<D>(
    r: Result<
        (
            D,
            Option<tokio::sync::mpsc::Receiver<crate::flow::rt::dto::StreamingResponseData>>,
        ),
        Error,
    >,
) -> axum::response::Response
where
    D: serde::Serialize + 'static + std::marker::Send,
{
    match r {
        Ok((d, receiver)) => {
            let builder = Response::builder().status(200);
            if receiver.is_none() {
                let res = ResponseData {
                    status: StatusCode::OK.as_u16(),
                    data: Some(d),
                    err: None,
                };
                let body = axum::body::Body::from(serde_json::to_string(&res).unwrap());
                builder
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(body)
                    .unwrap()
            } else {
                // log::info!("Response is chunked");
                let s = tokio_stream::wrappers::ReceiverStream::new(receiver.unwrap());
                let body = axum::body::Body::from_stream(s.map(|d| {
                    // let r = crate::flow::rt::dto::ResponseData::new_with_plain_text_answer(d);
                    // let res = ResponseData {
                    //     status: StatusCode::OK.as_u16(),
                    //     data: Some(r),
                    //     err: None,
                    // };
                    let body = serde_json::to_string(&d).unwrap();
                    Ok::<_, std::convert::Infallible>(body)
                }));
                builder
                    .header(header::TRANSFER_ENCODING, "chunked")
                    .header(header::CONTENT_TYPE, "application/x-ndjson")
                    .body(body)
                    .unwrap()
            }
        }
        Err(e) => {
            let builder = Response::builder().status(200);
            let res: ResponseData<D> = ResponseData {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                data: None,
                err: Some(e),
            };
            let body = axum::body::Body::from(serde_json::to_string(&res).unwrap());
            builder
                .header(header::CONTENT_TYPE, "application/json")
                .body(body)
                .unwrap()
        }
    }
}

// pub(crate) enum ResponseDataHolder<D> {
//     Normal(D),
//     Chunked(tokio::sync::mpsc::UnboundedReceiver<D>),
// }

// pub(crate) fn t<D>(d: ResponseDataHolder<D>) -> axum::response::Response
// where
//     D: serde::Serialize + 'static + std::marker::Send,
// {
//     let builder = Response::builder().status(200);
//     match d {
//         ResponseDataHolder::Normal(d) => {
//             let res = ResponseData {
//                 status: StatusCode::OK.as_u16(),
//                 data: Some(d),
//                 err: None,
//             };
//             let body = axum::body::Body::from(serde_json::to_string(&res).unwrap());
//             builder
//                 .header(header::CONTENT_TYPE, "application/json")
//                 .body(body)
//                 .unwrap()
//         }
//         ResponseDataHolder::Chunked(r) => {
//             let s = tokio_stream::wrappers::UnboundedReceiverStream::new(r);
//             let body = axum::body::Body::from_stream(s.map(|d| {
//                 let res = ResponseData {
//                     status: StatusCode::OK.as_u16(),
//                     data: Some(d),
//                     err: None,
//                 };
//                 let body = serde_json::to_string(&res).unwrap();
//                 Ok::<_, std::convert::Infallible>(body)
//             }));
//             builder
//                 .header(header::TRANSFER_ENCODING, "chunked")
//                 .body(body)
//                 .unwrap()
//         }
//     }
// }

pub(crate) fn to_res<D>(r: Result<D, Error>) -> impl IntoResponse
where
    D: serde::Serialize + 'static,
{
    // let now = std::time::Instant::now();
    let data = match r {
        Ok(d) => {
            let res = ResponseData {
                status: StatusCode::OK.as_u16(),
                data: Some(&d),
                err: None,
            };
            serde_json::to_string(&res).unwrap()
            // simd_json::to_string(&res).unwrap()
        }
        Err(e) => {
            let res: ResponseData<D> = ResponseData {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                data: None,
                err: Some(e),
            };
            serde_json::to_string(&res).unwrap()
            // simd_json::to_string(&res).unwrap()
        }
    };
    // log::info!("serialize used time:{:?}", now.elapsed());
    let mut header_map = HeaderMap::new();
    header_map.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    (StatusCode::OK, header_map, data)
}

pub(crate) fn is_en(headers: &axum::http::HeaderMap) -> bool {
    let client_language = headers
        .get("Accept-Language")
        .map_or_else(|| "en-US", |v| v.to_str().unwrap_or("en-US"));
    if !client_language.is_empty() && client_language.starts_with("en") {
        true
    } else {
        *IS_EN
    }
}
