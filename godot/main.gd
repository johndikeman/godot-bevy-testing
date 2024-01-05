extends Node

@export var ecs: BevyECS

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


func check_data():
	print_debug(ecs.get_shared_data())
