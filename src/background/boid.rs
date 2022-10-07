use super::math::{self, Mean, Vector2D, WeightedMean};
use super::settings::Settings;
use super::simulation::SIZE;
use rand::Rng;
use std::iter;
use yew::{html, Html};

#[derive(Clone, Debug, PartialEq)]
pub struct Boid {
    position: Vector2D,
    velocity: Vector2D,
    radius: f64,
}

impl Boid {
    pub fn new_random(settings: &Settings) -> Self {
        let mut rng = rand::thread_rng();

        let radius = 2.0;

        Self {
            position: Vector2D::new(rng.gen::<f64>() * SIZE.x, rng.gen::<f64>() * SIZE.y),
            velocity: Vector2D::from_polar(rng.gen::<f64>() * math::TAU, settings.max_speed),
            radius,
        }
    }

    fn coherence(&self, boids: VisibleBoidIter, factor: f64) -> Vector2D {
        Vector2D::weighted_mean(
            boids.map(|other| (other.boid.position, other.boid.radius * other.boid.radius)),
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

    fn keep_in_bounds(&mut self, settings: &Settings) {
        let min = SIZE * settings.border_margin;
        let max = SIZE - min;

        let mut v = Vector2D::default();

        let turn_speed = self.velocity.magnitude() * settings.turn_speed_ratio;
        let pos = self.position;
        if pos.x < min.x {
            v.x += turn_speed;
        }
        if pos.x > max.x {
            v.x -= turn_speed
        }

        if pos.y < min.y {
            v.y += turn_speed;
        }
        if pos.y > max.y {
            v.y -= turn_speed;
        }

        self.velocity += v;
    }

    fn update_velocity(&mut self, settings: &Settings, boids: VisibleBoidIter) {
        let v = self.velocity
            + self.coherence(boids.clone(), settings.cohesion_factor)
            + self.separation(boids.clone(), settings)
            + self.alignment(boids, settings.alignment_factor);
        self.velocity = v.clamp_magnitude(settings.max_speed);
    }

    fn update(&mut self, settings: &Settings, boids: VisibleBoidIter) {
        self.update_velocity(settings, boids);
        self.keep_in_bounds(settings);
        self.position += self.velocity;
    }

    pub fn update_all(settings: &Settings, boids: &mut [Self]) {
        for i in 0..boids.len() {
            let (before, after) = boids.split_at_mut(i);
            let (boid, after) = after.split_first_mut().unwrap();
            let visible_boids =
                VisibleBoidIter::new(before, after, boid.position, settings.visible_range);

            boid.update(settings, visible_boids);
        }
    }

    pub fn render(&self) -> Html {
        let color = "hsl(0rad, 50%, 0%)";

        let cx = format!("{}", self.position.x);
        let cy = format!("{}", self.position.y);
        let radius = format!("{}", self.radius);
        html! { <circle cx={cx} cy={cy} r={radius} fill={color} /> }
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