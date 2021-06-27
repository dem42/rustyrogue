use log::info;
use mimic_common::{
    apptime::AppTime,
    config::MimicConfig,
    uniforms::{update_uniform_buffer, UniformBufferObject, UniformMetadata},
};
use mimic_frontend::{
    main_loop::{Application, MainLoopBuilder},
    render_commands::RenderCommands,
};
//////////////////////// Consts ///////////////////////
const WINDOW_TITLE: &'static str = "Vulkan Demo";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
//////////////////////// Structs ///////////////////////
#[derive(Default)]
struct Demo {
    scene_sent: bool,
}
//////////////////////// Impls ///////////////////////
impl Application for Demo {
    fn update(
        &mut self,
        render_commands: &mut RenderCommands,
        apptime: &AppTime,
        config: &MimicConfig,
    ) {
        render_commands.request_redraw = true;

        if !self.scene_sent && apptime.elapsed_since_game_start.as_secs_f32() > 5.0 {
            render_commands.draw_textured_model(
                config.resolve_resource("res/textures/texture.jpg").unwrap(),
                config.resolve_resource("res/models/cube.obj").unwrap(),
                config
                    .resolve_resource("res/shaders/spv/cube.vert.spv")
                    .unwrap(),
                config
                    .resolve_resource("res/shaders/spv/cube.frag.spv")
                    .unwrap(),
                UniformMetadata::new::<UniformBufferObject>(update_uniform_buffer),
            );
            self.scene_sent = true;
        }
    }
}
//////////////////////// Fns ///////////////////////
fn main() {
    env_logger::init();
    info!("Hello demo");
    MainLoopBuilder::new()
        .with_window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .expect("Failed to create window")
        .run(Demo::default());
}
