extends Node2D

var flake = preload("res://snowflake.tscn")
@export var cam: Camera2D
@export var num_flakes: int
@export var spread: int
@export var s_between_max: float
@export var s_between_min: float

func _ready():
	schedule_flake()
	
func schedule_flake():
	get_tree().create_timer(RandomNumberGenerator.new().randf_range(s_between_min, s_between_max)).timeout.connect(add_flake)
		

func add_flake():
	var new_flake = flake.instantiate()
	self.get_global_transform()
	new_flake.position = Vector2(RandomNumberGenerator.new().randf_range(cam.position.x - spread, cam.position.x + spread),-1000)
	add_child(new_flake)
	
	schedule_flake()
	
