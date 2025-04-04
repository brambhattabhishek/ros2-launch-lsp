use tower_lsp::{LspService, Server};
use my_rust_lib::some_function;

mod server;
mod python_parser;

#[tokio::main]
async fn main() {
    // Call a Rust function from the my_rust_lib crate
    some_function();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    // Build the LSP service and socket
    let (service, socket) = LspService::build(|client| server::Backend::new(client)).finish();

    // Serve using stdin, stdout, and the socket
    Server::new(stdin, stdout, socket).serve(service).await;
}
