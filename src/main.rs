use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};
use local_ip_address::local_ip;
use qr2term::print_qr;
use std::net::SocketAddr;

async fn handle_left() -> impl IntoResponse {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let _ = enigo.key(Key::LeftArrow, Click);
    "OK"
}

async fn handle_right() -> impl IntoResponse {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let _ = enigo.key(Key::RightArrow, Click);
    "OK"
}

async fn index() -> impl IntoResponse {
    Html(
        r#"
    <html>
    <head>
        <title>PPT Remote</title>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <style>
            body {
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                height: 100vh;
                margin: 0;
                background-color: #f2f2f2;
                font-family: -apple-system, BlinkMacSystemFont, "Helvetica Neue", sans-serif;
            }
            h2 {
                font-size: 3em;
                margin-bottom: 40px;
            }
            button {
                width: 80%;
                padding: 30px;
                font-size: 2em;
                margin: 20px;
                border-radius: 12px;
                border: none;
                background-color: #007bff;
                color: white;
                box-shadow: 0 4px 8px rgba(0,0,0,0.2);
            }
            button:active {
                background-color: #0056b3;
            }
        </style>
    </head>
    <body>
        <h2>📽️ PPT Remote</h2>
        <button onclick="fetch('/left')">⬅️ Previous</button>
        <button onclick="fetch('/right')">➡️ Next</button>
    </body>
    </html>
    "#,
    )
}

#[tokio::main]
async fn main() {
    // get local IP
    let ip = local_ip().unwrap();
    let addr = SocketAddr::from((ip, 6666));

    // generate QR code
    let url = format!("http://{}:8000", ip);
    println!("📱 Please Scan the QR code to open the page:");
    print_qr(&url).unwrap();
    // let code = QrCode::new(&url).unwrap();
    // let image = code.render::<Luma<u8>>().build();
    // image.save("qrcode.png").unwrap();

    // construct the server
    let app = Router::new()
        .route("/", get(index))
        .route("/left", get(handle_left))
        .route("/right", get(handle_right));

    println!("🚀 Server Running: http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
