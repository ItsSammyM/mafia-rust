use std::{ops::{Deref, DerefMut}};
use crate::game::phase::PhaseType;
use super::player::Player;

type ResetFunction<T> = fn(&Player) -> T;

pub struct PhaseResetting<T: Copy> {
    value: T,
    default: ResetFunction<T>,
    reset_phase: PhaseType,
}

impl<T: Copy> PhaseResetting<T> {
    pub fn new(initial: T, default: ResetFunction<T>, reset_phase: PhaseType) -> Self {
        PhaseResetting {
            value: initial,
            default,
            reset_phase,
        }
    }

    pub fn reset(&mut self, player: &Player, phase: PhaseType) {
        if self.reset_phase == phase {
            self.value = (self.default)(player);
        }
    }
}

impl<T: Copy> Deref for PhaseResetting<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Copy> DerefMut for PhaseResetting<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
