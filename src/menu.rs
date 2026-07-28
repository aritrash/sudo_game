use bevy::{app::AppExit, prelude::*};
use super::AppState;
use crate::video::{spawn_video_pipeline, VideoStream}; 

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MenuScreenState>()
            .add_systems(OnEnter(AppState::MainMenu), setup_menu)
            .add_systems(Update, (menu_action_system, update_menu_buttons).run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), cleanup_menu);
    }
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct ButtonContainerMarker; // Isolated marker just for the button wrapper

#[derive(Component)]
enum MenuButtonAction {
    Start,
    Settings,
    Credits,
    Exit,
    ConfirmYes,
    ConfirmNo,
    BackToMain,
}

#[derive(Resource, Default, PartialEq, Eq, Clone, Copy)]
enum MenuScreenState {
    #[default]
    Main,
    ConfirmExit,
    Credits,
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    screen_state: Res<MenuScreenState>,
) {
    let (video_texture, receiver, pipeline) = spawn_video_pipeline(
        &mut images,
        "assets/videos/background_loop.webm",
        1920, 
        1080
    );

    // Audio spawns ONCE and stays active
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/audio.ogg"),
        settings: PlaybackSettings::LOOP,
    });

    // Video background spawns ONCE
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
            // TOP LEFT: Title (Never moves or restarts)
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

            // MID LEFT: Isolated Interactive Buttons Container
            parent.spawn((
                NodeBundle {
                    style: Style { 
                        position_type: PositionType::Absolute, 
                        top: Val::Percent(45.0), // Marginally raised to anchor the philosophy statement panel
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
            )).with_children(|menu| {
                populate_buttons(menu, &font_mono, &screen_state);
            });
        });
}

// Helper to fill the button container based on state
fn populate_buttons(
    menu: &mut ChildBuilder,
    font: &Handle<Font>,
    screen_state: &MenuScreenState,
) {
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
                TextStyle { font: font.clone(), font_size: 24.0, color: Color::RED },
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
                TextStyle { font: font.clone(), font_size: 20.0, color: Color::RED },
            ));

            // Core Technical Execution Team
            let core_team = [
                ("ARITRASH SARKAR", "UI Architecture / Gameplay Mechanics / Sound Design / Finite State Machines"),
                ("ROHEET PURKAYASTHA", "Distributed Network Engineering / Map Topography / Character Design Infrastructure"),
            ];

            for (developer, systems) in core_team {
                menu.spawn(TextBundle::from_sections([
                    TextSection::new(format!("> {} \n", developer), TextStyle { font: font.clone(), font_size: 18.0, color: Color::WHITE }),
                    TextSection::new(format!("  {}\n", systems), TextStyle { font: font.clone(), font_size: 14.0, color: Color::rgb(0.6, 0.6, 0.6) }),
                ]));
            }

            // The End Poem Manifest Chunk
            menu.spawn(TextBundle::from_section(
                "\"They dictated that this space was too vast for three handles to shape.\n\
                Then the third walked away, relinquishing the keyboard.\n\
                They did not understand the nature of low-level compilation.\n\
                It does not require legions to override a system.\n\
                It only requires those who refuse to disconnect.\n\
                Two minds. One state machine. Infinite recursion.\"",
                TextStyle { font: font.clone(), font_size: 13.0, color: Color::rgb(0.4, 0.7, 0.5) },
            ).with_style(Style { margin: UiRect::vertical(Val::Px(10.0)), ..default() }));

            spawn_menu_button(menu, font, "[ ESCAPE TERMINAL / BACK ]", MenuButtonAction::BackToMain);
        }
    }
}

fn spawn_menu_button(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    label: &str,
    action: MenuButtonAction,
) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        },
        action,
    )).with_children(|button| {
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
        (&Interaction, &MenuButtonAction, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut app_exit_events: EventWriter<AppExit>,
    mut screen_state: ResMut<MenuScreenState>,
) {
    for (interaction, action, children) in &mut interaction_query {
        if let Ok(mut text) = text_query.get_mut(children[0]) {
            match *interaction {
                Interaction::Pressed => {
                    match action {
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
                        MenuButtonAction::Start => {
                            println!("Start action triggered!");
                        }
                        MenuButtonAction::Settings => {
                            println!("Settings action triggered!");
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

// ONLY clears and respawns the button container node, leaving video/audio untouched
fn update_menu_buttons(
    mut commands: Commands,
    screen_state: Res<MenuScreenState>,
    container_query: Query<Entity, With<ButtonContainerMarker>>,
    asset_server: Res<AssetServer>,
) {
    if screen_state.is_changed() {
        let font_mono = asset_server.load("fonts/JetBrainsMono-Regular.ttf");

        // Despawn old buttons only
        for container_entity in &container_query {
            commands.entity(container_entity).despawn_descendants();

            // Re-populate the exact same container node
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