
use godot::prelude::*;
use bevy::prelude::*;
use godot::engine::Node;

use rand::Rng;

struct MyExtension;

#[derive(GodotClass)]
#[class(base=Node)]
struct BevyECS {
    app: App,
    #[base]
    node: Base<Node>
}

#[derive(Resource)]
struct GodotSharedData {
    positions: Vec<Vec2>
}

#[derive(Resource)]
struct Gravity {
    value: f32
}

impl Default for Gravity {
    fn default() -> Gravity {
        Self {
            value: 0.5
        }
    }
}

#[derive(Resource)]
struct Wind {
    value: f32
}


impl Default for GodotSharedData {
    fn default() -> GodotSharedData {
        Self {
            positions: [].to_vec()
        }
    }
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32 
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32
}

#[godot_api]
impl BevyECS {
    fn bevy_startup(mut data: ResMut<GodotSharedData>, mut commands: Commands) {
        let num_particles = 10000;
        let mut rng = rand::thread_rng();
        data.positions.resize(num_particles, Vec2 {x: -1.0, y: -1.0});

        for ind in 0..num_particles {
            let position = Vec2 {
                x: rng.gen_range(0.0..1.0),
                y: 0.0
            };

            let velocity = Vec2 {
                x: rng.gen_range(-0.1..0.1),
                y: rng.gen_range(-0.1..0.1),
            };

            commands.spawn( (Position{x: position.x, y: position.y },Velocity{x: velocity.x, y: velocity.y}));
            data.positions[ind] = position;
        }
    } 

    fn send_godot_data(mut data: ResMut<GodotSharedData>, particles: Query<&Position>) {
        let mut ind = 0;
        for position in &particles {
            data.positions[ind] = Vec2{x: position.x, y: position.y};
            ind += 1;
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


    #[func]
    fn get_shared_data(&mut self) -> Array<Vector2> {
        
        let mut ret = Array::default();

        let resource = &self.app.world.get_resource::<GodotSharedData>();
        match resource {
            None => return ret,
            Some(res) => {
                for position in &res.positions {
                    ret.push(Vector2 { x: position.x, y: position.y });
                }
            }
        }
            
        return ret
    }

}

#[godot_api]
impl INode for BevyECS {
    fn init(node: Base<Node>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        
        Self {
            node,
            app: App::new() 
        }
    }

    fn ready(&mut self) {
        self.app.init_resource::<GodotSharedData>();
        self.app.init_resource::<Gravity>();

        self.app.add_plugins(MinimalPlugins);
        
        self.app.add_systems(Startup, BevyECS::bevy_startup);
        self.app.add_systems(Update, BevyECS::send_godot_data);
        self.app.add_systems(Update, BevyECS::move_with_velocity);
        self.app.add_systems(Update, BevyECS::gravity);


        godot_print!("bevy ecs ready!"); // Prints to the Godot console
    }

    fn process(&mut self, _delta: f64) {
        self.app.update();
    }
}


#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
