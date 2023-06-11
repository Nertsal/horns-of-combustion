use super::*;

#[derive(Debug, Clone)]
pub struct ScreenShake {
    pub duration: Time,
    pub amplitude: Coord,
}

impl ScreenShake {
    pub fn new() -> Self {
        Self {
            duration: Time::ZERO,
            amplitude: Coord::ZERO,
        }
    }

    pub fn update(&mut self, delta_time: Time) {
        self.duration = (self.duration - delta_time).max(Time::ZERO);
    }

    pub fn merge(&mut self, other: Self) {
        *self = Self {
            duration: self.duration.max(other.duration),
            amplitude: self.amplitude.max(other.amplitude),
        };
    }

    pub fn get(&self) -> vec2<Coord> {
        let dir = Angle::from_degrees(r32(thread_rng().gen_range(0.0..360.0)));
        let amplitude = self.amplitude * self.duration.min(Time::ONE);
        dir.unit_vec() * amplitude
    }

    pub fn apply_to_camera(&mut self, camera: &mut Camera, delta_time: Time) {
        let velocity = self.get();
        camera.center += velocity * delta_time;
    }
}
