use std::task::Wake;

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
        let num_particles = 50000;
        let mut rng = rand::thread_rng();
        data.positions.resize(num_particles, Vec2 {x: -1.0, y: -1.0});

        for ind in 0..num_particles {
            let position = Vec2 {
                x: rng.gen_range(-100.0..100.0),
                y: rng.gen_range(-100.0..100.0)
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
    
    fn move_with_velocity(mut particles: Query<(&mut Position, &Velocity)>) {
        for (mut position, velocity) in &mut particles {
            position.x += velocity.x;
            position.y += velocity.y;
        }
    }

    #[func]
    fn get_shared_data(&mut self) -> Array<Vector2> {
        
        let mut ret = Array::default();


        for position in &self.app.world.get_resource::<GodotSharedData>().unwrap().positions {
            ret.push(Vector2 { x: position.x, y: position.y });
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

        self.app.add_systems(Startup, BevyECS::bevy_startup);
        self.app.add_systems(Update, BevyECS::send_godot_data);
        self.app.add_systems(Update, BevyECS::move_with_velocity);


        godot_print!("bevy ecs ready!"); // Prints to the Godot console
    }

    fn process(&mut self, _delta: f64) {
        self.app.update();
    }
}


#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
