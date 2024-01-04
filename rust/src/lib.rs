use godot::prelude::*;
use bevy::prelude::*;
use godot::engine::Node;

struct MyExtension;

#[derive(GodotClass)]
#[class(base=Node)]
struct BevyECS {
    app: App,
    #[base]
    node: Base<Node>
}
impl BevyECS {
    fn hell_world() {
        godot_print!("bevy ecs printing");
    }
}

#[godot_api]
impl INode for BevyECS {
    fn init(node: Base<Node>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        
        let mut app = App::new();
        app.add_systems(Update, BevyECS::hell_world);
        Self {
            node,
            app
        }
    }
    

    fn ready(&mut self) {
        godot_print!("bevy ecs ready!"); // Prints to the Godot console
    }

    fn process(&mut self, delta: f64) {
        self.app.update();
    }
}


#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
