use crate::AppState;
use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingTimer>()
            .add_systems(OnEnter(AppState::Loading), setup_loading_screen)
            .add_systems(
                Update,
                (tick_loading_timer, rotate_spinner_system).run_if(in_state(AppState::Loading)),
            )
            .add_systems(OnExit(AppState::Loading), cleanup_loading_screen);
    }
}

#[derive(Component)]
struct OnLoadingScreen;

#[derive(Component)]
struct SpinnerMarker;

#[derive(Resource)]
struct LoadingTimer(Timer);

impl Default for LoadingTimer {
    fn default() -> Self {
        LoadingTimer(Timer::from_seconds(2.0, TimerMode::Once))
    }
}

fn setup_loading_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_mono = asset_server.load("fonts/JetBrainsMono-Regular.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            OnLoadingScreen,
        ))
        .with_children(|parent| {
            // Bottom-Right Placement Layout Container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(40.0),
                        right: Val::Px(40.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(15.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|container| {
                    // "Please Wait" Text
                    container.spawn(TextBundle::from_section(
                        "PLEASE WAIT",
                        TextStyle {
                            font: font_mono.clone(),
                            font_size: 20.0,
                            color: Color::rgb(0.7, 0.7, 0.7),
                        },
                    ));

                    // Clean Procedural Circular Spinner Parent Node
                    container
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            SpinnerMarker,
                        ))
                        .with_children(|spinner_root| {
                            // Spawning 8 nodes in a precise dot ring mimicking a clean buffer icon
                            let dot_count = 8;
                            for i in 0..dot_count {
                                let angle =
                                    (i as f32) * (2.0 * std::f32::consts::PI / dot_count as f32);
                                let radius = 10.0;
                                let x = angle.cos() * radius;
                                let y = angle.sin() * radius;

                                // Smoothly shade the colors around the circle to create the trailing gradient
                                let alpha = (i as f32 + 1.0) / dot_count as f32;
                                let dot_color = Color::rgba(1.0, 1.0, 1.0, alpha);

                                spinner_root.spawn(NodeBundle {
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        width: Val::Px(4.0),
                                        height: Val::Px(4.0),
                                        left: Val::Px(13.0 + x), // Centers the points inside the box boundary
                                        top: Val::Px(13.0 + y),
                                        ..default()
                                    },
                                    background_color: dot_color.into(),
                                    ..default()
                                });
                            }
                        });
                });
        });
}

fn tick_loading_timer(
    time: Res<Time>,
    mut timer: ResMut<LoadingTimer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        next_state.set(AppState::MainMenu);
    }
}

fn rotate_spinner_system(time: Res<Time>, mut query: Query<&mut Transform, With<SpinnerMarker>>) {
    for mut transform in &mut query {
        // Rotates the entire dot assembly smoothly over time
        transform.rotate_z(-5.0 * time.delta_seconds());
    }
}

fn cleanup_loading_screen(mut commands: Commands, query: Query<Entity, With<OnLoadingScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
