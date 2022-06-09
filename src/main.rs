use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy::window::WindowResized;

mod bounding_box;
use bounding_box::*;

#[derive(SystemLabel, Debug, Eq, PartialEq, Hash, Clone)]
enum SystemLabels {
    CardClick,
    CardDrag,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<WindowResized>()
        .init_resource::<WorldMousePosition>()
        .add_startup_system(setup)
        .add_system(world_mouse_position_writer)
        .add_system(card_position)
        .add_system(card_click.label(SystemLabels::CardClick))
        .add_system(
            card_drag
                .label(SystemLabels::CardDrag)
                .after(SystemLabels::CardClick),
        )
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
struct Target(usize);

#[derive(Component)]
struct Draggable(Option<Vec2>);

impl Draggable {
    fn new() -> Self {
        Self(None)
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
        let box_size = Size::new(100., 200.);

        let card = commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("cards/layout.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(box_size.width, box_size.height)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0., 0., 2.),
                    ..default()
                },
                ..default()
            })
            .insert(Card::new())
            .insert(Hoverable::new())
            .insert(Draggable::new())
            .id();

        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let text_style = TextStyle {
            font,
            font_size: 20.0,
            color: Color::RED,
        };
        let text_alignment = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        };

        let text_bundle = commands
            .spawn_bundle(Text2dBundle {
                text: Text::with_section("+1", text_style.clone(), text_alignment),
                transform: Transform {
                    translation: Vec3::new(0., -50., 3.),
                    ..default()
                },
                text_2d_bounds: Text2dBounds { size: box_size },
                ..default()
            })
            .id();

        commands.entity(card).push_children(&[text_bundle]);
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
    mut query: Query<(Entity, &mut Transform, &Sprite, &mut Hoverable), With<Card>>,
) {
    let hovered_entity = query
        .iter()
        .find(|(_, _, _, hoverable)| hoverable.0)
        .map(|(entity, _, _, _)| entity);

    for (entity, mut transform, sprite, mut hoverable) in query.iter_mut() {
        if let Some(size) = sprite.custom_size {
            let is_hovered = BoundingBox::new(transform.translation, size).point_in(world_pos.0);
            let is_this_hovered = Some(entity) == hovered_entity;

            hoverable.0 =
                (is_hovered && hovered_entity.is_none()) || (is_hovered && is_this_hovered);
        }
    }
}

fn card_position(
    q_hand: Query<&Hand>,
    mut query: Query<(Entity, &mut Transform, &Hoverable, &Draggable), With<Card>>,
) {
    for hand in q_hand.iter() {
        let cnt = hand.cards.len() as f32;

        for (entity, mut transform, hoverable, draggable) in query.iter_mut() {
            if draggable.0.is_none() {
                let pos = hand.cards.iter().position(|e| e == &entity);
                if let Some(i) = pos {
                    let i = i as f32;
                    let i = i - (cnt / 2.);
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
}

fn card_drag(
    mut commands: Commands,
    world_pos: Res<WorldMousePosition>,
    mut query: Query<(Entity, &mut Transform, &Draggable), With<Card>>,
) {
    for (entity, mut transform, draggable) in query.iter_mut() {
        if let Some(offset) = draggable.0 {
            println!("Moving card {:?}", entity);
            transform.translation.x = world_pos.0.x + offset.x;
            transform.translation.y = world_pos.0.y + offset.y;
            transform.translation.z = 10.;
        } else {
            transform.translation.z = 2.;
        }
    }
}

fn card_click(
    mut commands: Commands,
    world_pos: Res<WorldMousePosition>,
    mouse_input: Res<Input<MouseButton>>,
    mut q_hand: Query<&mut Hand>,
    mut query: Query<(Entity, &mut Transform, &Sprite, &mut Draggable), With<Card>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (entity, mut transform, sprite, mut draggable) in query.iter_mut() {
            if let Some(size) = sprite.custom_size {
                if BoundingBox::new(transform.translation, size).point_in(world_pos.0) {
                    let ox = transform.translation.x - world_pos.0.x;
                    let oy = transform.translation.y - world_pos.0.y;
                    let offset = Vec2::new(ox, oy);
                    println!("Setting {:?} as draggable with offset {:?}", entity, offset);
                    draggable.0 = Some(offset);
                }
            }
        }
    } else if mouse_input.just_released(MouseButton::Left) {
        let mut hand = q_hand.single_mut();

        println!("Unsetting draggable on all");

        for (entity, _, _, mut draggable) in query.iter_mut() {
            if draggable.0.is_some() {
                draggable.0 = None;
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
