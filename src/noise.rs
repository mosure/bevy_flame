use bevy::prelude::*;
use bevy_ort::models::flame::FlameInput;
use noisy_bevy::simplex_noise_2d_seeded;


pub struct NoisyFlamePlugin;
impl Plugin for NoisyFlamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<NoisyFlameInput>();

        app.add_systems(
            Update,
            convert_noisy_flame_input,
        );
    }
}


#[derive(Debug, Reflect)]
pub struct NoiseScale {
    pub time: f32,
    pub value: f32,
}

impl NoiseScale {
    pub fn new(time: f32, value: f32) -> Self {
        Self {
            time,
            value,
        }
    }
}

// TODO: add helpers to populate tensor with noise
#[derive(Debug, Component, Reflect)]
pub struct NoisyFlameInput {
    pub expression: NoiseScale,
    pub eye: NoiseScale,
    pub neck: NoiseScale,
    pub shape: NoiseScale,
    pub time_scale: f32,
}

impl Default for NoisyFlameInput {
    fn default() -> Self {
        Self {
            expression: NoiseScale::new(1.0, 2.0),
            eye: NoiseScale::new(1.0, 0.6),
            neck: NoiseScale::new(0.3, 0.3),
            shape: NoiseScale::new(0.2, 2.0),
            time_scale: 0.7,
        }
    }
}

impl NoisyFlameInput {
    pub fn generate(
        &self,
        time: f32,
    ) -> FlameInput {
        let time = time * self.time_scale;
        let mut flame_input = FlameInput::default();

        flame_input.expression[0].iter_mut()
            .enumerate()
            .for_each(|(i, value)| {
                *value = simplex_noise_2d_seeded(
                    Vec2::new(self.expression.time * time, 0.0),
                    i as f32 * 1.3,
                ) * self.expression.value;
            });

        flame_input.eye[0].iter_mut()
            .enumerate()
            .for_each(|(i, value)| {
                *value = simplex_noise_2d_seeded(
                    Vec2::new(self.eye.time * time, 0.0),
                    i as f32 * 1.17,
                ) * self.eye.value;
            });

        flame_input.neck[0].iter_mut()
            .enumerate()
            .for_each(|(i, value)| {
                *value = simplex_noise_2d_seeded(
                    Vec2::new(self.neck.time * time, 0.0),
                    i as f32 * 1.47,
                ) * self.neck.value;
            });

        flame_input.shape[0].iter_mut()
            .enumerate()
            .for_each(|(i, value)| {
                *value = simplex_noise_2d_seeded(
                    Vec2::new(self.shape.time * time, 0.0),
                    i as f32 * 1.7,
                ) * self.shape.value;
            });

        flame_input
    }
}


fn convert_noisy_flame_input(
    mut commands: Commands,
    time: Res<Time>,
    mut noisy_flame_inputs: Query<
        (
            Entity,
            &NoisyFlameInput,
        ),
    >,
) {
    for (
        entity,
        noisy_flame_input,
    ) in noisy_flame_inputs.iter_mut() {
        commands.entity(entity)
            .insert(noisy_flame_input.generate(time.elapsed_seconds()));
    }
}
