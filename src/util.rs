use bevy::prelude::*;

// ---- SCENES ----
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
  #[default]
  Movement,
  Menu,
}

// ---- RESOURCES ----
#[derive(Resource)]
pub struct Sec(pub Timer);

// ---- COMPONENTS ----