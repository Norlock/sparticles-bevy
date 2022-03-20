use crate::emitters::emitter::gen_abs_range;
use crate::emitters::emitter::gen_dyn_range;
use crate::emitters::emitter::EmitOptions;
use crate::emitters::emitter::EmitterParticleAttributes;
use crate::emitters::emitter::LifeCycle;
use crate::emitters::emitter::Particle;
use crate::emitters::emitter::ParticleAttributes;
use crate::emitters::emitter::Velocity;
use crate::Angles;
use crate::Emitter;
use bevy::{
    core_pipeline::Transparent3d,
    ecs::system::{lifetimeless::*, SystemParamItem},
    math::prelude::*,
    pbr::{MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup, SetMeshViewBindGroup},
    prelude::*,
    render::{
        mesh::GpuBufferInfo,
        render_asset::RenderAssets,
        render_component::{ExtractComponent, ExtractComponentPlugin},
        render_phase::{
            AddRenderCommand, DrawFunctions, EntityRenderCommand, RenderCommandResult, RenderPhase,
            SetItemPipeline, TrackedRenderPass,
        },
        render_resource::*,
        renderer::RenderDevice,
        view::{ComputedVisibility, ExtractedView, Msaa, Visibility},
        RenderApp, RenderStage,
    },
};
use bytemuck::{Pod, Zeroable};
use rand::thread_rng;

#[derive(Component)]
pub struct InstanceMaterialData(pub Vec<InstanceData>);

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct InstanceData {
    position: Vec3,
    scale: f32,
    color: [f32; 4],
}

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_shader)
            .add_plugin(CustomMaterialPlugin)
            .add_system(spawn_particles);
    }
}

fn setup_shader(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn().insert_bundle((
        meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.1,
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        InstanceMaterialData(Vec::new()),
        Visibility::default(),
        ComputedVisibility::default(),
    ));
}

impl ExtractComponent for InstanceMaterialData {
    type Query = &'static InstanceMaterialData;
    type Filter = ();

    fn extract_component(item: bevy::ecs::query::QueryItem<Self::Query>) -> Self {
        InstanceMaterialData(item.0.clone())
    }
}

pub struct CustomMaterialPlugin;

impl Plugin for CustomMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractComponentPlugin::<InstanceMaterialData>::default());
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawCustom>()
            .init_resource::<CustomPipeline>()
            .init_resource::<SpecializedPipelines<CustomPipeline>>()
            .add_system_to_stage(RenderStage::Queue, queue_custom)
            .add_system_to_stage(RenderStage::Extract, prepare_particles)
            .add_system_to_stage(RenderStage::Prepare, prepare_instance_buffers);
    }
}

fn spawn_particles(
    time: Res<Time>,
    mut emitter_query: Query<
        (
            Entity,
            &mut LifeCycle,
            &EmitOptions,
            &EmitterParticleAttributes,
        ),
        With<Emitter>,
    >,
    mut commands: Commands,
) {
    //event!(Level::INFO, "test2");

    let total_elapsed_ms = time.time_since_startup().as_millis();

    for (emitter_id, mut life_cycle, emit_options, particle_attributes) in emitter_query.iter_mut()
    {
        let elapsed_ms = life_cycle.elapsed_ms(total_elapsed_ms);
        let out_of_time = life_cycle.duration_ms < elapsed_ms;
        let new_iteration = elapsed_ms as i32 / emit_options.delay_between_emission_ms as i32;

        if out_of_time {
            continue;
        } else if new_iteration == life_cycle.iteration {
            continue;
        }

        life_cycle.iteration = new_iteration;

        let mut rng = thread_rng();

        for _ in 0..emit_options.particles_per_emission {
            let emitter_length = gen_abs_range(&mut rng, emit_options.emitter_size.length);
            let emitter_depth = gen_abs_range(&mut rng, emit_options.emitter_size.depth);
            let distortion = gen_dyn_range(&mut rng, emit_options.emission_distortion);

            let Angles { elevation, bearing } = emit_options.angle_radians;
            // Used to emit perpendicular of emitter.
            let perpendicular = elevation.cos() * -1.;
            let x = distortion + emitter_length * perpendicular * bearing.cos();
            let y = distortion + emitter_length * elevation.sin() * bearing.cos();
            let z = (distortion + emitter_depth) + emitter_length * bearing.sin();

            let diffusion_elevation_delta =
                gen_dyn_range(&mut rng, emit_options.diffusion_radians.elevation);
            let bearing_radians = gen_dyn_range(&mut rng, emit_options.diffusion_radians.bearing);
            let elevation_radians =
                emit_options.angle_emission_radians() + diffusion_elevation_delta;

            // Used to emit perpendicular of emitter.
            let perpendicular = elevation_radians.cos() * -1.;
            let vx = particle_attributes.speed * perpendicular * bearing_radians.cos();
            let vy = particle_attributes.speed * elevation_radians.sin() * bearing_radians.cos();
            let vz = particle_attributes.speed * bearing_radians.sin();

            let speed = Velocity { vx, vy, vz };
            let life_cycle = LifeCycle {
                spawned_at: total_elapsed_ms,
                duration_ms: particle_attributes.duration_ms,
                iteration: -1,
            };

            let attributes = ParticleAttributes {
                friction_coefficient: particle_attributes.friction_coefficient,
                radius: particle_attributes.radius,
                mass: particle_attributes.mass,
                color: particle_attributes.color,
            };

            commands
                .spawn()
                .insert(Parent(emitter_id))
                .insert_bundle((
                    speed,
                    life_cycle,
                    attributes,
                    Particle,
                    Transform::from_xyz(x, y, z),
                ))
                .id();

            //let mut instance_data = instance_query.single_mut();
            //instance_data.0.push(InstanceData {
            //position: Vec3::new(x, y, z),
            //scale: 1.,
            //color: particle_attributes.color.as_rgba_f32(),
            //});
        }
    }
}

fn prepare_particles(
    mut instance_query: Query<&mut InstanceMaterialData>,
    particle_query: Query<(&Transform, &ParticleAttributes), With<Particle>>,
) {
    let mut instances = Vec::new();
    let mut instance_data = instance_query.single_mut();

    for (transform, attributes) in particle_query.iter() {
        instances.push(InstanceData {
            position: transform.translation,
            color: attributes.color.as_rgba_f32(),
            scale: 1.,
        });
    }

    instance_data.0 = instances;
}

fn queue_custom(
    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
    custom_pipeline: Res<CustomPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedPipelines<CustomPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    material_meshes: Query<
        (Entity, &MeshUniform),
        (With<Handle<Mesh>>, With<InstanceMaterialData>),
    >,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>,
) {
    let draw_custom = transparent_3d_draw_functions
        .read()
        .get_id::<DrawCustom>()
        .unwrap();

    let key = MeshPipelineKey::from_msaa_samples(msaa.samples)
        | MeshPipelineKey::from_primitive_topology(PrimitiveTopology::TriangleList);
    let pipeline = pipelines.specialize(&mut pipeline_cache, &custom_pipeline, key);

    for (view, mut transparent_phase) in views.iter_mut() {
        let view_matrix = view.transform.compute_matrix();
        let view_row_2 = view_matrix.row(2);
        for (entity, mesh_uniform) in material_meshes.iter() {
            transparent_phase.add(Transparent3d {
                entity,
                pipeline,
                draw_function: draw_custom,
                distance: view_row_2.dot(mesh_uniform.transform.col(3)),
            });
        }
    }
}

#[derive(Component)]
pub struct InstanceBuffer {
    buffer: Buffer,
    length: usize,
}

fn prepare_instance_buffers(
    mut commands: Commands,
    query: Query<(Entity, &InstanceMaterialData)>,
    render_device: Res<RenderDevice>,
) {
    for (entity, instance_data) in query.iter() {
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("instance data buffer"),
            contents: bytemuck::cast_slice(instance_data.0.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        commands.entity(entity).insert(InstanceBuffer {
            buffer,
            length: instance_data.0.len(),
        });
    }
}

pub struct CustomPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
}

impl FromWorld for CustomPipeline {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        asset_server.watch_for_changes().unwrap();
        let shader = asset_server.load("shaders/instancing.wgsl");

        let mesh_pipeline = world.get_resource::<MeshPipeline>().unwrap();

        CustomPipeline {
            shader,
            mesh_pipeline: mesh_pipeline.clone(),
        }
    }
}

impl SpecializedPipeline for CustomPipeline {
    type Key = MeshPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut descriptor = self.mesh_pipeline.specialize(key);
        descriptor.vertex.shader = self.shader.clone();
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceData>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 3, // shader locations 0-2 are taken up by Position, Normal and UV attributes
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: VertexFormat::Float32x4.size(),
                    shader_location: 4,
                },
            ],
        });
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        descriptor.layout = Some(vec![
            self.mesh_pipeline.view_layout.clone(),
            self.mesh_pipeline.mesh_layout.clone(),
        ]);

        descriptor
    }
}

type DrawCustom = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    DrawMeshInstanced,
);

pub struct DrawMeshInstanced;
impl EntityRenderCommand for DrawMeshInstanced {
    type Param = (
        SRes<RenderAssets<Mesh>>,
        SQuery<Read<Handle<Mesh>>>,
        SQuery<Read<InstanceBuffer>>,
    );
    #[inline]
    fn render<'w>(
        _view: Entity,
        item: Entity,
        (meshes, mesh_query, instance_buffer_query): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let mesh_handle = mesh_query.get(item).unwrap();
        let instance_buffer = instance_buffer_query.get(item).unwrap();

        let gpu_mesh = match meshes.into_inner().get(mesh_handle) {
            Some(gpu_mesh) => gpu_mesh,
            None => return RenderCommandResult::Failure,
        };

        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));

        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        match &gpu_mesh.buffer_info {
            GpuBufferInfo::Indexed {
                buffer,
                index_format,
                count,
            } => {
                pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                pass.draw_indexed(0..*count, 0, 0..instance_buffer.length as u32);
            }
            GpuBufferInfo::NonIndexed { vertex_count } => {
                pass.draw_indexed(0..*vertex_count, 0, 0..instance_buffer.length as u32);
            }
        }
        RenderCommandResult::Success
    }
}
