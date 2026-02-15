use tokio::net::TcpStream;
use crate::core::linalg::tridiagonal_system::TridiagonalSystem;
use tokio_tungstenite::{accept_async, tungstenite::Result};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};

pub struct WSHandler {
    clients: usize,
}

impl WSHandler {

    pub fn new() -> Self {
        Self {
            clients: 0,
        }
    }

    // TODO: add query indexing
    pub async fn handler(&self, stream: TcpStream) -> Result<()> {
        let ws_stream = accept_async(stream).await?;
        println!("Connection");

        let (mut sender, mut receiver) = ws_stream.split();

        while let Some(raw) = receiver.next().await {
            let raw = raw?;
            
            if !raw.is_text() {
                continue;
            }

            let rawjson = raw.to_text().expect("Is not a valid JSON");
            let json = serde_json::from_str::<Value>(rawjson).expect("Can't parse JSON");

            if let Some(action_val) = json.get("action") {
                if let Some(action) = action_val.as_str() {
                    match action {
                        "test" => {
                            if let Some(test_val) = json.get("test") {
                                if let Some(test) = test_val.as_str() {
                                    match selectTest(&String::from(test)) {
                                        Ok(system) => {
                                            let t1 = system.checkT1();
                                            let t2 = system.checkT2();
                                            // TODO: add error handler
                                            let result = system.solve().expect("Can't solve system");
                                            let operated = &system * &result;
                                            let rightSide = system.getRight();
                                            let residual = &operated - &rightSide;
                                            let output = json!({
                                                "th1": t1,
                                                "th2": t2,
                                                "result": result,
                                                // "rightSide": rightSide,
                                                // "operated": operated,
                                                "residual": residual
                                            });
                                            sender.send(output.to_string().into()).await?;
                                        },
                                        Err(e) => {
                                            sender.send(e.into()).await?;
                                        }
                                    }
                                }
                            }
                        },
                        // TODO: refactor
                        _ => sender.send("Unknown action".into()).await?
                    }
                }
            }

        }

        fn selectTest(test: &String) -> Result<TridiagonalSystem<f64>, &str> {
            let path = format!("tests/tridiagonal/{}.txt", test);
            Ok(TridiagonalSystem::load(&path).expect("Error loading test"))
        }

        Ok(())
    }

}