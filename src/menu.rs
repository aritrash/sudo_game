use super::AppState;
use crate::audiomanager::music::{MusicTrack, PlayMusicEvent};
use crate::video::{spawn_video_pipeline, VideoStream};
use bevy::{app::AppExit, prelude::*};

use crate::audiomanager::sfx::{PlaySfxEvent, SoundEffect};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuScreenState>()
            .add_systems(OnEnter(AppState::MainMenu), setup_menu)
            .add_systems(
                Update,
                (menu_action_system, update_menu_buttons).run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), cleanup_menu);
    }
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct ButtonContainerMarker;

#[derive(Component)]
enum MenuButtonAction {
    Start,
    Settings,
    Credits,
    Exit,
    ConfirmYes,
    ConfirmNo,
    BackToMain,
    HowToPlay,
    CreateServer,
    JoinServer,
}

#[derive(Resource, Default, PartialEq, Eq, Clone, Copy)]
enum MenuScreenState {
    #[default]
    Main,
    ConfirmExit,
    Credits,
    StartSubMenu,
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    screen_state: Res<MenuScreenState>,
    mut play_music: EventWriter<PlayMusicEvent>,
) {
    let (video_texture, receiver, pipeline) = spawn_video_pipeline(
        &mut images,
        "assets/videos/background_loop.webm",
        1920,
        1080,
    );

    // Audio directory targeted directly
    play_music.send(PlayMusicEvent {
        track: MusicTrack::MainMenu,
    });

    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            image: video_texture.clone().into(),
            z_index: ZIndex::Global(-1),
            ..default()
        },
        VideoStream {
            receiver,
            target_image: video_texture,
            pipeline,
        },
        OnMainMenuScreen,
    ));

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
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(40.0),
                        left: Val::Px(40.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|title_container| {
                    title_container.spawn(TextBundle::from_section(
                        "S.U.D.O.",
                        TextStyle {
                            font: font_title.clone(),
                            font_size: 64.0,
                            color: Color::WHITE,
                        },
                    ));
                    title_container.spawn(
                        TextBundle::from_sections([
                            TextSection::new(
                                "System User ",
                                TextStyle {
                                    font: font_mono.clone(),
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                },
                            ),
                            TextSection::new(
                                "Deception ",
                                TextStyle {
                                    font: font_mono.clone(),
                                    font_size: 18.0,
                                    color: Color::RED,
                                },
                            ),
                            TextSection::new(
                                "Override",
                                TextStyle {
                                    font: font_mono.clone(),
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                },
                            ),
                        ])
                        .with_style(Style {
                            margin: UiRect::top(Val::Px(5.0)),
                            ..default()
                        }),
                    );
                });

            // MID LEFT / MAIN AREA: Dynamic Content Parent Node
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Percent(45.0),
                            left: Val::Px(40.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart,
                            row_gap: Val::Px(15.0),
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    },
                    ButtonContainerMarker,
                ))
                .with_children(|menu| {
                    populate_buttons(menu, &font_mono, &screen_state);
                });
        });
}

fn populate_buttons(menu: &mut ChildBuilder, font: &Handle<Font>, screen_state: &MenuScreenState) {
    match screen_state {
        MenuScreenState::Main => {
            let menu_items = [
                ("> START", MenuButtonAction::Start),
                ("> SETTINGS", MenuButtonAction::Settings),
                ("> CREDITS", MenuButtonAction::Credits),
                ("> EXIT", MenuButtonAction::Exit),
            ];
            for (label, action) in menu_items {
                spawn_menu_button(menu, font, label, action);
            }
        }
        MenuScreenState::ConfirmExit => {
            menu.spawn(TextBundle::from_section(
                "> TERMINATE SESSION?",
                TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: Color::RED,
                },
            ));

            let confirm_items = [
                ("[ YES ]", MenuButtonAction::ConfirmYes),
                ("[ NO ]", MenuButtonAction::ConfirmNo),
            ];
            for (label, action) in confirm_items {
                spawn_menu_button(menu, font, label, action);
            }
        }
        MenuScreenState::Credits => {
            menu.spawn(TextBundle::from_section(
                "SYSTEM DEVELOPMENT PROTOCOL LOG\n\
                ==============================",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::RED,
                },
            ));

            let core_team = [
                (
                    "ARITRASH SARKAR",
                    "Technical Director / Gameplay Systems / UI Architecture / Audio Design / Finite State Machines"
                ),
                (
                    "ROHEET PURKAYASTHA",
                    "Lead Multiplayer & Network Systems / Gameplay Systems / Distributed Networking"
                ),
                (
                    "SANJANA SEAL",
                    "Level Systems Engineering / Map Architecture / World Layout Design"
                ),
                (
                    "SOUMAK MUKHERJEE",
                    "Lead Technical Artist / 3D Assets / Blender / Environment Modelling"
                ),
            ];

            for (developer, systems) in core_team {
                menu.spawn(TextBundle::from_sections([
                    TextSection::new(
                        format!("> {} \n", developer),
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        format!("  {}\n", systems),
                        TextStyle {
                            font: font.clone(),
                            font_size: 14.0,
                            color: Color::rgb(0.6, 0.6, 0.6),
                        },
                    ),
                ]));
            }

            menu.spawn(
                TextBundle::from_section(
                    "\"They dictated that this space was too vast for three handles to shape.\n\
                Then the third walked away, relinquishing the keyboard.\n\
                They did not understand the nature of low-level compilation.\n\
                It does not require legions to override a system.\n\
                It only requires those who refuse to disconnect.\n\
                Two minds. One state machine. Infinite recursion.\"",
                    TextStyle {
                        font: font.clone(),
                        font_size: 13.0,
                        color: Color::rgb(0.4, 0.7, 0.5),
                    },
                )
                .with_style(Style {
                    margin: UiRect::vertical(Val::Px(10.0)),
                    ..default()
                }),
            );

            spawn_menu_button(
                menu,
                font,
                "[ ESCAPE TERMINAL / BACK ]",
                MenuButtonAction::BackToMain,
            );
        }
        MenuScreenState::StartSubMenu => {
            menu.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(30.0),
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            })
            .with_children(|row| {
                let sub_modes = [
                    (
                        "HOW TO PLAY",
                        "[ ? ]",
                        MenuButtonAction::HowToPlay,
                        "Operational Protocols",
                    ),
                    (
                        "CREATE SERVER",
                        "[ H ]",
                        MenuButtonAction::CreateServer,
                        "Host Local Node",
                    ),
                    (
                        "JOIN SERVER",
                        "[ J ]",
                        MenuButtonAction::JoinServer,
                        "Connect Remote Socket",
                    ),
                ];

                for (title, glyph, action, description) in sub_modes {
                    row.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(260.0),
                                height: Val::Px(200.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                padding: UiRect::all(Val::Px(15.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            border_color: BorderColor(Color::rgb(0.3, 0.3, 0.3)),
                            background_color: Color::rgba(0.05, 0.05, 0.05, 0.85).into(),
                            ..default()
                        },
                        action,
                    ))
                    .with_children(|panel| {
                        // Clean procedural terminal placeholder glyph
                        panel.spawn(
                            TextBundle::from_section(
                                glyph,
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 36.0,
                                    color: Color::RED,
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::bottom(Val::Px(15.0)),
                                ..default()
                            }),
                        );

                        panel
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|labels| {
                                labels.spawn(TextBundle::from_section(
                                    title,
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 18.0,
                                        color: Color::WHITE,
                                    },
                                ));
                                labels.spawn(
                                    TextBundle::from_section(
                                        description,
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 12.0,
                                            color: Color::rgb(0.5, 0.5, 0.5),
                                        },
                                    )
                                    .with_style(Style {
                                        margin: UiRect::top(Val::Px(4.0)),
                                        ..default()
                                    }),
                                );
                            });
                    });
                }
            });

            spawn_menu_button(menu, font, "[ BACK ]", MenuButtonAction::BackToMain);
        }
    }
}

fn spawn_menu_button(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    label: &str,
    action: MenuButtonAction,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            action,
        ))
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            ));
        });
}

fn menu_action_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &MenuButtonAction,
            &Children,
            Option<&mut BorderColor>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut app_exit_events: EventWriter<AppExit>,
    mut screen_state: ResMut<MenuScreenState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut play_music: EventWriter<PlayMusicEvent>,
    mut play_sfx: EventWriter<PlaySfxEvent>,
) {
    // Collect entities to safely modify text modifications later, preventing double-borrow errors
    let mut text_to_update: Option<(Entity, Color)> = None;

    for (interaction, action, children, opt_border) in &mut interaction_query {
        // Resolve target text entity safely from sub-nodes or direct layouts
        let mut target_text_entity = None;
        for child in children.iter() {
            if text_query.get(*child).is_ok() {
                target_text_entity = Some(*child);
                break;
            }
            // Dive one level deep to handle nested panel structures
            if let Ok(children_deep) = text_query.get_mut(*child) {
                let _ = children_deep; // structural safety check
            }
        }

        // Fallback to primary index if flat array structures are used
        let text_entity = target_text_entity.unwrap_or(children[0]);

        match *interaction {
            Interaction::Pressed => {
                play_sfx.send(PlaySfxEvent {
                    effect: SoundEffect::ButtonClick,
                });

                match action {
                    MenuButtonAction::Start => {
                        *screen_state = MenuScreenState::StartSubMenu;
                    }
                    MenuButtonAction::Exit => {
                        *screen_state = MenuScreenState::ConfirmExit;
                    }
                    MenuButtonAction::ConfirmYes => {
                        app_exit_events.send(AppExit::default());
                    }
                    MenuButtonAction::ConfirmNo | MenuButtonAction::BackToMain => {
                        *screen_state = MenuScreenState::Main;
                    }
                    MenuButtonAction::Credits => {
                        *screen_state = MenuScreenState::Credits;
                    }
                    MenuButtonAction::HowToPlay => {
                        *screen_state = MenuScreenState::Main;
                        play_music.send(PlayMusicEvent {
                            track: MusicTrack::Gameplay,
                        });
                        next_state.set(AppState::Tutorial);
                    }
                    MenuButtonAction::CreateServer => {
                        println!("Create Server pipeline active!");
                    }
                    MenuButtonAction::JoinServer => {
                        println!("Join Server connection socket prompt active!");
                    }
                    MenuButtonAction::Settings => {
                        println!("Settings action triggered!");
                    }
                }
            }
            Interaction::Hovered => {
                text_to_update = Some((text_entity, Color::RED));
                if let Some(mut border) = opt_border {
                    border.0 = Color::RED;
                }
            }
            Interaction::None => {
                text_to_update = Some((text_entity, Color::WHITE));
                if let Some(mut border) = opt_border {
                    border.0 = Color::rgb(0.3, 0.3, 0.3);
                }
            }
        }
    }

    // Apply the text modifications cleanly outside the query loop iteration
    if let Some((entity, color)) = text_to_update {
        if let Ok(mut text) = text_query.get_mut(entity) {
            text.sections[0].style.color = color;
        }
    }
}

fn update_menu_buttons(
    mut commands: Commands,
    screen_state: Res<MenuScreenState>,
    container_query: Query<Entity, With<ButtonContainerMarker>>,
    asset_server: Res<AssetServer>,
) {
    if screen_state.is_changed() {
        let font_mono = asset_server.load("fonts/JetBrainsMono-Regular.ttf");
        for container_entity in &container_query {
            commands.entity(container_entity).despawn_descendants();
            commands.entity(container_entity).with_children(|menu| {
                populate_buttons(menu, &font_mono, &screen_state);
            });
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<OnMainMenuScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
