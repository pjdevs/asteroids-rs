use crate::asteroid::core::prelude::*;
use crate::asteroid::game::prelude::*;
use crate::asteroid::utils::prelude::*;
use bevy::prelude::*;

pub struct AsteroidGameUiPlugin;

impl Plugin for AsteroidGameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AsteroidGameState::Game),
            (ui_setup_score, ui_setup_lives),
        )
        .add_systems(
            OnExit(AsteroidGameState::Game),
            (despawn_entities_with::<Node>,),
        )
        // TODO Check if we can observe only in Game State
        .observe(ui_score_system)
        .observe(ui_lives_system);
    }
}

#[derive(Component)]
struct ScoreText;

impl ScoreText {
    fn get_score_text(&self, score: u64) -> String {
        format!("Score: {}", score)
    }
}

#[derive(Component)]
struct LivesContaier;

pub fn ui_setup_score(mut commands: Commands) {
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

pub fn ui_setup_lives(mut commands: Commands) {
    let lives_container = NodeBundle {
        style: Style {
            top: Val::Px(2.5),
            left: Val::Px(150.0),
            width: Val::Px(150.0),
            height: Val::Px(32.0),
            border: UiRect::ZERO,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexStart,
            ..Default::default()
        },
        ..Default::default()
    };

    commands.spawn((lives_container, LivesContaier));
}

fn ui_score_system(
    _trigger: Trigger<ScoreChanged>,
    score: Res<Score>,
    mut query: Query<(&mut Text, &ScoreText)>,
) {
    for (mut text, score_text) in &mut query {
        text.sections[0].value = score_text.get_score_text(score.get_score());
    }
}

fn ui_lives_system(
    _trigger: Trigger<PlayerLivesChanged>,
    mut commands: Commands,
    lives: Res<PlayerLives>,
    player_assets: Res<AsteroidPlayerAssets>,
    container_query: Query<Entity, With<LivesContaier>>,
) {
    let container = container_query.single();
    commands.entity(container).despawn_descendants();

    for (player_id, player_lives) in lives.get_lives().iter() {
        for _ in 0..*player_lives {
            let life_icon = commands
                .spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        ..Default::default()
                    },
                    image: UiImage::new(
                        player_assets
                            .get_texture_by_player_id(player_id)
                            .expect("Players texture should be loaded"),
                    ),
                    ..Default::default()
                })
                .id();

            commands.entity(container).add_child(life_icon);
        }

        let spacer = commands
            .spawn(NodeBundle {
                style: Style {
                    min_width: Val::Px(30.0),
                    min_height: Val::Px(30.0),
                    // border: UiRect::all(Val::Px(10.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();
        commands.entity(container).add_child(spacer);
    }
}
