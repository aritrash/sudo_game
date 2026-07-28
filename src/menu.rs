use bevy::prelude::*;
use super::AppState;
use crate::video::{spawn_video_pipeline, VideoStream}; 

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), setup_menu)
            .add_systems(OnExit(AppState::MainMenu), cleanup_menu);
    }
}

#[derive(Component)]
struct OnMainMenuScreen;

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    // 1. Kick off the Video Pipeline
    // Place your video in `assets/videos/background_loop.mp4`
    let (video_texture, receiver, pipeline) = spawn_video_pipeline(
        &mut images,
        "assets/videos/background_loop.webm",
        1920, 
        1080
    );

    // 2. Play the separated Audio track
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/audio.ogg"),
        settings: PlaybackSettings::LOOP,
    });

    // 3. Spawn the Video Background behind the UI
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            image: video_texture.clone().into(),
            z_index: ZIndex::Global(-1), // Forces the video to render underneath the text
            ..default()
        },
        VideoStream {
            receiver,
            target_image: video_texture,
            pipeline,
        },
        OnMainMenuScreen,
    ));

    // 4. UI Setup
    let font_title = asset_server.load("fonts/Lastica.ttf");
    let font_mono = asset_server.load("fonts/JetBrainsMono-Regular.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            // TOP LEFT: Title
            parent.spawn(NodeBundle {
                style: Style { 
                    position_type: PositionType::Absolute, 
                    top: Val::Px(40.0), 
                    left: Val::Px(40.0), 
                    flex_direction: FlexDirection::Column, 
                    ..default() 
                },
                background_color: Color::NONE.into(),
                ..default()
            }).with_children(|title_container| {
                title_container.spawn(TextBundle::from_section("S.U.D.O.", TextStyle { font: font_title.clone(), font_size: 64.0, color: Color::WHITE }));
                title_container.spawn(TextBundle::from_sections([
                    TextSection::new("System User ", TextStyle { font: font_mono.clone(), font_size: 18.0, color: Color::WHITE }),
                    TextSection::new("Deception ", TextStyle { font: font_mono.clone(), font_size: 18.0, color: Color::RED }),
                    TextSection::new("Override", TextStyle { font: font_mono.clone(), font_size: 18.0, color: Color::WHITE }),
                ]).with_style(Style { margin: UiRect::top(Val::Px(5.0)), ..default() }));
            });

            // MID LEFT: Interactive Buttons
            parent.spawn(NodeBundle {
                style: Style { 
                    position_type: PositionType::Absolute, 
                    top: Val::Percent(60.0), // Anchors just below the vertical midpoint
                    left: Val::Px(40.0), 
                    flex_direction: FlexDirection::Column, 
                    align_items: AlignItems::FlexStart, 
                    row_gap: Val::Px(15.0), 
                    ..default() 
                },
                background_color: Color::NONE.into(),
                ..default()
            }).with_children(|menu| {
                let menu_items = ["> INITIALIZE_OVERRIDE", "> SYSTEM_DIAGNOSTICS", "> TERMINATE_SESSION"];
                for item in menu_items {
                    menu.spawn(TextBundle::from_section(item, TextStyle { font: font_mono.clone(), font_size: 24.0, color: Color::WHITE }));
                }
            });
        });
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<OnMainMenuScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}