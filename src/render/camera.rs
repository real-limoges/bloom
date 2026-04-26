use crate::spatial::AABB;
use glam::Mat4;

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    target_x: f32,
    target_y: f32,
    target_zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

impl Camera {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
            target_x: 0.0,
            target_y: 0.0,
            target_zoom: 1.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let t = 1.0 - (-5.0 * dt).exp();
        self.x += (self.target_x - self.x) * t;
        self.y += (self.target_y - self.y) * t;
        self.zoom += (self.target_zoom - self.zoom) * t;
    }

    pub fn focus_on(&mut self, x: f32, y: f32, zoom: f32) {
        self.target_x = x;
        self.target_y = y;
        self.target_zoom = zoom;
    }

    /// Fit an AABB into the canvas with a fractional padding margin (e.g. 0.1 = 10%
    /// breathing room on each side). Sets targets — call `update` to converge.
    pub fn fit_to_bounds(&mut self, bounds: &AABB, canvas_w: f32, canvas_h: f32, padding: f32) {
        if canvas_w <= 0.0 || canvas_h <= 0.0 {
            return;
        }
        // A NaN in any component would propagate into the camera target and
        // freeze it there forever. Skip and keep the previous target.
        if !(bounds.min_x.is_finite()
            && bounds.min_y.is_finite()
            && bounds.max_x.is_finite()
            && bounds.max_y.is_finite())
        {
            return;
        }
        let bbox_w = bounds.width().max(1.0);
        let bbox_h = bounds.height().max(1.0);
        let scale = 1.0 + padding * 2.0;
        let zoom_x = canvas_w / (bbox_w * scale);
        let zoom_y = canvas_h / (bbox_h * scale);
        let zoom = zoom_x.min(zoom_y).clamp(0.01, 10.0);

        let (cx, cy) = bounds.center();
        self.target_x = cx;
        self.target_y = cy;
        self.target_zoom = zoom;
    }

    pub fn world_to_screen(&self, wx: f32, wy: f32, canvas_w: f64, canvas_h: f64) -> (f64, f64) {
        let sx = ((wx - self.x) * self.zoom + canvas_w as f32 / 2.0) as f64;
        let sy = ((wy - self.y) * self.zoom + canvas_h as f32 / 2.0) as f64;
        (sx, sy)
    }

    pub fn screen_to_world(&self, sx: f64, sy: f64, canvas_w: f64, canvas_h: f64) -> (f32, f32) {
        let wx = (sx as f32 - canvas_w as f32 / 2.0) / self.zoom + self.x;
        let wy = (sy as f32 - canvas_h as f32 / 2.0) / self.zoom + self.y;
        (wx, wy)
    }

    pub fn view_projection_matrix(&self, canvas_w: f32, canvas_h: f32) -> Mat4 {
        let half_w = canvas_w / (2.0 * self.zoom);
        let half_h = canvas_h / (2.0 * self.zoom);

        // bottom > top so that positive y goes downward (matching world_to_screen)
        Mat4::orthographic_rh(
            self.x - half_w,
            self.x + half_w,
            self.y + half_h,
            self.y - half_h,
            -1.0,
            1.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const W: f64 = 800.0;
    const H: f64 = 600.0;

    #[test]
    fn new_defaults() {
        let c = Camera::new();
        assert_eq!(c.x, 0.0);
        assert_eq!(c.y, 0.0);
        assert_eq!(c.zoom, 1.0);
    }

    #[test]
    fn world_to_screen_origin_maps_to_center() {
        let c = Camera::new();
        let (sx, sy) = c.world_to_screen(0.0, 0.0, W, H);
        assert!((sx - W / 2.0).abs() < 1e-4);
        assert!((sy - H / 2.0).abs() < 1e-4);
    }

    #[test]
    fn screen_to_world_roundtrip() {
        let mut c = Camera::new();
        c.x = 10.0;
        c.y = -5.0;
        c.zoom = 2.0;

        let (sx, sy) = c.world_to_screen(30.0, 40.0, W, H);
        let (wx, wy) = c.screen_to_world(sx, sy, W, H);
        assert!((wx - 30.0).abs() < 1e-3, "wx={wx}");
        assert!((wy - 40.0).abs() < 1e-3, "wy={wy}");
    }

    #[test]
    fn focus_on_sets_targets() {
        let mut c = Camera::new();
        c.focus_on(100.0, 200.0, 3.0);
        // Before update, position hasn't changed
        assert_eq!(c.x, 0.0);
        assert_eq!(c.y, 0.0);
        assert_eq!(c.zoom, 1.0);
    }

    #[test]
    fn update_moves_toward_target() {
        let mut c = Camera::new();
        c.focus_on(100.0, 0.0, 1.0);
        c.update(0.016); // ~1 frame at 60fps
        assert!(c.x > 0.0, "camera should move toward target");
        assert!(c.x < 100.0, "camera should not overshoot");
    }

    #[test]
    fn update_converges() {
        let mut c = Camera::new();
        c.focus_on(50.0, -30.0, 2.0);
        for _ in 0..1000 {
            c.update(0.016);
        }
        assert!((c.x - 50.0).abs() < 1e-2);
        assert!((c.y - -30.0).abs() < 1e-2);
        assert!((c.zoom - 2.0).abs() < 1e-2);
    }

    #[test]
    fn view_proj_origin_maps_to_clip_origin() {
        let c = Camera::new();
        let vp = c.view_projection_matrix(800.0, 600.0);
        let clip = vp * glam::Vec4::new(0.0, 0.0, 0.0, 1.0);
        assert!((clip.x / clip.w).abs() < 1e-4);
        assert!((clip.y / clip.w).abs() < 1e-4);
    }

    #[test]
    fn view_proj_zoom_halves_visible_area() {
        let mut c = Camera::new();
        let vp1 = c.view_projection_matrix(800.0, 600.0);
        let clip1 = vp1 * glam::Vec4::new(100.0, 0.0, 0.0, 1.0);

        c.zoom = 2.0;
        let vp2 = c.view_projection_matrix(800.0, 600.0);
        let clip2 = vp2 * glam::Vec4::new(100.0, 0.0, 0.0, 1.0);

        // At 2x zoom, the same world point should be further in clip space
        assert!((clip2.x / clip2.w).abs() > (clip1.x / clip1.w).abs());
    }

    #[test]
    fn fit_to_bounds_centers_and_zooms_square() {
        let mut c = Camera::new();
        let b = AABB {
            min_x: -50.0,
            min_y: -50.0,
            max_x: 50.0,
            max_y: 50.0,
        };
        // Square canvas, square bounds, no padding → zoom 1.0 and centered at 0.
        c.fit_to_bounds(&b, 100.0, 100.0, 0.0);
        assert!((c.target_x - 0.0).abs() < 1e-5);
        assert!((c.target_y - 0.0).abs() < 1e-5);
        assert!((c.target_zoom - 1.0).abs() < 1e-5, "zoom={}", c.target_zoom);
    }

    #[test]
    fn fit_to_bounds_offset_center() {
        let mut c = Camera::new();
        let b = AABB {
            min_x: 100.0,
            min_y: 200.0,
            max_x: 300.0,
            max_y: 400.0,
        };
        c.fit_to_bounds(&b, 400.0, 400.0, 0.0);
        assert!((c.target_x - 200.0).abs() < 1e-4);
        assert!((c.target_y - 300.0).abs() < 1e-4);
        // bbox 200x200 fit into 400x400 → zoom 2.0
        assert!((c.target_zoom - 2.0).abs() < 1e-4);
    }

    #[test]
    fn fit_to_bounds_aspect_takes_smaller_zoom() {
        let mut c = Camera::new();
        // Wide bounds (200x10) into tall canvas (100x200): width is the binding side.
        let b = AABB {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 200.0,
            max_y: 10.0,
        };
        c.fit_to_bounds(&b, 100.0, 200.0, 0.0);
        // zoom_x = 100/200 = 0.5, zoom_y = 200/10 = 20 → min = 0.5
        assert!((c.target_zoom - 0.5).abs() < 1e-4, "zoom={}", c.target_zoom);
    }

    #[test]
    fn fit_to_bounds_padding_shrinks_zoom() {
        let mut c_nopad = Camera::new();
        let mut c_pad = Camera::new();
        let b = AABB {
            min_x: -50.0,
            min_y: -50.0,
            max_x: 50.0,
            max_y: 50.0,
        };
        c_nopad.fit_to_bounds(&b, 100.0, 100.0, 0.0);
        c_pad.fit_to_bounds(&b, 100.0, 100.0, 0.25);
        assert!(
            c_pad.target_zoom < c_nopad.target_zoom,
            "padding should shrink zoom: no-pad={} padded={}",
            c_nopad.target_zoom,
            c_pad.target_zoom
        );
    }

    #[test]
    fn fit_to_bounds_zero_canvas_no_op() {
        let mut c = Camera::new();
        let (tx, ty, tz) = (c.target_x, c.target_y, c.target_zoom);
        let b = AABB {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 10.0,
            max_y: 10.0,
        };
        c.fit_to_bounds(&b, 0.0, 100.0, 0.0);
        assert_eq!((c.target_x, c.target_y, c.target_zoom), (tx, ty, tz));
        c.fit_to_bounds(&b, 100.0, 0.0, 0.0);
        assert_eq!((c.target_x, c.target_y, c.target_zoom), (tx, ty, tz));
    }

    #[test]
    fn fit_to_bounds_degenerate_bounds() {
        // A zero-area bbox should not divide by zero or blow up zoom.
        let mut c = Camera::new();
        let b = AABB {
            min_x: 5.0,
            min_y: 5.0,
            max_x: 5.0,
            max_y: 5.0,
        };
        c.fit_to_bounds(&b, 100.0, 100.0, 0.0);
        assert!(c.target_zoom.is_finite());
        assert!(c.target_zoom <= 10.0, "zoom clamped: {}", c.target_zoom);
        assert!((c.target_x - 5.0).abs() < 1e-5);
        assert!((c.target_y - 5.0).abs() < 1e-5);
    }

    #[test]
    fn zoom_affects_world_to_screen() {
        let mut c = Camera::new();
        c.zoom = 2.0;
        let (sx1, _) = c.world_to_screen(10.0, 0.0, W, H);

        c.zoom = 1.0;
        let (sx2, _) = c.world_to_screen(10.0, 0.0, W, H);

        // At 2x zoom, the point should be further from center than at 1x
        let center = W / 2.0;
        assert!((sx1 - center).abs() > (sx2 - center).abs());
    }
}
