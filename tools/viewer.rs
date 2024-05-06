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

#[cfg(target_arch = "wasm32")]
use bevy_ort::{Onnx, Session};

use bevy_flame::{
    noise::{
        NoisyFlamePlugin,
        NoisyFlameInput,
    },
    upload::MeshUploadPlugin,
};


pub fn log(msg: &str) {
    #[cfg(debug_assertions)]
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::log_1(&msg.into());
    }
    #[cfg(debug_assertions)]
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("{}", msg);
    }
}


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
    #[serde(default)]
    width: f32,

    #[arg(long, default_value = "1080.0", help = "editor output height")]
    #[serde(default)]
    height: f32,

    #[arg(long, default_value = "true")]
    #[serde(default)]
    pub show_fps: bool,

    #[arg(long, default_value = "true")]
    #[serde(default)]
    pub editor: bool,
}


fn main() {
    log("bevy_flame starting...");
    #[cfg(debug_assertions)]
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
    }

    #[cfg(target_arch = "wasm32")]
    {
        ort::wasm::initialize();
    }

    let args = parse_args::<BevyFlameArgs>();

    let mut app = App::new();

    #[cfg(target_arch = "wasm32")]
    let primary_window = Some(Window {
        // fit_canvas_to_parent: true,
        canvas: Some("#bevy".to_string()),
        mode: bevy::window::WindowMode::Windowed,
        prevent_default_event_handling: true,
        title: "bevy_flame".to_string(),

        #[cfg(feature = "perftest")]
        present_mode: bevy::window::PresentMode::AutoNoVsync,
        #[cfg(not(feature = "perftest"))]
        present_mode: bevy::window::PresentMode::AutoVsync,

        ..default()
    });

    #[cfg(not(target_arch = "wasm32"))]
    let primary_window = Some(Window {
        canvas: Some("#bevy".to_string()),
        mode: bevy::window::WindowMode::Windowed,
        prevent_default_event_handling: false,
        resolution: (args.width, args.height).into(),
        title: "bevy_flame".to_string(),

        #[cfg(feature = "perftest")]
        present_mode: bevy::window::PresentMode::AutoNoVsync,
        #[cfg(not(feature = "perftest"))]
        present_mode: bevy::window::PresentMode::AutoVsync,

        ..default()
    });

    app.add_plugins(
            DefaultPlugins
            .set(WindowPlugin {
                primary_window,
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
        .add_systems(Startup, setup)
        .add_systems(Update, press_esc_close);

    app.add_systems(Startup, load);

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


#[cfg(not(target_arch = "wasm32"))]
fn load(
    asset_server: Res<AssetServer>,
    mut flame: ResMut<Flame>,
) {
    flame.onnx = asset_server.load("models/flame.onnx");
}

#[cfg(target_arch = "wasm32")]
static ORT_DATA: &[u8] = include_bytes!("../assets/models/flame.ort");

#[cfg(target_arch = "wasm32")]
fn load(
    mut flame: ResMut<Flame>,
    mut onnx: ResMut<Assets<Onnx>>,
) {
    flame.onnx = onnx.add(
        Onnx::from_in_memory(
            Session::builder()
                .unwrap()
                .commit_from_memory_directly(ORT_DATA)
                .unwrap()
        )
    );
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
