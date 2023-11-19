use super::math::{self, Mean, Vector2D, WeightedMean};
use super::settings::Settings;
use rand::Rng;
use std::iter;
use yew::{html, Html};

const BOID_RAD: f64 = 1.5;

#[derive(Clone, Debug, PartialEq)]
pub struct Boid {
    position: Vector2D,
    velocity: Vector2D,
}


impl Boid {
    pub fn new_random(settings: &Settings, bounds: &Vector2D) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            position: Vector2D::new(rng.gen::<f64>() * bounds.x, rng.gen::<f64>() * bounds.y),
            velocity: Vector2D::from_polar(rng.gen::<f64>() * math::TAU, settings.max_speed),
        }
    }

    fn coherence(&self, boids: VisibleBoidIter, factor: f64) -> Vector2D {
        Vector2D::weighted_mean(
            boids.map(|other| (other.boid.position, BOID_RAD * BOID_RAD)),
        )
        .map(|mean| (mean - self.position) * factor)
        .unwrap_or_default()
    }

    fn separation(&self, boids: VisibleBoidIter, settings: &Settings) -> Vector2D {
        let accel = boids
            .filter_map(|other| {
                if other.distance > settings.min_distance {
                    None
                } else {
                    Some(-other.offset)
                }
            })
            .sum::<Vector2D>();
        accel * settings.separation_factor
    }

    fn alignment(&self, boids: VisibleBoidIter, factor: f64) -> Vector2D {
        Vector2D::mean(boids.map(|other| other.boid.velocity))
            .map(|mean| (mean - self.velocity) * factor)
            .unwrap_or_default()
    }

    fn keep_in_bounds(&mut self, new_bounds: &Vector2D) {
        if self.position.x <= 0.0 {
            self.velocity.x = -self.velocity.x;
            self.position.x = 0.0;
        }
        if self.position.x >= new_bounds.x {
            self.velocity.x = -self.velocity.x;
            self.position.x = new_bounds.x;
        }

        if self.position.y <= 0.0 {
            self.velocity.y = -self.velocity.y;
            self.position.y = 0.0;
        }
        if self.position.y >= new_bounds.y {
            self.velocity.y = -self.velocity.y;
            self.position.y = new_bounds.y;
        }
    }

    fn update_velocity(&mut self, settings: &Settings, boids: VisibleBoidIter) {
        let v = self.velocity
            + self.coherence(boids.clone(), settings.cohesion_factor)
            + self.separation(boids.clone(), settings)
            + self.alignment(boids, settings.alignment_factor);
        self.velocity = v.clamp_magnitude(settings.max_speed);
    }

    fn update(&mut self, settings: &Settings, boids: VisibleBoidIter, new_bounds: &Vector2D) {
        self.update_velocity(settings, boids);
        self.keep_in_bounds(new_bounds);
        self.position += self.velocity;
    }

    pub fn update_all(settings: &Settings, boids: &mut [Self], new_bounds: &Vector2D) {
        for i in 0..boids.len() {
            let (before, after) = boids.split_at_mut(i);
            let (boid, after) = after.split_first_mut().unwrap();
            let visible_boids =
                VisibleBoidIter::new(before, after, boid.position, settings.visible_range);

            boid.update(settings, visible_boids, new_bounds);
        }
    }

    pub fn render(&self) -> Html {
        let cx = format!("{}", self.position.x as i32);
        let cy = format!("{}", self.position.y as i32);
        let radius = format!("{}", BOID_RAD as i32);
        html! { <circle cx={cx} cy={cy} r={radius} class="boid"/> }
    }
}

#[derive(Debug)]
struct VisibleBoid<'a> {
    boid: &'a Boid,
    offset: Vector2D,
    distance: f64,
}

#[derive(Clone, Debug)]
struct VisibleBoidIter<'boid> {
    // Pay no mind to this mess of a type.
    // It's just `before` and `after` joined together.
    it: iter::Chain<std::slice::Iter<'boid, Boid>, std::slice::Iter<'boid, Boid>>,
    position: Vector2D,
    visible_range: f64,
}

impl<'boid> VisibleBoidIter<'boid> {
    fn new(
        before: &'boid [Boid],
        after: &'boid [Boid],
        position: Vector2D,
        visible_range: f64,
    ) -> Self {
        Self {
            it: before.iter().chain(after),
            position,
            visible_range,
        }
    }
}

impl<'boid> Iterator for VisibleBoidIter<'boid> {
    type Item = VisibleBoid<'boid>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            ref mut it,
            position,
            visible_range,
        } = *self;

        it.find_map(move |other| {
            let offset = other.position - position;
            let distance = offset.magnitude();

            if distance > visible_range {
                None
            } else {
                Some(VisibleBoid {
                    boid: other,
                    offset,
                    distance,
                })
            }
        })
    }
}
