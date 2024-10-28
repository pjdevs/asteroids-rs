use crate::asteroid::{states::AsteroidGameState, systems::despawn_entities_with};
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AsteroidMenuUiSystem {
    UpdateUi,
}

pub struct AsteroidMenuUiPlugin;

impl Plugin for AsteroidMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonEvent>()
            .add_systems(
                OnEnter(AsteroidGameState::MainMenu),
                (ui_menu_setup_system,),
            )
            .add_systems(
                OnExit(AsteroidGameState::MainMenu),
                (despawn_entities_with::<Node>,),
            )
            .add_systems(
                Update,
                (
                    ui_button_released_system,
                    ui_play_system,
                    ui_button_style_system,
                )
                    .run_if(in_state(AsteroidGameState::MainMenu))
                    .in_set(AsteroidMenuUiSystem::UpdateUi),
            );
    }
}

#[derive(Component)]
enum MenuButtonAction {
    Play,
    // Options,
    // Exit,
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
        .spawn((
            button_node,
            LastInteraction::default(),
            MenuButtonAction::Play,
        ))
        .id();
    let button_text = commands.spawn(button_text_node).id();

    commands.entity(button).push_children(&[button_text]);
    commands.entity(container).push_children(&[button]);
}

fn ui_play_system(
    mut events: EventReader<ButtonEvent>,
    mut next_state: ResMut<NextState<AsteroidGameState>>,
    query: Query<&MenuButtonAction>,
) {
    for event in events.read() {
        match event {
            ButtonEvent::Clicked(entity) => {
                if let Ok(action) = query.get(*entity) {
                    match action {
                        MenuButtonAction::Play => {
                            next_state.set(AsteroidGameState::GameLoadingScreen)
                        }
                    }
                }
            }
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

// Release buttons

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

        last.last = *current;
    }
}
