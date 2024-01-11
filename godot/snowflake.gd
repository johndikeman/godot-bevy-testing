extends Node2D 

@export var size: int
@export var color: Color
@export var num_points: int
@export var sprite: SnowflakeLines
var image: Image


func _ready():
	var rng = RandomNumberGenerator.new()
	# create the image texture we need
	var _selected_points = select_points(size, size)
	var s = rng.randf_range(1,5)

	self.scale = Vector2(s,s)
	sprite.position = Vector2(size / -2,size / -2)
	sprite.set_points(_selected_points)
	sprite.color = color
	

func select_points(height: int, width: int) -> PackedVector2Array:
	var rng = RandomNumberGenerator.new()
	# pick points for the top left of the image first, then reflect the other points?
	var working_size = size / 2
	var edge_one = Vector2(working_size, rng.randi_range(0, working_size - 1))
	var edge_two = Vector2(rng.randi_range(0, working_size - 1), working_size)
	var edge_three = reflect_quadrant_position(edge_one, working_size, true, false)
	var edge_four = reflect_quadrant_position(edge_two, working_size, false, true)
	
	var all_points: Array[Vector2] = [edge_one]
	
	var middle_points = []
	for a in range(num_points):
		middle_points.append(Vector2(
			rng.randi_range(1, working_size - 1), 
			rng.randi_range(1, working_size - 1))
		)
	# topleft quad points
	all_points.append_array(middle_points.duplicate())
	all_points.append(edge_two)
	
	var backward = middle_points.duplicate()
	backward.reverse()
	
	for pt in backward:
		# add the midpoitns backwards reflected to the list
		all_points.append(reflect_quadrant_position(pt, working_size, true, false))
	
	all_points.append(edge_three)
	
	# dont have to reverse for lower left quad
	for pt in middle_points.duplicate():
		all_points.append(reflect_quadrant_position(pt, working_size, true, true))
	all_points.append(edge_four)
	
	for pt in backward:
		# add the midpoitns backwards reflected to the list
		all_points.append(reflect_quadrant_position(pt, working_size, false, true))
	all_points.append(edge_one)
	return PackedVector2Array(all_points)
	
func reflect_quadrant_position(position: Vector2, quadrant_size: int, x: bool = false, y: bool = false) -> Vector2:
	var compare = [x,y]
	match compare:
		[true, false]: return Vector2(position.x, quadrant_size * 2 - position.y)
		[false, true]: return Vector2(quadrant_size * 2 - position.x, position.y)
		[true, true]: return Vector2(quadrant_size * 2 - position.x, quadrant_size * 2 - position.y)
	
	return position
