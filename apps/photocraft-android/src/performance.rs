//! Thermal policy and bounded render diagnostics for Android.
use std::time::{Duration, Instant};
use photocraft_ui_egui::PhotocraftApp;

pub(crate) struct MobilePerformance {
    last_thermal: i32,
    last_fallback: Option<String>,
    last_report: Instant,
}

impl Default for MobilePerformance {
    fn default() -> Self {
        Self { last_thermal: 0, last_fallback: None, last_report: Instant::now() }
    }
}

impl MobilePerformance {
    pub(crate) fn regulate_thermal(&mut self, app: &mut PhotocraftApp, level: i32) {
        app.min_frame_ms = match level { 3.. => Some(50.0), 2 => Some(33.3), _ => None };
        if level >= 3 && self.last_thermal < 3 {
            // Repeated purges on every hot frame create long stalls.
            app.purge_memory_under_pressure(40);
        }
        if level != self.last_thermal {
            log::info!("Android thermal {} -> {}", self.last_thermal, level);
            self.last_thermal = level;
        }
    }

    pub(crate) fn report_if_due(&mut self, app: &PhotocraftApp) {
        let fallback = app.perf.gpu_fallback.clone();
        if fallback == self.last_fallback && self.last_report.elapsed() < Duration::from_secs(15) {
            return;
        }
        self.last_fallback = fallback.clone();
        self.last_report = Instant::now();
        let tiles = photocraft_raster::telemetry::snapshot_and_reset();
        log::info!(
            "mobile perf: ui={:.1}ms max_ui={:.1}ms composite={:.1}ms upload={:.1}ms gpu={:?}ms budget={}MB refresh={} px={} fallback={:?} tile_hits={} tile_misses={} decoded_MB={:.1}",
            app.perf.ui_ms, app.perf.ui_max_ms, app.perf.composite_ms,
            app.perf.upload_ms, app.perf.gpu_ms, app.perf.gpu_budget_mb,
            app.perf.last_refresh, app.perf.last_refresh_px, fallback,
            tiles.cache_hits, tiles.cache_misses, tiles.decoded_bytes as f64 / (1024.0 * 1024.0),
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn thermal_caps_are_bounded() {
        let cap = |level: i32| match level { 3.. => Some(50.0), 2 => Some(33.3), _ => None };
        assert_eq!(cap(0), None);
        assert_eq!(cap(2), Some(33.3));
        assert_eq!(cap(3), Some(50.0));
    }
}
