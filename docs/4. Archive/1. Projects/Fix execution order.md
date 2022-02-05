[[Player state refactor]] will likely change up the systems a lot, so do this after that

Can you make a system set automatically execute in a deterministic order?

- Check that crouching doesn't spazz
- Jump height inconsistency among others is likely due to the ambiguous system execution order.
- There is a bug with 1f moves occasionally not counting for mobility, this is probably fixed wit the ordering

Remaining trace from bevy logging:
```
2022-02-05T11:47:54.650510Z  INFO bevy_ecs::schedule::stage: Execution order ambiguities detected, you might want to add an explicit dependency relation between some of these systems:
 * Parallel systems:
 -- "bevy_input::gamepad::gamepad_event_system" and "bevy_input::gamepad::gamepad_connection_system"
    conflicts: ["bevy_ecs::event::Events<bevy_input::gamepad::GamepadEvent>"]
 -- "bevy_egui::systems::process_input" and "bevy_ui::focus::ui_focus_system"
    conflicts: ["bevy_window::windows::Windows"]

2022-02-05T11:47:54.651769Z  INFO bevy_input::gamepad: Gamepad(0) Connected
New gamepad connected with ID: Gamepad(0)
2022-02-05T11:47:54.676201Z  INFO bevy_ecs::schedule::stage: Execution order ambiguities detected, you might want to add an explicit dependency relation between some of these systems:
 * Parallel systems:
 -- "bevy_ui::widget::image::image_node_system" and "bevy_ui::widget::text::text_system"
    conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::image::Image>", "bevy_ui::ui_node::CalculatedSize"]
 -- "bevy_ui::widget::image::image_node_system" and "bevy_text::text2d::text2d_system"
    conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::image::Image>"]
 -- "bevy_ui::widget::image::image_node_system" and "bevy_egui::update_egui_textures"
    conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::image::Image>"]
 -- "bevy_ui::widget::text::text_system" and "bevy_text::text2d::text2d_system"
    conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::image::Image>", "bevy_asset::assets::Assets<bevy_sprite::texture_atlas::TextureAtlas>", "bevy_asset::assets::Assets<bevy_text::font_atlas_set::FontAtlasSet>", "bevy_text::pipeline::TextPipeline<bevy_ecs::entity::Entity>"]   
 -- "bevy_ui::widget::text::text_system" and "bevy_egui::update_egui_textures"
    conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::image::Image>"]
 -- "bevy_ui::flex::flex_node_system" and "bevy_transform::hierarchy::hierarchy_maintenance_system::parent_update_system"
    conflicts: ["bevy_transform::components::children::Children"]
 -- "bevy_ui::update::ui_z_system" and "bevy_transform::hierarchy::hierarchy_maintenance_system::parent_update_system"
    conflicts: ["bevy_transform::components::children::Children"]
 -- "bevy_render::view::visibility::update_frusta<bevy_render::camera::projection::PerspectiveProjection>" and "bevy_render::view::visibility::update_frusta<bevy_render::camera::projection::OrthographicProjection>"
    conflicts: ["bevy_render::primitives::Frustum"]
 -- "bevy_render::view::visibility::update_frusta<bevy_render::camera::projection::PerspectiveProjection>" and "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::PerspectiveProjection>"
    conflicts: ["bevy_render::camera::projection::PerspectiveProjection"]
 -- "bevy_render::view::visibility::update_frusta<bevy_render::camera::projection::PerspectiveProjection>" and "bevy_pbr::light::assign_lights_to_clusters"
    conflicts: ["bevy_render::primitives::Frustum"]
 -- "bevy_render::view::visibility::update_frusta<bevy_render::camera::projection::PerspectiveProjection>" and "bevy_pbr::light::update_directional_light_frusta"
    conflicts: ["bevy_render::primitives::Frustum"]
 -- "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::OrthographicProjection>" and "bevy_render::view::visibility::update_frusta<bevy_render::camera::projection::OrthographicProjection>"
    conflicts: ["bevy_render::camera::projection::OrthographicProjection"]
 -- "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::OrthographicProjection>" and "bevy_pbr::light::update_clusters" 
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::OrthographicProjection>" and "bevy_render::camera::active_cameras::active_cameras_system"
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::OrthographicProjection>" and "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::PerspectiveProjection>"
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::OrthographicProjection>" and "bevy_pbr::light::assign_lights_to_clusters"
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_render::view::visibility::update_frusta<bevy_render::camera::projection::OrthographicProjection>" and "bevy_pbr::light::assign_lights_to_clusters"
    conflicts: ["bevy_render::primitives::Frustum"]
 -- "bevy_render::view::visibility::update_frusta<bevy_render::camera::projection::OrthographicProjection>" and "bevy_pbr::light::update_directional_light_frusta"
    conflicts: ["bevy_render::primitives::Frustum"]
 -- "bevy_text::text2d::text2d_system" and "bevy_egui::update_egui_textures"
    conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::image::Image>"]
 -- "bevy_egui::systems::process_output" and "bevy_egui::update_egui_textures"
    conflicts: ["bevy_egui::EguiContext"]
 -- "bevy_pbr::light::update_clusters" and "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::PerspectiveProjection>"  
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_render::camera::active_cameras::active_cameras_system" and "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::PerspectiveProjection>"
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::PerspectiveProjection>" and "bevy_pbr::light::assign_lights_to_clusters"
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_render::view::visibility::check_visibility" and "bevy_pbr::light::update_directional_light_frusta"
    conflicts: ["bevy_render::primitives::Frustum"]
 -- "bevy_pbr::light::assign_lights_to_clusters" and "bevy_pbr::light::update_directional_light_frusta"
    conflicts: ["bevy_render::primitives::Frustum"]
 * Exclusive systems at start of stage:
 -- "bevy_winit::change_window" and "bevy_pbr::light::add_clusters"
 -- "bevy_winit::change_window" and "bevy_audio::audio_output::play_queued_audio_system<bevy_audio::audio_source::AudioSource>"
 -- "bevy_pbr::light::add_clusters" and "bevy_audio::audio_output::play_queued_audio_system<bevy_audio::audio_source::AudioSource>"

2022-02-05T11:47:54.756598Z  INFO bevy_ecs::schedule::stage: Execution order ambiguities detected, you might want to add an explicit dependency relation between some of these systems:
 * Parallel systems:
 -- "bevy_ui::render::extract_uinodes" and "bevy_egui::render_systems::extract_egui_textures"
    conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::image::Image>"]
 -- "bevy_ui::render::extract_uinodes" and "bevy_sprite::render::extract_sprites"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_uinodes" and "bevy_render::render_resource::pipeline_cache::RenderPipelineCache::extract_shaders"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_uinodes" and "bevy_core_pipeline::extract_clear_color"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_uinodes" and "bevy_render::view::window::extract_windows"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_uinodes" and "bevy_text::text2d::extract_text2d_sprite"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_uinodes" and "bevy_sprite::render::extract_sprite_events"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_egui::render_systems::extract_egui_textures" and "bevy_render::render_asset::extract_render_asset<bevy_render::texture::image::Image>" 
    conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::image::Image>"]
 -- "bevy_render::camera::extract_cameras" and "bevy_pbr::render::light::extract_lights"
    conflicts: ["bevy_render::view::visibility::VisibleEntities"]
 -- "bevy_sprite::render::extract_sprites" and "bevy_render::render_resource::pipeline_cache::RenderPipelineCache::extract_shaders"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_sprite::render::extract_sprites" and "bevy_ui::render::extract_text_uinodes"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_sprite::render::extract_sprites" and "bevy_core_pipeline::extract_clear_color"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_sprite::render::extract_sprites" and "bevy_render::view::window::extract_windows"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_sprite::render::extract_sprites" and "bevy_sprite::render::extract_sprite_events"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_render::render_resource::pipeline_cache::RenderPipelineCache::extract_shaders" and "bevy_ui::render::extract_text_uinodes"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_render::render_resource::pipeline_cache::RenderPipelineCache::extract_shaders" and "bevy_core_pipeline::extract_clear_color"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_render::render_resource::pipeline_cache::RenderPipelineCache::extract_shaders" and "bevy_render::view::window::extract_windows"        
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_render::render_resource::pipeline_cache::RenderPipelineCache::extract_shaders" and "bevy_text::text2d::extract_text2d_sprite"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_render::render_resource::pipeline_cache::RenderPipelineCache::extract_shaders" and "bevy_sprite::render::extract_sprite_events"        
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_text_uinodes" and "bevy_core_pipeline::extract_clear_color"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_text_uinodes" and "bevy_render::view::window::extract_windows"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_text_uinodes" and "bevy_text::text2d::extract_text2d_sprite"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_text_uinodes" and "bevy_sprite::render::extract_sprite_events"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_core_pipeline::extract_clear_color" and "bevy_render::view::window::extract_windows"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_core_pipeline::extract_clear_color" and "bevy_text::text2d::extract_text2d_sprite"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_core_pipeline::extract_clear_color" and "bevy_sprite::render::extract_sprite_events"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_render::view::window::extract_windows" and "bevy_text::text2d::extract_text2d_sprite"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_render::view::window::extract_windows" and "bevy_sprite::render::extract_sprite_events"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_text::text2d::extract_text2d_sprite" and "bevy_sprite::render::extract_sprite_events"
    conflicts: ["bevy_render::RenderWorld"]
