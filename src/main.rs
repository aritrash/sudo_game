mod splash;

use bevy::prelude::*;
use splash::SplashPlugin;

// The global runlevels for the application.
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    SplashStudio,
    SplashWarning,
    SplashBoot,
    MainMenu, 
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "S.U.D.O. - System User Deception Override".into(),
                resolution: (1280.0, 720.0).into(),
                resizable: false, 
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_plugins(SplashPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}