//! Central application state manager.
//!
//! Coordinates the indexer, watcher, and frontend communication.

use crate::{
    config,
    error::{ChroniclerError, Result},
    events::FileEvent,
    importer,
    indexer::Indexer,
    models::{FileNode, FullPageData, PageHeader, RenderedPage},
    renderer::Renderer,
    watcher::Watcher,
};
use parking_lot::{Mutex, RwLock};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::{AppHandle, Emitter};
use tokio::sync::broadcast;
use tracing::{info, instrument};

/// The main `World` struct containing all application subsystems and state.
///
/// This struct acts as the single source of truth for the backend. It is managed
/// directly by `tauri::State`. Its fields are wrapped in thread-safe containers
/// like `Arc<RwLock<T>>` to allow for granular locking, preventing performance
/// bottlenecks where a long write operation would block unrelated read operations.
#[derive(Debug, Clone)]
pub struct World {
    /// The root directory of the worldbuilding vault, protected for concurrent access.
    root_path: Arc<RwLock<Option<PathBuf>>>,
    /// Thread-safe, shared access to the vault indexer.
    pub indexer: Arc<RwLock<Indexer>>,
    /// The application's file system watcher. Wrapped in a Mutex to allow safe swapping
    /// when the vault path changes.
    watcher: Arc<Mutex<Option<Watcher>>>,
    /// The application's Markdown renderer. Wrapped in an Arc as it is read-only after creation.
    pub renderer: Arc<Renderer>,
}

impl World {
    /// Creates a new, uninitialized `World` instance.
    ///
    /// This constructor sets up the shared, thread-safe state containers. The actual
    /// vault data is not loaded until `initialize_vault` is called.
    pub fn new() -> Self {
        // The indexer is created empty and wrapped for concurrent access.
        let indexer = Arc::new(RwLock::new(Indexer::default()));
        // The renderer is initialized with a clonable handle to the indexer.
        let renderer = Arc::new(Renderer::new(indexer.clone()));

        Self {
            root_path: Arc::new(RwLock::new(None)),
            indexer,
            renderer,
            // The watcher starts as None and is created when a vault is initialized.
            watcher: Arc::new(Mutex::new(None)),
        }
    }

    /// Initializes the world by performing a full scan of the vault directory and starting
    /// the file watcher. This is an internal method called by `change_vault`.
    /// This function modifies the interior state via locks.
    fn initialize(&self, root_path: &Path, app_handle: AppHandle) -> Result<()> {
        info!(path = %root_path.display(), "Initializing or changing vault.");

        // --- 1. Perform Initial Scan on a new Indexer instance ---
        // This is done outside of any locks to avoid blocking other operations during the scan.
        let mut new_indexer_instance = Indexer::new(root_path);
        new_indexer_instance.full_scan(root_path)?;

        // --- 2. Start File Watcher ---
        let mut new_watcher = Watcher::new();
        new_watcher.start(root_path)?;

        // --- 3. Subscribe to File Events ---
        let event_receiver = new_watcher.subscribe();

        // --- 4. Lock and Update Shared State ---
        // The lock scope is kept as short as possible.
        {
            // The watcher is replaced. The old watcher is dropped, automatically stopping its thread.
            *self.watcher.lock() = Some(new_watcher);
            *self.root_path.write() = Some(root_path.to_path_buf());
            // The fully scanned indexer replaces the old one.
            *self.indexer.write() = new_indexer_instance;
        }

        // --- 5. Spawn Background Event Processing Task ---
        // The task is given its own handle to the indexer state.
        let indexer_clone = self.indexer.clone();
        // Use Tauri's async runtime instead of tokio::spawn
        tauri::async_runtime::spawn(async move {
            Self::process_file_events(app_handle, indexer_clone, event_receiver).await;
        });

        info!(
            "World initialized successfully for path: {}",
            root_path.display()
        );
        Ok(())
    }

    /// Changes the vault path, saves the configuration, and re-initializes the world.
    pub fn change_vault(&self, path: String, app_handle: AppHandle) -> Result<()> {
        // 1. Save the new path to the configuration file.
        config::set_vault_path(path.clone(), &app_handle)?;

        // 2. Initialize the world with the new path.
        self.initialize(Path::new(&path), app_handle)
    }

    /// Background task that processes file events and updates the indexer.
    ///
    /// This runs in a separate async task and handles the event loop for file changes.
    /// It continues until the event channel is closed or an unrecoverable error occurs.
    ///
    /// # Arguments
    /// * `app_handle` - A handle to the Tauri application
    /// * `indexer` - Shared reference to the indexer
    /// * `mut event_receiver` - Receiver for file change events
    #[instrument(level = "debug", skip(app_handle, indexer, event_receiver))]
    async fn process_file_events(
        app_handle: AppHandle,
        indexer: Arc<RwLock<Indexer>>,
        mut event_receiver: broadcast::Receiver<FileEvent>,
    ) {
        loop {
            match event_receiver.recv().await {
                Ok(event) => {
                    // Scope the write lock to release it before emitting the event
                    {
                        let mut indexer = indexer.write();
                        indexer.handle_file_event(&event);
                    } // Lock is released here

                    // Emit an event to notify the frontend that the index has changed
                    if let Err(e) = app_handle.emit("index-updated", ()) {
                        tracing::error!("Failed to emit index-updated event: {}", e);
                    }
                }
                Err(broadcast::error::RecvError::Closed) => {
                    tracing::info!("Event channel closed, stopping file event processing");
                    break;
                }
                Err(broadcast::error::RecvError::Lagged(skipped)) => {
                    tracing::warn!(
                        "File event processing fell behind, skipped {} events",
                        skipped
                    );
                    // Continue processing - the indexer will eventually catch up
                }
            }
        }
        tracing::info!("File event processing task stopped");
    }

    /// Converts docx files and adds them to the vault, then updates the index.
    pub fn import_docx_files(
        &self,
        app_handle: &AppHandle,
        docx_paths: Vec<PathBuf>,
    ) -> Result<Vec<PathBuf>> {
        let output_dir = self
            .root_path
            .read()
            .clone()
            .ok_or(ChroniclerError::VaultNotInitialized)?;

        let converted_paths =
            importer::convert_docx_to_markdown(app_handle, docx_paths, output_dir)?;

        let mut indexer = self.indexer.write();
        for path in &converted_paths {
            indexer.update_file(path);
        }

        Ok(converted_paths)
    }

    // --- Data Accessors ---

    /// Returns all tags and the pages that reference them, sorted alphabetically.
    pub fn get_all_tags(&self) -> Result<Vec<(String, Vec<PageHeader>)>> {
        self.indexer.read().get_all_tags()
    }

    /// Returns the file tree structure of the vault for frontend display.
    pub fn get_file_tree(&self) -> Result<FileNode> {
        self.indexer.read().get_file_tree()
    }

    /// Processes raw markdown content and returns the fully rendered page data.
    pub fn render_page_preview(&self, content: &str) -> Result<RenderedPage> {
        // This operation does not lock the renderer, only the indexer internally for link resolution.
        self.renderer.render_page_preview(content)
    }

    /// Renders a string of pure Markdown to a `RenderedPage` object.
    /// This bypasses all wikilink and frontmatter processing.
    pub fn render_markdown(&self, markdown: &str) -> Result<RenderedPage> {
        // This is a pure function and doesn't require any state locks.
        self.renderer.render_markdown(markdown)
    }

    /// Fetches and renders all data required for the main file view.
    pub fn build_page_view(&self, path: &str) -> Result<FullPageData> {
        // The renderer handles its own internal locking of the indexer.
        self.renderer.build_page_view(path)
    }

    /// Returns a list of all directory paths in the vault.
    pub fn get_all_directory_paths(&self) -> Result<Vec<PathBuf>> {
        self.indexer.read().get_all_directory_paths()
    }

    // --- File System Operations ---

    /// Writes content to a page on disk.
    /// This method doesn't need to modify the index directly, as the file watcher
    /// will detect the change and send an event.
    pub fn write_page_content(&self, path: &str, content: &str) -> Result<()> {
        let path_buf = PathBuf::from(path);
        if let Some(parent) = path_buf.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path_buf, content).map_err(Into::into)
    }

    /// Creates a new, empty markdown file and synchronously updates the index.
    pub fn create_new_file(&self, parent_dir: String, file_name: String) -> Result<PageHeader> {
        self.indexer.write().create_new_file(parent_dir, file_name)
    }

    /// Creates a new, empty folder.
    pub fn create_new_folder(&self, parent_dir: String, folder_name: String) -> Result<()> {
        self.indexer
            .write()
            .create_new_folder(parent_dir, folder_name)
    }

    /// Renames a file or folder and synchronously updates the index.
    pub fn rename_path(&self, path: PathBuf, new_name: String) -> Result<()> {
        self.indexer.write().rename_path(path, new_name)
    }

    /// Deletes a file or folder and synchronously updates the index.
    pub fn delete_path(&self, path: PathBuf) -> Result<()> {
        self.indexer.write().delete_path(path)
    }
}

/// Provides a default, empty `World` instance.
///
/// This implementation allows for the creation of a `World` using `World::default()`.
impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
