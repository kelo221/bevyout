[bevy](../index.html)::[pbr](index.html)

# Function get\_mesh\_instance\_world\_from\_local 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1809-1814)

```rust
pub fn get_mesh_instance_world_from_local(
    entity: MainEntity,
    current_uniform_index: InputUniformIndex,
    render_mesh_instances: &RenderMeshInstances,
    maybe_batched_instance_buffers: Option<&BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
) -> Affine3
```

Returns the world-from-local transform for the given mesh instance.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/shader\_advanced/custom\_shader\_instancing.rs ([lines 182-187](../../src/custom_shader_instancing/custom_shader_instancing.rs.html#182-187))

```rust
137fn queue_custom(
138    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
139    custom_pipeline: Res<CustomPipeline>,
140    mut pipelines: ResMut<SpecializedMeshPipelines<CustomPipeline>>,
141    pipeline_cache: Res<PipelineCache>,
142    meshes: Res<RenderAssets<RenderMesh>>,
143    render_mesh_instances: Res<RenderMeshInstances>,
144    maybe_batched_instance_buffers: Option<
145        Res<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
146    >,
147    material_meshes: Query<(Entity, &MainEntity), With<InstanceMaterialData>>,
148    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent3d>>,
149    views: Query<&ExtractedView>,
150    view_key_cache: Res<ViewKeyCache>,
151) {
152    let draw_custom = transparent_3d_draw_functions.read().id::<DrawCustom>();
153
154    for view in &views {
155        let Some(transparent_phase) = transparent_render_phases.get_mut(&view.retained_view_entity)
156        else {
157            continue;
158        };
159
160        let Some(&view_key) = view_key_cache.get(&view.retained_view_entity) else {
161            continue;
162        };
163
164        for (entity, main_entity) in &material_meshes {
165            let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(*main_entity)
166            else {
167                continue;
168            };
169            let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id()) else {
170                continue;
171            };
172            let key = view_key
173                | MeshPipelineKey::from_primitive_topology_and_strip_index(
174                    mesh.primitive_topology(),
175                    mesh.index_format(),
176                );
177            let pipeline = pipelines
178                .specialize(&pipeline_cache, &custom_pipeline, key, &mesh.layout)
179                .unwrap();
180            transparent_phase.add_retained(Transparent3d {
181                sorting_info: TransparentSortingInfo3d::Sorted {
182                    mesh_center: pbr::get_mesh_instance_world_from_local(
183                        *main_entity,
184                        mesh_instance.current_uniform_index,
185                        &render_mesh_instances,
186                        maybe_batched_instance_buffers.as_deref(),
187                    )
188                    .transform_point3(
189                        meshes
190                            .get(mesh_instance.mesh_asset_id())
191                            .unwrap()
192                            .aabb_center,
193                    ),
194                    depth_bias: 0.0,
195                },
196                entity: (entity, *main_entity),
197                pipeline,
198                draw_function: draw_custom,
199                distance: 0.0,
200                batch_range: 0..1,
201                extra_index: PhaseItemExtraIndex::None,
202                indexed: true,
203            });
204        }
205    }
206}
```

Hide additional examples

examples/shader\_advanced/custom\_render\_phase.rs ([lines 622-627](../../src/custom_render_phase/custom_render_phase.rs.html#622-627))

```rust
531fn queue_custom_meshes(
532    custom_draw_functions: Res<DrawFunctions<Stencil3d>>,
533    mut pipelines: ResMut<SpecializedMeshPipelines<StencilPipeline>>,
534    pipeline_cache: Res<PipelineCache>,
535    custom_draw_pipeline: Res<StencilPipeline>,
536    render_meshes: Res<RenderAssets<RenderMesh>>,
537    render_mesh_instances: Res<RenderMeshInstances>,
538    maybe_batched_instance_buffers: Option<
539        Res<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
540    >,
541    mut custom_render_phases: ResMut<ViewSortedRenderPhases<Stencil3d>>,
542    mut views: Query<(&ExtractedView, &RenderVisibleEntities)>,
543    view_key_cache: Res<ViewKeyCache>,
544    dirty_specializations: Res<DirtySpecializations>,
545    mut pending_custom_mesh_queues: ResMut<PendingCustomMeshQueues>,
546    has_marker: Query<(), With<DrawStencil>>,
547) {
548    for (view, visible_entities) in &mut views {
549        let Some(custom_phase) = custom_render_phases.get_mut(&view.retained_view_entity) else {
550            continue;
551        };
552        let draw_custom = custom_draw_functions.read().id::<DrawMesh3dStencil>();
553
554        let Some(&view_key) = view_key_cache.get(&view.retained_view_entity) else {
555            continue;
556        };
557
558        // Since our phase can work on any 3d mesh we can reuse the default mesh 3d filter
559        let Some(render_visible_mesh_entities) = visible_entities.get::<Mesh3d>() else {
560            continue;
561        };
562
563        let view_pending_custom_mesh_queues =
564            pending_custom_mesh_queues.prepare_for_new_frame(view.retained_view_entity);
565
566        // First, remove meshes that need to be respecialized, and those that were removed, from the bins.
567        for &main_entity in dirty_specializations
568            .iter_to_dequeue(view.retained_view_entity, render_visible_mesh_entities)
569        {
570            custom_phase.remove(Entity::PLACEHOLDER, main_entity);
571        }
572
573        for (render_entity, visible_entity) in dirty_specializations.iter_to_queue(
574            view.retained_view_entity,
575            render_visible_mesh_entities,
576            &view_pending_custom_mesh_queues.prev_frame,
577        ) {
578            // We only want meshes with the marker component to be queued to our phase.
579            if has_marker.get(*render_entity).is_err() {
580                continue;
581            }
582            let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(*visible_entity)
583            else {
584                // We couldn't fetch the mesh, probably because it hasn't been
585                // loaded yet. Add the entity to the list of pending custom mesh
586                // queues and bail.
587                view_pending_custom_mesh_queues
588                    .current_frame
589                    .insert((*render_entity, *visible_entity));
590                continue;
591            };
592            let Some(mesh) = render_meshes.get(mesh_instance.mesh_asset_id()) else {
593                continue;
594            };
595
596            // Specialize the key for the current mesh entity
597            // For this example we only specialize based on the mesh topology
598            // but you could have more complex keys and that's where you'd need to create those keys
599            let mut mesh_key = view_key;
600            mesh_key |= MeshPipelineKey::from_primitive_topology_and_strip_index(
601                mesh.primitive_topology(),
602                mesh.index_format(),
603            );
604
605            let pipeline_id = pipelines.specialize(
606                &pipeline_cache,
607                &custom_draw_pipeline,
608                mesh_key,
609                &mesh.layout,
610            );
611            let pipeline_id = match pipeline_id {
612                Ok(id) => id,
613                Err(err) => {
614                    error!("{}", err);
615                    continue;
616                }
617            };
618            // At this point we have all the data we need to create a phase item and add it to our
619            // phase
620            custom_phase.add_retained(Stencil3d {
621                sorting_info: TransparentSortingInfo3d::Sorted {
622                    mesh_center: pbr::get_mesh_instance_world_from_local(
623                        *visible_entity,
624                        mesh_instance.current_uniform_index,
625                        &render_mesh_instances,
626                        maybe_batched_instance_buffers.as_deref(),
627                    )
628                    .transform_point3(
629                        render_meshes
630                            .get(mesh_instance.mesh_asset_id())
631                            .unwrap()
632                            .aabb_center,
633                    ),
634                    depth_bias: 0.0,
635                },
636                distance: FloatOrd(0.0),
637                entity: (Entity::PLACEHOLDER, *visible_entity),
638                pipeline: pipeline_id,
639                draw_function: draw_custom,
640                // Sorted phase items aren't batched
641                batch_range: 0..1,
642                extra_index: PhaseItemExtraIndex::None,
643                indexed: mesh.indexed(),
644            });
645        }
646    }
647}
```