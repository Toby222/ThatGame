use macroquad::{
    camera::Camera2D,
    math::Rect,
    prelude::{is_key_down, is_key_pressed, set_camera, trace, vec2, ImageFormat, KeyCode, Vec2},
    text::draw_text,
    texture::{FilterMode, Texture2D},
    time::get_frame_time,
    window::{next_frame, screen_height, screen_width},
};
use macroquad_platformer::{Actor, Tile, World};
use macroquad_tiled::Map;

const MAP_1_JSON: &'static str = include_str!("../resources/tilemaps/map-1.tmj");
const BLUISH_TILES: &'static [u8] = include_bytes!("../resources/bluish-tiles.png");

struct GameLevels {
    level_1: Level,
}

struct Player {
    collider: Actor,
    speed: Vec2,
}

struct Level {
    map: Map,
    world: World,
}
impl Level {
    const fn new(map: Map, world: World) -> Self {
        Self { map, world }
    }
}

fn load_levels() -> GameLevels {
    let bluish_tiles_texture =
        Texture2D::from_file_with_format(BLUISH_TILES, Some(ImageFormat::Png));
    bluish_tiles_texture.set_filter(FilterMode::Nearest);
    let map_textures = [("../bluish-tiles.png", bluish_tiles_texture)];

    let map_1 = macroquad_tiled::load_map(MAP_1_JSON, &map_textures, &[])
        .unwrap_or_else(|err| panic!("Failed to load map 1: {err}"));
    let mut world_1 = World::new();

    let mut static_colliders = vec![];
    for (_x, _y, tile) in map_1.tiles("bricks", None) {
        static_colliders.push(if tile.is_some() {
            Tile::Solid
        } else {
            Tile::Empty
        });
    }

    world_1.add_static_tiled_layer(
        static_colliders,
        map_1.raw_tiled_map.tilewidth as f32,
        map_1.raw_tiled_map.tileheight as f32,
        map_1.raw_tiled_map.width as _,
        1,
    );

    GameLevels {
        level_1: Level::new(map_1, world_1),
    }
}

#[macroquad::main("ThatGame")]
async fn main() -> Result<(), macroquad_tiled::Error> {
    let levels = load_levels();
    let mut world = levels.level_1.world;
    let map = levels.level_1.map;

    let map_width = (map.raw_tiled_map.width * map.raw_tiled_map.tilewidth) as f32;
    let map_height = (map.raw_tiled_map.height * map.raw_tiled_map.tileheight) as f32;

    let mut player = Player {
        collider: world.add_actor(vec2(map_width / 2.0, map_height / 2.0), 16, 16),
        speed: Vec2::ZERO,
    };

    let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));

    loop {
        set_camera(&camera);
        map.draw_tiles("wall", Rect::new(0.0, 0.0, map_width, map_height), None);
        map.draw_tiles("bricks", Rect::new(0.0, 0.0, map_width, map_height), None);

        // draw player
        {
            // sprite id from tiled
            const PLAYER_SPRITE: u32 = 40;

            let pos = world.actor_pos(player.collider);
            map.spr(
                "bluish-tiles",
                PLAYER_SPRITE,
                Rect::new(pos.x, pos.y, 16.0, 16.0),
            );
            trace!("Position: {}", pos);
        }

        // player movement control
        {
            let pos = world.actor_pos(player.collider);
            let on_ground = world.collide_check(player.collider, pos + vec2(0., 1.));

            if on_ground == false {
                player.speed.y += 500. * get_frame_time();
            } else {
                player.speed.y = f32::max(0.0, player.speed.y);
            }

            if is_key_down(KeyCode::Right) {
                player.speed.x = 100.0;
            } else if is_key_down(KeyCode::Left) {
                player.speed.x = -100.0;
            } else {
                player.speed.x = 0.;
            }

            if is_key_pressed(KeyCode::Space) {
                if on_ground {
                    player.speed.y = -120.;
                }
            }

            world.move_h(player.collider, player.speed.x * get_frame_time());
            world.move_v(player.collider, player.speed.y * get_frame_time());
            let pos = world.actor_pos(player.collider);
            draw_text(
                format!("{pos} - {} - {}", on_ground, world.solid_at(pos)).as_str(),
                0.0,
                20.0,
                20.0,
                macroquad::color::WHITE,
            )
        }

        next_frame().await
    }
}
