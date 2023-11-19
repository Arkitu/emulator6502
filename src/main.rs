use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;

type Memory = Vec<u8>; // with length of 65536 bits

fn main() {
    let env = std::env::args().collect::<Vec<_>>();
    let filename = &env[env.len()-1];

    let file_content = std::fs::read_to_string(filename).expect("Can't read the assembly file");

    let mut memory: Memory = vec![0; 65536];

    let pixel_buffer_size = PixelBufferSize {
        size: UVec2::new(32, 32),       // amount of pixels
        pixel_size: UVec2::new(16, 16), // size of each pixel in the screen
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PixelBufferPlugin)  // Add this plugin
        .add_systems(Startup, pixel_buffer_setup(pixel_buffer_size)) // Setup system
        .add_systems(Update, update)
        .run()
}

fn update(mut pb: QueryPixelBuffer) {
    // Set each pixel to a random color
    pb.frame().per_pixel(|_, _| Pixel::random());
}