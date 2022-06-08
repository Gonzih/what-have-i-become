use bevy::prelude::*;
use bevy::render::render_phase::EntityPhaseItem;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowResized;

mod bounding_box;
use bounding_box::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<WindowResized>()
        .init_resource::<WorldMousePosition>()
        .add_startup_system(setup)
        .add_system(world_mouse_position_writer)
        .add_system(card_position)
        .add_system(card_click)
        .add_system(card_hoverable)
        .add_system(card_hovered)
        .add_system(resize_notificator)
        .run();
}

struct WorldMousePosition(Vec2);

impl Default for WorldMousePosition {
    fn default() -> Self {
        WorldMousePosition(Vec2::new(0., 0.))
    }
}

#[derive(Component)]
struct Hoverable(bool);

impl Hoverable {
    fn new() -> Self {
        Self(false)
    }
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
                transform: Transform { ..default() },
                ..default()
            })
            .insert(Card::new())
            .insert(Hoverable::new())
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
    hand.spawn_card(&mut commands, &mut meshes, &mut materials, &asset_server);
    hand.spawn_card(&mut commands, &mut meshes, &mut materials, &asset_server);
    hand.spawn_card(&mut commands, &mut meshes, &mut materials, &asset_server);
    hand.spawn_card(&mut commands, &mut meshes, &mut materials, &asset_server);
    hand.spawn_card(&mut commands, &mut meshes, &mut materials, &asset_server);
    hand.spawn_card(&mut commands, &mut meshes, &mut materials, &asset_server);
    hand.spawn_card(&mut commands, &mut meshes, &mut materials, &asset_server);
    hand.spawn_card(&mut commands, &mut meshes, &mut materials, &asset_server);

    commands.entity(hand_id).insert(hand);
}

fn resize_notificator(mut events: EventReader<WindowResized>) {
    for e in events.iter() {
        println!("width = {} height = {}", e.width, e.height);
    }
}

fn card_hovered(mut query: Query<(&mut Transform, &Hoverable), With<Card>>) {
    for (mut transform, hoverable) in query.iter_mut() {
        if hoverable.0 {
            transform.scale.x = 1.1;
            transform.scale.y = 1.1;
        } else {
            transform.scale.x = 1.;
            transform.scale.y = 1.;
        }
    }
}

fn card_hoverable(
    world_pos: Res<WorldMousePosition>,
    mut query: Query<(&mut Transform, &Sprite, &mut Hoverable), With<Card>>,
) {
    for (mut transform, sprite, mut hoverable) in query.iter_mut() {
        if let Some(size) = sprite.custom_size {
            hoverable.0 = BoundingBox::new(transform.translation, size).point_in(world_pos.0);
        }
    }
}

fn card_position(
    q_hand: Query<&Hand>,
    mut query: Query<(Entity, &mut Transform, &Hoverable), With<Card>>,
) {
    for hand in q_hand.iter() {
        let cnt = hand.cards.len() as f32;

        for (entity, mut transform, hoverable) in query.iter_mut() {
            let pos = hand.cards.iter().position(|e| e == &entity);
            if let Some(i) = pos {
                let i = i as f32;
                let i = (i - (cnt / 2.));
                let x: f32 = i * 110.;
                let mut y: f32 = i.abs() * -30.0;

                if hoverable.0 {
                    y += 10.;
                }

                transform.translation.x = x;
                transform.translation.y = y;
            }
        }
    }
}

fn card_click(
    mut commands: Commands,
    world_pos: Res<WorldMousePosition>,
    mouse_input: Res<Input<MouseButton>>,
    mut q_hand: Query<&mut Hand>,
    mut query: Query<(Entity, &mut Transform, &Sprite), With<Card>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for mut hand in q_hand.iter_mut() {
            for (entity, mut transform, sprite) in query.iter_mut() {
                if let Some(size) = sprite.custom_size {
                    if BoundingBox::new(transform.translation, size).point_in(world_pos.0) {
                        println!("Removing card {:?}", entity);
                        commands.entity(entity).despawn_recursive();

                        let pos = hand.cards.iter().position(|e| e == &entity);
                        if let Some(i) = pos {
                            hand.cards.remove(i);
                        }
                    }
                }
            }
        }
    }
}

fn world_mouse_position_writer(
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut world_pos: ResMut<WorldMousePosition>,
) {
    let (camera, camera_transform) = q_camera.single();

    if let Some(wnd) = windows.get_primary() {
        if let Some(screen_pos) = wnd.cursor_position() {
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix.inverse();
            let w_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
            let w_pos: Vec2 = w_pos.truncate();

            world_pos.0 = w_pos;
        }
    }
}
