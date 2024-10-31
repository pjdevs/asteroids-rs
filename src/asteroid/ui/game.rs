use crate::asteroid::core::prelude::*;
use crate::asteroid::game::prelude::*;
use crate::asteroid::utils::prelude::*;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AsteroidGameUiSystem {
    UpdateUi,
}

pub struct AsteroidGameUiPlugin;

impl Plugin for AsteroidGameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AsteroidGameState::Game), (ui_in_game_setup_system,))
            .add_systems(
                OnExit(AsteroidGameState::Game),
                (despawn_entities_with::<Node>,),
            )
            .add_systems(
                Update,
                ui_score_system
                    .run_if(in_state(AsteroidGameState::Game))
                    .in_set(AsteroidGameUiSystem::UpdateUi),
            );
    }
}

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
        top: Val::Px(5.0),
        left: Val::Px(5.0),
        ..Default::default()
    });

    commands.spawn((text, score_text));
}

fn ui_score_system(score: Res<Score>, mut query: Query<(&mut Text, &ScoreText)>) {
    for (mut text, score_text) in &mut query {
        text.sections[0].value = score_text.get_score_text(score.get_score());
    }
}
