use bevy::prelude::*;
use bevy_renet::renet::{RenetClient, DefaultChannel};
use fallout_equestria_tactics::{foe_client_plugin::FoEClientPlugin, messages::ClientMessage};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FoEClientPlugin)
        .add_startup_system(setup)
        .add_system(handle_button)
        .run();
}


const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
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
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Ready",
                TextStyle{
                    font: asset_server.load("fonts/Overseer.otf"),
                    font_size: 46.0,
                    ..default()
                },
            ));
        });
}

fn handle_button(
    mut interaction_query: Query<(&Children, &Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
    mut client: ResMut<RenetClient>,
) {
    for (children, interaction, mut background_color) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *background_color = PRESSED_BUTTON.into();
                let message = bincode::serialize(&ClientMessage::ClientReady).unwrap();
                client.send_message(DefaultChannel::Reliable, message);
            },
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *background_color = HOVERED_BUTTON.into();
            },
            Interaction::None => {
                text.sections[0].value = "Ready".to_string();
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}