//! This example creates a custom [`SystemParam`] struct that counts the number of players.

use bevy::{ecs::system::SystemParam, prelude::*};

fn main() {
    App::new()
        .insert_resource(PlayerCount(0))
        .add_systems(Startup, spawn)
        .add_systems(Update, count_players)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PlayerCount(usize);

///[`SystemParam`] 结构可以包含也可以包含在
///系统函数签名。
///
///在此示例中，它包括查询和可变资源。
#[derive(SystemParam)]
struct PlayerCounter<'w, 's> {
    players: Query<'w, 's, &'static Player>,
    count: ResMut<'w, PlayerCount>,
}

impl<'w, 's> PlayerCounter<'w, 's> {
    fn count(&mut self) {
        self.count.0 = self.players.iter().len();
    }
}

/// Spawn some players to count
fn spawn(mut commands: Commands) {
    commands.spawn(Player);
    commands.spawn(Player);
    commands.spawn(Player);
}

/// The [`SystemParam`] can be used directly in a system argument.
fn count_players(mut counter: PlayerCounter) {
    counter.count();

    println!("{} players in the game", counter.count.0);
}
