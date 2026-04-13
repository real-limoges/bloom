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

    pub fn snap_to(&mut self, x: f32, y: f32, zoom: f32) {
        self.x = x;
        self.y = y;
        self.zoom = zoom;
        self.target_x = x;
        self.target_y = y;
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
