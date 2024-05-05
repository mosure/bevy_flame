use bevy::prelude::*;
use bevy_ort::models::flame::FlameOutput;


pub struct MeshUploadPlugin;
impl Plugin for MeshUploadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_flame_output);
    }
}


fn on_flame_output(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    flame_outputs: Query<
        (
            Entity,
            &FlameOutput,
            &Handle<Mesh>,
        ),
    >,
) {
    for (
        entity,
        flame_output,
        flame_mesh_handle,
    ) in flame_outputs.iter() {
        commands.entity(entity)
            .remove::<FlameOutput>();

        let mut mesh = flame_output.mesh();
        mesh.duplicate_vertices();
        mesh.compute_flat_normals();

        meshes.insert(flame_mesh_handle, mesh);
    }
}
