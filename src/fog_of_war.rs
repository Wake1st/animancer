use bevy::prelude::*;

pub struct FogOfWarPlugin;

impl Plugin for FogOfWarPlugin {
    fn build(&self, app: &mut App) {
        app.update().draw(draw)
    }
}

fn setup(gfx: &mut Graphics) -> State {
    let texture = gfx
        .create_texture()
        .from_image(include_bytes!("assets/pattern.png"))
        .build()
        .unwrap();
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    // Draw some geometry as mask
    let mut mask = gfx.create_draw();
    mask.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0))
        .stroke(20.0);
    mask.circle(30.0).position(400.0, 350.0);
    mask.circle(50.0).position(400.0, 350.0).stroke(10.0);
    mask.circle(70.0).position(400.0, 350.0).stroke(10.0);

    let mut draw = gfx.create_draw();
    draw.clear(Color::new(0.1, 0.2, 0.3, 1.0));

    // Draw a pattern with the mask
    draw.mask(Some(&mask));
    draw.pattern(&state.img)
        .size(800.0, 600.0)
        .image_offset(-state.count, -state.count);
    draw.mask(None);

    gfx.render(&draw);
}
