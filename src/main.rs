use bevy::prelude::*;
use bevy::render::render_phase::EntityPhaseItem;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowResized;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<WindowResized>()
        .add_startup_system(setup)
        .add_system(card_hover)
        .add_system(resize_notificator)
        .run();
}
#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Card {}

impl Card {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Component)]
struct Hand {
    entity: Entity,
    cards: Vec<Entity>,
}

impl Hand {
    fn new(entity: Entity) -> Self {
        Self {
            entity,
            cards: Vec::new(),
        }
    }

    fn spawn_card(
        &mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        asset_server: &Res<AssetServer>,
    ) {
        let card = commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("cards/layout.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 200.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..default()
                },
                ..default()
            })
            .insert(Card::new())
            .id();

        // commands.entity(self.entity).push_children(&[card]);
        self.cards.push(card);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    let hand_id = commands.spawn().id();
    let mut hand = Hand::new(hand_id);

    hand.spawn_card(&mut commands, &mut meshes, &mut materials, &asset_server);

    commands.entity(hand_id).insert(hand);
}

fn resize_notificator(mut events: EventReader<WindowResized>) {
    for e in events.iter() {
        println!("width = {} height = {}", e.width, e.height);
    }
}

fn card_hover(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut query: Query<(&mut Transform, &Sprite), With<Card>>,
) {
    let (camera, camera_transform) = q_camera.single();

    let wnd = wnds.get_primary().unwrap();

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();

        for (mut transform, sprite) in query.iter_mut() {
            if let Some(size) = sprite.custom_size {
                let pointer_x = world_pos.x;
                let pointer_y = world_pos.y;

                if BoundingBox::new(transform.translation, size).point_in(world_pos) {
                    transform.translation.x -= 0.1;
                    transform.translation.y -= 0.1;
                    eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
                    eprintln!("Transform {:?}", transform);
                }
            }
        }
    }
}

struct BoundingBox {
    center: Vec3,
    size: Vec2,
}

impl BoundingBox {
    fn new(center: Vec3, size: Vec2) -> Self {
        Self { center, size }
    }

    fn point_in(&self, point: Vec2) -> bool {
        let start_x = self.center.x - self.size.x / 2.;
        let start_y = self.center.y - self.size.y / 2.;
        let end_x = self.center.x + self.size.x / 2.;
        let end_y = self.center.y + self.size.y / 2.;

        start_x < point.x && end_x > point.x && start_y < point.y && end_y > point.y
    }
}
