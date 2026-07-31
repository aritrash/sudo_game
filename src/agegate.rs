use crate::AppState;
use bevy::{app::AppExit, prelude::*};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub struct AgeGatePlugin;

impl Plugin for AgeGatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AgeInputBuffer>()
            .add_systems(OnEnter(AppState::AgeGate), check_cache_and_setup)
            .add_systems(
                Update,
                (handle_keyboard_input, process_age_gate_logic).run_if(in_state(AppState::AgeGate)),
            )
            .add_systems(OnExit(AppState::AgeGate), cleanup_age_gate);
    }
}

const CACHE_FILE_PATH: &str = "user_session.json";

#[derive(Component)]
struct OnAgeGateScreen;

#[derive(Component)]
struct DateDisplayMarker;

#[derive(Component)]
struct FeedbackMessageMarker;

#[derive(Resource, Default)]
struct AgeInputBuffer {
    digits: String,
    is_rejected: bool,
}

fn check_cache_and_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if Path::new(CACHE_FILE_PATH).exists() {
        if let Ok(mut file) = File::open(CACHE_FILE_PATH) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if contents.contains("\"age_verified\": true") {
                    next_state.set(AppState::Loading);
                    return;
                }
            }
        }
    }

    let font_mono = asset_server.load("fonts/JetBrainsMono-Regular.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            OnAgeGateScreen,
        ))
        .with_children(|parent| {
            // Accessible, Clean Notice
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "AGE VERIFICATION REQUIRED\n\
                    -------------------------\n\
                    Please confirm your date of birth to proceed.",
                    TextStyle {
                        font: font_mono.clone(),
                        font_size: 16.0,
                        color: Color::rgb(0.7, 0.7, 0.7),
                    },
                ).with_justify(JustifyText::Center),
                ..default()
            });

            // Date Prompt Label
            parent.spawn(TextBundle::from_section(
                "ENTER DATE OF BIRTH (MM/DD/YYYY):",
                TextStyle {
                    font: font_mono.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));

            // Blanks
            parent.spawn((
                TextBundle::from_section(
                    "__/__/____",
                    TextStyle {
                        font: font_mono.clone(),
                        font_size: 28.0,
                        color: Color::RED,
                    },
                ),
                DateDisplayMarker,
            ));

            // Feedback Message
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: font_mono.clone(),
                            font_size: 16.0,
                            color: Color::YELLOW,
                        },
                    ).with_justify(JustifyText::Center),
                    ..default()
                },
                FeedbackMessageMarker,
            ));

            // Clear, Plain-English Legal Footers
            parent.spawn(NodeBundle {
                style: Style {
                    margin: UiRect::top(Val::Px(40.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                ..default()
            }).with_children(|footer| {
                footer.spawn(TextBundle::from_section(
                    "[ PRIVACY POLICY: Your data is processed locally and never shared with third parties. ]",
                    TextStyle { font: font_mono.clone(), font_size: 12.0, color: Color::rgb(0.5, 0.5, 0.5) },
                ));
                footer.spawn(TextBundle::from_section(
                    "[ PARENTAL GUIDELINES: For safety details, please review 'guidelines.html' in your application folder. ]",
                    TextStyle { font: font_mono.clone(), font_size: 12.0, color: Color::rgb(0.5, 0.5, 0.5) },
                ));
            });
        });
}

fn handle_keyboard_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut input_buffer: ResMut<AgeInputBuffer>,
    mut display_query: Query<&mut Text, With<DateDisplayMarker>>,
) {
    if input_buffer.is_rejected {
        return;
    }

    let mut changed = false;

    if keyboard_input.just_pressed(KeyCode::Backspace)
        || keyboard_input.just_pressed(KeyCode::Delete)
    {
        if !input_buffer.digits.is_empty() {
            input_buffer.digits.pop();
            changed = true;
        }
    }

    for ev in char_evr.read() {
        if let Some(ch) = ev.char.chars().next() {
            if ch.is_ascii_digit() && input_buffer.digits.len() < 8 {
                input_buffer.digits.push(ch);
                changed = true;
            }
        }
    }

    if changed {
        if let Ok(mut text) = display_query.get_single_mut() {
            let s = &input_buffer.digits;
            let mut formatted = String::new();

            for i in 0..10 {
                if i == 2 || i == 5 {
                    formatted.push('/');
                } else {
                    let digit_idx = if i < 2 {
                        i
                    } else if i < 5 {
                        i - 1
                    } else {
                        i - 2
                    };
                    if let Some(c) = s.chars().nth(digit_idx) {
                        formatted.push(c);
                    } else {
                        formatted.push('_');
                    }
                }
            }
            text.sections[0].value = formatted;
        }
    }
}

fn process_age_gate_logic(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut input_buffer: ResMut<AgeInputBuffer>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit_events: EventWriter<AppExit>,
    mut feedback_query: Query<&mut Text, With<FeedbackMessageMarker>>,
) {
    // Check for standard Enter OR Numpad Enter keys
    let enter_pressed = keyboard_input.just_pressed(KeyCode::Enter)
        || keyboard_input.just_pressed(KeyCode::NumpadEnter);

    if input_buffer.is_rejected {
        if enter_pressed || keyboard_input.just_pressed(KeyCode::Escape) {
            exit_events.send(AppExit::default());
        }
        return;
    }

    if enter_pressed && input_buffer.digits.len() == 8 {
        let digits = &input_buffer.digits;

        let month: u32 = digits[0..2].parse().unwrap_or(0);
        let day: u32 = digits[2..4].parse().unwrap_or(0);
        let year: i32 = digits[4..8].parse().unwrap_or(0);

        if month < 1 || month > 12 || day < 1 || day > 31 {
            if let Ok(mut feedback) = feedback_query.get_single_mut() {
                feedback.sections[0].value = "INVALID DATE. PLEASE TRY AGAIN.".to_string();
                feedback.sections[0].style.color = Color::YELLOW;
            }
            return;
        }

        // HARDCODED COMPUTE SYSTEM FOR THE CURRENT DATE MATRIX (JANUARY 1, 2026)
        // Required criteria: Born on or before January 1, 2013.
        /*
            ****************************************************************
            ATTENTION DEVELOPER:
            Please remember to update the target_year field in the following
            code by +1 every year. This keeps the actual game logic working
            year after year.
            ****************************************************************
        */

        let target_year = 2013;
        let target_month = 1;
        let target_day = 1;

        let mut access_granted = false;

        if year < target_year {
            access_granted = true;
        } else if year == target_year {
            if month < target_month {
                access_granted = true;
            } else if month == target_month && day <= target_day {
                access_granted = true;
            }
        }

        if let Ok(mut feedback) = feedback_query.get_single_mut() {
            if access_granted {
                feedback.sections[0].value = "ACCESS GRANTED. LOADING MAIN TERMINAL...".to_string();
                feedback.sections[0].style.color = Color::GREEN;

                if let Ok(mut file) = File::create(CACHE_FILE_PATH) {
                    let json_payload = "{\n  \"age_verified\": true\n}";
                    let _ = file.write_all(json_payload.as_bytes());
                }

                next_state.set(AppState::Loading);
            } else {
                feedback.sections[0].value = "ACCESS DENIED: This game is not suitable for your age group.\nPress ENTER to exit application.".to_string();
                feedback.sections[0].style.color = Color::RED;
                input_buffer.is_rejected = true;
            }
        }
    }
}

fn cleanup_age_gate(mut commands: Commands, query: Query<Entity, With<OnAgeGateScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
