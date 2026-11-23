//! Immutable Android session checkpoints: publish manifest after all file writes.
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use photocraft_doc::Document;
use photocraft_format::{PcraftWriter, SaveOptions};
use photocraft_ui_egui::PhotocraftApp;
use crate::session_store::{SessionDocEntry, SessionManifest, cleanup_orphaned_files};

static SERIAL: AtomicU64 = AtomicU64::new(0);

pub(crate) struct Checkpoint {
    documents: Vec<(Arc<Document>, SessionDocEntry)>,
    active_index: usize,
}

impl Checkpoint {
    /// Shallow snapshot: no disk I/O or serialization on the UI thread.
    pub(crate) fn capture(app: &PhotocraftApp) -> Self {
        let documents = app.session.documents().iter().map(|state| {
            let entry = SessionDocEntry {
                id: state.doc.id.0,
                name: state.doc.name.clone(),
                file_name: String::new(),
                original_path: state.path.clone(),
                revision: state.revision,
                saved_revision: state.saved_revision,
                is_dirty: state.is_dirty(),
            };
            (Arc::clone(&state.doc), entry)
        }).collect();
        Self { documents, active_index: app.session.active_index().unwrap_or(0) }
    }

    pub(crate) fn persist(self, directory: &Path) -> Result<usize, String> {
        std::fs::create_dir_all(directory).map_err(|e| format!("checkpoint directory: {e}"))?;
        let manifest_path = directory.join("session_manifest.json");
        if self.documents.is_empty() {
            if manifest_path.exists() {
                std::fs::remove_file(&manifest_path).map_err(|e| format!("remove manifest: {e}"))?;
            }
            cleanup_orphaned_files(directory, &[]);
            return Ok(0);
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
        let batch = format!("{}-{}", now.as_nanos(), SERIAL.fetch_add(1, Ordering::Relaxed));
        let mut writer = PcraftWriter::new();
        let options = SaveOptions::default();
        let mut entries = Vec::with_capacity(self.documents.len());

        for (index, (doc, mut entry)) in self.documents.into_iter().enumerate() {
            // Never overwrite files referenced by the previously committed manifest.
            entry.file_name = format!("doc_{}_{}_{}.pcraft", entry.id, batch, index);
            writer.save_path(&doc, &directory.join(&entry.file_name), &options)
                .map_err(|e| format!("checkpoint document {}: {e}", entry.id))?;
            entries.push(entry);
        }

        let count = entries.len();
        let manifest = SessionManifest {
            version: 1,
            active_index: self.active_index.min(count.saturating_sub(1)),
            saved_at_epoch_ms: now.as_millis().min(u64::MAX as u128) as u64,
            documents: entries.clone(),
        };
        let bytes = serde_json::to_vec_pretty(&manifest).map_err(|e| format!("serialize manifest: {e}"))?;
        photocraft_format::atomic_write(&manifest_path, &bytes)
            .map_err(|e| format!("publish checkpoint manifest: {e}"))?;

        // Only now may old checkpoint files be deleted.
        let keep = entries.into_iter().map(|e| e.file_name).collect::<Vec<_>>();
        cleanup_orphaned_files(directory, &keep);
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use photocraft_color::{Color, ColorMode, SampleType};
    use photocraft_doc::Size;
    use photocraft_engine::Session;
    use photocraft_ui_egui::Services;

    #[test]
    fn checkpoint_manifest_is_replaced_only_after_file_save() {
        let dir = std::env::temp_dir().join(format!("sudadesign_checkpoint_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let mut session = Session::new();
        session.add_document(Document::with_background(
            "Test", Size::new(64, 64), ColorMode::Rgb, SampleType::U8, Color::WHITE,
        ), None);
        let app = PhotocraftApp::new(session, Services::default());
        assert_eq!(Checkpoint::capture(&app).persist(&dir).unwrap_or(0), 1);
        let first = std::fs::read(dir.join("session_manifest.json")).unwrap_or_default();
        assert!(!first.is_empty());
        assert_eq!(Checkpoint::capture(&app).persist(&dir).unwrap_or(0), 1);
        let second = std::fs::read(dir.join("session_manifest.json")).unwrap_or_default();
        assert_ne!(first, second);
        let _ = std::fs::remove_dir_all(dir);
    }
}
