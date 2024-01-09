extends ColorRect

@onready var pixel = preload("res://snowflake.tscn")
@export var ecs: BevyECS
@export var pixel_parent: Node 

var pixels := {}


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var all_positions = ecs.get_shared_data()
	
	# print_debug(all_positions)

	# this will be all the entity IDs we have nodes for 
	var current_instance_ids = pixels.keys()

	for eid in all_positions.keys():
		var viewport_size = self.get_viewport().get_size()
		var pos = all_positions.get(eid)
		if pixels.has(eid):
			pixels[eid].position.x = pos[0] * viewport_size.x  - viewport_size.x / 2
			pixels[eid].position.y = pos[1] * viewport_size.y - viewport_size.y / 2
			# if we found data for the instance, remove it from our tracker list
			current_instance_ids.erase(eid)
		else:
			var new_pixel = pixel.instantiate()
			pixels[eid] = new_pixel
			pixels[eid].position.x = pos[0] * viewport_size.x  - viewport_size.x / 2
			pixels[eid].position.y = pos[1] * viewport_size.y - viewport_size.y / 2
			pixel_parent.add_child(new_pixel)

	# we remove ones that are real from this list as we go so if there's any remaining they are dead
	for dead_snowflake in current_instance_ids:
		pixels[dead_snowflake].queue_free()
		pixels.erase(dead_snowflake)

	# problem: rust is adding/removing the entities
	# we're tracking a dictionary of references for the display for these entities
	# so if there's a entry in the ECS data that we don't have then we initialize it
	# and if there's an entry that we have that the ECS data doesn't we assume it's been deleted and delete the node on godots end
	# one way of doing this to prevent iterating twice is just have bevy set the positon on cleaned up nodes to -1,-1
	# but this makes the shared data hashmap larger all the time and defeats the purpose of cleaning entities up
