use super::AppState;
use bevy::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            // Start the initial 2.5 second timer for the Studio screen
            .insert_resource(SplashTimer(Timer::from_seconds(2.5, TimerMode::Once)))
            // Studio Screen Runlevel
            .add_systems(OnEnter(AppState::SplashStudio), setup_studio)
            .add_systems(
                Update,
                tick_splash_timer.run_if(in_state(AppState::SplashStudio)),
            )
            .add_systems(OnExit(AppState::SplashStudio), cleanup_splash)
            // Warning Screen Runlevel
            .add_systems(OnEnter(AppState::SplashWarning), setup_warning)
            .add_systems(
                Update,
                tick_splash_timer.run_if(in_state(AppState::SplashWarning)),
            )
            .add_systems(OnExit(AppState::SplashWarning), cleanup_splash)
            // Fake Boot Sequence Runlevel
            .insert_resource(BootSequence::default())
            .add_systems(OnEnter(AppState::SplashBoot), setup_boot)
            .add_systems(
                Update,
                animate_boot_console.run_if(in_state(AppState::SplashBoot)),
            )
            .add_systems(OnExit(AppState::SplashBoot), cleanup_splash);
    }
}

// ----------------------------------------------------------------------------
// RESOURCES & COMPONENTS
// ----------------------------------------------------------------------------

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource)]
struct SplashTimer(Timer);

#[derive(Component)]
struct BootConsoleText;

#[derive(Resource)]
struct BootSequence {
    timer: Timer,
    step: usize,
    logs: Vec<&'static str>,
}

impl Default for BootSequence {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.8, TimerMode::Repeating),
            step: 0,
            logs: vec![
                "[BOOT] Initializing core modules...",
                "[BOOT] Mounting virtual filesystem...",
                "[OK] Allocating memory buffers...",
                "[WARN] Unrecognized daemon detected in sector 4...",
                "[BOOT] Bypassing security protocols...",
                "[OK] S.U.D.O. environment ready.",
            ],
        }
    }
}

// ----------------------------------------------------------------------------
// SYSTEMS
// ----------------------------------------------------------------------------

/// Progresses the timer and transitions between the static splash states.
fn tick_splash_timer(
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        match state.get() {
            AppState::SplashStudio => {
                next_state.set(AppState::SplashWarning);
                // Give them 4 seconds to read the warning text
                timer
                    .0
                    .set_duration(std::time::Duration::from_secs_f32(4.0));
                timer.0.reset();
            }
            AppState::SplashWarning => {
                next_state.set(AppState::SplashBoot);
                // We don't reset the timer here, as SplashBoot is driven by BootSequence
            }
            _ => {}
        }
    }
}

/// A generic cleanup function to unmount the UI graph between state changes.
fn cleanup_splash(mut commands: Commands, query: Query<Entity, With<OnSplashScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// ----------------------------------------------------------------------------
// UI LAYOUTS
// ----------------------------------------------------------------------------

fn setup_studio(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(180.0),
                    margin: UiRect::bottom(Val::Px(60.0)),
                    ..default()
                },
                image: asset_server.load("textures/ae_studios_logo.png").into(),
                ..default()
            });

            parent.spawn(TextBundle::from_section(
                "S.U.D.O. and all associated software, mechanics, and visual assets are the exclusive copyrighted\nproperties of Asta Epsilon Studios.",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ).with_text_justify(JustifyText::Center));
        });
}

fn setup_warning(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Warning",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 32.0,
                    color: Color::RED,
                },
            ).with_style(Style { margin: UiRect::bottom(Val::Px(20.0)), ..default() }));

            parent.spawn(TextBundle::from_section(
                "This game features flashing lights, rapid geometric transitions, and intense visual effects that may\ntrigger discomfort or seizures for photosensitive viewers. Viewer discretion is advised.",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ).with_text_justify(JustifyText::Center));
        });
}

fn setup_boot(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            // Main Title
            parent.spawn(TextBundle::from_section(
                "S.U.D.O.",
                TextStyle {
                    font: asset_server.load("fonts/Lastica.ttf"),
                    font_size: 80.0,
                    color: Color::WHITE,
                },
            ));

            // Subtitle
            parent.spawn(TextBundle::from_sections([
                TextSection::new(
                    "System User ",
                    TextStyle {
                        font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    "Deception ",
                    TextStyle {
                        font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                        font_size: 24.0,
                        color: Color::RED,
                    },
                ),
                TextSection::new(
                    "Override",
                    TextStyle {
                        font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ),
            ]));

            // ESRB Logo (Absolute Bottom Right)
            parent.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    right: Val::Px(20.0),
                    width: Val::Px(80.0),
                    ..default()
                },
                image: asset_server.load("textures/esrb_teen_logo.png").into(),
                ..default()
            });

            // Console Boot Text (Absolute Bottom Left)
            parent.spawn((
                TextBundle::from_section(
                    "> _",
                    TextStyle {
                        font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                        font_size: 16.0,
                        color: Color::rgb(0.0, 0.8, 0.0), // Terminal Green
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    left: Val::Px(20.0),
                    ..default()
                }),
                BootConsoleText,
            ));
        });
}

/// Cycles through the terminal strings and advances to the Main Menu when finished.
fn animate_boot_console(
    time: Res<Time>,
    mut boot_seq: ResMut<BootSequence>,
    mut query: Query<&mut Text, With<BootConsoleText>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if boot_seq.timer.tick(time.delta()).just_finished() {
        if let Ok(mut text) = query.get_single_mut() {
            if boot_seq.step < boot_seq.logs.len() {
                text.sections[0].value = format!("> {}", boot_seq.logs[boot_seq.step]);
                boot_seq.step += 1;
            } else {
                // Boot sequence finished, load the actual game menu
                next_state.set(AppState::AgeGate);
            }
        }
    }
}
