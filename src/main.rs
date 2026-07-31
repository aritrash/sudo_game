mod splash;
mod menu;
mod video;
mod agegate;
mod loading;
mod tutorial;
mod audiomanager;
mod gameplay;

use bevy::prelude::*;
use splash::SplashPlugin;
use menu::MenuPlugin;
use bevy::render::camera::ClearColorConfig;
use bevy::window::PrimaryWindow;
use tutorial::TutorialPlugin;
use audiomanager::AudioManagerPlugin;
use bevy_kira_audio::prelude::*;
use gameplay::GameplayPlugin;

// The global runlevels for the application.
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    SplashStudio,
    SplashWarning,
    SplashBoot,
    AgeGate,
    Loading,
    MainMenu,
    Tutorial,
}

fn main() {
    // --- GSTREAMER SANITY CHECK ---
    gstreamer::init().unwrap();
    println!("GStreamer successfully linked! Version: {}", gstreamer::version_string());
    // ------------------------------
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "S.U.D.O. - System User Deception Override".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .insert_resource(Msaa::Sample4)
        .add_plugins(SplashPlugin)
        .add_plugins(agegate::AgeGatePlugin)
        .add_plugins(loading::LoadingPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(AudioManagerPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(video::VideoPlugin)
        .add_plugins(GameplayPlugin)
        .add_plugins(TutorialPlugin)
        .add_systems(Startup, (setup_camera, maximize_window))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        ..default()
    });
}

fn maximize_window(mut query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = query.get_single_mut() {
        window.set_maximized(true);
    }
}