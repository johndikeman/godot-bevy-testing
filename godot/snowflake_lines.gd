class_name SnowflakeLines extends Sprite2D

var _points: Array
var color: Color

func set_points(points: Array):
	_points = points
	queue_redraw()

func set_color(c: Color):
	color = c

func _draw():
	if !_points:
		return
		
	# update the sprite texture with the newly completed image
	draw_polyline(_points, color, 1)

	
