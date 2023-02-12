use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use fallout_equestria_tactics::messages::ClientMessage;

use crate::common::ClientState;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_font);
        app.add_system_set(
            SystemSet::on_enter(ClientState::Connected).with_system(setup_ready_button),
        )
        .add_system_set(
            SystemSet::on_update(ClientState::Connected).with_system(handle_ready_button),
        )
        .add_system_set(SystemSet::on_exit(ClientState::Connected).with_system(remove_read_button));
        app.add_system_set(SystemSet::on_enter(ClientState::Acting).with_system(setup_acting))
            .add_system_set(SystemSet::on_update(ClientState::Acting).with_system(update_acting))
            .add_system_set(SystemSet::on_exit(ClientState::Acting).with_system(exit_acting));
        app.add_system_set(SystemSet::on_enter(ClientState::Idling).with_system(setup_idling))
            .add_system_set(SystemSet::on_update(ClientState::Idling).with_system(update_idling))
            .add_system_set(SystemSet::on_exit(ClientState::Idling).with_system(exit_idling));
        info!("GuiPlugin loaded");
    }
}

fn load_font() {}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct ReadyButton;

fn setup_ready_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
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
                    font: asset_server.load("fonts/Overseer.otf"),
                    font_size: 46.0,
                    ..default()
                },
            ));
        });
}

fn handle_ready_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut client: ResMut<RenetClient>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON.into();
                let message = bincode::serialize(&ClientMessage::ClientReady).unwrap();
                client.send_message(DefaultChannel::Reliable, message);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn remove_read_button(mut commands: Commands, query: Query<Entity, With<ReadyButton>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
struct EndTurnButton;

fn setup_acting(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
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
        .insert(EndTurnButton)
        .insert(Name::from("End Turn Button"))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "End Turn",
                TextStyle {
                    font: asset_server.load("fonts/Overseer.otf"),
                    font_size: 46.0,
                    ..default()
                },
            ));
        });
}

fn update_acting(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<EndTurnButton>),
    >,
    mut client: ResMut<RenetClient>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON.into();
                let message = bincode::serialize(&ClientMessage::EndTurn).unwrap();
                client.send_message(DefaultChannel::Reliable, message);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn exit_acting(mut commands: Commands, query: Query<Entity, With<EndTurnButton>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
struct IdleText;

fn setup_idling(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(
            TextBundle::from_section(
                "Waiting for enemy turn",
                TextStyle {
                    font: asset_server.load("fonts/Overseer.otf"),
                    font_size: 46.0,
                    ..default()
                },
            )
            .with_text_alignment(TextAlignment::CENTER),
        )
        .insert(IdleText)
        .insert(Name::from("Waiting Text"));
}

fn update_idling() {}

fn exit_idling(mut commands: Commands, query: Query<Entity, With<IdleText>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
