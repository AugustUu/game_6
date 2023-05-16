use std::println;

use bevy::prelude::*;

use crate::{GameState, loader::FontAssets};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(GameState::Menu)))
        .add_system(click_play_button.in_set(OnUpdate(GameState::Menu)))
        .add_system(cleanup_menu.in_schedule(OnExit(GameState::Menu)));;
    }
}
fn setup_menu(mut commands: Commands, fonts: Res<FontAssets>){
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: bevy::prelude::BackgroundColor(Color::rgb(0.15, 0.15, 0.15)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Play",
                TextStyle {
                    font : fonts.font.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                }
            ));
        });
}

fn click_play_button(mut state: ResMut<NextState<GameState>>,interaction_query: Query<(&Interaction),(Changed<Interaction>, With<Button>)>){
    if let Ok(interaction) = interaction_query.get_single(){
        if let Interaction::Clicked = interaction {
            state.set(GameState::Playing)
        }
    }   
}

fn cleanup_menu(mut commands: Commands, button: Query<Entity, With<Button>>){
    // clean camera and menue
}