//! ================================================================================================
//! Orientation User Interface
//!
//! Handles all player-facing UI used during the Orientation Facility.
//!
//! Responsibilities
//! ----------------
//! • Objective panel
//! • Dialogue panel
//! • Interaction prompt
//!
//! This module intentionally contains no gameplay, dialogue or
//! progression logic. It only renders UI from Orientation state.
//! ================================================================================================

use bevy::prelude::*;

use crate::AppState;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct OrientationUiPlugin;

impl Plugin for OrientationUiPlugin {
    fn build(&self, app: &mut App) {
        app
            ////////////////////////////////////////////////////////////////////
            // Lifecycle
            ////////////////////////////////////////////////////////////////////
            .add_systems(
                OnEnter(AppState::Tutorial),
                spawn_orientation_ui,
            )
            .add_systems(
                OnExit(AppState::Tutorial),
                despawn_orientation_ui,
            )

            ////////////////////////////////////////////////////////////////////
            // Update
            ////////////////////////////////////////////////////////////////////
            .add_systems(
                Update,
                (
                    update_objective_panel,
                    update_dialogue_panel,
                    update_interaction_prompt,
                )
                .run_if(in_state(AppState::Tutorial)),
            );
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Root entity of the Orientation UI.
#[derive(Component)]
pub struct OrientationUi;

/// Left-side objective panel.
#[derive(Component)]
pub struct ObjectivePanel;

/// Right-side dialogue panel.
#[derive(Component)]
pub struct DialoguePanel;

/// Center-screen interaction prompt.
#[derive(Component)]
pub struct InteractionPrompt;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Text Components
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct ObjectiveText;

#[derive(Component)]
pub struct DialogueText;

#[derive(Component)]
pub struct InteractionPromptText;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Lifecycle Systems
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn spawn_orientation_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            OrientationUi,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Stretch,
                    padding: UiRect::all(Val::Px(24.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..default()
            },
        ))
        .with_children(|parent| {

            ////////////////////////////////////////////////////////////////////
            // Objective Panel
            ////////////////////////////////////////////////////////////////////

            parent
                .spawn((
                    ObjectivePanel,
                    NodeBundle {
                        style: Style {
                            width: Val::Px(340.0),
                            height: Val::Px(170.0),
                            padding: UiRect::all(Val::Px(18.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgba(0.05, 0.05, 0.05, 0.82)),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ObjectiveText,
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                                font_size: 28.0,
                                color: Color::WHITE,
                            },
                        ),
                    ));
                });

            ////////////////////////////////////////////////////////////////////
            // Dialogue Panel
            ////////////////////////////////////////////////////////////////////

            parent
                .spawn((
                    DialoguePanel,
                    NodeBundle {
                        style: Style {
                            width: Val::Px(430.0),
                            height: Val::Px(220.0),
                            padding: UiRect::all(Val::Px(18.0)),
                            align_self: AlignSelf::FlexEnd,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgba(0.05, 0.05, 0.05, 0.82)),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        DialogueText,
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                                font_size: 28.0,
                                color: Color::WHITE,
                            },
                        ),
                    ));
                });
        });

    ////////////////////////////////////////////////////////////////////////
    // Interaction Prompt
    ////////////////////////////////////////////////////////////////////////

    commands.spawn((
        OrientationUi,
        InteractionPrompt,
        InteractionPromptText,
        TextBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                    font_size: 26.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(40.0),
                left: Val::Percent(50.0),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
    ));
}

pub fn despawn_orientation_ui(
    mut commands: Commands,
    ui: Query<Entity, With<OrientationUi>>,
) {
    for entity in &ui {
        commands.entity(entity).despawn_recursive();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Update Systems
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn update_objective_panel(
    objective: Res<OrientationObjective>,
    mut query: Query<&mut Text, With<ObjectiveText>>,
) {
    if !objective.is_changed() {
        return;
    }

    let Ok(mut text) = query.get_single_mut() else {
        return;
    };

    text.sections[0].value = format!(
        "{}\n\n{}",
        objective.title,
        objective.description,
    );
}

pub fn update_dialogue_panel(
    stage: Res<OrientationStage>,
    mut query: Query<&mut Text, With<DialogueText>>,
) {
    if !stage.is_changed() {
        return;
    }

    let Ok(mut text) = query.get_single_mut() else {
        return;
    };

    let message = dialogue::message(*stage);

    text.sections[0].value = format!(
        "{}\n\n{}",
        message.title,
        message.body,
    );
}

pub fn update_interaction_prompt(
    mut query: Query<
        (&mut Text, &mut Visibility),
        With<InteractionPromptText>,
    >,
) {
    let Ok((mut text, mut visibility)) = query.get_single_mut() else {
        return;
    };

    *visibility = Visibility::Hidden;

    text.sections[0].value.clear();
}