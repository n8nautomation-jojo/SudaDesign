//! SudaDesign Android native application entry point and host integration.
//!
//! Provides the cdylib entry point `android_main` invoked by Android's `GameActivity`.
//! Runs the desktop-class `PhotocraftApp` within `eframe` and `wgpu`, wrapped in
//! mobile-responsive adaptation and the SudaDesign animated splash screen.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

pub mod ime;
pub mod responsive;
pub mod services;
pub mod session_store;
pub mod splash;
mod checkpoint;
mod checkpoint_worker;
mod performance;

#[cfg(target_os = "android")]
mod app_runner {
    use std::path::PathBuf;
    use std::sync::Arc;

    use eframe::App;
    use photocraft_engine::Session;
    use photocraft_ui_egui::theme::ThemeKind;
    use photocraft_ui_egui::PhotocraftApp;

    use crate::responsive::ResponsiveState;
    use crate::services;
    use crate::splash::SplashState;
    use crate::checkpoint_worker::CheckpointWorker;
    use crate::performance::MobilePerformance;

    /// Multi-touch gesture recognizer for mobile canvas workflows.
    /// Recognizes quick multi-finger taps:
    /// - 2-finger tap = `edit.undo`
    /// - 3-finger tap = `edit.redo`
    #[derive(Default)]
    struct TouchGestureTracker {
        touches: std::collections::HashMap<u64, (std::time::Instant, egui::Pos2)>,
        max_touches: usize,
        gesture_start: Option<std::time::Instant>,
        moved_too_far: bool,
    }

    impl TouchGestureTracker {
        fn process_events(&mut self, events: &[egui::Event]) -> Option<&'static str> {
            let mut triggered = None;
            for ev in events {
                if let egui::Event::Touch { id, phase, pos, .. } = ev {
                    match phase {
                        egui::TouchPhase::Start => {
                            let now = std::time::Instant::now();
                            if self.touches.is_empty() {
                                self.gesture_start = Some(now);
                                self.max_touches = 0;
                                self.moved_too_far = false;
                            }
                            self.touches.insert(id.0, (now, *pos));
                            self.max_touches = self.max_touches.max(self.touches.len());
                        }
                        egui::TouchPhase::Move => {
                            if let Some((_, start_pos)) = self.touches.get(&id.0) {
                                let dist = (pos.x - start_pos.x).hypot(pos.y - start_pos.y);
                                if dist > 25.0 {
                                    self.moved_too_far = true;
                                }
                            }
                        }
                        egui::TouchPhase::End => {
                            self.touches.remove(&id.0);
                            if self.touches.is_empty() {
                                if let Some(start) = self.gesture_start.take() {
                                    if !self.moved_too_far && start.elapsed().as_millis() < 300 {
                                        if self.max_touches == 2 {
                                            triggered = Some("edit.undo");
                                        } else if self.max_touches == 3 {
                                            triggered = Some("edit.redo");
                                        }
                                    }
                                }
                            }
                        }
                        egui::TouchPhase::Cancel => {
                            self.touches.remove(&id.0);
                            if self.touches.is_empty() {
                                self.gesture_start = None;
                            }
                        }
                    }
                }
            }
            triggered
        }
    }

    /// Mobile-adapted wrapper around desktop `PhotocraftApp` with SudaDesign branding splash.
    ///
    /// Adds Android-specific concerns on top of the desktop app:
    /// - Animated SudaDesign splash screen on first launch.
    /// - Soft keyboard management: shows/hides the Android IME when the Type tool
    ///   is active or any text widget has focus, so the user can actually type.
    /// - Mobile-responsive layout adaptation.
    pub struct AndroidPhotocraftApp {
        inner: PhotocraftApp,
        responsive: ResponsiveState,
        splash: SplashState,
        ime: crate::ime::AndroidImeBridge,
        touch_tracker: TouchGestureTracker,
        was_paused: bool,
        /// Cloned handle to the Android activity — allows `show_soft_input` / `hide_soft_input`.
        android_app: winit::platform::android::activity::AndroidApp,
        session_dir: PathBuf,
        last_periodic_save: std::time::Instant,
        checkpoint_worker: Option<CheckpointWorker>,
        performance: MobilePerformance,
    }

    impl AndroidPhotocraftApp {
        pub fn new(
            app: PhotocraftApp,
            ctx: &egui::Context,
            android_app: winit::platform::android::activity::AndroidApp,
            session_dir: PathBuf,
        ) -> Self {
            Self {
                inner: app,
                responsive: ResponsiveState::default(),
                splash: SplashState::new(ctx),
                ime: crate::ime::AndroidImeBridge::new(),
                touch_tracker: TouchGestureTracker::default(),
                was_paused: false,
                android_app,
                session_dir: session_dir.clone(),
                last_periodic_save: std::time::Instant::now(),
                checkpoint_worker: CheckpointWorker::start(session_dir).ok(),
                performance: MobilePerformance::default(),
            }
        }
    }

    impl App for AndroidPhotocraftApp {
        fn on_exit(&mut self) {
            log::info!("App on_exit: saving active session and preferences...");
            let saved = self.checkpoint_worker.take()
                .is_some_and(|worker| worker.finish(&self.inner).is_ok());
            if !saved {
                let _ = crate::session_store::save_active_session(&mut self.inner, &self.session_dir);
            }
            let _ = self.inner.save_preferences();
            self.inner.on_exit();
        }

        fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
            // When app is in background, sleep to consume 0% CPU and do not run frame logic.
            let is_paused = services::is_app_paused();
            if is_paused {
                if !self.was_paused {
                    // Just transitioned to background: save active session & preferences, then purge transient caches
                    log::info!("App backgrounded: saving active session and preferences before RAM purge...");
                    if let Some(worker) = self.checkpoint_worker.as_ref() {
                        worker.schedule(&self.inner);
                    } else {
                        let _ = crate::session_store::save_active_session(&mut self.inner, &self.session_dir);
                    }
                    let _ = self.inner.save_preferences();
                    self.inner.purge_memory_under_pressure(60);
                    self.was_paused = true;
                }
                std::thread::sleep(std::time::Duration::from_millis(80));
                return;
            } else if self.was_paused {
                // Resumed from background: wake up UI.
                self.was_paused = false;
                ctx.request_repaint();
            }

            // While the splash is still animating, skip the inner app logic to avoid flicker.
            if !self.splash.is_done() {
                return;
            }

            // Periodic autosave of active session (every 60s if any document is dirty) to protect against sudden battery cutoff
            let now = std::time::Instant::now();
            if now.duration_since(self.last_periodic_save).as_secs() >= 60 {
                self.last_periodic_save = now;
                let any_dirty = self.inner.session.documents().iter().any(|d| d.is_dirty());
                if any_dirty {
                    if let Some(worker) = self.checkpoint_worker.as_ref() {
                        worker.schedule(&self.inner);
                    } else {
                        let _ = crate::session_store::save_active_session(&mut self.inner, &self.session_dir);
                    }
                }
            }

            // Check if Android OS reported a low memory warning
            if let Some(level) = services::poll_trim_memory_signal() {
                log::warn!("Android OS low memory (level {level}): purging GPU/CPU caches and trimming history.");
                self.inner.purge_memory_under_pressure(level);
                ctx.request_repaint();
            }

            self.performance.regulate_thermal(&mut self.inner, services::current_thermal_status());

            // Apply mobile responsive layout adaptation before logic pass.
            self.responsive.adapt(&mut self.inner, ctx);

            // Synchronize Safe Area & IME Insets into egui context
            let insets = crate::responsive::SafeAreaInsets::current();
            photocraft_ui_egui::safe_area::set_safe_area_insets(
                ctx,
                photocraft_ui_egui::safe_area::SafeAreaInsets {
                    left: insets.left,
                    top: insets.top,
                    right: insets.right,
                    bottom: insets.bottom,
                    ime: insets.ime,
                },
            );
            if insets.ime == 0.0
                && self.ime.keyboard_visible
                && !ctx.egui_wants_keyboard_input()
                && self.inner.ui.text_edit.is_none()
                && ctx.memory(|m| m.focused()).is_none()
            {
                self.ime.keyboard_visible = false;
                self.inner.ui.shell.soft_keyboard_requested = false;
            }

            self.inner.logic(ctx, frame);
            self.performance.report_if_due(&self.inner);
        }

        fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
            if services::is_app_paused() || !self.splash.is_done() {
                return;
            }

            // Detect multi-touch tap gestures:
            // 2-finger tap = undo (edit.undo)
            // 3-finger tap = redo (edit.redo)
            if let Some(cmd) = self.touch_tracker.process_events(&raw_input.events) {
                log::info!("Multi-touch tap gesture detected: triggering {cmd}");
                let _ = self.inner.run(cmd, serde_json::json!({}));
                ctx.request_repaint();
            }

            // Synchronize Android virtual keyboard (IME) input into egui raw_input.events
            let has_type_edit = self.inner.ui.text_edit.is_some();
            self.ime.sync(
                &self.android_app,
                ctx,
                raw_input,
                has_type_edit,
                &mut self.inner.ui.shell,
            );

            self.inner.raw_input_hook(ctx, raw_input);
        }

        fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
            if services::is_app_paused() {
                return;
            }

            // Draw the animated SudaDesign splash overlay; when finished it becomes transparent.
            if self.splash.draw(ui) {
                return;
            }

            let insets = crate::responsive::SafeAreaInsets::current();
            photocraft_ui_egui::safe_area::set_safe_area_insets(
                ui.ctx(),
                photocraft_ui_egui::safe_area::SafeAreaInsets {
                    left: insets.left,
                    top: insets.top,
                    right: insets.right,
                    bottom: insets.bottom,
                    ime: insets.ime,
                },
            );
            let tokens = photocraft_ui_egui::theme::Tokens::get(ui.ctx());
            let chrome_fill = tokens.chrome;

            // 1. Top Safe Area (System status bar + top camera cutout)
            if insets.top > 0.0 {
                egui::Panel::top("android_safe_top")
                    .exact_size(insets.top)
                    .resizable(false)
                    .frame(egui::Frame::NONE.fill(chrome_fill))
                    .show(ui, |_| {});
            }

            // 2. Bottom Area: Soft Keyboard Reserve (when IME is active) OR System Navigation Bar
            if insets.ime > 0.0 {
                egui::Panel::bottom("android_ime_reserve")
                    .exact_size(insets.ime)
                    .resizable(false)
                    .frame(egui::Frame::NONE.fill(egui::Color32::TRANSPARENT))
                    .show(ui, |_| {});
            } else if insets.bottom > 0.0 {
                egui::Panel::bottom("android_safe_bottom")
                    .exact_size(insets.bottom)
                    .resizable(false)
                    .frame(egui::Frame::NONE.fill(chrome_fill))
                    .show(ui, |_| {});
            }

            // 3. Left Safe Area (Punch hole, notch, or left navigation bar)
            if insets.left > 0.0 {
                egui::Panel::left("android_safe_left")
                    .exact_size(insets.left)
                    .resizable(false)
                    .frame(egui::Frame::NONE.fill(chrome_fill))
                    .show(ui, |_| {});
            }

            // 4. Right Safe Area (Right camera cutout or right navigation bar)
            if insets.right > 0.0 {
                egui::Panel::right("android_safe_right")
                    .exact_size(insets.right)
                    .resizable(false)
                    .frame(egui::Frame::NONE.fill(chrome_fill))
                    .show(ui, |_| {});
            }

            self.inner.ui(ui, frame);
        }
    }

    // SAFETY: Entry point invoked by Android GameActivity JNI loader.
    // The AndroidApp handle is guaranteed valid by the Android runtime for the duration of the activity.
    #[allow(unsafe_code)]
    #[unsafe(no_mangle)]
    pub fn android_main(app: winit::platform::android::activity::AndroidApp) {
        android_logger::init_once(
            android_logger::Config::default().with_max_level(log::LevelFilter::Info),
        );
        log::info!("SudaDesign Android native entry point starting");

        // Initialize global Rayon thread pool optimized for mobile big.LITTLE ARM architecture.
        // On 8-core mobile SoCs (e.g. 4 big + 4 LITTLE cores), spawning 8 threads pins all cores,
        // causing thermal throttling, scheduling stragglers on slow LITTLE cores, and battery drain.
        // Capping to 2-4 threads keeps the phone cool, ensures high IPC on performance cores,
        // and leaves efficiency cores free for Android OS, audio, and SurfaceFlinger.
        let num_cpus = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
        let mobile_workers = (num_cpus / 2).clamp(2, 4);
        let _ = rayon::ThreadPoolBuilder::new()
            .num_threads(mobile_workers)
            .thread_name(|i| format!("sudadesign-worker-{i}"))
            .build_global();

        let mut options = eframe::NativeOptions {
            android_app: Some(app.clone()),
            ..Default::default()
        };
        photocraft_ui_egui::gpu_canvas::use_adapter_limits(&mut options.wgpu_options.wgpu_setup);

        let inbox = Arc::default();
        let place_inbox = Arc::default();
        let s = services::create_services(inbox, place_inbox, app.clone());
        let session = Session::new();

        // Setup session directory for active documents persistence & recovery
        let int_dir = services::internal_files_dir(&app);
        let session_dir = int_dir.join("session");
        let session_dir_for_wrapper = session_dir.clone();

        // Keep a clone of the app handle so `AndroidPhotocraftApp` can show/hide the keyboard.
        let android_app_for_ui = app.clone();

        let run_res = eframe::run_native(
            "SudaDesign",
            options,
            Box::new(move |cc| {
                PhotocraftApp::install_type_reset_hooks(&cc.egui_ctx);
                PhotocraftApp::setup_context(&cc.egui_ctx, ThemeKind::Pro);
                let mut inner = PhotocraftApp::new(session, s);
                inner.background_jobs = true;
                inner.set_theme(&cc.egui_ctx, ThemeKind::Pro);
                if let Some(rs) = cc.wgpu_render_state.clone() {
                    inner.set_wgpu(rs);
                }

                // Restore active workspace session from previous run
                let restored = crate::session_store::restore_session_if_present(&mut inner, &session_dir);
                log::info!("SudaDesign startup: restored {restored} active document(s) from previous session");
                inner.sync_recent();

                let wrapper = AndroidPhotocraftApp::new(
                    inner,
                    &cc.egui_ctx,
                    android_app_for_ui,
                    session_dir_for_wrapper,
                );
                Ok(Box::new(wrapper))
            }),
        );

        if let Err(e) = run_res {
            log::error!("SudaDesign Android native app terminated with error: {e}");
        }
    }
}

#[cfg(target_os = "android")]
pub use app_runner::android_main;

#[cfg(not(target_os = "android"))]
pub fn check_compilation() -> bool {
    true
}
