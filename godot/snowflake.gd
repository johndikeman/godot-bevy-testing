extends Sprite2D

@export var size: int
@export var color: Color
@export var num_points: int
var image: Image
var _selected_points: Array
# dg/s ?
var _rotate_speed: float
var _velocity_y: float
var _velocity_x: float
var _gravity: float = 10


func _ready():
	var rng = RandomNumberGenerator.new()
	# create the image texture we need
	_selected_points = select_points(size, size)
	_velocity_y = rng.randf_range(1,.5)
	_velocity_x = 0
	
	_rotate_speed = rng.randf_range(-1,1)
	var s = rng.randf_range(1,5)
	self.scale = Vector2(s,s)
	
func _draw():
	if !_selected_points:
		return
		
	# update the sprite texture with the newly completed image
	self.draw_polyline(_selected_points, color, 1)
	

func _process(delta: float):
	self.set_rotation_degrees(self.rotation_degrees + _rotate_speed * 10 * delta)
	self.set_position(self.get_position() + Vector2(_velocity_x * delta, _velocity_y * delta))
	_velocity_y += _gravity * delta

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
