use tower_lsp::jsonrpc::Error as JsonRpcError;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use crate::python_parser::{parse_launch_file, ParseResult};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
struct DocumentData {
    text: String,
    parse_result: ParseResult,
}

#[derive(Debug)]
pub struct Backend {
    client: Client,
    documents: Arc<RwLock<HashMap<Url, DocumentData>>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn parse_document(&self, uri: &Url, text: &str) {
        match parse_launch_file(text) {
            Ok(parse_result) => {
                let mut documents = self.documents.write().await;
                documents.insert(
                    uri.clone(),
                    DocumentData {
                        text: text.to_string(),
                        parse_result,
                    },
                );
            }
            Err(e) => {
                let _ = self.client.log_message(MessageType::ERROR, format!("Parse error: {}", e)).await;
            }
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult, JsonRpcError> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "ROS2 Launch LSP".to_string(),
                version: None,
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string(), ",".to_string()]),
                    ..Default::default()
                }),
                definition_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
        })
    }


    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.parse_document(&uri, &text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.last() {
            self.parse_document(&uri, &change.text).await;
        }
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>, JsonRpcError> {
        let uri = params.text_document_position.text_document.uri;
        let documents = self.documents.read().await;
        let doc = match documents.get(&uri) {
            Some(d) => d,
            None => return Ok(None),
        };

        let mut items = vec![
            CompletionItem::new_simple("Node".into(), "ROS Node action".into()),
            CompletionItem::new_simple("LaunchConfiguration".into(), "Substitution".into()),
            CompletionItem::new_simple("IncludeLaunchDescription".into(), "Include other launch file".into()),
        ];

        for sub in &doc.parse_result.substitutions {
            items.push(CompletionItem::new_simple(
                sub.name.clone(),
                "Launch configuration".into(),
            ));
        }

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>, JsonRpcError> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        
        let documents = self.documents.read().await;
        let doc = match documents.get(&uri) {
            Some(d) => d,
            None => return Ok(None),
        };

        for sub in &doc.parse_result.substitutions {
            if pos.line == sub.lineno as u32 - 1 && 
               pos.character >= sub.col_offset as u32 &&
               pos.character <= (sub.col_offset + sub.name.len()) as u32 
            {
                return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                    uri: uri.clone(),
                    range: Range {
                        start: Position::new(
                            sub.lineno as u32 - 1,
                            sub.col_offset as u32
                        ),
                        end: Position::new(
                            sub.lineno as u32 - 1,
                            (sub.col_offset + sub.name.len()) as u32
                        ),
                    },
                })));
            }
        }

        Ok(None)
    }

    async fn shutdown(&self) -> Result<(), JsonRpcError> {
        Ok(())
    }
}