use crate::loading::FontAssets;
use crate::palette::Palette;
use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

fn setup_menu(mut commands: Commands, font_assets: Res<FontAssets>, palette: Res<Palette>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(120.0),
                height: Val::Px(50.0),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: palette.dark.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Play",
                TextStyle {
                    font: font_assets.fira_sans.clone(),
                    font_size: 40.0,
                    color: palette.white,
                },
            ));
        });
}

fn click_play_button(
    palette: Res<Palette>,
    mut state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = palette.orange.into();
            }
            Interaction::None => {
                *color = palette.red.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, button: Query<Entity, With<Button>>) {
    commands.entity(button.single()).despawn_recursive();
}
