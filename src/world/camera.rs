use graphics::math::Matrix2d;
use graphics::Transformed;

/// structure that correspond to a point of view on the world
pub struct Camera {
    /// the center of the point of view
	pub x: f64, 
    /// the center of the point of view
	pub y: f64, 
	pub zoom: f64,
	pub width: f64,
	pub height: f64,
}

impl Camera {
    pub fn new(x: f64, y: f64, width: f64, height: f64, zoom: f64) -> Camera {
        Camera {
            x: x,
            y: y,
            width: width,
            height: height,
            zoom: zoom,
        }
    }

	pub fn trans(&self, transform: Matrix2d) -> Matrix2d {
		transform.trans(-self.x,-self.y)
			.trans(self.width/2.,self.height/2.)
			.zoom(self.zoom)
	}
}

