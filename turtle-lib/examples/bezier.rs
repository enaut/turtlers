//! Cubic Bézier curve example
//! <https://en.wikipedia.org/wiki/B%C3%A9zier_curve>

use turtle_lib::{turtle_main, vec2};

struct CubicBezier {
    point0: (f32, f32),
    point1: (f32, f32),
    point2: (f32, f32),
    point3: (f32, f32),
}

impl CubicBezier {
    /// Returns the value of this curve at the given parameter t (0.0 to 1.0)
    pub fn at(&self, t: f64) -> (f32, f32) {
        let t = t as f32;
        let mt = 1.0 - t; // (1 - t)

        // Cubic Bézier formula from Wikipedia
        let p0_weight = mt.powi(3);
        let p1_weight = 3.0 * mt.powi(2) * t;
        let p2_weight = 3.0 * mt * t.powi(2);
        let p3_weight = t.powi(3);

        (
            self.point0.0 * p0_weight
                + self.point1.0 * p1_weight
                + self.point2.0 * p2_weight
                + self.point3.0 * p3_weight,
            self.point0.1 * p0_weight
                + self.point1.1 * p1_weight
                + self.point2.1 * p2_weight
                + self.point3.1 * p3_weight,
        )
    }
}

#[turtle_main("Bézier Curve")]
fn draw(turtle: &mut TurtlePlan) {
    let curve = CubicBezier {
        point0: (-200.0, -100.0),
        point1: (-100.0, 400.0),
        point2: (100.0, -500.0),
        point3: (300.0, 200.0),
    };

    let start = curve.at(0.0);
    turtle.pen_up().go_to(vec2(start.0, start.1)).pen_down();

    let samples = 100;
    for i in 0..samples {
        let t = f64::from(i) / f64::from(samples);
        let point = curve.at(t);
        turtle.go_to(vec2(point.0, point.1));
    }
}
