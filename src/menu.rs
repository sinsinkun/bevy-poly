use bevy::prelude::*;

use crate::util::{ AppState, Sec };

// ---- PLUGIN ----
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
  fn name(&self) -> &str {
    "Menu Screen"
  }

  fn build(&self, app: &mut App) {
    app.add_system(start.in_schedule(OnEnter(AppState::Menu)))
      .add_system(update.run_if(in_state(AppState::Menu)))
      .add_system(end.in_schedule(OnExit(AppState::Menu)));
  }
}

// ---- COMPONENTS ----
#[derive(Component, Debug)]
struct Iter(i32);

// ---- SYSTEMS ----
fn start(mut commands: Commands) {
  println!("start scene 1");
  commands.spawn(Iter(0));
}

fn update(
  time: Res<Time>, 
  mut timer: ResMut<Sec>,
  mut iter_q: Query<&mut Iter>,
  mut next_state: ResMut<NextState<AppState>>
) {
  if !timer.0.tick(time.delta()).just_finished() {
    return;
  }

  println!("running scene 1");
  for mut iter in &mut iter_q {
    if iter.0 == 3 {
      next_state.set(AppState::Scene2);
    } else {
      iter.0 = iter.0 + 1;
    }
  }
}

fn end(
  mut commands: Commands, 
  q: Query<Entity, With<Iter>>
) {
  println!("end scene 1");
  // despawn things related to scene 1
  for e in &q {
    commands.entity(e).despawn();
  }
}