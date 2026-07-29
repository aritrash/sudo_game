use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::pbr::CascadeShadowConfigBuilder;
use crate::AppState;

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TutorialStage>()
            // FIX: Initialize this as an actual state machine, not a raw resource!
            .init_state::<TutorialPlayState>() 
            .add_systems(OnEnter(AppState::Tutorial), setup_tutorial_arena)
            .add_systems(Update, (
                handle_pause_toggle,
                player_look_system.run_if(in_state(TutorialPlayState::Playing)),
                pause_menu_button_system.run_if(in_state(TutorialPlayState::Paused)),
            ).run_if(in_state(AppState::Tutorial)))
            .add_systems(OnExit(AppState::Tutorial), cleanup_tutorial_arena);
    }
}

// Marker components for handling player entities and UI assets
#[derive(Component)]
pub struct PlayerController;

#[derive(Component)]
pub struct TutorialCamera;

#[derive(Component)]
struct OnTutorialLayer;

#[derive(Component)]
struct PauseMenuUI;

#[derive(Component)]
enum PauseMenuAction {
    Resume,
    Settings,
    ExitTutorial,
}

// Sub-states to cleanly manage pause overlays within the tutorial loop
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default, Resource)]
pub enum TutorialPlayState {
    #[default]
    Playing,
    Paused,
}

#[derive(Resource, Default, PartialEq, Eq, Clone, Copy)]
pub enum TutorialStage {
    #[default]
    Introduction,
    MovementWS,
    MovementAD,
    CameraLook,
    PacketPickup,
    TimerNotice,
    InfectionHazard,
    QuarantineRouting,
    Decontamination,
    ChatBriefing,
    Complete,
}

fn setup_tutorial_arena(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut play_state: ResMut<NextState<TutorialPlayState>>,
) {
    println!("[SYSTEM] Spawning Tutorial Arena Geometry and Player Rig.");
    play_state.set(TutorialPlayState::Playing);

    if let Ok(mut window) = q_windows.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }

    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/gameplay_ambient.ogg"),
            settings: PlaybackSettings::LOOP,
        },
        OnTutorialLayer,
    ));

    // --- UPGRADED AAA LIGHTING PLATFORM ---
    
    // Balanced directional lighting striking down the corridor with high-fidelity shadows
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::rgb(0.9, 0.95, 1.0), // Stark, crisp light
                illuminance: 5500.0,              // Increased intensity
                shadows_enabled: true,
                shadow_depth_bias: 0.03,          // Smooths out shadow acne artifacts
                shadow_normal_bias: 0.6,          // Prevents blocky shadow steps on seams
                ..default()
            },
            // Pointing down and slightly across the corridor to illuminate both walls smoothly
            transform: Transform::from_xyz(2.0, 10.0, 5.0).looking_at(Vec3::new(0.0, 0.0, -40.0), Vec3::Y),
            cascade_shadow_config: CascadeShadowConfigBuilder {
                first_cascade_far_bound: 7.0,
                maximum_distance: 80.0,
                ..default()
            }
            .build(),
            ..default()
        },
        OnTutorialLayer,
    ));

    // Elevate ambient lighting to eliminate pitch-black voids
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.18, 0.20, 0.25), // Cyberpunk cold blue-grey tint
        brightness: 450.0,                  // Significantly raised base floor luminosity
    });

    // --- GEOMETRY & MATERIALS ---
    let floor_mat = materials.add(StandardMaterial {
        base_color: Color::rgb(0.12, 0.12, 0.15),
        perceptual_roughness: 0.6,
        metallic: 0.2, // Gives a slight sheen under the light
        ..default()
    });
    
    let wall_mat = materials.add(StandardMaterial {
        base_color: Color::rgb(0.08, 0.08, 0.12),
        perceptual_roughness: 0.7,
        ..default()
    });

    // Hallway Floor
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(10.0, 150.0)),
            transform: Transform::from_xyz(0.0, 0.0, -50.0),
            material: floor_mat,
            ..default()
        },
        OnTutorialLayer,
    ));

    // Left Wall
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.2, 6.0, 150.0)),
            transform: Transform::from_xyz(-5.0, 3.0, -50.0),
            material: wall_mat.clone(),
            ..default()
        },
        OnTutorialLayer,
    ));

    // Right Wall
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.2, 6.0, 150.0)),
            transform: Transform::from_xyz(5.0, 3.0, -50.0),
            material: wall_mat,
            ..default()
        },
        OnTutorialLayer,
    ));

    // Player Controller Hierarchy
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 1.8, 0.0),
                ..default()
            },
            PlayerController,
            OnTutorialLayer,
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3dBundle {
                    transform: Transform::IDENTITY,
                    ..default()
                },
                TutorialCamera,
            ));
        });
}

// Dynamic mouse movement look tracking
fn player_look_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut q_player: Query<&mut Transform, (With<PlayerController>, Without<TutorialCamera>)>,
    mut q_camera: Query<&mut Transform, (With<TutorialCamera>, Without<PlayerController>)>,
) {
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        delta += event.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    let sensitivity = 0.0015; 
    let delta_yaw = -delta.x * sensitivity;
    let delta_pitch = -delta.y * sensitivity;

    if let Ok(mut player_transform) = q_player.get_single_mut() {
        player_transform.rotate_y(delta_yaw);
    }

    if let Ok(mut camera_transform) = q_camera.get_single_mut() {
        let new_rotation = camera_transform.rotation * Quat::from_rotation_x(delta_pitch);
        let raw_euler = new_rotation.to_euler(EulerRot::YXZ);
        let clamped_pitch = raw_euler.1.clamp(-1.4, 1.4);
        camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, raw_euler.0, clamped_pitch, raw_euler.2);
    }
}

// Detect Escape clicks to safely halt simulation and restore cursor inputs
fn handle_pause_toggle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<TutorialPlayState>>,
    mut play_state: ResMut<NextState<TutorialPlayState>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_pause_ui: Query<Entity, With<PauseMenuUI>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let mut window = q_windows.single_mut();
        
        match current_state.get() {
            TutorialPlayState::Playing => {
                // Shift system to paused mode and unlock cursor access
                play_state.set(TutorialPlayState::Paused);
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;

                // Spawn the sleek side panel overlay menu
                let font_mono = asset_server.load("fonts/JetBrainsMono-Regular.ttf");
                spawn_side_pause_menu(&mut commands, &font_mono);
            }
            TutorialPlayState::Paused => {
                // Re-engage execution layers and hide mouse pointer coordinates
                play_state.set(TutorialPlayState::Playing);
                window.cursor.grab_mode = CursorGrabMode::Locked;
                window.cursor.visible = false;

                // Strip the pause overlay UI entities away
                for entity in &q_pause_ui {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

fn spawn_side_pause_menu(commands: &mut Commands, font: &Handle<Font>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(320.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(30.0)),
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::rgba(0.03, 0.03, 0.05, 0.95).into(),
                z_index: ZIndex::Global(10),
                ..default()
            },
            PauseMenuUI,
            OnTutorialLayer,
        ))
        .with_children(|parent| {
            // Core Pause Diagnostics Section Label
            parent.spawn(TextBundle::from_section(
                "SYSTEM PAUSED\n===============",
                TextStyle { font: font.clone(), font_size: 22.0, color: Color::RED },
            ).with_style(Style { margin: UiRect::bottom(Val::Px(15.0)), ..default() }));

            let items = [
                ("> RESUME", PauseMenuAction::Resume),
                ("> SETTINGS", PauseMenuAction::Settings),
                ("> EXIT TUTORIAL", PauseMenuAction::ExitTutorial),
            ];

            for (label, action) in items {
                parent.spawn((
                    ButtonBundle {
                        style: Style { padding: UiRect::all(Val::Px(6.0)), ..default() },
                        background_color: Color::NONE.into(),
                        ..default()
                    },
                    action,
                )).with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        label,
                        TextStyle { font: font.clone(), font_size: 20.0, color: Color::WHITE },
                    ));
                });
            }
        });
}

// Reactive checks for mouse operations inside the Pause overlay options
fn pause_menu_button_system(
    mut interaction_query: Query<
        (&Interaction, &PauseMenuAction, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut global_state: ResMut<NextState<AppState>>,
    mut play_state: ResMut<NextState<TutorialPlayState>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    q_pause_ui: Query<Entity, With<PauseMenuUI>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (interaction, action, children) in &mut interaction_query {
        if let Ok(mut text) = text_query.get_mut(children[0]) {
            match *interaction {
                Interaction::Pressed => {
                    // Trigger snap tactile feedback sound effect loop
                    commands.spawn(AudioBundle {
                        source: asset_server.load("audio/click_button.ogg"),
                        settings: PlaybackSettings::ONCE,
                    });

                    let mut window = q_windows.single_mut();

                    // Cleanup the visual pause panel structure completely
                    for entity in &q_pause_ui {
                        commands.entity(entity).despawn_recursive();
                    }

                    match action {
                        PauseMenuAction::Resume => {
                            play_state.set(TutorialPlayState::Playing);
                            window.cursor.grab_mode = CursorGrabMode::Locked;
                            window.cursor.visible = false;
                        }
                        PauseMenuAction::ExitTutorial => {
                            // Turn mouse back into pointer before handing flow to global Main Menu workspace
                            window.cursor.grab_mode = CursorGrabMode::None;
                            window.cursor.visible = true;
                            global_state.set(AppState::MainMenu);
                        }
                        PauseMenuAction::Settings => {
                            println!("[SYSTEM DIAGNOSTIC] Settings log placeholder called.");
                        }
                    }
                }
                Interaction::Hovered => {
                    text.sections[0].style.color = Color::RED;
                }
                Interaction::None => {
                    text.sections[0].style.color = Color::WHITE;
                }
            }
        }
    }
}

fn cleanup_tutorial_arena(
    mut commands: Commands, 
    query: Query<Entity, With<OnTutorialLayer>>,
) {
    println!("[SYSTEM] Tearing down Tutorial Arena assets.");
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}