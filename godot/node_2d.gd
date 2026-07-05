extends Node2D

enum FloweryState {
	FLYING, 
	WALKING, 
	IDLE, 
	AURAFARM,
	FLYTOIDLE
	}

var current_state = FloweryState.IDLE

# preload audio files we might need
var falling = preload("res://voicelines/falling.wav")
var flesh = preload("res://voicelines/flesh.wav")
var sustingus = preload("res://voicelines/sustingus.wav")
var frandisco = preload("res://voicelines/frandisco.wav")

# set basic things
var speed = 100
var direction = Vector2(1,0)
var screen_size = Vector2()
var window_size = Vector2(300,300)
var is_dragging = false
var drag_offset = Vector2()
var idle_timer = 0.0
var is_idling = false

# easier to remember definitions of our specific nodes
@onready var sprite = $Sprite
@onready var area = $Sprite/Area
@onready var voice = $Voice

# run as soon as the game launches
func _ready() -> void:
	screen_size = Vector2(DisplayServer.screen_get_size())
	area.input_event.connect(_on_area_input)

# run whenever you interact with the collision box
func _on_area_input(_viewport, event, _shape_idx):
	if event is InputEventMouseButton and event.button_index == MOUSE_BUTTON_LEFT:
		if event.pressed:
			is_dragging = true
			play_animation("Grabbed")
			var mouse_pos = Vector2(DisplayServer.mouse_get_position())
			var win_pos = Vector2(DisplayServer.window_get_position())
			drag_offset = mouse_pos - win_pos
		else:
			play_animation("Standing")
			play_line(sustingus)
			is_dragging = false

# play a specific animation
func play_animation(name: String) -> void:
	sprite.play(name)
	# get the texture
	var texture = sprite.sprite_frames.get_frame_texture(name,0)
	# set window size to size of texture
	DisplayServer.window_set_size(Vector2i(texture.get_width() * sprite.scale.x, texture.get_height() * sprite.scale.y))
	pass

# run every tick
func _physics_process(_delta: float) -> void:
	# if he's being dragged
	if is_dragging:
		# move him
		var mouse_pos = Vector2(DisplayServer.mouse_get_position())
		var new_win_pos = mouse_pos - drag_offset
		DisplayServer.window_set_position(Vector2i(new_win_pos))

	pass
	
func play_line(line) -> void:
	if !voice.is_playing():
		voice.stream = line
		voice.play()
