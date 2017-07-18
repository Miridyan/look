extern crate gl;
extern crate glutin;

mod backend;

// pub mod platform;
pub mod primative;

#[cfg(feature = "default")]
pub use backend::opengl::*;
#[cfg(feature = "with_vk")]
pub use backend::vulkan::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
