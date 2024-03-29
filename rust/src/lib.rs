use bevy::prelude::*;
use godot::engine::Node;
use godot::prelude::*;
use rand::Rng;
use std::time::Duration;
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
struct GodotSharedData(HashMap<u32, Vec3>);

impl Default for GodotSharedData {
    fn default() -> GodotSharedData {
        Self ( 
            HashMap::new(),
        ) 
    }
}

#[derive(Resource)]
struct NewSnowflakeTimer {
    timer: Timer,
    min_time: f32,
    max_time: f32,
}

impl Default for NewSnowflakeTimer {
    fn default() -> NewSnowflakeTimer {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            min_time: 0.1,
            max_time: 0.5,
        }
    }
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

#[derive(Resource, Default)]
struct Wind(f32);


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
struct RotationVelocity(f32);

#[derive(Component)]
struct Rotation(f32);


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
                Rotation( 0.0 ),
                RotationVelocity(0.0)
            ));
            data.0.insert(entity_commands.id().index(), Vec3 {x: position.x, y: position.y, z: 0.0});
        }
    }

    fn send_godot_data(mut data: ResMut<GodotSharedData>, particles: Query<(Entity, &Position, &Rotation)>) {
        for (entity, position, rotation) in &particles {
            data.0.insert(
                entity.index(),
                Vec3 {
                    x: position.x,
                    y: position.y,
                    z: rotation.0
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

    fn flake_rotation(time: Res<Time>, mut rotations: Query<(&mut Rotation, &RotationVelocity)>) {
        for (mut rotation, r_v) in &mut rotations {
            rotation.0 += r_v.0 * 1000.0 * time.delta_seconds();
        }
    }

    fn wind(wind: Res<Wind>, time: Res<Time>, mut positions: Query<(&mut Position, &mut RotationVelocity)>) {
        // directly apply the wind value to the positions since it's a fixed velocity wind?
        for (mut position, mut r_vel) in &mut positions {
            position.x += wind.0 * time.delta_seconds();
            r_vel.0 -= wind.0 * time.delta_seconds();
        }
    }

    fn should_be_cleaned_up(
        mut data: ResMut<GodotSharedData>,
        mut commands: Commands,
        positions: Query<(Entity, &Position)>,
    ) {
        for (entity, position) in &positions {
            if position.y >= 1.0 {
                data.0.remove(&entity.index());
                commands.entity(entity).despawn();
            }
        }
    }

    fn spawn_new_flakes(
        time: Res<Time>,
        mut timer_res: ResMut<NewSnowflakeTimer>,
        mut commands: Commands,
    ) {
        timer_res.timer.tick(time.delta());
        if timer_res.timer.finished() {
            let mut rng = rand::thread_rng();
            let position = Vec2 {
                x: rng.gen_range(0.0..1.0),
                y: -0.1,
            };

            let velocity = Vec2 {
                x: rng.gen_range(-0.1..0.1),
                y: rng.gen_range(-0.1..0.1),
            };

            commands.spawn((
                Position {
                    x: position.x,
                    y: position.y,
                },
                Velocity {
                    x: velocity.x,
                    y: velocity.y,
                },
                Rotation( rng.gen_range(0.0..180.0) ),
                RotationVelocity(0.0)
            ));
            // reset the timer
            let seconds: f32 = rng.gen_range(timer_res.min_time..timer_res.max_time);
            timer_res
                .timer
                .set_duration(Duration::from_secs_f32(seconds));
        }
    }

    #[func]
    fn edit_snowflake_timer_params(&mut self, min_time: f32, max_time: f32) {
        let resource = &mut self.app.world.resource_mut::<NewSnowflakeTimer>();
        resource.max_time = max_time;
        resource.min_time = min_time;
    }

    #[func]
    fn edit_wind(&mut self, wind_val: f32) {
        let resource = &mut self.app.world.resource_mut::<Wind>();
        resource.0 = wind_val;
    }

    #[func]
    fn edit_gravity(&mut self, gravity_val: f32) {
        let resource = &mut self.app.world.resource_mut::<Gravity>();
        resource.value = gravity_val;
    }

    #[func]
    fn get_shared_data(&mut self) -> Dictionary {
        let mut ret = Dictionary::new();

        let resource = &self.app.world.get_resource::<GodotSharedData>();
        match resource {
            None => return ret,
            Some(res) => {
                for (entity, data) in &res.0 {
                    ret.insert(
                        *entity,
                        Vector3 {
                            x: data.x,
                            y: data.y,
                            z: data.z
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
        self.app.init_resource::<NewSnowflakeTimer>();
        self.app.init_resource::<Gravity>();
        self.app.init_resource::<Wind>();

        self.app.add_plugins(MinimalPlugins);

        self.app.add_systems(Startup, BevyECS::bevy_startup);

        self.app.add_systems(Update, BevyECS::send_godot_data);
        self.app.add_systems(Update, BevyECS::wind);
        self.app.add_systems(Update, BevyECS::flake_rotation);
        self.app.add_systems(Update, BevyECS::move_with_velocity);
        self.app.add_systems(
            Update,
            BevyECS::should_be_cleaned_up.after(BevyECS::send_godot_data),
        );
        self.app.add_systems(Update, BevyECS::gravity);
        self.app.add_systems(Update, BevyECS::spawn_new_flakes);

        godot_print!("bevy ecs ready!"); // Prints to the Godot console
    }

    fn process(&mut self, _delta: f64) {
        self.app.update();
    }
}

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
