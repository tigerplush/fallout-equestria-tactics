use bevy::prelude::*;
use bevy_renet::renet::{RenetClient, DefaultChannel};
use fallout_equestria_tactics::messages::ClientMessage;

use crate::{common::ClientState, gui_plugin::{PRESSED_BUTTON, HOVERED_BUTTON, NORMAL_BUTTON, FontHandle, Canvas, HOVERED_PRESSED_BUTTON, HOVERED_PRESSED_BUTTON2}};

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(
                ClientState::Lobby
            ).with_system(setup_lobby_canvas)
        )
        .add_system_set(
            SystemSet::on_update(
                ClientState::Lobby)
                .with_system(handle_ready_button)
        )
        .add_system_set(
            SystemSet::on_exit(ClientState::Lobby)
            .with_system(remove_read_button)
        );
        info!("LobbyPlugin has been loaded");
    }
}

#[derive(Component)]
struct ReadyButton;

fn setup_lobby_canvas(
    mut commands: Commands,
    font_handle: Res<FontHandle>,
    canvas: Res<Canvas>,
) {
    commands
        .entity(canvas.0)
        .with_children(|parent| {
            parent
            .spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            })
            .insert(ReadyButton)
            .insert(Name::from("Ready Button"))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Ready",
                    TextStyle {
                        font: font_handle.0.clone(),
                        font_size: 46.0,
                        ..default()
                    },
                ));
            });
        });
}

fn handle_ready_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut client: ResMut<RenetClient>,
    mut ready: Local<bool>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON.into();
                *ready = !*ready;
                // maybe change this to fire an event? so that GUI and logic are separated
                let message = bincode::serialize(&ClientMessage::ClientReady).unwrap();
                client.send_message(DefaultChannel::Reliable, message);
            }
            Interaction::Hovered => {
                match *ready {
                    true => *background_color = HOVERED_PRESSED_BUTTON.into(),
                    false => *background_color = HOVERED_BUTTON.into(),
                };
            }
            Interaction::None => {
                match *ready {
                    true => *background_color = HOVERED_PRESSED_BUTTON2.into(),
                    false => *background_color = NORMAL_BUTTON.into(),
                };
            }
        }
    }
}

fn remove_read_button(mut commands: Commands, query: Query<Entity, With<ReadyButton>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}