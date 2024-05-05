use bevy::{
    prelude::*,
    app::AppExit,
    diagnostic::{
        DiagnosticsStore,
        FrameTimeDiagnosticsPlugin,
    },
};
use bevy_args::{
    BevyArgsPlugin,
    Deserialize,
    Parser,
    Serialize,
    parse_args,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ort::{
    BevyOrtPlugin,
    models::flame::{
        FlameOutput,
        Flame,
        FlamePlugin,
    },
};
use bevy_panorbit_camera::{
    PanOrbitCamera,
    PanOrbitCameraPlugin,
};

use bevy_flame::{
    noise::{
        NoisyFlamePlugin,
        NoisyFlameInput,
    },
    upload::MeshUploadPlugin,
};


#[derive(
    Clone,
    Default,
    Debug,
    Resource,
    Serialize,
    Deserialize,
    Parser,
)]
#[command(about = "a GUI editor application for developing ladon kernels", version)]
struct BevyFlameArgs {
    #[arg(long, default_value = "1920.0", help = "editor output width")]
    width: f32,

    #[arg(long, default_value = "1080.0", help = "editor output height")]
    height: f32,

    #[arg(long, default_value = "true")]
    pub show_fps: bool,

    #[arg(long, default_value = "true")]
    pub editor: bool,
}


fn main() {
    let args = parse_args::<BevyFlameArgs>();

    let mut app = App::new();
    app.add_plugins(
            DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    mode: bevy::window::WindowMode::Windowed,
                    resolution: (args.width, args.height).into(),
                    title: "bevy_flame".to_string(),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins((
            BevyArgsPlugin::<BevyFlameArgs>::default(),
            BevyOrtPlugin,
            FlamePlugin,
            PanOrbitCameraPlugin,
            MeshUploadPlugin,
            NoisyFlamePlugin,
        ))
        .add_systems(Startup, load_flame)
        .add_systems(Startup, setup)
        .add_systems(Update, press_esc_close);

    app.insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)));

    if args.editor {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    if args.show_fps {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.add_systems(PostStartup, fps_display_setup);
        app.add_systems(Update, fps_update_system);
    }

    app.run();
}


fn load_flame(
    asset_server: Res<AssetServer>,
    mut flame: ResMut<Flame>,
) {
    flame.onnx = asset_server.load("models/flame.onnx");
}


fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        NoisyFlameInput::default(),
        PbrBundle {
            mesh: meshes.add(FlameOutput::default().mesh()),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("#d7bd96").unwrap(),
                ..default()
            }),
            ..default()
        },
    ));

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: 1_500.,
            ..default()
        },
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.8).looking_at(Vec3::default(), Vec3::Y),
            ..default()
        },
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 900.0,
        },
        PanOrbitCamera {
            allow_upside_down: true,
            ..default()
        },
    ));
}



fn fps_display_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "fps: ",
                TextStyle {
                    font: asset_server.load("fonts/Caveat-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/Caveat-Medium.ttf"),
                font_size: 60.0,
                color: Color::GOLD,
            }),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        FpsText,
    ));
}

#[derive(Component)]
struct FpsText;

fn fps_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{:.2}", value);
            }
        }
    }
}


pub fn press_esc_close(
    keys: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
