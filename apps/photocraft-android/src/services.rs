//! Android platform services for SudaDesign.
//!
//! SAF file-dialog bridge, codec services, persistent font loading, and platform hooks — no panics.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use photocraft_codecs::{ChannelLayout, EncodeOptions, Image};
use photocraft_doc::Document;
use photocraft_ui_egui::{ExportSettings, FileDialogAnswer, FileDialogReply, FileDialogRequest, Services};

/// Asynchronous inbox queue for files received via Android intents or SAF.
pub type AndroidInbox = Arc<Mutex<Vec<(String, Vec<u8>)>>>;

// ---------------------------------------------------------------------------
// Global pending-dialog state
// ---------------------------------------------------------------------------

static PENDING_OPEN: Mutex<Option<FileDialogReply>> = Mutex::new(None);
static PENDING_SAVE: Mutex<Option<FileDialogReply>> = Mutex::new(None);

#[cfg(target_os = "android")]
static MAIN_ACTIVITY_CLASS: Mutex<Option<jni::objects::Global<jni::objects::JClass<'static>>>> = Mutex::new(None);

// ---------------------------------------------------------------------------
// JNI bootstrap
// ---------------------------------------------------------------------------

#[cfg(target_os = "android")]
#[allow(unsafe_code)]
fn init_jvm(android_app: &winit::platform::android::activity::AndroidApp) {
    // SAFETY: vm_as_ptr() is guaranteed non-null and valid by the Android runtime.
    let _vm = unsafe {
        jni::JavaVM::from_raw(android_app.vm_as_ptr().cast())
    };
}

fn cancel_pending(slot: &Mutex<Option<FileDialogReply>>) {
    if let Some(r) = slot.lock().ok().and_then(|mut g| g.take()) {
        r.send(None);
    }
}

// ---------------------------------------------------------------------------
// SAF JNI Invocation Helpers
// ---------------------------------------------------------------------------

#[cfg(target_os = "android")]
fn request_open_document_jni(multiple: bool, extensions: &[String]) -> Result<(), String> {
    let vm = jni::JavaVM::singleton().map_err(|e| e.to_string())?;
    vm.attach_current_thread(|env: &mut jni::Env<'_>| -> jni::errors::Result<()> {
        let csv = extensions.join(",");
        let j_csv = env.new_string(&csv)?;
        let method_name = jni::strings::JNIString::from("requestOpenDocument");
        let sig = jni::signature::RuntimeMethodSignature::from_str("(ZLjava/lang/String;)V")?;
        let args = [
            jni::objects::JValue::Bool(multiple),
            jni::objects::JValue::Object(j_csv.as_ref()),
        ];

        let class_guard = MAIN_ACTIVITY_CLASS.lock().ok();
        if let Some(Some(cls)) = class_guard.as_deref() {
            env.call_static_method(cls, &method_name, sig.method_signature(), &args)?;
        } else {
            let cls_name = jni::strings::JNIString::from("ai/storyteller/photocraft/MainActivity");
            let cls = env.find_class(&cls_name)?;
            env.call_static_method(cls, &method_name, sig.method_signature(), &args)?;
        }
        Ok(())
    }).map_err(|e| e.to_string())
}

#[cfg(target_os = "android")]
fn request_create_document_jni(suggested: &str, mime_type: &str) -> Result<(), String> {
    let vm = jni::JavaVM::singleton().map_err(|e| e.to_string())?;
    vm.attach_current_thread(|env: &mut jni::Env<'_>| -> jni::errors::Result<()> {
        let j_sug = env.new_string(suggested)?;
        let j_mime = env.new_string(mime_type)?;
        let method_name = jni::strings::JNIString::from("requestCreateDocument");
        let sig = jni::signature::RuntimeMethodSignature::from_str("(Ljava/lang/String;Ljava/lang/String;)V")?;
        let args = [
            jni::objects::JValue::Object(j_sug.as_ref()),
            jni::objects::JValue::Object(j_mime.as_ref()),
        ];

        let class_guard = MAIN_ACTIVITY_CLASS.lock().ok();
        if let Some(Some(cls)) = class_guard.as_deref() {
            env.call_static_method(cls, &method_name, sig.method_signature(), &args)?;
        } else {
            let cls_name = jni::strings::JNIString::from("ai/storyteller/photocraft/MainActivity");
            let cls = env.find_class(&cls_name)?;
            env.call_static_method(cls, &method_name, sig.method_signature(), &args)?;
        }
        Ok(())
    }).map_err(|e| e.to_string())
}

#[cfg(target_os = "android")]
fn write_to_content_uri_jni(uri: &str, bytes: &[u8]) -> Result<(), String> {
    let vm = jni::JavaVM::singleton().map_err(|e| e.to_string())?;
    let ok = vm.attach_current_thread(|env: &mut jni::Env<'_>| -> jni::errors::Result<bool> {
        let j_uri = env.new_string(uri)?;
        let j_bytes = env.byte_array_from_slice(bytes)?;
        let method_name = jni::strings::JNIString::from("writeToContentUri");
        let sig = jni::signature::RuntimeMethodSignature::from_str("(Ljava/lang/String;[B)Z")?;
        let args = [
            jni::objects::JValue::Object(j_uri.as_ref()),
            jni::objects::JValue::Object(j_bytes.as_ref()),
        ];

        let class_guard = MAIN_ACTIVITY_CLASS.lock().ok();
        let ret = if let Some(Some(cls)) = class_guard.as_deref() {
            env.call_static_method(cls, &method_name, sig.method_signature(), &args)?
        } else {
            let cls_name = jni::strings::JNIString::from("ai/storyteller/photocraft/MainActivity");
            let cls = env.find_class(&cls_name)?;
            env.call_static_method(cls, &method_name, sig.method_signature(), &args)?
        };
        ret.z()
    }).map_err(|e| e.to_string())?;

    if ok {
        Ok(())
    } else {
        Err(format!("ContentResolver failed to write to URI: {uri}"))
    }
}

fn mime_for_suggested(s: &str) -> &'static str {
    let lower = s.to_ascii_lowercase();
    if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else if lower.ends_with(".psd") {
        "image/vnd.adobe.photoshop"
    } else if lower.ends_with(".tif") || lower.ends_with(".tiff") {
        "image/tiff"
    } else if lower.ends_with(".pcraft") {
        "application/octet-stream"
    } else {
        "*/*"
    }
}

// ---------------------------------------------------------------------------
// JNI callbacks — exported symbols called from MainActivity
// ---------------------------------------------------------------------------

/// Called by `MainActivity.nativeInit()`.
#[cfg(target_os = "android")]
#[allow(unsafe_code, non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_ai_storyteller_photocraft_MainActivity_nativeInit<'local>(
    _env: jni::EnvUnowned<'local>,
    this: jni::objects::JObject<'local>,
) {
    log::info!("MainActivity.nativeInit invoked");
    let result: jni::errors::Result<()> = jni::JavaVM::singleton().and_then(|vm| {
        vm.attach_current_thread(|env: &mut jni::Env<'_>| {
            let cls = env.get_object_class(&this)?;
            let global_cls = env.new_global_ref(cls)?;
            if let Ok(mut g) = MAIN_ACTIVITY_CLASS.lock() {
                *g = Some(global_cls);
                log::info!("MainActivity class cached globally via nativeInit");
            }
            Ok(())
        })
    });
    if let Err(e) = result {
        log::error!("nativeInit JNI error: {e}");
    }
}

/// Called by `MainActivity.onFilePicked(name: String, bytes: ByteArray)`.
#[cfg(target_os = "android")]
#[allow(unsafe_code, non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_ai_storyteller_photocraft_MainActivity_onFilePicked<'local>(
    _env: jni::EnvUnowned<'local>,
    _this: jni::objects::JObject<'local>,
    j_name: jni::objects::JString<'local>,
    j_bytes: jni::objects::JByteArray<'local>,
) {
    let result: jni::errors::Result<(String, Vec<u8>)> =
        jni::JavaVM::singleton().and_then(|vm| {
            vm.attach_current_thread(|env: &mut jni::Env<'_>| {
                let chars = j_name.mutf8_chars(env)?;
                let name = String::from(chars);
                let bytes: Vec<u8> = env.convert_byte_array(&j_bytes)?;
                Ok((name, bytes))
            })
        });

    match result {
        Ok((name, bytes)) => {
            let reply = match PENDING_OPEN.lock().ok().and_then(|mut g| g.take()) {
                Some(r) => r,
                None => { log::warn!("onFilePicked: no pending reply"); return; }
            };
            log::info!("onFilePicked: '{}' ({} bytes)", name, bytes.len());
            reply.send(Some(FileDialogAnswer::Contents(name, bytes)));
        }
        Err(e) => {
            log::error!("onFilePicked: JNI error: {e}");
            cancel_pending(&PENDING_OPEN);
        }
    }
}

/// Called by `MainActivity.onFileDescriptorPicked(name: String, fd: Int, startOffset: Long, length: Long)`.
/// Provides zero-copy, streaming access to large files directly from the underlying Android file descriptor.
#[cfg(target_os = "android")]
#[allow(unsafe_code, non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_ai_storyteller_photocraft_MainActivity_onFileDescriptorPicked<'local>(
    _env: jni::EnvUnowned<'local>,
    _this: jni::objects::JObject<'local>,
    j_name: jni::objects::JString<'local>,
    raw_fd: jni::sys::jint,
    start_offset: jni::sys::jlong,
    length: jni::sys::jlong,
) {
    let name_res: jni::errors::Result<String> = jni::JavaVM::singleton().and_then(|vm| {
        vm.attach_current_thread(|env: &mut jni::Env<'_>| {
            let chars = j_name.mutf8_chars(env)?;
            Ok(String::from(chars))
        })
    });

    let name = match name_res {
        Ok(n) => n,
        Err(e) => {
            log::error!("onFileDescriptorPicked: failed to get string: {e}");
            if raw_fd >= 0 {
                use std::os::unix::io::FromRawFd;
                // SAFETY: Close leaked FD if JNI string retrieval fails.
                unsafe { drop(std::fs::File::from_raw_fd(raw_fd)) };
            }
            cancel_pending(&PENDING_OPEN);
            return;
        }
    };

    if raw_fd < 0 {
        log::error!("onFileDescriptorPicked: invalid fd {raw_fd}");
        cancel_pending(&PENDING_OPEN);
        return;
    }

    use std::os::unix::io::FromRawFd;
    // SAFETY: MainActivity.kt detached the file descriptor from ParcelFileDescriptor via detachFd(),
    // transferring exclusive ownership to Rust. FromRawFd consumes raw_fd and will close it when dropped.
    let file = unsafe { std::fs::File::from_raw_fd(raw_fd) };

    let start_off = if start_offset > 0 { start_offset as u64 } else { 0 };
    let actual_len = if length > 0 {
        length as u64
    } else {
        match file.metadata() {
            Ok(m) => m.len().saturating_sub(start_off),
            Err(e) => {
                log::warn!("onFileDescriptorPicked: could not query metadata len: {e}");
                0
            }
        }
    };

    let reply = match PENDING_OPEN.lock().ok().and_then(|mut g| g.take()) {
        Some(r) => r,
        None => {
            log::warn!("onFileDescriptorPicked: no pending reply");
            return;
        }
    };

    log::info!("onFileDescriptorPicked: '{}' fd={} (start_offset={}, len={})", name, raw_fd, start_off, actual_len);
    let file_source = photocraft_psd::FileSource::with_range(file, start_off, actual_len);
    let source = photocraft_psd::Source::new(file_source);
    reply.send(Some(FileDialogAnswer::Source(name, source)));
}

/// Called by `MainActivity.onFilePickCancelled()`.
#[cfg(target_os = "android")]
#[allow(unsafe_code, non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_ai_storyteller_photocraft_MainActivity_onFilePickCancelled<'local>(
    _env: jni::EnvUnowned<'local>,
    _this: jni::objects::JObject<'local>,
) {
    cancel_pending(&PENDING_OPEN);
    log::info!("onFilePickCancelled");
}

/// Called by `MainActivity.onSaveUriPicked(uri: String)`.
#[cfg(target_os = "android")]
#[allow(unsafe_code, non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_ai_storyteller_photocraft_MainActivity_onSaveUriPicked<'local>(
    _env: jni::EnvUnowned<'local>,
    _this: jni::objects::JObject<'local>,
    j_uri: jni::objects::JString<'local>,
) {
    let result: jni::errors::Result<String> =
        jni::JavaVM::singleton().and_then(|vm| {
            vm.attach_current_thread(|env: &mut jni::Env<'_>| {
                let chars = j_uri.mutf8_chars(env)?;
                Ok(String::from(chars))
            })
        });

    match result {
        Ok(uri) => {
            if let Some(r) = PENDING_SAVE.lock().ok().and_then(|mut g| g.take()) {
                log::info!("onSaveUriPicked: '{uri}'");
                r.send(Some(FileDialogAnswer::SaveTo(uri)));
            }
        }
        Err(e) => {
            log::error!("onSaveUriPicked: JNI error: {e}");
            cancel_pending(&PENDING_SAVE);
        }
    }
}

/// Backward compatibility alias for onSaveUriPicked
#[cfg(target_os = "android")]
#[allow(unsafe_code, non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_ai_storyteller_photocraft_MainActivity_onSavePathPicked<'local>(
    env: jni::EnvUnowned<'local>,
    this: jni::objects::JObject<'local>,
    j_path: jni::objects::JString<'local>,
) {
    Java_ai_storyteller_photocraft_MainActivity_onSaveUriPicked(env, this, j_path);
}

/// Called by `MainActivity.onSaveCancelled()`.
#[cfg(target_os = "android")]
#[allow(unsafe_code, non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_ai_storyteller_photocraft_MainActivity_onSaveCancelled<'local>(
    _env: jni::EnvUnowned<'local>,
    _this: jni::objects::JObject<'local>,
) {
    cancel_pending(&PENDING_SAVE);
    log::info!("onSaveCancelled");
}

/// Called by `MainActivity.onWindowInsetsChanged(left: Float, top: Float, right: Float, bottom: Float, ime: Float)`.
#[cfg(target_os = "android")]
#[allow(unsafe_code, non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_ai_storyteller_photocraft_MainActivity_onWindowInsetsChanged<'local>(
    _env: jni::EnvUnowned<'local>,
    _this: jni::objects::JObject<'local>,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    ime: f32,
) {
    log::info!("onWindowInsetsChanged: left={left}, top={top}, right={right}, bottom={bottom}, ime={ime}");
    crate::responsive::SafeAreaInsets::set(left, top, right, bottom, ime);
}

static TRIM_MEMORY_SIGNAL: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

/// Check if Android OS sent a memory pressure warning and return the level.
pub fn poll_trim_memory_signal() -> Option<i32> {
    let val = TRIM_MEMORY_SIGNAL.swap(0, std::sync::atomic::Ordering::AcqRel);
    if val > 0 {
        Some(val)
    } else {
        None
    }
}

/// Called by `MainActivity.onTrimMemorySignal(level: Int)`.
#[cfg(target_os = "android")]
#[allow(unsafe_code, non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_ai_storyteller_photocraft_MainActivity_onTrimMemorySignal<'local>(
    _env: jni::EnvUnowned<'local>,
    _this: jni::objects::JObject<'local>,
    level: i32,
) {
    log::warn!("Android OS onTrimMemorySignal received with level={level}");
    TRIM_MEMORY_SIGNAL.store(level.max(1), std::sync::atomic::Ordering::Release);
}

static THERMAL_STATUS: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

/// Check current thermal status reported by Android PowerManager (0=None, 1=Light, 2=Moderate, 3=Severe, 4=Critical).
pub fn current_thermal_status() -> i32 {
    THERMAL_STATUS.load(std::sync::atomic::Ordering::Relaxed)
}

/// Called by `MainActivity.onThermalStatusChanged(status: Int)`.
#[cfg(target_os = "android")]
#[allow(unsafe_code, non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_ai_storyteller_photocraft_MainActivity_onThermalStatusChanged<'local>(
    _env: jni::EnvUnowned<'local>,
    _this: jni::objects::JObject<'local>,
    status: i32,
) {
    log::warn!("Android OS onThermalStatusChanged: status={status}");
    THERMAL_STATUS.store(status, std::sync::atomic::Ordering::Release);
}

static APP_PAUSED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Whether the app is currently in background / paused by the OS.
pub fn is_app_paused() -> bool {
    APP_PAUSED.load(std::sync::atomic::Ordering::Relaxed)
}

/// Called by `MainActivity.onAppPauseChanged(paused: Boolean)`.
#[cfg(target_os = "android")]
#[allow(unsafe_code, non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_ai_storyteller_photocraft_MainActivity_onAppPauseChanged<'local>(
    _env: jni::EnvUnowned<'local>,
    _this: jni::objects::JObject<'local>,
    paused: bool,
) {
    APP_PAUSED.store(paused, std::sync::atomic::Ordering::Release);
    if paused {
        log::info!("App paused: freeing transient render caches to save battery and RAM");
        photocraft_compose::purge_effect_cache();
    }
}

// ---------------------------------------------------------------------------
// Persistent Fonts Loading
// ---------------------------------------------------------------------------

/// Scan persistent fonts directory and register every font into photocraft_text::shared().
pub fn load_persistent_fonts(fonts_dir: &Path) {
    let Ok(entries) = std::fs::read_dir(fonts_dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_ascii_lowercase();
            if matches!(ext.as_str(), "ttf" | "otf" | "ttc" | "otc" | "woff" | "woff2") {
                if let Ok(bytes) = std::fs::read(&path) {
                    if let Ok(mut engine) = photocraft_text::shared().lock() {
                        let names = engine.fonts.register_font_data(bytes);
                        log::info!("Loaded persistent font {:?}: {:?}", path.file_name(), names);
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// create_services
// ---------------------------------------------------------------------------

type SharedRecovery = std::rc::Rc<std::cell::RefCell<Option<photocraft_format::RecoveryStore>>>;

fn with_store<R>(store: &SharedRecovery, f: impl FnOnce(&mut photocraft_format::RecoveryStore) -> R) -> Result<R, String> {
    let mut slot = store.try_borrow_mut().map_err(|_| "crash recovery is busy".to_string())?;
    Ok(f(slot.as_mut().ok_or("no recovery directory")?))
}

/// Retrieve the app-private internal files directory.
pub fn internal_files_dir(android_app: &winit::platform::android::activity::AndroidApp) -> PathBuf {
    android_app
        .internal_data_path()
        .unwrap_or_else(|| PathBuf::from("/data/user/0/com.sudadesign.app/files"))
}

/// Construct the platform `Services` struct for Android.
pub fn create_services(
    inbox: AndroidInbox,
    place_inbox: AndroidInbox,
    android_app: winit::platform::android::activity::AndroidApp,
) -> Services {
    // Bootstrap JVM singleton — must happen before any JNI callback fires.
    #[cfg(target_os = "android")]
    init_jvm(&android_app);

    // Setup persistent fonts directory in app-private files:
    let int_dir = internal_files_dir(&android_app);
    let fonts_dir = int_dir.join("fonts");
    let _ = std::fs::create_dir_all(&fonts_dir);
    // SAFETY: Called early at startup during single-threaded initialization.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var("PHOTOCRAFT_FONTS_DIR", &fonts_dir);
    }
    load_persistent_fonts(&fonts_dir);

    // Preferences file persistence:
    let prefs_path = int_dir.join("preferences.json");
    let prefs_load_path = prefs_path.clone();
    let prefs_save_path = prefs_path.clone();

    // Crash recovery storage:
    let recovery_dir = int_dir.join("recovery");
    let _ = std::fs::create_dir_all(&recovery_dir);
    let recover_dir_copy = recovery_dir.clone();
    let store: SharedRecovery = std::rc::Rc::new(std::cell::RefCell::new(Some(
        photocraft_format::RecoveryStore::new(recovery_dir),
    )));
    let (s1, s2, s4, s5) = (store.clone(), store.clone(), store.clone(), store.clone());

    // External-files dir:
    let ext_dir: PathBuf = android_app
        .external_data_path()
        .unwrap_or_else(|| PathBuf::from(
            "/storage/emulated/0/Android/data/com.sudadesign.app/files",
        ));

    let write_ext = ext_dir.clone();
    let write_ext_save = ext_dir.clone();

    Services {
        import: Some(Box::new(|name: &str, bytes: &[u8], opts: &photocraft_io::ImportOptions| {
            photocraft_io::import_with_options(name, bytes, opts)
                .map(|r| (r.document, r.warnings))
                .map_err(|e| e.to_string())
        })),

        export: Some(Box::new(|doc: &Document, path: &str, settings: &ExportSettings| {
            photocraft_io::export(doc, path, &settings.io_options())
                .map(|r| (r.bytes, r.warnings))
                .map_err(|e| e.to_string())
        })),

        encode_png: Some(Box::new(|w: u32, h: u32, rgba: &[u8]| {
            let img = Image::from_u8(w, h, ChannelLayout::Rgba, rgba.to_vec())
                .map_err(|e| e.to_string())?;
            photocraft_codecs::encode(
                &img, photocraft_codecs::Format::Png, &EncodeOptions::default(),
            ).map_err(|e| e.to_string())
        })),

        write: Some(Box::new(move |path: &str, bytes: &[u8]| {
            if path.starts_with("content://") {
                #[cfg(target_os = "android")]
                {
                    return write_to_content_uri_jni(path, bytes);
                }
                #[cfg(not(target_os = "android"))]
                {
                    return Err("content:// URIs only supported on Android".into());
                }
            }
            let dest = resolve_path(path, &write_ext);
            if let Some(p) = dest.parent() {
                std::fs::create_dir_all(p).map_err(|e| e.to_string())?;
            }
            std::fs::write(&dest, bytes).map_err(|e| e.to_string())
        })),

        file_stamp: Some(Box::new(|path: &str| {
            let meta = std::fs::metadata(path).ok()?;
            Some(photocraft_ui_egui::FileStamp {
                len: meta.len(),
                modified: meta.modified().ok(),
            })
        })),

        file_dialog: Some(Box::new(move |request, _parent, reply| {
            match request {
                FileDialogRequest::Open { multiple, extensions, .. } => {
                    if let Ok(mut g) = PENDING_OPEN.lock() {
                        *g = Some(reply);
                    } else {
                        reply.send(None);
                        return;
                    }
                    #[cfg(target_os = "android")]
                    {
                        let exts = extensions.unwrap_or_default();
                        if let Err(e) = request_open_document_jni(multiple, &exts) {
                            log::error!("file_dialog Open JNI: {e}");
                            cancel_pending(&PENDING_OPEN);
                        }
                    }
                    #[cfg(not(target_os = "android"))]
                    {
                        cancel_pending(&PENDING_OPEN);
                    }
                }
                FileDialogRequest::Save { suggested } => {
                    #[cfg(target_os = "android")]
                    {
                        if let Ok(mut g) = PENDING_SAVE.lock() {
                            *g = Some(reply);
                        } else {
                            reply.send(None);
                            return;
                        }
                        let mime = mime_for_suggested(&suggested);
                        if let Err(e) = request_create_document_jni(&suggested, mime) {
                            log::error!("file_dialog Save JNI: {e}");
                            cancel_pending(&PENDING_SAVE);
                        }
                    }
                    #[cfg(not(target_os = "android"))]
                    {
                        let path = resolve_path(&suggested, &write_ext)
                            .to_string_lossy()
                            .into_owned();
                        log::info!("file_dialog Save → '{path}'");
                        reply.send(Some(FileDialogAnswer::SaveTo(path)));
                    }
                }
                FileDialogRequest::Folder { .. } => reply.send(None),
            }
        })),

        inbox: Some(inbox),
        place_inbox: Some(place_inbox),

        save_file: Some(Arc::new(move |doc, path, settings, ctl| {
            ctl.progress(0.0, "Encoding");
            let opts = settings.io_options();
            let cancelled = || ctl.check().is_err();
            let progress = |f: f32| ctl.progress(f * 0.90, "");
            let interrupt = photocraft_raster::Interrupt::new(&cancelled, &progress);
            let r = photocraft_io::export_with(doc, path, &opts, &interrupt)
                .map_err(|e| e.to_string())?;
            ctl.commit()?;
            ctl.progress(0.90, "Writing");
            if path.starts_with("content://") {
                #[cfg(target_os = "android")]
                {
                    write_to_content_uri_jni(path, &r.bytes)?;
                }
                #[cfg(not(target_os = "android"))]
                {
                    return Err("content:// URIs only supported on Android".into());
                }
            } else {
                let dest = resolve_path(path, &write_ext_save);
                if let Some(p) = dest.parent() {
                    let _ = std::fs::create_dir_all(p);
                }
                std::fs::write(&dest, &r.bytes).map_err(|e| e.to_string())?;
            }
            ctl.progress(1.0, "");
            Ok(r.warnings)
        })),

        load_prefs: Some(Box::new(move || {
            std::fs::read_to_string(&prefs_load_path).ok()
        })),

        save_prefs: Some(Box::new(move |text: &str| {
            photocraft_format::atomic_write(&prefs_save_path, text.as_bytes()).map_err(|e| e.to_string())
        })),

        autosave: Some(Box::new(move |doc: &Arc<Document>, revision: u64, path: Option<&str>| {
            with_store(&s1, |s| s.autosave_checked(doc, revision, path.map(str::to_string)))?
        })),

        autosave_results: Some(Box::new(move || with_store(&s4, |s| s.take_completed()).unwrap_or_default())),

        flush_autosaves: Some(Box::new(move || with_store(&s5, photocraft_format::RecoveryStore::flush).unwrap_or_default())),

        discard_autosave: Some(Box::new(move |id: u64| {
            let _ = with_store(&s2, |s| s.discard(id));
        })),

        recover: Some(Box::new(move || {
            photocraft_format::list_recovery(&recover_dir_copy)
                .into_iter()
                .map(|e| photocraft_ui_egui::Recoverable {
                    key: e.info.key.clone(),
                    name: e.info.document_name.clone(),
                    path: e.info.original_path.clone(),
                    load: Box::new(move || photocraft_format::recover(&e).map_err(|e| e.to_string())),
                })
                .collect()
        })),

        adopt_autosave: Some(Box::new(move |id: u64, key: &str| {
            let _ = with_store(&store, |s| s.adopt(id, key));
        })),

        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn resolve_path(path: &str, ext_dir: &Path) -> PathBuf {
    let p = Path::new(path);
    if p.is_absolute() { p.to_path_buf() } else { ext_dir.join(p) }
}
