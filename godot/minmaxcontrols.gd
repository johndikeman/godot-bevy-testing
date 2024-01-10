extends VBoxContainer

@export var min_slider: Slider
@export var max_slider: Slider
@export var min_label: RichTextLabel
@export var max_label: RichTextLabel

@export var wind_label: RichTextLabel
@export var wind_slider: Slider

@export var grav_label: RichTextLabel
@export var grav_slider: Slider

@export var bevy: BevyECS

var _bevy_default_min = 0.1
var _bevy_default_max = 0.5

# Called when the node enters the scene tree for the first time.
func _ready():
	min_slider.value = _bevy_default_min
	max_slider.value = _bevy_default_max
	min_slider.drag_ended.connect(update_value)
	max_slider.drag_ended.connect(update_value)

	wind_slider.drag_ended.connect(update_wind)
	grav_slider.drag_ended.connect(update_grav)

func update_value(_val_changed: bool):
	min_label.text = "min flake interval: " + str(min_slider.value)
	max_label.text = "max flake interval: " + str(max_slider.value)

	bevy.edit_snowflake_timer_params(min_slider.value, max_slider.value)
	

func update_wind(_val_changed: bool):
	wind_label.text = "wind: " + str(wind_slider.value)
	bevy.edit_wind(wind_slider.value)

func update_grav(_val_cahnged:bool):
	grav_label.text = "gravity: " + str(grav_slider.value)
	bevy.edit_gravity(grav_slider.value)

