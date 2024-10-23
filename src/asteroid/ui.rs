use super::gameplay::Score;
use crate::asteroid::states::AsteroidGameState;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AsteroidUiSystem {
    UpdateCoreUi,
    UpdateMenuUi,
    UpdateInGameUi,
}

pub struct AsteroidUiPlugin;

impl Plugin for AsteroidUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonEvent>()
            .add_systems(
                Update,
                ui_button_released_system.in_set(AsteroidUiSystem::UpdateCoreUi),
            )
            .add_systems(
                Update,
                ui_score_system.in_set(AsteroidUiSystem::UpdateInGameUi),
            )
            .add_systems(
                Update,
                (
                    ui_button_released_system,
                    ui_play_system,
                    ui_button_style_system,
                )
                    .in_set(AsteroidUiSystem::UpdateMenuUi),
            );
    }
}

// Menu

fn ui_play_system(
    mut events: EventReader<ButtonEvent>,
    mut next_state: ResMut<NextState<AsteroidGameState>>,
) {
    for event in events.read() {
        match event {
            ButtonEvent::Clicked(_) => next_state.set(AsteroidGameState::GameLoadingScreen),
        }
    }
}

fn ui_button_style_system(
    mut query: Query<(&mut BorderColor, &Interaction), Changed<Interaction>>,
) {
    for (mut border, interaction) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                border.0 = Color::srgb(0.5, 0.5, 0.5);
            }
            Interaction::Hovered => {
                border.0 = Color::WHITE;
            }
            Interaction::None => {
                border.0 = Color::BLACK;
            }
        }
    }
}

pub fn ui_menu_setup_system(mut commands: Commands) {
    let container_node = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    let button_node = ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
        ..default()
    };

    let button_text_node = TextBundle::from_section(
        "Play",
        TextStyle {
            font_size: 40.0,
            color: Color::srgb(0.9, 0.9, 0.9),
            ..default()
        },
    );

    let container = commands.spawn(container_node).id();
    let button = commands
        .spawn(button_node)
        .insert(LastInteraction::default())
        .id();
    let button_text = commands.spawn(button_text_node).id();

    commands.entity(button).push_children(&[button_text]);
    commands.entity(container).push_children(&[button]);
}

// In Game

#[derive(Component)]
struct ScoreText;

impl ScoreText {
    fn get_score_text(&self, score: u64) -> String {
        format!("Score: {}", score)
    }
}

pub fn ui_in_game_setup_system(mut commands: Commands) {
    let score_text = ScoreText;
    let text = TextBundle::from_section(
        score_text.get_score_text(0),
        TextStyle {
            font_size: 24.0,
            color: Color::WHITE,
            ..Default::default()
        },
    )
    .with_text_justify(JustifyText::Center)
    .with_style(Style {
        position_type: PositionType::Absolute,
        bottom: Val::Px(5.0),
        right: Val::Px(5.0),
        ..Default::default()
    });

    commands.spawn((text, score_text));
}

fn ui_score_system(score: Res<Score>, mut query: Query<(&mut Text, &ScoreText)>) {
    for (mut text, score_text) in &mut query {
        text.sections[0].value = score_text.get_score_text(score.get_score());
    }
}

// Release buttons

#[derive(Bundle)]
pub struct GameButtonBundle {
    button: ButtonBundle,
    last_interaction: LastInteraction,
}

#[derive(Component, Default)]
struct LastInteraction {
    last: Interaction,
}

#[derive(Event)]
enum ButtonEvent {
    Clicked(Entity),
}

fn ui_button_released_system(
    mut event: EventWriter<ButtonEvent>,
    mut query: Query<(Entity, &mut LastInteraction, &Interaction), Changed<Interaction>>,
) {
    for (entity, mut last, current) in &mut query {
        if last.last == Interaction::Pressed && *current == Interaction::Hovered {
            event.send(ButtonEvent::Clicked(entity));
        }

        println!("Last state was {:?} and is now {:?}", last.last, current);
        last.last = *current;
    }
}
