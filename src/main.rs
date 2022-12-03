use bevy::prelude::*;

mod args;
mod credit_image;
mod menu;
mod network;
mod player;
mod procedural_functions;
mod save;
mod states;
mod world;

const TITLE: &str = "The Krusty Krabs";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

#[derive(Component)]
pub struct CharacterCamera;

fn main() {
    let args = args::get_args();
    warn!("game arguments: {:?}", args);
    let mut app = App::new();

    match args {
        args::GameArgs::Server(args) => {
            // server specific plugins
            // DefaultPlugins minus the unnecessary ones
            app.add_plugins(MinimalPlugins)
                .add_plugin(bevy::log::LogPlugin)
                .add_plugin(TransformPlugin)
                .add_plugin(HierarchyPlugin)
                .add_plugin(bevy::diagnostic::DiagnosticsPlugin)
                .add_plugin(bevy::asset::AssetPlugin)
                .add_plugin(bevy::scene::ScenePlugin);

            // TODO:
            // server player plugin

            app.add_plugin(states::server::StatePlugin);

            // server network plugin
            app.add_plugin(network::server::ServerPlugin { args });

            app.add_plugin(world::server::WorldPlugin);

            // server save/load plugin
            app.add_plugin(save::server::SaveLoadPlugin);
        }

        args::GameArgs::Client(args) => {
            // client specific plugins

            // default plugins
            app.add_plugins(DefaultPlugins);

            // our plugins
            app.add_plugin(states::client::StatePlugin)
                .add_plugin(credit_image::CreditImagePlugin)
                .add_plugin(menu::MenuPlugin)
                .insert_resource(WindowDescriptor {
                    title: String::from(TITLE),
                    width: WIN_W,
                    height: WIN_H,
                    ..default()
                })
                .insert_resource(ClearColor(Color::rgb(0.0, 0.6, 0.8)))
                .add_startup_system(|mut c: Commands| {
                    c.spawn_bundle(Camera2dBundle::default())
                        .insert(CharacterCamera);
                })
                .add_startup_system(setup_background)
                // TODO: rework for client
                .add_plugin(world::client::WorldPlugin)
                .add_plugin(player::PlayerPlugin);

            // client network plugin
            app.add_plugin(network::client::ClientPlugin { args });
        }
    }

    app.run();
}

fn setup_background(mut c: Commands, asset_server: Res<AssetServer>) {
    c.spawn_bundle(SpriteBundle {
        texture: asset_server.load("Background1.png"),
        transform: Transform {
            scale: Vec3::from_array([8., 8., 0.]),
            ..default()
        },
        ..default()
    });
}
