//! Single background writer with one replaceable pending snapshot.
use std::path::PathBuf;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

use photocraft_ui_egui::PhotocraftApp;
use crate::checkpoint::Checkpoint;

#[derive(Default)]
struct Pending {
    latest: Option<Checkpoint>,
    shutdown: bool,
    last_error: Option<String>,
}
type Shared = Arc<(Mutex<Pending>, Condvar)>;

pub(crate) struct CheckpointWorker {
    shared: Shared,
    worker: Option<JoinHandle<()>>,
}

impl CheckpointWorker {
    pub(crate) fn start(directory: PathBuf) -> std::io::Result<Self> {
        let shared: Shared = Arc::new((Mutex::new(Pending::default()), Condvar::new()));
        let copy = Arc::clone(&shared);
        let worker = thread::Builder::new().name("sudadesign-checkpoint".into())
            .spawn(move || worker_loop(copy, directory))?;
        Ok(Self { shared, worker: Some(worker) })
    }

    pub(crate) fn schedule(&self, app: &PhotocraftApp) {
        let snapshot = Checkpoint::capture(app);
        let (lock, wake) = &*self.shared;
        match lock.lock() {
            Ok(mut pending) if !pending.shutdown => {
                pending.latest = Some(snapshot);
                wake.notify_one();
            }
            Ok(_) => log::warn!("checkpoint request after shutdown"),
            Err(_) => log::error!("checkpoint queue poisoned"),
        }
    }

    /// Flush and join only during normal exit; frame callbacks never wait on I/O.
    pub(crate) fn finish(mut self, app: &PhotocraftApp) -> Result<(), String> {
        self.schedule(app);
        let (lock, wake) = &*self.shared;
        {
            let mut pending = lock.lock().map_err(|_| "checkpoint queue poisoned".to_string())?;
            pending.shutdown = true;
            wake.notify_one();
        }
        if let Some(worker) = self.worker.take() {
            worker.join().map_err(|_| "checkpoint writer panicked".to_string())?;
        }
        let state = lock.lock().map_err(|_| "checkpoint queue poisoned".to_string())?;
        state.last_error.clone().map_or(Ok(()), Err)
    }
}

fn worker_loop(shared: Shared, directory: PathBuf) {
    let (lock, wake) = &*shared;
    loop {
        let snapshot = {
            let mut pending = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            while pending.latest.is_none() && !pending.shutdown {
                pending = wake.wait(pending).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
            match pending.latest.take() {
                Some(snapshot) => snapshot,
                None if pending.shutdown => return,
                None => continue,
            }
        };
        let result = snapshot.persist(&directory);
        let mut pending = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match result {
            Ok(count) => {
                pending.last_error = None;
                log::info!("Android checkpoint saved {count} document(s)");
            }
            Err(error) => {
                log::error!("Android checkpoint failed: {error}");
                pending.last_error = Some(error);
            }
        }
    }
}
