Jump height inconsistency among others is likely due to the ambiguous system execution order.

[[Player state refactor]] will likely change up the systems a lot, so do this after that

There is a bug with 1f moves occasionally not counting for mobility, this is probably fixed wit the ordering

Bevy log:
```
Jan 23 18:10:37.278 INFO bevy_ecs::schedule::stage: Execution order ambiguities detected, you might want to add an explicit dependency relation between some of these systems:

 * Parallel systems:

 -- "&bevy_egui::systems::process_input" and "&bevy_ui::focus::ui_focus_system"

 conflicts: ["bevy_window::windows::Windows"]

  

Jan 23 18:10:37.946 INFO bevy_ecs::schedule::stage: Execution order ambiguities detected, you might want to add an explicit dependency relation between some of these systems:

 * Parallel systems:

 -- "&whoops_all_grapplers_lib::damage::health::apply_hits" and "&whoops_all_grapplers_lib::spawner::handle_hitbox_events"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::damage::health::apply_hits" and "&whoops_all_grapplers_lib::physics::move_players"

 conflicts: ["player_state::PlayerState", "whoops_all_grapplers_lib::physics::PlayerVelocity"]

 -- "&whoops_all_grapplers_lib::damage::health::apply_hits" and "&whoops_all_grapplers_lib::character::movement::movement"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::damage::health::apply_hits" and "&whoops_all_grapplers_lib::damage::health::refill_meter"

 conflicts: ["player_state::PlayerState", "whoops_all_grapplers_lib::damage::health::Health"]

 -- "&whoops_all_grapplers_lib::damage::health::apply_hits" and "&whoops_all_grapplers_lib::physics::player_input"

 conflicts: ["player_state::PlayerState", "whoops_all_grapplers_lib::physics::PlayerVelocity"]

 -- "&whoops_all_grapplers_lib::damage::health::apply_hits" and "&whoops_all_grapplers_lib::damage::register_hits"

 conflicts: ["whoops_all_grapplers_lib::damage::health::Health"]

 -- "&whoops_all_grapplers_lib::damage::health::apply_hits" and "&whoops_all_grapplers_lib::game_flow::check_dead"

 conflicts: ["whoops_all_grapplers_lib::damage::health::Health"]

 -- "&whoops_all_grapplers_lib::damage::health::apply_hits" and "&whoops_all_grapplers_lib::character::reset"

 conflicts: ["player_state::PlayerState", "whoops_all_grapplers_lib::damage::health::Health"]

 -- "&whoops_all_grapplers_lib::damage::health::apply_hits" and "&whoops_all_grapplers_lib::character::move_activation::move_activator"

 conflicts: ["player_state::PlayerState", "input_parsing::input_parser::InputParser"]

 -- "&whoops_all_grapplers_lib::damage::health::apply_hits" and "&whoops_all_grapplers_lib::ui::bars::update"

 conflicts: ["whoops_all_grapplers_lib::damage::health::Health"]

 -- "&whoops_all_grapplers_lib::damage::health::apply_hits" and "&whoops_all_grapplers_lib::physics::sideswitcher"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::damage::health::apply_hits" and "&whoops_all_grapplers_lib::spawner::handle_requests"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::damage::health::apply_hits" and "&whoops_all_grapplers_lib::clock::reset_timer"

 conflicts: ["whoops_all_grapplers_lib::clock::Clock"]

 -- "&whoops_all_grapplers_lib::damage::health::apply_hits" and "&input_parsing::input_parser::parse_input"

 conflicts: ["player_state::PlayerState", "input_parsing::input_parser::InputParser"]

 -- "&whoops_all_grapplers_lib::damage::health::apply_hits" and "&whoops_all_grapplers_lib::physics::push_players"

 conflicts: ["player_state::PlayerState", "whoops_all_grapplers_lib::physics::PlayerVelocity"]

 -- "&whoops_all_grapplers_lib::spawner::handle_hitbox_events" and "&whoops_all_grapplers_lib::physics::move_players"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::spawner::handle_hitbox_events" and "&whoops_all_grapplers_lib::character::movement::movement"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::spawner::handle_hitbox_events" and "&whoops_all_grapplers_lib::damage::health::refill_meter"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::spawner::handle_hitbox_events" and "&whoops_all_grapplers_lib::physics::player_input"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::spawner::handle_hitbox_events" and "&whoops_all_grapplers_lib::character::reset"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::spawner::handle_hitbox_events" and "&whoops_all_grapplers_lib::character::move_activation::move_activator"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::spawner::handle_hitbox_events" and "&whoops_all_grapplers_lib::physics::sideswitcher"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::spawner::handle_hitbox_events" and "&whoops_all_grapplers_lib::spawner::handle_requests"

 conflicts: ["player_state::PlayerState", "whoops_all_grapplers_lib::spawner::Spawner"]

 -- "&whoops_all_grapplers_lib::spawner::handle_hitbox_events" and "&whoops_all_grapplers_lib::clock::reset_timer"

 conflicts: ["whoops_all_grapplers_lib::clock::Clock"]

 -- "&whoops_all_grapplers_lib::spawner::handle_hitbox_events" and "&input_parsing::input_parser::parse_input"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::spawner::handle_hitbox_events" and "&whoops_all_grapplers_lib::physics::push_players"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::physics::move_players" and "&whoops_all_grapplers_lib::character::movement::movement"

 conflicts: ["player_state::PlayerState", "bevy_transform::components::transform::Transform"]

 -- "&whoops_all_grapplers_lib::physics::move_players" and "&whoops_all_grapplers_lib::damage::health::refill_meter"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::physics::move_players" and "&whoops_all_grapplers_lib::physics::player_input"

 conflicts: ["player_state::PlayerState", "whoops_all_grapplers_lib::physics::PlayerVelocity"]

 -- "&whoops_all_grapplers_lib::physics::move_players" and "&whoops_all_grapplers_lib::character::reset"

 conflicts: ["player_state::PlayerState", "bevy_transform::components::transform::Transform"]

 -- "&whoops_all_grapplers_lib::physics::move_players" and "&whoops_all_grapplers_lib::character::move_activation::move_activator"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::physics::move_players" and "&whoops_all_grapplers_lib::physics::sideswitcher"

 conflicts: ["player_state::PlayerState", "bevy_transform::components::transform::Transform"]

 -- "&whoops_all_grapplers_lib::physics::move_players" and "&whoops_all_grapplers_lib::spawner::handle_requests"

 conflicts: ["player_state::PlayerState", "bevy_transform::components::transform::Transform"]

 -- "&whoops_all_grapplers_lib::physics::move_players" and "&input_parsing::input_parser::parse_input"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::physics::move_players" and "&whoops_all_grapplers_lib::physics::push_players"

 conflicts: ["player_state::PlayerState", "bevy_transform::components::transform::Transform", "whoops_all_grapplers_lib::physics::PlayerVelocity"]

 -- "&whoops_all_grapplers_lib::physics::move_players" and "&whoops_all_grapplers_lib::physics::move_constants"

 conflicts: ["bevy_transform::components::transform::Transform"]

 -- "&whoops_all_grapplers_lib::character::movement::movement" and "&whoops_all_grapplers_lib::damage::health::refill_meter"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::character::movement::movement" and "&whoops_all_grapplers_lib::physics::player_input"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::character::movement::movement" and "&whoops_all_grapplers_lib::damage::register_hits"

 conflicts: ["bevy_sprite::sprite::Sprite"]

 -- "&whoops_all_grapplers_lib::character::movement::movement" and "&whoops_all_grapplers_lib::character::reset"

 conflicts: ["player_state::PlayerState", "bevy_transform::components::transform::Transform"]

 -- "&whoops_all_grapplers_lib::character::movement::movement" and "&whoops_all_grapplers_lib::character::move_activation::move_activator"

 conflicts: ["player_state::PlayerState", "input_parsing::input_parser::InputParser"]

 -- "&whoops_all_grapplers_lib::character::movement::movement" and "&whoops_all_grapplers_lib::physics::sideswitcher"

 conflicts: ["player_state::PlayerState", "bevy_transform::components::transform::Transform"]

 -- "&whoops_all_grapplers_lib::character::movement::movement" and "&whoops_all_grapplers_lib::spawner::handle_requests"

 conflicts: ["player_state::PlayerState", "bevy_transform::components::transform::Transform"]

 -- "&whoops_all_grapplers_lib::character::movement::movement" and "&input_parsing::input_parser::parse_input"

 conflicts: ["player_state::PlayerState", "input_parsing::input_parser::InputParser"]

 -- "&whoops_all_grapplers_lib::character::movement::movement" and "&whoops_all_grapplers_lib::physics::push_players"

 conflicts: ["player_state::PlayerState", "bevy_transform::components::transform::Transform"]

 -- "&whoops_all_grapplers_lib::character::movement::movement" and "&whoops_all_grapplers_lib::physics::move_constants"

 conflicts: ["bevy_transform::components::transform::Transform"]

 -- "&whoops_all_grapplers_lib::damage::health::refill_meter" and "&whoops_all_grapplers_lib::physics::player_input"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::damage::health::refill_meter" and "&whoops_all_grapplers_lib::damage::register_hits"

 conflicts: ["whoops_all_grapplers_lib::damage::health::Health"]

 -- "&whoops_all_grapplers_lib::damage::health::refill_meter" and "&whoops_all_grapplers_lib::game_flow::check_dead"

 conflicts: ["whoops_all_grapplers_lib::damage::health::Health"]

 -- "&whoops_all_grapplers_lib::damage::health::refill_meter" and "&whoops_all_grapplers_lib::character::reset"

 conflicts: ["player_state::PlayerState", "whoops_all_grapplers_lib::damage::health::Health", "whoops_all_grapplers_lib::meter::Meter"]

 -- "&whoops_all_grapplers_lib::damage::health::refill_meter" and "&whoops_all_grapplers_lib::character::move_activation::move_activator"

 conflicts: ["player_state::PlayerState", "whoops_all_grapplers_lib::meter::Meter"]

 -- "&whoops_all_grapplers_lib::damage::health::refill_meter" and "&whoops_all_grapplers_lib::ui::bars::update"

 conflicts: ["whoops_all_grapplers_lib::damage::health::Health", "whoops_all_grapplers_lib::meter::Meter"]

 -- "&whoops_all_grapplers_lib::damage::health::refill_meter" and "&whoops_all_grapplers_lib::physics::sideswitcher"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::damage::health::refill_meter" and "&whoops_all_grapplers_lib::spawner::handle_requests"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::damage::health::refill_meter" and "&input_parsing::input_parser::parse_input"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::damage::health::refill_meter" and "&whoops_all_grapplers_lib::physics::push_players"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::physics::player_input" and "&whoops_all_grapplers_lib::character::reset"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::physics::player_input" and "&whoops_all_grapplers_lib::character::move_activation::move_activator"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::physics::player_input" and "&whoops_all_grapplers_lib::physics::sideswitcher"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::physics::player_input" and "&whoops_all_grapplers_lib::spawner::handle_requests"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::physics::player_input" and "&input_parsing::input_parser::parse_input"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::physics::player_input" and "&whoops_all_grapplers_lib::physics::push_players"

 conflicts: ["player_state::PlayerState", "whoops_all_grapplers_lib::physics::PlayerVelocity"]

 -- "&whoops_all_grapplers_lib::damage::register_hits" and "&whoops_all_grapplers_lib::game_flow::check_dead"

 conflicts: ["whoops_all_grapplers_lib::damage::health::Health"]

 -- "&whoops_all_grapplers_lib::damage::register_hits" and "&whoops_all_grapplers_lib::character::reset"

 conflicts: ["whoops_all_grapplers_lib::damage::health::Health"]

 -- "&whoops_all_grapplers_lib::damage::register_hits" and "&whoops_all_grapplers_lib::ui::bars::update"

 conflicts: ["whoops_all_grapplers_lib::damage::health::Health"]

 -- "&whoops_all_grapplers_lib::game_flow::check_dead" and "&whoops_all_grapplers_lib::game_flow::tick_countdown"

 conflicts: ["bevy_ecs::schedule::state::State<whoops_all_grapplers_lib::game_flow::GameState>"]

 -- "&whoops_all_grapplers_lib::game_flow::check_dead" and "&whoops_all_grapplers_lib::character::reset"

 conflicts: ["whoops_all_grapplers_lib::damage::health::Health"]

 -- "&whoops_all_grapplers_lib::game_flow::check_dead" and "&whoops_all_grapplers_lib::game_flow::tick_countdown"

 conflicts: ["bevy_ecs::schedule::state::State<whoops_all_grapplers_lib::game_flow::GameState>"]

 -- "&whoops_all_grapplers_lib::game_flow::check_dead" and "&whoops_all_grapplers_lib::clock::reset_timer"

 conflicts: ["whoops_all_grapplers_lib::clock::Clock"]

 -- "&whoops_all_grapplers_lib::game_flow::tick_countdown" and "&whoops_all_grapplers_lib::game_flow::restart_countdown"

 conflicts: ["whoops_all_grapplers_lib::game_flow::InterFrameCountdown"]

 -- "&whoops_all_grapplers_lib::game_flow::tick_countdown" and "&whoops_all_grapplers_lib::game_flow::tick_countdown"

 conflicts: ["bevy_ecs::schedule::state::State<whoops_all_grapplers_lib::game_flow::GameState>", "whoops_all_grapplers_lib::game_flow::InterFrameCountdown"] 

 -- "&whoops_all_grapplers_lib::character::reset" and "&whoops_all_grapplers_lib::character::move_activation::move_activator"

 conflicts: ["player_state::PlayerState", "whoops_all_grapplers_lib::meter::Meter"]

 -- "&whoops_all_grapplers_lib::character::reset" and "&whoops_all_grapplers_lib::ui::bars::update"

 conflicts: ["whoops_all_grapplers_lib::damage::health::Health", "whoops_all_grapplers_lib::meter::Meter"]

 -- "&whoops_all_grapplers_lib::character::reset" and "&whoops_all_grapplers_lib::physics::sideswitcher"

 conflicts: ["player_state::PlayerState", "bevy_transform::components::transform::Transform"]

 -- "&whoops_all_grapplers_lib::character::reset" and "&whoops_all_grapplers_lib::spawner::handle_requests"

 conflicts: ["bevy_transform::components::transform::Transform"]

 -- "&whoops_all_grapplers_lib::character::reset" and "&whoops_all_grapplers_lib::physics::push_players"

 conflicts: ["bevy_transform::components::transform::Transform"]

 -- "&whoops_all_grapplers_lib::character::reset" and "&whoops_all_grapplers_lib::physics::move_constants"

 conflicts: ["bevy_transform::components::transform::Transform"]

 -- "&whoops_all_grapplers_lib::character::move_activation::move_activator" and "&whoops_all_grapplers_lib::ui::bars::update"

 conflicts: ["whoops_all_grapplers_lib::meter::Meter"]

 -- "&whoops_all_grapplers_lib::character::move_activation::move_activator" and "&whoops_all_grapplers_lib::physics::sideswitcher"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::character::move_activation::move_activator" and "&whoops_all_grapplers_lib::spawner::handle_requests"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::character::move_activation::move_activator" and "&input_parsing::input_parser::parse_input"

 conflicts: ["player_state::PlayerState", "input_parsing::input_parser::InputParser"]

 -- "&whoops_all_grapplers_lib::character::move_activation::move_activator" and "&whoops_all_grapplers_lib::physics::push_players"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::ui::round_text::round_start" and "&whoops_all_grapplers_lib::clock::update_timer"

 conflicts: ["bevy_text::text::Text"]

 -- "&whoops_all_grapplers_lib::ui::round_text::round_start" and "&whoops_all_grapplers_lib::ui::round_text::round_over"

 conflicts: ["bevy_text::text::Text"]

 -- "&whoops_all_grapplers_lib::game_flow::restart_countdown" and "&whoops_all_grapplers_lib::game_flow::tick_countdown"

 conflicts: ["whoops_all_grapplers_lib::game_flow::InterFrameCountdown"]

 -- "&whoops_all_grapplers_lib::clock::update_timer" and "&whoops_all_grapplers_lib::clock::reset_timer"

 conflicts: ["whoops_all_grapplers_lib::clock::Clock"]

 -- "&whoops_all_grapplers_lib::clock::update_timer" and "&whoops_all_grapplers_lib::ui::round_text::round_over"

 conflicts: ["bevy_text::text::Text"]

 -- "&whoops_all_grapplers_lib::physics::sideswitcher" and "&whoops_all_grapplers_lib::spawner::handle_requests"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::physics::sideswitcher" and "&input_parsing::input_parser::parse_input"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::physics::sideswitcher" and "&whoops_all_grapplers_lib::physics::push_players"

 conflicts: ["player_state::PlayerState"]

 -- "&whoops_all_grapplers_lib::physics::sideswitcher" and "&whoops_all_grapplers_lib::physics::move_constants"

 conflicts: ["bevy_transform::components::transform::Transform"]

 -- "&whoops_all_grapplers_lib::spawner::handle_requests" and "&whoops_all_grapplers_lib::clock::reset_timer"

 conflicts: ["whoops_all_grapplers_lib::clock::Clock"]

 -- "&whoops_all_grapplers_lib::spawner::handle_requests" and "&whoops_all_grapplers_lib::physics::move_constants"

 conflicts: ["bevy_transform::components::transform::Transform"]

 -- "&whoops_all_grapplers_lib::physics::push_players" and "&whoops_all_grapplers_lib::physics::move_constants"

 conflicts: ["bevy_transform::components::transform::Transform"]

  

New gamepad connected with ID: Gamepad(0)

Jan 23 18:10:38.002 INFO bevy_ecs::schedule::stage: Execution order ambiguities detected, you might want to add an explicit dependency relation between some of these systems:

 * Parallel systems:

 -- "&bevy_ui::widget::text::text_system" and "&bevy_ui::widget::image::image_node_system"

 conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::texture::Texture>", "bevy_ui::ui_node::CalculatedSize"]

 -- "&bevy_ui::widget::text::text_system" and "&bevy_sprite::sprite::sprite_system"

 conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::texture::Texture>"]

 -- "&bevy_ui::widget::text::text_system" and "&bevy_text::text2d::text2d_system"

 conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::texture::Texture>", "bevy_asset::assets::Assets<bevy_sprite::texture_atlas::TextureAtlas>", "bevy_asset::assets::Assets<bevy_text::font_atlas_set::FontAtlasSet>", "bevy_text::pipeline::TextPipeline<bevy_ecs::entity::Entity>"]

 -- "&bevy_ui::widget::image::image_node_system" and "&bevy_text::text2d::text2d_system"

 conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::texture::Texture>"]

 -- "&bevy_ui::flex::flex_node_system" and "&whoops_all_grapplers_lib::camera::center_camera"

 conflicts: ["bevy_transform::components::transform::Transform"]

 -- "&bevy_ui::flex::flex_node_system" and "&bevy_transform::hierarchy::hierarchy_maintenance_system::parent_update_system"

 conflicts: ["bevy_transform::components::children::Children"]

 -- "&bevy_ui::update::ui_z_system" and "&whoops_all_grapplers_lib::camera::center_camera"

 conflicts: ["bevy_transform::components::transform::Transform"]

 -- "&bevy_ui::update::ui_z_system" and "&bevy_transform::hierarchy::hierarchy_maintenance_system::parent_update_system"

 conflicts: ["bevy_transform::components::children::Children"]

 -- "&bevy_render::camera::camera::camera_system<bevy_render::camera::projection::OrthographicProjection>" and "&bevy_render::camera::camera::camera_system<bevy_render::camera::projection::PerspectiveProjection>"

 conflicts: ["bevy_render::camera::camera::Camera"]

 -- "&bevy_render::camera::camera::camera_system<bevy_render::camera::projection::OrthographicProjection>" and "&bevy_render::camera::active_cameras::active_cameras_system"

 conflicts: ["bevy_render::camera::camera::Camera"]

 -- "&bevy_render::camera::camera::camera_system<bevy_render::camera::projection::OrthographicProjection>" and "&bevy_render::camera::camera::camera_system<whoops_all_grapplers_lib::camera::SimpleOrthoProjection>"

 conflicts: ["bevy_render::camera::camera::Camera"]

 -- "&bevy_render::shader::shader_defs::asset_shader_defs_system<bevy_pbr::material::StandardMaterial>" and "&bevy_render::shader::shader_defs::asset_shader_defs_system<bevy_sprite::color_material::ColorMaterial>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_sprite::sprite::sprite_system" and "&bevy_text::text2d::text2d_system"

 conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::texture::Texture>"]

 -- "&whoops_all_grapplers_lib::camera::center_camera" and "&bevy_transform::transform_propagate_system::transform_propagate_system"

 conflicts: ["bevy_transform::components::transform::Transform"]

 -- "&bevy_render::camera::camera::camera_system<bevy_render::camera::projection::PerspectiveProjection>" and "&bevy_render::camera::active_cameras::active_cameras_system"

 conflicts: ["bevy_render::camera::camera::Camera"]

 -- "&bevy_render::camera::camera::camera_system<bevy_render::camera::projection::PerspectiveProjection>" and "&bevy_render::camera::camera::camera_system<whoops_all_grapplers_lib::camera::SimpleOrthoProjection>"

 conflicts: ["bevy_render::camera::camera::Camera"]

 -- "&bevy_render::camera::active_cameras::active_cameras_system" and "&bevy_render::camera::camera::camera_system<whoops_all_grapplers_lib::camera::SimpleOrthoProjection>"

 conflicts: ["bevy_render::camera::camera::Camera"]

 -- "&bevy_render::camera::visible_entities::visible_entities_system" and "&bevy_render::camera::camera::camera_system<whoops_all_grapplers_lib::camera::SimpleOrthoProjection>"

 conflicts: ["bevy_render::camera::camera::Camera"]

 * Exclusive systems at start of stage:

 -- "bevy_audio::audio_output::play_queued_audio_system<bevy_audio::audio_source::AudioSource>" and "bevy_winit::change_window"

  

Jan 23 18:10:38.159 INFO bevy_ecs::schedule::stage: Execution order ambiguities detected, you might want to add an explicit dependency relation between some of these systems:

 * Parallel systems:

 -- "&bevy_egui::transform_node::transform_node_system" and "&bevy_pbr::render_graph::lights_node::lights_node_system"

 conflicts: ["bevy_render::renderer::render_resource::render_resource_bindings::RenderResourceBindings"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_pbr::material::StandardMaterial>" and "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_sprite::sprite::Sprite>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_pbr::material::StandardMaterial>" and "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_sprite::color_material::ColorMaterial>"

 conflicts: ["bevy_render::renderer::render_resource::render_resource_bindings::AssetRenderResourceBindings", "bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_pbr::material::StandardMaterial>" and "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_transform::components::global_transform::GlobalTransform>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_pbr::material::StandardMaterial>" and "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_sprite::texture_atlas::TextureAtlasSprite>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_pbr::material::StandardMaterial>" and "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_ui::ui_node::Node>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_pbr::material::StandardMaterial>" and "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_sprite::texture_atlas::TextureAtlas>"

 conflicts: ["bevy_render::renderer::render_resource::render_resource_bindings::AssetRenderResourceBindings", "bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_sprite::sprite::Sprite>" and "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_sprite::color_material::ColorMaterial>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_sprite::sprite::Sprite>" and "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_transform::components::global_transform::GlobalTransform>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_sprite::sprite::Sprite>" and "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_sprite::texture_atlas::TextureAtlasSprite>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_sprite::sprite::Sprite>" and "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_ui::ui_node::Node>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_sprite::sprite::Sprite>" and "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_sprite::texture_atlas::TextureAtlas>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_sprite::color_material::ColorMaterial>" and "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_transform::components::global_transform::GlobalTransform>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_sprite::color_material::ColorMaterial>" and "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_sprite::texture_atlas::TextureAtlasSprite>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_sprite::color_material::ColorMaterial>" and "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_ui::ui_node::Node>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_sprite::color_material::ColorMaterial>" and "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_sprite::texture_atlas::TextureAtlas>"

 conflicts: ["bevy_render::renderer::render_resource::render_resource_bindings::AssetRenderResourceBindings", "bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_transform::components::global_transform::GlobalTransform>" and "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_sprite::texture_atlas::TextureAtlasSprite>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_transform::components::global_transform::GlobalTransform>" and "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_ui::ui_node::Node>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_transform::components::global_transform::GlobalTransform>" and "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_sprite::texture_atlas::TextureAtlas>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_sprite::texture_atlas::TextureAtlasSprite>" and "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_ui::ui_node::Node>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_sprite::texture_atlas::TextureAtlasSprite>" and "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_sprite::texture_atlas::TextureAtlas>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::render_resources_node::render_resources_node_system<bevy_ui::ui_node::Node>" and "&bevy_render::render_graph::nodes::render_resources_node::asset_render_resources_node_system<bevy_sprite::texture_atlas::TextureAtlas>"

 conflicts: ["bevy_render::pipeline::render_pipelines::RenderPipelines"]

 -- "&bevy_render::render_graph::nodes::camera_node::camera_node_system" and "&bevy_render::render_graph::nodes::camera_node::camera_node_system"

 conflicts: ["bevy_render::camera::active_cameras::ActiveCameras"]

 -- "&bevy_render::render_graph::nodes::camera_node::camera_node_system" and "&bevy_render::render_graph::nodes::camera_node::camera_node_system"

 conflicts: ["bevy_render::camera::active_cameras::ActiveCameras"]

 -- "&bevy_render::render_graph::nodes::camera_node::camera_node_system" and "&bevy_render::render_graph::nodes::camera_node::camera_node_system"

 conflicts: ["bevy_render::camera::active_cameras::ActiveCameras"]

  

Jan 23 18:10:38.271 INFO bevy_ecs::schedule::stage: Execution order ambiguities detected, you might want to add an explicit dependency relation between some of these systems:

 * Parallel systems:

 -- "&bevy_ui::widget::text::draw_text_system" and "&bevy_text::text2d::draw_text2d_system"

 conflicts: ["bevy_asset::assets::Assets<bevy_render::shader::shader::Shader>", "bevy_asset::assets::Assets<bevy_render::pipeline::pipeline::PipelineDescriptor>",

"bevy_render::pipeline::pipeline_compiler::PipelineCompiler", "bevy_render::renderer::render_resource::render_resource_bindings::RenderResourceBindings", "bevy_render::renderer::render_resource::render_resource_bindings::AssetRenderResourceBindings", "bevy_render::renderer::render_resource::shared_buffers::SharedBuffers", "bevy_render::draw::Draw"]

 -- "&bevy_ui::widget::text::draw_text_system" and "&bevy_render::pipeline::render_pipelines::draw_render_pipelines_system"

 conflicts: ["bevy_asset::assets::Assets<bevy_render::shader::shader::Shader>", "bevy_asset::assets::Assets<bevy_render::pipeline::pipeline::PipelineDescriptor>",

"bevy_render::pipeline::pipeline_compiler::PipelineCompiler", "bevy_render::renderer::render_resource::render_resource_bindings::RenderResourceBindings", "bevy_render::renderer::render_resource::render_resource_bindings::AssetRenderResourceBindings", "bevy_render::renderer::render_resource::shared_buffers::SharedBuffers", "bevy_render::draw::Draw"]

 -- "&bevy_text::text2d::draw_text2d_system" and "&bevy_render::pipeline::render_pipelines::draw_render_pipelines_system"

 conflicts: ["bevy_asset::assets::Assets<bevy_render::shader::shader::Shader>", "bevy_asset::assets::Assets<bevy_render::pipeline::pipeline::PipelineDescriptor>",

"bevy_render::pipeline::pipeline_compiler::PipelineCompiler", "bevy_render::renderer::render_resource::render_resource_bindings::RenderResourceBindings", "bevy_render::renderer::render_resource::render_resource_bindings::AssetRenderResourceBindings", "bevy_render::renderer::render_resource::shared_buffers::SharedBuffers", "bevy_render::draw::Draw"]```