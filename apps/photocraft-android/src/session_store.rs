//! Active session persistence and instant restoration for Android.
//!
//! Preserves open documents across application backgrounding, task dismissal,
//! system process kills (LMK), and reboots. Restores the user's workspace
//! and active design seamlessly on launch.
//!
//! Conforms strictly to zero-panic policy (Rule 9).

use std::path::Path;
use serde::{Deserialize, Serialize};

use photocraft_format::{PcraftWriter, SaveOptions};
use photocraft_ui_egui::PhotocraftApp;

/// Metadata for a single document persisted in the active session.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SessionDocEntry {
    pub id: u64,
    pub name: String,
    pub file_name: String,
    pub original_path: Option<String>,
    pub revision: u64,
    pub saved_revision: u64,
    pub is_dirty: bool,
}

/// Root manifest describing the persisted active workspace session.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SessionManifest {
    pub version: u32,
    pub active_index: usize,
    pub saved_at_epoch_ms: u64,
    pub documents: Vec<SessionDocEntry>,
}

/// Persist the active workspace session into `session_dir`.
///
/// Saves all open documents to `<session_dir>/doc_<id>.pcraft` and writes
/// `session_manifest.json` atomically. Cleans up any removed document files.
/// Returns the number of documents saved.
pub fn save_active_session(app: &mut PhotocraftApp, session_dir: &Path) -> Result<usize, String> {
    if let Err(e) = std::fs::create_dir_all(session_dir) {
        log::error!("session_store: failed to create session dir {:?}: {e}", session_dir);
        return Err(e.to_string());
    }

    let manifest_path = session_dir.join("session_manifest.json");
    let docs = app.session.documents();
    if docs.is_empty() {
        // No open documents: remove manifest and old pcraft files so next launch starts clean.
        if manifest_path.exists() {
            let _ = std::fs::remove_file(&manifest_path);
        }
        cleanup_orphaned_files(session_dir, &[]);
        log::info!("session_store: 0 open documents, session cleared");
        return Ok(0);
    }

    let mut entries = Vec::with_capacity(docs.len());
    let mut writer = PcraftWriter::new();
    let opts = SaveOptions::default();

    for st in docs {
        let file_name = format!("doc_{}.pcraft", st.doc.id.0);
        let doc_path = session_dir.join(&file_name);

        match writer.save_path(&st.doc, &doc_path, &opts) {
            Ok(_) => {
                entries.push(SessionDocEntry {
                    id: st.doc.id.0,
                    name: st.doc.name.clone(),
                    file_name,
                    original_path: st.path.clone(),
                    revision: st.revision,
                    saved_revision: st.saved_revision,
                    is_dirty: st.is_dirty(),
                });
            }
            Err(e) => {
                log::error!("session_store: failed to save doc {} ({}): {e}", st.doc.id.0, st.doc.name);
            }
        }
    }

    if entries.is_empty() {
        return Err("Failed to save any open documents in session".to_string());
    }

    let active_index = app.session.active_index().unwrap_or(0);
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    let manifest = SessionManifest {
        version: 1,
        active_index,
        saved_at_epoch_ms: now_ms,
        documents: entries.clone(),
    };

    let json_bytes = serde_json::to_vec_pretty(&manifest)
        .map_err(|e| format!("Failed to serialize session manifest: {e}"))?;

    photocraft_format::atomic_write(&manifest_path, &json_bytes)
        .map_err(|e| format!("Failed to write session manifest: {e}"))?;

    // Sweep old/closed .pcraft files
    let keep_files: Vec<String> = entries.into_iter().map(|e| e.file_name).collect();
    cleanup_orphaned_files(session_dir, &keep_files);

    log::info!(
        "session_store: successfully saved active session ({} docs, active: {})",
        docs.len(),
        active_index
    );
    Ok(docs.len())
}

/// Restore previously open designs from `session_dir` into `app.session`.
///
/// Returns the number of documents successfully restored.
pub fn restore_session_if_present(app: &mut PhotocraftApp, session_dir: &Path) -> usize {
    let manifest_path = session_dir.join("session_manifest.json");
    if !manifest_path.exists() {
        log::info!("session_store: no previous session manifest found at {:?}", manifest_path);
        return 0;
    }

    let manifest_str = match std::fs::read_to_string(&manifest_path) {
        Ok(s) => s,
        Err(e) => {
            log::warn!("session_store: could not read session manifest: {e}");
            return 0;
        }
    };

    let manifest: SessionManifest = match serde_json::from_str(&manifest_str) {
        Ok(m) => m,
        Err(e) => {
            log::warn!("session_store: could not parse session manifest: {e}");
            return 0;
        }
    };

    if manifest.documents.is_empty() {
        log::info!("session_store: manifest contains 0 documents");
        return 0;
    }

    let mut restored_count: usize = 0;
    for entry in &manifest.documents {
        let doc_path = session_dir.join(&entry.file_name);
        if !doc_path.exists() {
            log::warn!("session_store: document file {:?} missing on disk", doc_path);
            continue;
        }

        match photocraft_format::load_path(&doc_path) {
            Ok(mut doc) => {
                doc.name = entry.name.clone();
                let idx = app.session.add_document(doc, entry.original_path.clone());
                if let Some(st) = app.session.documents_mut().get_mut(idx) {
                    st.saved_revision = entry.saved_revision;
                    st.revision = entry.revision;
                }
                restored_count += 1;
                log::info!("session_store: restored doc '{}' (id: {})", entry.name, entry.id);
            }
            Err(e) => {
                log::error!("session_store: failed to load doc {:?}: {e}", doc_path);
            }
        }
    }

    if restored_count > 0 {
        let target_active = manifest.active_index.min(restored_count.saturating_sub(1));
        app.session.set_active(target_active);
        app.sync_recent();
        log::info!(
            "session_store: restored {} documents (active index: {})",
            restored_count,
            target_active
        );
    }

    restored_count
}

fn cleanup_orphaned_files(session_dir: &Path, keep_files: &[String]) {
    let Ok(entries) = std::fs::read_dir(session_dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.ends_with(".pcraft") && !keep_files.iter().any(|k| k == name) {
                let _ = std::fs::remove_file(&path);
                let _ = std::fs::remove_dir_all(&path);
            }
        }
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
    fn test_session_save_and_restore_cycle() {
        let temp_dir = std::env::temp_dir().join(format!("suda_test_session_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&temp_dir);

        let mut session = Session::new();
        let doc1 = Document::with_background("Design 1", Size::new(100, 100), ColorMode::Rgb, SampleType::U8, Color::WHITE);
        let doc2 = Document::with_background("Design 2", Size::new(200, 200), ColorMode::Rgb, SampleType::U8, Color::BLACK);
        session.add_document(doc1, Some("/storage/test/design1.psd".into()));
        session.add_document(doc2, None);
        session.set_active(1);

        let mut app = PhotocraftApp::new(session, Services::default());
        let saved = save_active_session(&mut app, &temp_dir).unwrap();
        assert_eq!(saved, 2);

        // Verify manifest exists and is valid
        let manifest_file = temp_dir.join("session_manifest.json");
        assert!(manifest_file.exists());

        // Create fresh app and restore
        let fresh_session = Session::new();
        let mut fresh_app = PhotocraftApp::new(fresh_session, Services::default());
        assert_eq!(fresh_app.session.documents().len(), 0);

        let restored = restore_session_if_present(&mut fresh_app, &temp_dir);
        assert_eq!(restored, 2);
        assert_eq!(fresh_app.session.documents().len(), 2);
        assert_eq!(fresh_app.session.documents()[0].doc.name, "Design 1");
        assert_eq!(fresh_app.session.documents()[0].path.as_deref(), Some("/storage/test/design1.psd"));
        assert_eq!(fresh_app.session.documents()[1].doc.name, "Design 2");
        assert_eq!(fresh_app.session.active_index(), Some(1));

        // Test clearing session when documents are closed
        let empty_session = Session::new();
        let mut empty_app = PhotocraftApp::new(empty_session, Services::default());
        let saved_empty = save_active_session(&mut empty_app, &temp_dir).unwrap();
        assert_eq!(saved_empty, 0);
        assert!(!manifest_file.exists());

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}

