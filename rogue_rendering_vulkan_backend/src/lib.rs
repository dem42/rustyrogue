extern crate memoffset;
extern crate nalgebra_glm as glm;
extern crate rustylog;

pub mod buffers;
pub mod depth;
pub mod devices;
pub mod drawing;
pub mod graphics_pipeline;
pub mod models;
pub mod presentation;
pub mod textures;
pub mod uniforms;
pub mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
