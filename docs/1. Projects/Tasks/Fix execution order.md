[[Player state refactor]] will likely change up the systems a lot, so do this after that

Can you make a system set automatically execute in a deterministic order?

- Check that crouching doesn't spazz
- Jump height inconsistency among others is likely due to the ambiguous system execution order.
- There is a bug with 1f moves occasionally not counting for mobility, this is probably fixed wit the ordering

Bevy log:
```
2022-02-05T04:38:28.078466Z  INFO bevy_ecs::schedule::stage: Execution order ambiguities detected, you might want to add an explicit dependency relation between some of these systems:
 * Parallel systems:
 -- "bevy_input::gamepad::gamepad_event_system" and "bevy_input::gamepad::gamepad_connection_system"
    conflicts: ["bevy_ecs::event::Events<bevy_input::gamepad::GamepadEvent>"]
 -- "bevy_egui::systems::process_input" and "bevy_ui::focus::ui_focus_system"
    conflicts: ["bevy_window::windows::Windows"]

2022-02-05T04:38:28.079630Z  INFO bevy_input::gamepad: Gamepad(0) Connected
2022-02-05T04:38:28.098081Z  INFO bevy_ecs::schedule::stage: Execution order ambiguities detected, you might want to add an explicit dependency relation between some of these systems:
 * Parallel systems:
 -- "whoops_all_grapplers_lib::character::movement::movement" and "input_parsing::input_parser::parse_input"
    conflicts: ["input_parsing::input_parser::InputParser"]
 -- "whoops_all_grapplers_lib::character::movement::movement" and "whoops_all_grapplers_lib::damage::register_hits"
    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::movement::movement" and "whoops_all_grapplers_lib::physics::player_gravity"
    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::movement::movement" and "whoops_all_grapplers_lib::character::reset"
    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::movement::movement" and "whoops_all_grapplers_lib::character::move_activation::move_activator"
    conflicts: ["input_parsing::input_parser::InputParser", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::movement::movement" and "whoops_all_grapplers_lib::character::size_adjustment::size_adjustment"
    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::movement::movement" and "whoops_all_grapplers_lib::physics::player_input"
    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::movement::movement" and "whoops_all_grapplers_lib::character::move_advancement::move_advancement"
    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::movement::movement" and "whoops_all_grapplers_lib::physics::move_players"
    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::movement::movement" and "whoops_all_grapplers_lib::character::recovery::stun_recovery"
    conflicts: ["player_state::player_state::PlayerState"]
 -- "time::game_flow::restart_countdown" and "time::game_flow::tick_countdown"
    conflicts: ["time::game_flow::InterFrameCountdown"]
 -- "time::game_flow::restart_countdown" and "time::game_flow::tick_countdown"
    conflicts: ["time::game_flow::InterFrameCountdown"]
 -- "whoops_all_grapplers_lib::spawner::despawn_everything" and "whoops_all_grapplers_lib::damage::register_hits"
    conflicts: ["whoops_all_grapplers_lib::spawner::Spawner"]
 -- "whoops_all_grapplers_lib::spawner::despawn_everything" and "whoops_all_grapplers_lib::physics::player_gravity"
    conflicts: ["whoops_all_grapplers_lib::spawner::Spawner"]
 -- "whoops_all_grapplers_lib::spawner::despawn_everything" and "whoops_all_grapplers_lib::spawner::despawn_expired"
    conflicts: ["whoops_all_grapplers_lib::spawner::Spawner"]
 -- "whoops_all_grapplers_lib::spawner::despawn_everything" and "whoops_all_grapplers_lib::character::move_advancement::move_advancement"
    conflicts: ["whoops_all_grapplers_lib::spawner::Spawner"]
 -- "time::reset_timer" and "time::update_timer"
    conflicts: ["time::Clock"]
 -- "time::reset_timer" and "whoops_all_grapplers_lib::damage::register_hits"
    conflicts: ["time::Clock"]
 -- "time::reset_timer" and "whoops_all_grapplers_lib::damage::health::check_dead"
    conflicts: ["time::Clock"]
 -- "time::reset_timer" and "whoops_all_grapplers_lib::spawner::despawn_expired"
    conflicts: ["time::Clock"]
 -- "time::reset_timer" and "whoops_all_grapplers_lib::character::move_activation::move_activator"
    conflicts: ["time::Clock"]
 -- "time::reset_timer" and "whoops_all_grapplers_lib::character::move_advancement::move_advancement"
    conflicts: ["time::Clock"]
 -- "time::reset_timer" and "whoops_all_grapplers_lib::character::recovery::stun_recovery"
    conflicts: ["time::Clock"]
 -- "time::update_timer" and "whoops_all_grapplers_lib::ui::round_text::round_start"
    conflicts: ["bevy_text::text::Text"]
 -- "time::update_timer" and "whoops_all_grapplers_lib::ui::round_text::round_over"
    conflicts: ["bevy_text::text::Text"]
 -- "whoops_all_grapplers_lib::ui::round_text::round_start" and "whoops_all_grapplers_lib::ui::round_text::round_over"
    conflicts: ["bevy_text::text::Text"]
 -- "input_parsing::input_parser::parse_input" and "whoops_all_grapplers_lib::damage::register_hits"
    conflicts: ["input_parsing::input_parser::InputParser"]
 -- "input_parsing::input_parser::parse_input" and "whoops_all_grapplers_lib::physics::sideswitcher"
    conflicts: ["types::direction::LRDirection"]
 -- "input_parsing::input_parser::parse_input" and "whoops_all_grapplers_lib::character::move_activation::move_activator"
    conflicts: ["input_parsing::input_parser::InputParser"]
 -- "input_parsing::input_parser::parse_input" and "whoops_all_grapplers_lib::character::move_advancement::move_advancement"
    conflicts: ["input_parsing::input_parser::InputParser"]
 -- "whoops_all_grapplers_lib::damage::register_hits" and "whoops_all_grapplers_lib::damage::health::check_dead"
    conflicts: ["whoops_all_grapplers_lib::damage::health::Health"]
 -- "whoops_all_grapplers_lib::damage::register_hits" and "whoops_all_grapplers_lib::physics::player_gravity"
    conflicts: ["whoops_all_grapplers_lib::spawner::Spawner", "whoops_all_grapplers_lib::physics::PlayerVelocity", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::damage::register_hits" and "whoops_all_grapplers_lib::character::reset"
    conflicts: ["bevy_transform::components::transform::Transform", "whoops_all_grapplers_lib::damage::health::Health", "whoops_all_grapplers_lib::meter::Meter", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::damage::register_hits" and "whoops_all_grapplers_lib::spawner::despawn_expired"
    conflicts: ["whoops_all_grapplers_lib::spawner::Spawner"]
 -- "whoops_all_grapplers_lib::damage::register_hits" and "whoops_all_grapplers_lib::ui::bars::update"
    conflicts: ["whoops_all_grapplers_lib::damage::health::Health", "whoops_all_grapplers_lib::meter::Meter"]
 -- "whoops_all_grapplers_lib::damage::register_hits" and "whoops_all_grapplers_lib::physics::sideswitcher"
    conflicts: ["types::direction::LRDirection"]
 -- "whoops_all_grapplers_lib::damage::register_hits" and "whoops_all_grapplers_lib::character::move_activation::move_activator"
    conflicts: ["whoops_all_grapplers_lib::meter::Meter", "input_parsing::input_parser::InputParser", "player_state::player_state::PlayerState"]      
 -- "whoops_all_grapplers_lib::damage::register_hits" and "whoops_all_grapplers_lib::character::size_adjustment::size_adjustment"
    conflicts: ["bevy_sprite::sprite::Sprite", "bevy_transform::components::transform::Transform", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::damage::register_hits" and "whoops_all_grapplers_lib::physics::player_input"
    conflicts: ["whoops_all_grapplers_lib::physics::PlayerVelocity", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::damage::register_hits" and "whoops_all_grapplers_lib::character::move_advancement::move_advancement"
    conflicts: ["whoops_all_grapplers_lib::damage::health::Health", "whoops_all_grapplers_lib::spawner::Spawner", "whoops_all_grapplers_lib::physics::PlayerVelocity", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::damage::register_hits" and "whoops_all_grapplers_lib::physics::move_players"
    conflicts: ["bevy_transform::components::transform::Transform", "whoops_all_grapplers_lib::physics::PlayerVelocity", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::damage::register_hits" and "whoops_all_grapplers_lib::character::recovery::stun_recovery"
    conflicts: ["whoops_all_grapplers_lib::meter::Meter", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::damage::register_hits" and "whoops_all_grapplers_lib::physics::move_constants"
    conflicts: ["bevy_transform::components::transform::Transform"]
 -- "whoops_all_grapplers_lib::damage::health::check_dead" and "whoops_all_grapplers_lib::character::reset"
    conflicts: ["whoops_all_grapplers_lib::damage::health::Health"]
 -- "whoops_all_grapplers_lib::damage::health::check_dead" and "whoops_all_grapplers_lib::character::move_advancement::move_advancement"
    conflicts: ["whoops_all_grapplers_lib::damage::health::Health"]
 -- "whoops_all_grapplers_lib::damage::health::check_dead" and "time::game_flow::tick_countdown"
    conflicts: ["bevy_ecs::schedule::state::State<time::game_flow::GameState>"]
 -- "whoops_all_grapplers_lib::damage::health::check_dead" and "time::game_flow::tick_countdown"
    conflicts: ["bevy_ecs::schedule::state::State<time::game_flow::GameState>"]
 -- "whoops_all_grapplers_lib::physics::player_gravity" and "whoops_all_grapplers_lib::character::reset"
    conflicts: ["bevy_transform::components::transform::Transform", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::physics::player_gravity" and "whoops_all_grapplers_lib::spawner::despawn_expired"
    conflicts: ["whoops_all_grapplers_lib::spawner::Spawner"]
 -- "whoops_all_grapplers_lib::physics::player_gravity" and "whoops_all_grapplers_lib::character::move_activation::move_activator"
    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::physics::player_gravity" and "whoops_all_grapplers_lib::character::size_adjustment::size_adjustment"
    conflicts: ["bevy_transform::components::transform::Transform", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::physics::player_gravity" and "whoops_all_grapplers_lib::physics::player_input"
    conflicts: ["whoops_all_grapplers_lib::physics::PlayerVelocity", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::physics::player_gravity" and "whoops_all_grapplers_lib::character::move_advancement::move_advancement"
    conflicts: ["whoops_all_grapplers_lib::spawner::Spawner", "whoops_all_grapplers_lib::physics::PlayerVelocity", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::physics::player_gravity" and "whoops_all_grapplers_lib::physics::move_players"
    conflicts: ["bevy_transform::components::transform::Transform", "whoops_all_grapplers_lib::physics::PlayerVelocity", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::physics::player_gravity" and "whoops_all_grapplers_lib::character::recovery::stun_recovery"
    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::physics::player_gravity" and "whoops_all_grapplers_lib::physics::move_constants"
    conflicts: ["bevy_transform::components::transform::Transform"]
 -- "whoops_all_grapplers_lib::character::reset" and "whoops_all_grapplers_lib::ui::bars::update"
    conflicts: ["whoops_all_grapplers_lib::damage::health::Health", "whoops_all_grapplers_lib::meter::Meter"]
 -- "whoops_all_grapplers_lib::character::reset" and "whoops_all_grapplers_lib::physics::sideswitcher"
    conflicts: ["bevy_transform::components::transform::Transform"]
 -- "whoops_all_grapplers_lib::character::reset" and "whoops_all_grapplers_lib::character::move_activation::move_activator"
    conflicts: ["whoops_all_grapplers_lib::meter::Meter", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::reset" and "whoops_all_grapplers_lib::character::size_adjustment::size_adjustment"
    conflicts: ["bevy_transform::components::transform::Transform", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::reset" and "whoops_all_grapplers_lib::character::move_advancement::move_advancement"
    conflicts: ["bevy_transform::components::transform::Transform", "whoops_all_grapplers_lib::damage::health::Health", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::reset" and "whoops_all_grapplers_lib::physics::move_players"
    conflicts: ["bevy_transform::components::transform::Transform"]
 -- "whoops_all_grapplers_lib::character::reset" and "whoops_all_grapplers_lib::character::recovery::stun_recovery"
    conflicts: ["whoops_all_grapplers_lib::meter::Meter", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::reset" and "whoops_all_grapplers_lib::physics::move_constants"
    conflicts: ["bevy_transform::components::transform::Transform"]
 -- "whoops_all_grapplers_lib::spawner::despawn_expired" and "whoops_all_grapplers_lib::character::move_advancement::move_advancement"
    conflicts: ["whoops_all_grapplers_lib::spawner::Spawner"]
 -- "whoops_all_grapplers_lib::ui::bars::update" and "whoops_all_grapplers_lib::character::move_activation::move_activator"
    conflicts: ["whoops_all_grapplers_lib::meter::Meter"]
 -- "whoops_all_grapplers_lib::ui::bars::update" and "whoops_all_grapplers_lib::character::move_advancement::move_advancement"
    conflicts: ["whoops_all_grapplers_lib::damage::health::Health"]
 -- "whoops_all_grapplers_lib::ui::bars::update" and "whoops_all_grapplers_lib::character::recovery::stun_recovery"
    conflicts: ["whoops_all_grapplers_lib::meter::Meter"]
 -- "whoops_all_grapplers_lib::physics::sideswitcher" and "whoops_all_grapplers_lib::character::size_adjustment::size_adjustment"
    conflicts: ["bevy_transform::components::transform::Transform"]
 -- "whoops_all_grapplers_lib::physics::sideswitcher" and "whoops_all_grapplers_lib::physics::player_input"
    conflicts: ["types::direction::LRDirection"]
 -- "whoops_all_grapplers_lib::physics::sideswitcher" and "whoops_all_grapplers_lib::character::move_advancement::move_advancement"
    conflicts: ["types::direction::LRDirection"]
 -- "whoops_all_grapplers_lib::physics::sideswitcher" and "whoops_all_grapplers_lib::physics::move_players"
    conflicts: ["bevy_transform::components::transform::Transform"]
 -- "whoops_all_grapplers_lib::physics::sideswitcher" and "whoops_all_grapplers_lib::physics::move_constants"
    conflicts: ["bevy_transform::components::transform::Transform"]
 -- "whoops_all_grapplers_lib::character::move_activation::move_activator" and "whoops_all_grapplers_lib::character::size_adjustment::size_adjustment"    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::move_activation::move_activator" and "whoops_all_grapplers_lib::physics::player_input"
    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::move_activation::move_activator" and "whoops_all_grapplers_lib::character::move_advancement::move_advancement"
    conflicts: ["input_parsing::input_parser::InputParser", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::move_activation::move_activator" and "whoops_all_grapplers_lib::physics::move_players"
    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::move_activation::move_activator" and "whoops_all_grapplers_lib::character::recovery::stun_recovery"
    conflicts: ["whoops_all_grapplers_lib::meter::Meter", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::size_adjustment::size_adjustment" and "whoops_all_grapplers_lib::physics::player_input"
    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::size_adjustment::size_adjustment" and "whoops_all_grapplers_lib::character::move_advancement::move_advancement"
    conflicts: ["bevy_transform::components::transform::Transform", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::size_adjustment::size_adjustment" and "whoops_all_grapplers_lib::physics::move_players"
    conflicts: ["bevy_transform::components::transform::Transform", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::size_adjustment::size_adjustment" and "whoops_all_grapplers_lib::character::recovery::stun_recovery"        
    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::size_adjustment::size_adjustment" and "whoops_all_grapplers_lib::physics::move_constants"
    conflicts: ["bevy_transform::components::transform::Transform"]
 -- "whoops_all_grapplers_lib::physics::player_input" and "whoops_all_grapplers_lib::character::move_advancement::move_advancement"
    conflicts: ["whoops_all_grapplers_lib::physics::PlayerVelocity", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::physics::player_input" and "whoops_all_grapplers_lib::physics::move_players"
    conflicts: ["whoops_all_grapplers_lib::physics::PlayerVelocity"]
 -- "whoops_all_grapplers_lib::physics::player_input" and "whoops_all_grapplers_lib::character::recovery::stun_recovery"
    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::move_advancement::move_advancement" and "whoops_all_grapplers_lib::physics::move_players"
    conflicts: ["bevy_transform::components::transform::Transform", "whoops_all_grapplers_lib::physics::PlayerVelocity", "player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::move_advancement::move_advancement" and "whoops_all_grapplers_lib::character::recovery::stun_recovery"      
    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::character::move_advancement::move_advancement" and "whoops_all_grapplers_lib::physics::move_constants"
    conflicts: ["bevy_transform::components::transform::Transform"]
 -- "time::game_flow::tick_countdown" and "time::game_flow::tick_countdown"
    conflicts: ["bevy_ecs::schedule::state::State<time::game_flow::GameState>", "time::game_flow::InterFrameCountdown"]
 -- "whoops_all_grapplers_lib::physics::move_players" and "whoops_all_grapplers_lib::character::recovery::stun_recovery"
    conflicts: ["player_state::player_state::PlayerState"]
 -- "whoops_all_grapplers_lib::physics::move_players" and "whoops_all_grapplers_lib::physics::move_constants"
    conflicts: ["bevy_transform::components::transform::Transform"]

New gamepad connected with ID: Gamepad(0)
2022-02-05T04:38:28.155193Z  INFO bevy_ecs::schedule::stage: Execution order ambiguities detected, you might want to add an explicit dependency relation between some of these systems:
 * Parallel systems:
 -- "bevy_ui::widget::text::text_system" and "bevy_ui::widget::image::image_node_system"
    conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::image::Image>", "bevy_ui::ui_node::CalculatedSize"]
 -- "bevy_ui::widget::text::text_system" and "bevy_text::text2d::text2d_system"
    conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::image::Image>", "bevy_asset::assets::Assets<bevy_sprite::texture_atlas::TextureAtlas>", "bevy_asset::assets::Assets<bevy_text::font_atlas_set::FontAtlasSet>", "bevy_text::pipeline::TextPipeline<bevy_ecs::entity::Entity>"]
 -- "bevy_ui::widget::text::text_system" and "bevy_egui::update_egui_textures"
    conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::image::Image>"]
 -- "bevy_ui::widget::image::image_node_system" and "bevy_text::text2d::text2d_system"
    conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::image::Image>"]
 -- "bevy_ui::widget::image::image_node_system" and "bevy_egui::update_egui_textures"
    conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::image::Image>"]
 -- "bevy_ui::flex::flex_node_system" and "bevy_transform::hierarchy::hierarchy_maintenance_system::parent_update_system"
    conflicts: ["bevy_transform::components::children::Children"]
 -- "bevy_ui::flex::flex_node_system" and "whoops_all_grapplers_lib::camera::center_camera"
    conflicts: ["bevy_transform::components::transform::Transform"]
 -- "bevy_ui::update::ui_z_system" and "bevy_transform::hierarchy::hierarchy_maintenance_system::parent_update_system"
    conflicts: ["bevy_transform::components::children::Children"]
 -- "bevy_ui::update::ui_z_system" and "whoops_all_grapplers_lib::camera::center_camera"
    conflicts: ["bevy_transform::components::transform::Transform"]
 -- "bevy_transform::transform_propagate_system::transform_propagate_system" and "whoops_all_grapplers_lib::camera::center_camera"
    conflicts: ["bevy_transform::components::transform::Transform"]
 -- "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::PerspectiveProjection>" and "bevy_pbr::light::update_clusters"       
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::PerspectiveProjection>" and "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::OrthographicProjection>"
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::PerspectiveProjection>" and "bevy_render::view::visibility::update_frusta<bevy_render::camera::projection::PerspectiveProjection>"
    conflicts: ["bevy_render::camera::projection::PerspectiveProjection"]
 -- "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::PerspectiveProjection>" and "bevy_pbr::light::assign_lights_to_clusters"
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::PerspectiveProjection>" and "bevy_render::camera::active_cameras::active_cameras_system"
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::PerspectiveProjection>" and "bevy_render::camera::camera::camera_system<whoops_all_grapplers_lib::camera::SimpleOrthoProjection>"
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_pbr::light::update_clusters" and "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::OrthographicProjection>"      
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_pbr::light::update_clusters" and "bevy_render::camera::camera::camera_system<whoops_all_grapplers_lib::camera::SimpleOrthoProjection>"      
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::OrthographicProjection>" and "bevy_render::view::visibility::update_frusta<bevy_render::camera::projection::OrthographicProjection>"
    conflicts: ["bevy_render::camera::projection::OrthographicProjection"]
 -- "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::OrthographicProjection>" and "bevy_pbr::light::assign_lights_to_clusters"
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::OrthographicProjection>" and "bevy_render::camera::active_cameras::active_cameras_system"
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_render::camera::camera::camera_system<bevy_render::camera::projection::OrthographicProjection>" and "bevy_render::camera::camera::camera_system<whoops_all_grapplers_lib::camera::SimpleOrthoProjection>"
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_text::text2d::text2d_system" and "bevy_egui::update_egui_textures"
    conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::image::Image>"]
 -- "bevy_render::view::visibility::update_frusta<bevy_render::camera::projection::PerspectiveProjection>" and "bevy_render::view::visibility::update_frusta<bevy_render::camera::projection::OrthographicProjection>"
    conflicts: ["bevy_render::primitives::Frustum"]
 -- "bevy_render::view::visibility::update_frusta<bevy_render::camera::projection::PerspectiveProjection>" and "bevy_pbr::light::update_directional_light_frusta"
    conflicts: ["bevy_render::primitives::Frustum"]
 -- "bevy_render::view::visibility::update_frusta<bevy_render::camera::projection::PerspectiveProjection>" and "bevy_pbr::light::assign_lights_to_clusters"
    conflicts: ["bevy_render::primitives::Frustum"]
 -- "bevy_render::view::visibility::update_frusta<bevy_render::camera::projection::OrthographicProjection>" and "bevy_pbr::light::update_directional_light_frusta"
    conflicts: ["bevy_render::primitives::Frustum"]
 -- "bevy_render::view::visibility::update_frusta<bevy_render::camera::projection::OrthographicProjection>" and "bevy_pbr::light::assign_lights_to_clusters"
    conflicts: ["bevy_render::primitives::Frustum"]
 -- "bevy_render::view::visibility::check_visibility" and "bevy_pbr::light::update_directional_light_frusta"
    conflicts: ["bevy_render::primitives::Frustum"]
 -- "bevy_pbr::light::update_directional_light_frusta" and "bevy_pbr::light::assign_lights_to_clusters"
    conflicts: ["bevy_render::primitives::Frustum"]
 -- "bevy_pbr::light::assign_lights_to_clusters" and "bevy_render::camera::camera::camera_system<whoops_all_grapplers_lib::camera::SimpleOrthoProjection>"
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_render::camera::active_cameras::active_cameras_system" and "bevy_render::camera::camera::camera_system<whoops_all_grapplers_lib::camera::SimpleOrthoProjection>"
    conflicts: ["bevy_render::camera::camera::Camera"]
 -- "bevy_egui::update_egui_textures" and "bevy_egui::systems::process_output"
    conflicts: ["bevy_egui::EguiContext"]
 * Exclusive systems at start of stage:
 -- "bevy_winit::change_window" and "bevy_pbr::light::add_clusters"
 -- "bevy_winit::change_window" and "bevy_audio::audio_output::play_queued_audio_system<bevy_audio::audio_source::AudioSource>"
 -- "bevy_pbr::light::add_clusters" and "bevy_audio::audio_output::play_queued_audio_system<bevy_audio::audio_source::AudioSource>"

2022-02-05T04:38:28.347056Z  INFO bevy_ecs::schedule::stage: Execution order ambiguities detected, you might want to add an explicit dependency relation between some of these systems:
 * Parallel systems:
 -- "bevy_ui::render::extract_uinodes" and "bevy_sprite::render::extract_sprite_events"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_uinodes" and "bevy_sprite::render::extract_sprites"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_uinodes" and "bevy_text::text2d::extract_text2d_sprite"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_uinodes" and "bevy_core_pipeline::extract_clear_color"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_uinodes" and "bevy_egui::render_systems::extract_egui_textures"
    conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::image::Image>"]
 -- "bevy_ui::render::extract_uinodes" and "bevy_render::view::window::extract_windows"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_uinodes" and "bevy_render::render_resource::pipeline_cache::RenderPipelineCache::extract_shaders"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_text_uinodes" and "bevy_sprite::render::extract_sprite_events"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_text_uinodes" and "bevy_sprite::render::extract_sprites"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_text_uinodes" and "bevy_text::text2d::extract_text2d_sprite"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_text_uinodes" and "bevy_core_pipeline::extract_clear_color"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_text_uinodes" and "bevy_render::view::window::extract_windows"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_ui::render::extract_text_uinodes" and "bevy_render::render_resource::pipeline_cache::RenderPipelineCache::extract_shaders"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_sprite::render::extract_sprite_events" and "bevy_sprite::render::extract_sprites"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_sprite::render::extract_sprite_events" and "bevy_text::text2d::extract_text2d_sprite"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_sprite::render::extract_sprite_events" and "bevy_core_pipeline::extract_clear_color"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_sprite::render::extract_sprite_events" and "bevy_render::view::window::extract_windows"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_sprite::render::extract_sprite_events" and "bevy_render::render_resource::pipeline_cache::RenderPipelineCache::extract_shaders"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_sprite::render::extract_sprites" and "bevy_core_pipeline::extract_clear_color"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_sprite::render::extract_sprites" and "bevy_render::view::window::extract_windows"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_sprite::render::extract_sprites" and "bevy_render::render_resource::pipeline_cache::RenderPipelineCache::extract_shaders"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_text::text2d::extract_text2d_sprite" and "bevy_core_pipeline::extract_clear_color"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_text::text2d::extract_text2d_sprite" and "bevy_render::view::window::extract_windows"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_text::text2d::extract_text2d_sprite" and "bevy_render::render_resource::pipeline_cache::RenderPipelineCache::extract_shaders"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_render::camera::extract_cameras" and "bevy_pbr::render::light::extract_lights"
    conflicts: ["bevy_render::view::visibility::VisibleEntities"]
 -- "bevy_core_pipeline::extract_clear_color" and "bevy_render::view::window::extract_windows"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_core_pipeline::extract_clear_color" and "bevy_render::render_resource::pipeline_cache::RenderPipelineCache::extract_shaders"
    conflicts: ["bevy_render::RenderWorld"]
 -- "bevy_egui::render_systems::extract_egui_textures" and "bevy_render::render_asset::extract_render_asset<bevy_render::texture::image::Image>"      
    conflicts: ["bevy_asset::assets::Assets<bevy_render::texture::image::Image>"]
 -- "bevy_render::view::window::extract_windows" and "bevy_render::render_resource::pipeline_cache::RenderPipelineCache::extract_shaders"
    conflicts: ["bevy_render::RenderWorld"]
