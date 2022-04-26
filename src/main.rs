mod world;
mod particle;

use piston_window::*;
use world::World;

fn main() {
    let (width, height) = (1280.0, 960.0);
    let mut windows: PistonWindow = WindowSettings::new("particle", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create a windows.");
    
    let mut world = World::new(width, height);
    world.add_shapes(10000);

    while let Some(event) = windows.next() {
        world.update();

        windows.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.9], renderer);

            for s in &mut world.particles {
                let size = [s.position[0], s.position[1], s.width, s.height];
                rectangle(s.color, size, ctx.transform, renderer);
            }
        });
    }
}
