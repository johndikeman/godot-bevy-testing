extends ColorRect

@onready var pixel = preload("res://snowflake.tscn")
@export var ecs: BevyECS
@export var pixel_parent: Node 

var pixels := []

# Called when the node enters the scene tree for the first time.
func initial_populate(all_positions: Array):
	 
	for pos in all_positions:
		var new_pixel = pixel.instantiate()
		pixels.append(new_pixel)
		pixel_parent.add_child(new_pixel)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var all_positions = ecs.get_shared_data()
	if all_positions.size() == 0:
		return
	if pixels.size() == 0:
		initial_populate(all_positions)

	var ind = 0

	for pos in all_positions:
		var viewport_size = self.get_viewport().get_size()
		pixels[ind].position.x = pos[0] * viewport_size.x  - viewport_size.x / 2
		pixels[ind].position.y = pos[1] * viewport_size.y - viewport_size.y / 2
		ind += 1
