use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{LSPObject, Url};

pub use notification_params::*;

/// A notebook document.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocument {
    /// The notebook document's URI.
    uri: Url,
    /// The type of the notebook.
    notebook_type: String,
    /// The version number of this document (it will increase after each
    /// change, including undo/redo).
    version: i32,
    /// Additional metadata stored with the notebook
    /// document.
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<LSPObject>,
    /// The cells of a notebook.
    cells: Vec<NotebookCell>,
}

/// A notebook cell.
///
/// A cell's document URI must be unique across ALL notebook
/// cells and can therefore be used to uniquely identify a
/// notebook cell or the cell's text document.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCell {
    /// The cell's kind
    kind: NotebookCellKind,
    /// The URI of the cell's text document content.
    document: Url,
    /// Additional metadata stored with the cell.
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<LSPObject>,
    /// Additional execution summary information
    /// if supported by the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_summary: Option<ExecutionSummary>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionSummary {
    /// A strict monotonically increasing value
    /// indicating the execution order of a cell
    /// inside a notebook.
    execution_order: u32,
    /// Whether the execution was successful or
    /// not if known by the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    success: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum NotebookCellKind {
    /// A markup-cell is formatted source that is used for display.
    Markup = 1,
    /// A code-cell is source code.
    Code = 2,
}

/// Capabilities specific to the notebook document support.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentClientCapabilities {
    /// Capabilities specific to notebook document synchronization
    ///
    /// @since 3.17.0
    pub synchronization: NotebookDocumentSyncClientCapabilities,
}

/// Notebook specific client capabilities.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentSyncClientCapabilities {
    /// Whether implementation supports dynamic registration. If this is
    /// set to `true` the client supports the new
    /// `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
    /// return value for the corresponding server capability as well.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,

    /// The client supports sending execution summary data per cell.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_summary_report: Option<bool>,
}

///  Options specific to a notebook plus its cells
///  to be synced to the server.
///
///  If a selector provides a notebook document
///  filter but no cell selector all cells of a
///  matching notebook document will be synced.
///
///  If a selector provides no notebook document
///  filter but only a cell selector all notebook
///  documents that contain at least one matching
///  cell will be synced.
///
///  @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentSyncOptions {
    /// The notebooks to be synced
    notebook_selector: Vec<NotebookSelector>,
    /// Whether save notification should be forwarded to
    /// the server. Will only be honored if mode === `notebook`.
    #[serde(skip_serializing_if = "Option::is_none")]
    save: Option<bool>,
}

/// Registration options specific to a notebook.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentSyncRegistrationOptions {
    /// The notebooks to be synced
    notebook_selector: Vec<NotebookSelector>,
    /// Whether save notification should be forwarded to
    /// the server. Will only be honored if mode === `notebook`.
    #[serde(skip_serializing_if = "Option::is_none")]
    save: Option<bool>,
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

/// A notebook cell text document filter denotes a cell text
/// document by different properties.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCellTextDocumentFilter {
    /// A filter that matches against the notebook
    /// containing the notebook cell. If a string
    /// value is provided it matches against the
    /// notebook type. '*' matches every notebook.
    notebook: Notebook,
    /// A language id like `python`.
    ///
    /// Will be matched against the language id of the
    /// notebook cell document. '*' matches every language.
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
}

/// Selects the notebook cells to be synced
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookSelector {
    /// The notebook to be synced. If a string
    /// value is provided it matches against the
    /// notebook type. '*' matches every notebook.
    #[serde(skip_serializing_if = "Option::is_none")]
    notebook: Option<Notebook>,
    /// The cells of the matching notebook to be synced.
    #[serde(skip_serializing_if = "Option::is_none")]
    cells: Option<Vec<NotebookCellSelector>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCellSelector {
    language: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Notebook {
    String(String),
    NotebookDocumentFilter(NotebookDocumentFilter),
}

/// A notebook document filter denotes a notebook document by
/// different properties.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentFilter {
    /// The type of the enclosing notebook.
    #[serde(skip_serializing_if = "Option::is_none")]
    notebook_type: Option<String>,
    /// A Uri [scheme](#Uri.scheme), like `file` or `untitled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    scheme: Option<String>,
    /// A glob pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<String>,
}

mod notification_params {
    use serde::{Deserialize, Serialize};

    use crate::{
        TextDocumentContentChangeEvent, TextDocumentIdentifier, TextDocumentItem,
        VersionedTextDocumentIdentifier,
    };

    use super::*;

    /// The params sent in an open notebook document notification.
    ///
    /// @since 3.17.0
    #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DidOpenNotebookDocumentParams {
        /// The notebook document that got opened.
        notebook_document: NotebookDocument,
        /// The text documents that represent the content
        /// of a notebook cell.
        cell_text_documents: Vec<TextDocumentItem>,
    }

    /// The params sent in a change notebook document notification.
    ///
    /// @since 3.17.0
    #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DidChangeNotebookDocumentParams {
        /// The notebook document that did change. The version number points
        /// to the version after all provided changes have been applied.
        notebook_document: VersionedNotebookDocumentIdentifier,

        /// The actual changes to the notebook document.
        ///
        /// The change describes single state change to the notebook document.
        /// So it moves a notebook document, its cells and its cell text document
        /// contents from state S to S'.
        ///
        /// To mirror the content of a notebook using change events use the
        /// following approach:
        /// - start with the same initial content
        /// - apply the 'notebookDocument/didChange' notifications in the order
        ///   you receive them.
        change: NotebookDocumentChangeEvent,
    }

    /// A versioned notebook document identifier.
    ///
    /// @since 3.17.0
    #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VersionedNotebookDocumentIdentifier {
        /// The version number of this notebook document.
        version: i32,
        /// The notebook document's URI.
        uri: Url,
    }

    /// A change event for a notebook document.
    ///
    /// @since 3.17.0
    #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NotebookDocumentChangeEvent {
        /// The changed meta data if any.
        #[serde(skip_serializing_if = "Option::is_none")]
        metadata: Option<LSPObject>,

        /// Changes to cells
        #[serde(skip_serializing_if = "Option::is_none")]
        cells: Option<NotebookDocumentCellChange>,
    }

    #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NotebookDocumentCellChange {
        /// Changes to the cell structure to add or
        /// remove cells.
        #[serde(skip_serializing_if = "Option::is_none")]
        structure: Option<NotebookDocumentCellChangeStructure>,

        /// Changes to notebook cells properties like its
        /// kind, execution summary or metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<Vec<NotebookCell>>,

        /// Changes to the text content of notebook cells.
        #[serde(skip_serializing_if = "Option::is_none")]
        text_content: Option<Vec<NotebookDocumentChangeTextContent>>,
    }

    #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NotebookDocumentChangeTextContent {
        document: VersionedTextDocumentIdentifier,
        changes: Vec<TextDocumentContentChangeEvent>,
    }

    #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NotebookDocumentCellChangeStructure {
        /// The change to the cell array.
        array: NotebookCellArrayChange,
        /// Additional opened cell text documents.
        #[serde(skip_serializing_if = "Option::is_none")]
        did_open: Option<Vec<TextDocumentItem>>,
        /// Additional closed cell text documents.
        #[serde(skip_serializing_if = "Option::is_none")]
        did_close: Option<Vec<TextDocumentIdentifier>>,
    }

    /// A change describing how to move a `NotebookCell`
    /// array from state S to S'.
    ///
    /// @since 3.17.0
    #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NotebookCellArrayChange {
        /// The start offset of the cell that changed.
        start: u32,

        /// The deleted cells
        delete_count: u32,

        /// The new cells, if any
        #[serde(skip_serializing_if = "Option::is_none")]
        cells: Option<Vec<NotebookCell>>,
    }

    /// The params sent in a save notebook document notification.
    ///
    /// @since 3.17.0
    #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DidSaveNotebookDocumentParams {
        /// The notebook document that got saved.
        notebook_document: NotebookDocumentIdentifier,
    }

    /// A literal to identify a notebook document in the client.
    ///
    /// @since 3.17.0
    #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NotebookDocumentIdentifier {
        /// The notebook document's URI.
        uri: Url,
    }

    /// The params sent in a close notebook document notification.
    ///
    /// @since 3.17.0
    #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DidCloseNotebookDocumentParams {
        /// The notebook document that got closed.
        notebook_document: NotebookDocumentIdentifier,

        /// The text documents that represent the content
        /// of a notebook cell that got closed.
        cell_text_documents: Vec<TextDocumentIdentifier>,
    }
}
