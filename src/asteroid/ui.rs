use bevy::prelude::*;

use super::gameplay::Score;

pub struct AsteroidUiPlugin;

impl Plugin for AsteroidUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui_setup_system)
            .add_systems(Update, ui_score_system);
    }
}

#[derive(Component)]
struct ScoreText;

impl ScoreText {
    fn get_score_text(&self, score: u64) -> String {
        format!("Score: {}", score)
    }
}

fn ui_setup_system(mut commands: Commands) {
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
