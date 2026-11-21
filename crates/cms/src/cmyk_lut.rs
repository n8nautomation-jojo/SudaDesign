//! Precomputed CMYK-to-RGB lookup table for rapid colour conversion.
//!
//! Evaluates a 4-input CMYK transform into a 17×17×17×17 grid (83,521 nodes),
//! allowing instant tetrahedral or trilinear lookup without evaluating full
//! ICC curve-matrix-curve pipelines for every single pixel.

use crate::Transform;

// ── constants ────────────────────────────────────────────────────────────────

/// Default grid resolution along each of C, M, Y, K dimensions (17 points).
pub const CMYK_LUT_GRID: usize = 17;

// ── CmykLut ──────────────────────────────────────────────────────────────────

/// A precomputed 4-dimensional lookup table for fast CMYK → RGB conversion.
#[derive(Clone, Debug, PartialEq)]
pub struct CmykLut {
    /// Number of grid points along each axis (e.g. 17).
    pub size: usize,
    /// Pre-evaluated RGB node values (size^4 * 3 floats).
    pub data: Vec<[f32; 3]>,
}

impl CmykLut {
    /// Sample a 4→3 transform (CMYK → RGB) into a uniform 17^4 table.
    pub fn from_transform(t: &Transform) -> Result<Self, &'static str> {
        if t.inputs() != 4 || t.outputs() != 3 {
            return Err("CMYK LUT requires a 4-input 3-output transform");
        }
        let size = CMYK_LUT_GRID;
        let total = size * size * size * size;
        let mut data = Vec::with_capacity(total);
        let s = (size - 1) as f32;
        let mut out = [0.0f32; 16];

        for k in 0..size {
            for y in 0..size {
                for m in 0..size {
                    for c in 0..size {
                        let inp = [c as f32 / s, m as f32 / s, y as f32 / s, k as f32 / s];
                        t.eval(&inp, &mut out);
                        data.push([out[0], out[1], out[2]]);
                    }
                }
            }
        }
        Ok(Self { size, data })
    }

    /// Fast trilinear/multilinear sample of the 4D table for normalised [C, M, Y, K] in [0, 1].
    pub fn sample(&self, cmyk: [f32; 4]) -> [f32; 3] {
        let n = self.size;
        let loc = |v: f32| {
            let p = v.clamp(0.0, 1.0) * (n - 1) as f32;
            let i = (p as usize).min(n - 2);
            (i, p - i as f32)
        };
        let (c0, fc) = loc(cmyk[0]);
        let (m0, fm) = loc(cmyk[1]);
        let (y0, fy) = loc(cmyk[2]);
        let (k0, fk) = loc(cmyk[3]);

        let at = |c: usize, m: usize, y: usize, k: usize| -> [f32; 3] {
            let idx = c + m * n + y * n * n + k * n * n * n;
            self.data.get(idx).copied().unwrap_or([0.0, 0.0, 0.0])
        };

        let sample_cmy = |k: usize| -> [f32; 3] {
            let mut o = [0.0f32; 3];
            for (ch, ov) in o.iter_mut().enumerate() {
                let v = |c, m, y| at(c, m, y, k)[ch];
                let x00 = v(c0, m0, y0) + (v(c0 + 1, m0, y0) - v(c0, m0, y0)) * fc;
                let x10 = v(c0, m0 + 1, y0) + (v(c0 + 1, m0 + 1, y0) - v(c0, m0 + 1, y0)) * fc;
                let x01 = v(c0, m0, y0 + 1) + (v(c0 + 1, m0, y0 + 1) - v(c0, m0, y0 + 1)) * fc;
                let x11 = v(c0, m0 + 1, y0 + 1) + (v(c0 + 1, m0 + 1, y0 + 1) - v(c0, m0 + 1, y0 + 1)) * fc;
                let y0_val = x00 + (x10 - x00) * fm;
                let y1_val = x01 + (x11 - x01) * fm;
                *ov = y0_val + (y1_val - y0_val) * fy;
            }
            o
        };

        let lo = sample_cmy(k0);
        let hi = sample_cmy(k0 + 1);
        [
            lo[0] + (hi[0] - lo[0]) * fk,
            lo[1] + (hi[1] - lo[1]) * fk,
            lo[2] + (hi[2] - lo[2]) * fk,
        ]
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Builtin;
    use crate::Intent;

    #[test]
    fn cmyk_lut_from_builtin_transform() {
        let t = Transform::new(
            Builtin::CoatedCmyk.profile(),
            Builtin::Srgb.profile(),
            Intent::RelativeColorimetric,
            true,
        ).unwrap();
        let lut = CmykLut::from_transform(&t).unwrap();
        assert_eq!(lut.size, CMYK_LUT_GRID);
        assert_eq!(lut.data.len(), CMYK_LUT_GRID.pow(4));

        // Sample white (0, 0, 0, 0)
        let white = lut.sample([0.0, 0.0, 0.0, 0.0]);
        assert!(white[0] > 0.9 && white[1] > 0.9 && white[2] > 0.9);

        // Sample black (0, 0, 0, 1)
        let black = lut.sample([0.0, 0.0, 0.0, 1.0]);
        assert!(black[0] < 0.1 && black[1] < 0.1 && black[2] < 0.1);
    }
}

