use ggez::{
	graphics::{self, Color, DrawMode, DrawParam, Image, Mesh, Rect},
	event::{run, EventHandler},
	glam::{vec2, Vec2},
	Context, ContextBuilder, GameResult
};

fn main() {
	let (mut ctx, event_loop) = ContextBuilder::new("game", "Author")
		.build()
		.expect("ERROR: Could not create ggez context!");

	let my_game = Game::new(&mut ctx);

	run(ctx, event_loop, my_game);
}

struct Game {
	mesh: Mesh,
	texture: Image
}
impl Game {
    pub fn new(gfx: &mut Context) -> Self {
        let mesh = Mesh::new_rectangle(
            gfx,
            DrawMode::fill(),
            Rect::one(),
            Color::WHITE
        ).unwrap();

        Self {
            mesh,
            texture: Image::from_path(gfx, "/texture.png").unwrap()
        }
    }
}
impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult { Ok(()) }
    fn draw(&mut self, gfx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(gfx, Color::BLACK);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());
        canvas.draw_textured_mesh(
            self.mesh.clone(),
            self.texture.clone(),
            DrawParam::new().dest(Vec2::ZERO).scale(vec2(200.0, 200.0)).src(Rect {
                x: 0.0,
                y: 0.0,
                w: 1.0,
                h: 1.0
            })
        );
        canvas.finish(gfx)
    }
}
