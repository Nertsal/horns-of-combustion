use super::*;

pub type Hp = R32;

#[derive(Debug, Clone)]
pub struct Health {
    pub hp: Hp,
    pub max_hp: Hp,
}

impl Health {
    pub fn new(hp: impl Float) -> Self {
        let hp = hp.as_r32();
        Self { hp, max_hp: hp }
    }

    /// Returns whether current hp is at or above max hp.
    pub fn is_full(&self) -> bool {
        self.hp > self.max_hp
    }

    pub fn ratio(&self) -> R32 {
        if self.max_hp.approx_eq(&Hp::ZERO) {
            R32::ZERO
        } else {
            self.hp / self.max_hp
        }
    }

    pub fn damage(&mut self, hp: Hp) {
        self.change_hp(-hp)
    }

    pub fn heal(&mut self, hp: Hp) {
        self.change_hp(hp)
    }

    fn change_hp(&mut self, hp: Hp) {
        self.hp = (self.hp + hp).clamp(Hp::ZERO, self.max_hp);
    }

    pub fn is_alive(&self) -> bool {
        self.hp > Hp::ZERO
    }

    pub fn is_dead(&self) -> bool {
        !self.is_alive()
    }
}
