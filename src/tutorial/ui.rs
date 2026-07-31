//! Tutorial User Interface.
//!
//! This module implements the complete presentation layer for the
//! S.U.D.O. Orientation Facility.
//!
//! Responsibilities:
//! - Dialogue window
//! - Objective panel
//! - Voice-over playback
//! - Continue prompts
//! - Stage notifications
//! - Tutorial completion screen
//! - UI animations
//!
//! This module intentionally contains no gameplay logic.
//! It consumes the tutorial state exposed by the other modules.

use bevy::prelude::*;

use crate::tutorial::objectives::TutorialObjective;
use crate::tutorial::pong::{self, PongMessage};
use crate::tutorial::stages::TutorialStage;
use crate::tutorial::task::TutorialTaskState;
use crate::AppState;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Width of the dialogue panel.
pub const DIALOGUE_WIDTH: f32 = 700.0;

/// Height of the dialogue panel.
pub const DIALOGUE_HEIGHT: f32 = 220.0;

/// Width of the objective panel.
pub const OBJECTIVE_WIDTH: f32 = 340.0;

/// Height of the objective panel.
pub const OBJECTIVE_HEIGHT: f32 = 120.0;

/// Default fade duration.
pub const FADE_DURATION: f32 = 0.35;

/// Dialogue typing speed (characters per second).
pub const TYPEWRITER_SPEED: f32 = 48.0;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Resources
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Current dialogue displayed by Pong.
#[derive(Resource, Default)]
pub struct DialogueState {
    pub active: bool,
    pub current: Option<PongMessage>,
}

/// Stores the objective currently displayed.
#[derive(Resource, Default)]
pub struct ObjectiveState {
    pub visible: bool,
}

/// Whether the player may advance the dialogue.
#[derive(Resource, Default)]
pub struct ContinuePrompt {
    pub visible: bool,
}

/// Tracks whether the tutorial UI is currently animating.
#[derive(Resource, Default)]
pub struct UiAnimationState {
    pub busy: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Marker Components
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct DialoguePanel;

#[derive(Component)]
pub struct DialogueTitle;

#[derive(Component)]
pub struct DialogueBody;

#[derive(Component)]
pub struct ObjectivePanel;

#[derive(Component)]
pub struct ObjectiveTitle;

#[derive(Component)]
pub struct ObjectiveDescription;

#[derive(Component)]
pub struct ContinueText;

#[derive(Component)]
pub struct PongPortrait;

#[derive(Component)]
pub struct CompletionScreen;

#[derive(Component)]
pub struct TutorialUi;

#[derive(Component)]
pub struct TutorialDialogue;

#[derive(Component)]
pub struct TutorialObjectivePanel;

#[derive(Component)]
pub struct TutorialContinuePrompt;

#[derive(Component)]
pub struct TutorialCompletionScreen;

#[derive(Component)]
pub struct TutorialVoiceAudio;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Events
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Requests a dialogue window to open.
#[derive(Event)]
pub struct ShowDialogueEvent;

/// Requests the dialogue window to close.
#[derive(Event)]
pub struct HideDialogueEvent;

/// Refreshes the objective panel.
#[derive(Event)]
pub struct UpdateObjectiveEvent;

/// Plays Pong's current voice-over.
#[derive(Event)]
pub struct PlayVoiceEvent {
    pub voice_path: Option<&'static str>,
}

/// Shows the tutorial completion screen.
#[derive(Event)]
pub struct TutorialCompletedEvent;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct TutorialUiPlugin;

impl Plugin for TutorialUiPlugin {
    fn build(&self, app: &mut App) {
        app
            ////////////////////////////////////////////////////////////////////
            // Resources
            ////////////////////////////////////////////////////////////////////
            .init_resource::<DialogueState>()
            .init_resource::<ObjectiveState>()
            .init_resource::<ContinuePrompt>()
            .init_resource::<UiAnimationState>()
            ////////////////////////////////////////////////////////////////////
            // Events
            ////////////////////////////////////////////////////////////////////
            .add_event::<ShowDialogueEvent>()
            .add_event::<HideDialogueEvent>()
            .add_event::<UpdateObjectiveEvent>()
            .add_event::<PlayVoiceEvent>()
            ////////////////////////////////////////////////////////////////////
            // Setup Systems
            ////////////////////////////////////////////////////////////////////
            .add_systems(
                OnEnter(AppState::Tutorial),
                (
                    setup_tutorial_ui,
                    setup_completion_screen,
                    present_initial_tutorial_state,
                ),
            )
            ////////////////////////////////////////////////////////////////////
            // Dialogue Systems
            ////////////////////////////////////////////////////////////////////
            .add_systems(
                Update,
                (
                    show_dialogue,
                    hide_dialogue,
                    update_dialogue,
                    show_continue_prompt,
                    hide_continue_prompt,
                )
                    .run_if(in_state(AppState::Tutorial)),
            )
            ////////////////////////////////////////////////////////////////////
            // Objective Systems
            ////////////////////////////////////////////////////////////////////
            .add_systems(
                Update,
                (
                    initialize_objective,
                    update_objective,
                    refresh_objective,
                    show_objective_panel,
                    hide_objective_panel,
                )
                    .run_if(in_state(AppState::Tutorial)),
            )
            ////////////////////////////////////////////////////////////////////
            // Audio Systems
            ////////////////////////////////////////////////////////////////////
            .add_systems(
                Update,
                (
                    play_voice_over,
                    stop_voice_over,
                    replay_voice_over,
                    play_completion_sound,
                )
                    .run_if(in_state(AppState::Tutorial)),
            )
            ////////////////////////////////////////////////////////////////////
            // Animation Systems
            ////////////////////////////////////////////////////////////////////
            .add_systems(
                Update,
                (
                    animate_dialogue_in,
                    animate_dialogue_out,
                    animate_objective_panel,
                    animate_continue_prompt,
                    animate_completion_screen,
                )
                    .run_if(in_state(AppState::Tutorial)),
            )
            ////////////////////////////////////////////////////////////////////
            // Input Systems
            ////////////////////////////////////////////////////////////////////
            .add_systems(
                Update,
                (
                    continue_dialogue,
                    replay_dialogue,
                    dismiss_completion_screen,
                    block_input_during_animation,
                )
                    .run_if(in_state(AppState::Tutorial)),
            )
            ////////////////////////////////////////////////////////////////////
            // Update Systems
            ////////////////////////////////////////////////////////////////////
            .add_systems(
                Update,
                (
                    synchronize_dialogue_state,
                    synchronize_objective_state,
                    update_animation_state,
                )
                    .run_if(in_state(AppState::Tutorial)),
            );
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Setup Systems
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Creates the complete Orientation Facility UI.
pub fn setup_tutorial_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let regular_font: Handle<Font> = asset_server.load("fonts/Inter_18pt-Regular.ttf");
    let bold_font: Handle<Font> = asset_server.load("fonts/Inter_18pt-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Stretch,
                    ..default()
                },
                ..default()
            },
            TutorialUi,
        ))
        .with_children(|root| {
            root.spawn((NodeBundle {
                style: Style {
                    width: Val::Percent(30.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect::all(Val::Px(24.0)),
                    ..default()
                },
                ..default()
            },))
                .with_children(|left| {
                    left.spawn((
                        ObjectivePanel,
                        TutorialObjectivePanel,
                        NodeBundle {
                            style: Style {
                                width: Val::Px(OBJECTIVE_WIDTH),
                                height: Val::Px(OBJECTIVE_HEIGHT),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(16.0)),
                                row_gap: Val::Px(8.0),
                                ..default()
                            },
                            background_color: Color::rgba(0.05, 0.05, 0.08, 0.90).into(),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                    ))
                    .with_children(|panel| {
                        panel.spawn((
                            ObjectiveTitle,
                            TextBundle::from_section(
                                "OBJECTIVE",
                                TextStyle {
                                    font: bold_font.clone(),
                                    font_size: 24.0,
                                    color: Color::WHITE,
                                },
                            ),
                        ));

                        panel.spawn((
                            ObjectiveDescription,
                            TextBundle::from_section(
                                "Loading objective...",
                                TextStyle {
                                    font: regular_font.clone(),
                                    font_size: 18.0,
                                    color: Color::rgb(0.85, 0.85, 0.85),
                                },
                            ),
                        ));
                    });
                });

            root.spawn((NodeBundle {
                style: Style {
                    width: Val::Percent(70.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(24.0)),
                    ..default()
                },
                ..default()
            },))
                .with_children(|right| {
                    right
                        .spawn((
                            DialoguePanel,
                            TutorialDialogue,
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(DIALOGUE_WIDTH),
                                    height: Val::Px(DIALOGUE_HEIGHT),
                                    flex_direction: FlexDirection::Column,
                                    padding: UiRect::all(Val::Px(18.0)),
                                    row_gap: Val::Px(10.0),
                                    ..default()
                                },
                                background_color: Color::rgba(0.02, 0.02, 0.04, 0.94).into(),
                                visibility: Visibility::Hidden,
                                ..default()
                            },
                        ))
                        .with_children(|dialogue| {
                            dialogue.spawn((
                                DialogueTitle,
                                TextBundle::from_section(
                                    "PONG",
                                    TextStyle {
                                        font: bold_font.clone(),
                                        font_size: 26.0,
                                        color: Color::WHITE,
                                    },
                                ),
                            ));

                            dialogue.spawn((
                                DialogueBody,
                                TextBundle::from_section(
                                    "",
                                    TextStyle {
                                        font: regular_font.clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                ),
                            ));

                            dialogue.spawn((
                                ContinueText,
                                TutorialContinuePrompt,
                                TextBundle {
                                    text: Text::from_section(
                                        "Press [E] to continue",
                                        TextStyle {
                                            font: regular_font.clone(),
                                            font_size: 16.0,
                                            color: Color::rgb(0.65, 0.65, 0.65),
                                        },
                                    ),
                                    visibility: Visibility::Hidden,
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

/// Creates the tutorial completion overlay.
///
/// Hidden until the player graduates.
pub fn setup_completion_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bold_font: Handle<Font> = asset_server.load("fonts/Inter_18pt-Bold.ttf");

    commands
        .spawn((
            CompletionScreen,
            TutorialCompletionScreen,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.92).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "ORIENTATION COMPLETE",
                TextStyle {
                    font: bold_font,
                    font_size: 48.0,
                    color: Color::WHITE,
                },
            ));
        });
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Dialogue Systems
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Displays Pong's dialogue for the current tutorial stage.
pub fn show_dialogue(
    stage: Res<TutorialStage>,
    mut dialogue_state: ResMut<DialogueState>,
    mut dialogue_panel: Query<&mut Visibility, With<DialoguePanel>>,
    mut title: Query<&mut Text, (With<DialogueTitle>, Without<DialogueBody>)>,
    mut body: Query<&mut Text, (With<DialogueBody>, Without<DialogueTitle>)>,
    mut show_events: EventReader<ShowDialogueEvent>,
    mut play_voice: EventWriter<PlayVoiceEvent>,
) {
    if show_events.is_empty() {
        return;
    }

    show_events.clear();

    let message = message(*stage);

    dialogue_state.active = true;
    dialogue_state.current = Some(message.clone());

    if let Ok(mut visibility) = dialogue_panel.get_single_mut() {
        *visibility = Visibility::Visible;
    }

    if let Ok(mut dialogue_title) = title.get_single_mut() {
        dialogue_title.sections[0].value = message.title.to_string();
    }

    if let Ok(mut dialogue_body) = body.get_single_mut() {
        dialogue_body.sections[0].value = message.body.to_string();
    }

    play_voice.send(PlayVoiceEvent {
        voice_path: message.voice_path,
    });
}

/// Hides the dialogue window.
pub fn hide_dialogue(
    mut hide_events: EventReader<HideDialogueEvent>,
    mut dialogue_state: ResMut<DialogueState>,
    mut dialogue_panel: Query<&mut Visibility, (With<DialoguePanel>, Without<ContinueText>)>,
    mut continue_prompt: Query<&mut Visibility, (With<ContinueText>, Without<DialoguePanel>)>,
) {
    if hide_events.is_empty() {
        return;
    }

    hide_events.clear();

    dialogue_state.active = false;
    dialogue_state.current = None;

    if let Ok(mut visibility) = dialogue_panel.get_single_mut() {
        *visibility = Visibility::Hidden;
    }

    if let Ok(mut prompt) = continue_prompt.get_single_mut() {
        *prompt = Visibility::Hidden;
    }
}

/// Updates the dialogue if the tutorial stage changes.
pub fn update_dialogue(
    stage: Res<TutorialStage>,
    dialogue_state: Res<DialogueState>,
    mut title: Query<&mut Text, (With<DialogueTitle>, Without<DialogueBody>)>,
    mut body: Query<&mut Text, (With<DialogueBody>, Without<DialogueTitle>)>,
) {
    if !stage.is_changed() {
        return;
    }

    if !dialogue_state.active {
        return;
    }

    let message = message(*stage);

    if let Ok(mut dialogue_title) = title.get_single_mut() {
        dialogue_title.sections[0].value = message.title.to_string();
    }

    if let Ok(mut dialogue_body) = body.get_single_mut() {
        dialogue_body.sections[0].value = message.body.to_string();
    }
}

/// Shows the continue prompt after dialogue has finished.
pub fn show_continue_prompt(
    dialogue_state: Res<DialogueState>,
    mut continue_prompt: ResMut<ContinuePrompt>,
    mut text: Query<&mut Visibility, With<ContinueText>>,
) {
    if !dialogue_state.active {
        return;
    }

    if continue_prompt.visible {
        return;
    }

    continue_prompt.visible = true;

    if let Ok(mut visibility) = text.get_single_mut() {
        *visibility = Visibility::Visible;
    }
}

/// Hides the continue prompt when dialogue is no longer active.
pub fn hide_continue_prompt(
    dialogue_state: Res<DialogueState>,
    mut continue_prompt: ResMut<ContinuePrompt>,
    mut text: Query<&mut Visibility, With<ContinueText>>,
) {
    if dialogue_state.active {
        return;
    }

    if !continue_prompt.visible {
        return;
    }

    continue_prompt.visible = false;

    if let Ok(mut visibility) = text.get_single_mut() {
        *visibility = Visibility::Hidden;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Objective Systems
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Initializes the objective panel when the tutorial begins.
pub fn initialize_objective(
    objective: Res<TutorialObjective>,
    mut title: Query<&mut Text, (With<ObjectiveTitle>, Without<ObjectiveDescription>)>,
    mut description: Query<&mut Text, (With<ObjectiveDescription>, Without<ObjectiveTitle>)>,
) {
    if let Ok(mut objective_title) = title.get_single_mut() {
        objective_title.sections[0].value = objective.title.to_string();
    }

    if let Ok(mut objective_description) = description.get_single_mut() {
        objective_description.sections[0].value = objective.description.to_string();
    }
}

/// Refreshes the objective panel whenever the active objective changes.
pub fn update_objective(
    objective: Res<TutorialObjective>,
    mut title: Query<&mut Text, (With<ObjectiveTitle>, Without<ObjectiveDescription>)>,
    mut description: Query<&mut Text, (With<ObjectiveDescription>, Without<ObjectiveTitle>)>,
) {
    if !objective.is_changed() {
        return;
    }

    if let Ok(mut objective_title) = title.get_single_mut() {
        objective_title.sections[0].value = objective.title.to_string();
    }

    if let Ok(mut objective_description) = description.get_single_mut() {
        objective_description.sections[0].value = objective.description.to_string();
    }
}

/// Responds to an explicit objective refresh request.
///
/// Normally objectives update automatically through `Res::is_changed()`,
/// but this event allows the UI to be refreshed immediately after systems
/// that intentionally request it.
pub fn refresh_objective(
    mut refresh_events: EventReader<UpdateObjectiveEvent>,
    objective: Res<TutorialObjective>,
    mut title: Query<&mut Text, (With<ObjectiveTitle>, Without<ObjectiveDescription>)>,
    mut description: Query<&mut Text, (With<ObjectiveDescription>, Without<ObjectiveTitle>)>,
) {
    if refresh_events.is_empty() {
        return;
    }

    refresh_events.clear();

    if let Ok(mut objective_title) = title.get_single_mut() {
        objective_title.sections[0].value = objective.title.to_string();
    }

    if let Ok(mut objective_description) = description.get_single_mut() {
        objective_description.sections[0].value = objective.description.to_string();
    }
}

/// Shows the objective panel.
pub fn show_objective_panel(
    mut state: ResMut<ObjectiveState>,
    mut panel: Query<&mut Visibility, With<ObjectivePanel>>,
) {
    if state.visible {
        return;
    }

    state.visible = true;

    if let Ok(mut visibility) = panel.get_single_mut() {
        *visibility = Visibility::Visible;
    }
}

/// Hides the objective panel.
pub fn hide_objective_panel(
    mut state: ResMut<ObjectiveState>,
    mut panel: Query<&mut Visibility, With<ObjectivePanel>>,
) {
    if !state.visible {
        return;
    }

    state.visible = false;

    if let Ok(mut visibility) = panel.get_single_mut() {
        *visibility = Visibility::Hidden;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Audio Systems
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Plays Pong's dialogue voice-over.
///
/// This system is completely independent of the tutorial state.
/// It simply responds to PlayVoiceEvent by spawning an audio player.
pub fn play_voice_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<PlayVoiceEvent>,
) {
    for event in events.read() {
        if let Some(voice_path) = event.voice_path {
            commands
                .spawn(AudioBundle {
                    source: asset_server.load(voice_path),
                    settings: PlaybackSettings::ONCE,
                    ..default()
                })
                .insert(TutorialVoiceAudio);
        }
    }
}

/// Stops every currently playing voice-over.
///
/// This is primarily used when:
/// - skipping dialogue
/// - restarting the tutorial
/// - exiting the tutorial
pub fn stop_voice_over(
    mut commands: Commands,
    mut events: EventReader<HideDialogueEvent>,
    audio_players: Query<Entity, With<TutorialVoiceAudio>>,
) {
    if events.is_empty() {
        return;
    }

    events.clear();

    for entity in &audio_players {
        commands.entity(entity).despawn();
    }
}

/// Restarts the currently playing voice clip.
///
/// Useful when the player requests the dialogue again.
pub fn replay_voice_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    dialogue_state: Res<DialogueState>,
) {
    let Some(message) = &dialogue_state.current else {
        return;
    };

    let Some(voice_path) = message.voice_path else {
        return;
    };

    commands
        .spawn(AudioBundle {
            source: asset_server.load(voice_path),
            settings: PlaybackSettings::ONCE,
            ..default()
        })
        .insert(TutorialVoiceAudio);
}

/// Plays the tutorial completion sound.
///
/// This intentionally does not depend on Pong.
pub fn play_completion_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<TutorialCompletedEvent>,
) {
    if events.is_empty() {
        return;
    }

    events.clear();

    commands.spawn(AudioBundle {
        source: asset_server.load("audio/tutorial_complete.ogg"),
        settings: PlaybackSettings::ONCE,
        ..default()
    });
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Animation Systems
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Fades the dialogue panel into view.
pub fn animate_dialogue_in(
    dialogue_state: Res<DialogueState>,
    mut animation_state: ResMut<UiAnimationState>,
    mut panels: Query<&mut BackgroundColor, With<DialoguePanel>>,
) {
    if !dialogue_state.active {
        return;
    }

    if animation_state.busy {
        return;
    }

    animation_state.busy = true;

    for mut color in &mut panels {
        color.0.set_a(0.94);
    }

    animation_state.busy = false;
}

/// Fades the dialogue panel out.
pub fn animate_dialogue_out(
    dialogue_state: Res<DialogueState>,
    mut animation_state: ResMut<UiAnimationState>,
    mut panels: Query<&mut BackgroundColor, With<DialoguePanel>>,
) {
    if dialogue_state.active {
        return;
    }

    if animation_state.busy {
        return;
    }

    animation_state.busy = true;

    for mut color in &mut panels {
        color.0.set_a(0.0);
    }

    animation_state.busy = false;
}

/// Fades the objective panel.
pub fn animate_objective_panel(
    objective_state: Res<ObjectiveState>,
    mut panels: Query<&mut BackgroundColor, With<ObjectivePanel>>,
) {
    let alpha = if objective_state.visible { 0.90 } else { 0.0 };

    for mut color in &mut panels {
        color.0.set_a(alpha);
    }
}

/// Makes the continue prompt gently blink.
pub fn animate_continue_prompt(
    time: Res<Time>,
    continue_prompt: Res<ContinuePrompt>,
    mut query: Query<&mut Text, With<ContinueText>>,
) {
    if !continue_prompt.visible {
        return;
    }

    let alpha = ((time.elapsed_seconds() * 4.0).sin() * 0.5 + 0.5) * 0.5 + 0.5;

    for mut text in &mut query {
        if let Some(section) = text.sections.get_mut(0) {
            section.style.color.set_a(alpha);
        }
    }
}

/// Shows the tutorial completion overlay.
pub fn animate_completion_screen(
    mut events: EventReader<TutorialCompletedEvent>,
    mut screen: Query<&mut Visibility, With<CompletionScreen>>,
) {
    if events.is_empty() {
        return;
    }

    events.clear();

    if let Ok(mut visibility) = screen.get_single_mut() {
        *visibility = Visibility::Visible;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Input Systems
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Handles advancing dialogue when the player presses the interact key.
pub fn continue_dialogue(
    keyboard: Res<ButtonInput<KeyCode>>,
    dialogue_state: Res<DialogueState>,
    continue_prompt: Res<ContinuePrompt>,
    mut task: ResMut<TutorialTaskState>,
    mut hide_dialogue: EventWriter<HideDialogueEvent>,
) {
    if !dialogue_state.active {
        return;
    }

    if !continue_prompt.visible {
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyE) {
        task.complete();
        hide_dialogue.send(HideDialogueEvent);
    }
}

/// Replays the current dialogue voice-over.
pub fn replay_dialogue(
    keyboard: Res<ButtonInput<KeyCode>>,
    dialogue_state: Res<DialogueState>,
    mut play_voice: EventWriter<PlayVoiceEvent>,
) {
    if !dialogue_state.active {
        return;
    }

    if !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }

    let Some(message) = &dialogue_state.current else {
        return;
    };

    play_voice.send(PlayVoiceEvent {
        voice_path: message.voice_path,
    });
}

/// Allows the player to dismiss the completion screen.
pub fn dismiss_completion_screen(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut screen: Query<&mut Visibility, With<CompletionScreen>>,
) {
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }

    if let Ok(mut visibility) = screen.get_single_mut() {
        if *visibility == Visibility::Visible {
            *visibility = Visibility::Hidden;
        }
    }
}

/// Sends the initial tutorial presentation when the player enters the tutorial.
pub fn present_initial_tutorial_state(
    mut show_dialogue: EventWriter<ShowDialogueEvent>,
    mut update_objective: EventWriter<UpdateObjectiveEvent>,
) {
    show_dialogue.send(ShowDialogueEvent);
    update_objective.send(UpdateObjectiveEvent);
}

/// Prevents dialogue controls while another UI animation is running.
pub fn block_input_during_animation(
    animation_state: Res<UiAnimationState>,
    mut continue_prompt: ResMut<ContinuePrompt>,
) {
    if animation_state.busy {
        continue_prompt.visible = false;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Update Systems
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Keeps the dialogue state synchronized with its visibility.
pub fn synchronize_dialogue_state(
    dialogue_state: Res<DialogueState>,
    mut panels: Query<&mut Visibility, With<DialoguePanel>>,
) {
    let visibility = if dialogue_state.active {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    if let Ok(mut panel) = panels.get_single_mut() {
        *panel = visibility;
    }
}

/// Keeps the objective panel synchronized with its visibility state.
pub fn synchronize_objective_state(
    objective_state: Res<ObjectiveState>,
    mut panels: Query<&mut Visibility, With<ObjectivePanel>>,
) {
    let visibility = if objective_state.visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    if let Ok(mut panel) = panels.get_single_mut() {
        *panel = visibility;
    }
}

/// Resets transient UI state after animations complete.
///
/// This prepares the interface for the next interaction.
pub fn update_animation_state(mut animation_state: ResMut<UiAnimationState>) {
    if animation_state.busy {
        animation_state.busy = false;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Utility Functions
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Returns the Pong dialogue associated with a tutorial stage.
#[inline]
pub fn message(stage: TutorialStage) -> PongMessage {
    pong::message(stage)
}

/// Returns the objective associated with a tutorial stage.
#[inline]
pub fn objective(stage: TutorialStage) -> TutorialObjective {
    TutorialObjective::from_stage(stage)
}

/// Returns true if the tutorial has completed.
#[inline]
pub fn tutorial_completed(stage: TutorialStage) -> bool {
    stage == TutorialStage::Finished
}

/// Returns whether dialogue should currently be visible.
#[inline]
pub fn dialogue_visible(state: &DialogueState) -> bool {
    state.active
}

/// Returns whether the objective panel should currently be visible.
#[inline]
pub fn objective_visible(state: &ObjectiveState) -> bool {
    state.visible
}

/// Returns whether the continue prompt should currently be shown.
#[inline]
pub fn continue_prompt_visible(state: &ContinuePrompt) -> bool {
    state.visible
}

/// Returns whether UI animations are currently active.
#[inline]
pub fn animation_busy(state: &UiAnimationState) -> bool {
    state.busy
}
