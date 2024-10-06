use rpc_server::IResponse;
use tauri::{WebviewWindow, Emitter};

#[derive(Debug, Clone)]
pub struct IWindow {
  pub inner: WebviewWindow
}

impl IResponse for IWindow {
  fn send_listener(&self, data: &str) {
    let _ = self.inner.emit("rpc", data);
  }

  fn submit(&self, success: bool) {
    println!("Submit called with {success}");
  }
}