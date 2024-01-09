use bevy::prelude::*;
use godot::engine::Node;
use godot::prelude::*;
use rand::Rng;

use bevy::utils::HashMap;
struct MyExtension;

#[derive(GodotClass)]
#[class(base=Node)]
struct BevyECS {
    app: App,
    #[base]
    node: Base<Node>,
}

#[derive(Resource)]
struct GodotSharedData {
    positions: HashMap<u32, Vec2>,
}

#[derive(Resource)]
struct Gravity {
    value: f32,
}

impl Default for Gravity {
    fn default() -> Gravity {
        Self { value: 0.5 }
    }
}

#[derive(Resource)]
struct Wind {
    value: f32,
}

impl Default for GodotSharedData {
    fn default() -> GodotSharedData {
        Self {
            positions: HashMap::new(),
        }
    }
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct OffScreen();

#[godot_api]
impl BevyECS {
    fn bevy_startup(mut data: ResMut<GodotSharedData>, mut commands: Commands) {
        let num_particles = 10;
        let mut rng = rand::thread_rng();

        for _ in 0..num_particles {
            let position = Vec2 {
                x: rng.gen_range(0.0..1.0),
                y: 0.0,
            };

            let velocity = Vec2 {
                x: rng.gen_range(-0.1..0.1),
                y: rng.gen_range(-0.1..0.1),
            };

            let entity_commands = commands.spawn((
                Position {
                    x: position.x,
                    y: position.y,
                },
                Velocity {
                    x: velocity.x,
                    y: velocity.y,
                },
            ));
            data.positions
                .insert(entity_commands.id().index(), position);
        }
    }

    fn send_godot_data(mut data: ResMut<GodotSharedData>, particles: Query<(Entity, &Position)>) {
        for (entity, position) in &particles {
            data.positions.insert(
                entity.index(),
                Vec2 {
                    x: position.x,
                    y: position.y,
                },
            );
        }
    }

    fn move_with_velocity(time: Res<Time>, mut particles: Query<(&mut Position, &Velocity)>) {
        for (mut position, velocity) in &mut particles {
            position.x += velocity.x * time.delta_seconds();
            position.y += velocity.y * time.delta_seconds();
        }
    }

    fn gravity(gravity: Res<Gravity>, time: Res<Time>, mut velocities: Query<&mut Velocity>) {
        for mut velocity in &mut velocities {
            velocity.y += gravity.value * time.delta_seconds();
        }
    }

    fn should_be_cleaned_up(
        mut data: ResMut<GodotSharedData>,
        mut commands: Commands,
        positions: Query<(Entity, &Position)>,
    ) {
        for (entity, position) in &positions {
            if position.y >= 1.0 {
                data.positions.remove(&entity.index());
                commands.entity(entity).despawn();
                let eid = entity.index();
                let str: String = format!("entity {eid} despawned");
                godot_print!("{}", str);
                let keys: Vec<&u32> = data.positions.keys().collect();
                godot_print!("{:?}", data.positions);
            }
        }
    }

    #[func]
    fn get_shared_data(&mut self) -> Dictionary {
        let mut ret = Dictionary::new();

        let resource = &self.app.world.get_resource::<GodotSharedData>();
        match resource {
            None => return ret,
            Some(res) => {
                for (entity, position) in &res.positions {
                    ret.insert(
                        *entity,
                        Vector2 {
                            x: position.x,
                            y: position.y,
                        },
                    );
                }
            }
        }

        return ret;
    }
}

#[godot_api]
impl INode for BevyECS {
    fn init(node: Base<Node>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            node,
            app: App::new(),
        }
    }

    fn ready(&mut self) {
        self.app.init_resource::<GodotSharedData>();
        self.app.init_resource::<Gravity>();

        self.app.add_plugins(MinimalPlugins);

        self.app.add_systems(Startup, BevyECS::bevy_startup);
        self.app.add_systems(Update, BevyECS::send_godot_data);
        self.app.add_systems(Update, BevyECS::move_with_velocity);
        self.app.add_systems(Update, BevyECS::should_be_cleaned_up.after(BevyECS::send_godot_data));
        self.app.add_systems(Update, BevyECS::gravity);

        godot_print!("bevy ecs ready!"); // Prints to the Godot console
    }

    fn process(&mut self, _delta: f64) {
        self.app.update();
    }
}

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
