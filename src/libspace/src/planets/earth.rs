use crate::planets::body::Body;

pub struct Earth {}

impl Earth {
    pub fn body() -> Body {
        Body {
            radius_mean: 6371.0,
            radius_equatorial: 6378.137,
            radius_polar: 6356.752,
            mass: 5.97237e24,
            angular_speed_per_second: 7.2921150e-5,
        }
    }
}
