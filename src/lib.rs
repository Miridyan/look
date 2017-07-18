extern crate glutin;

#[cfg(feature = "default")]
extern crate gl;
#[cfg(feature = "with_vk")]
extern crate vulkano;

mod backend;

// pub mod platform;
pub mod primative;

#[cfg(feature = "default")]
use backend::opengl::*;
#[cfg(feature = "with_vk")]
use backend::vulkan::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
