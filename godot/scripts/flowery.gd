extends Flowery

enum FloweryState {
	FLYING, 
	WALKING, 
	IDLE, 
	AURAFARM,
	FLYTOIDLE,
	CONDESCEND
	}
enum Direction { LEFT, UP, RIGHT, DOWN }

var current_state = FloweryState.IDLE

# preload audio files we might need
var falling = preload("res://voicelines/falling.wav")
var flesh = preload("res://voicelines/flesh.wav")
var sustingus = preload("res://voicelines/sustingus.wav")
var frandisco = preload("res://voicelines/frandisco.wav")

var jarona1 = preload("res://voicelines/jarona1.wav")
var jarona2 = preload("res://voicelines/jarona2.wav")
var jarona3 = preload("res://voicelines/jarona3.wav")
var jarona4 = preload("res://voicelines/jarona4.wav")

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

# run as soon as the game launches
func _ready() -> void:
	velocity = Vector2i(1,0)

	play_animation("Standing")
	screen_size = Vector2(DisplayServer.screen_get_size())
	area.input_event.connect(_on_area_input)

# run whenever you interact with the collision box
func _on_area_input(_viewport, event, _shape_idx):
	if event is InputEventMouseButton and event.button_index == MOUSE_BUTTON_LEFT:
		if event.pressed:
			is_dragging = true
			jarona_voice()
			play_animation("Grabbed")
			var mouse_pos = Vector2(DisplayServer.mouse_get_position())
			var win_pos = Vector2(DisplayServer.window_get_position())
			drag_offset = mouse_pos - win_pos

func _unhandled_input(event):
	if event is InputEventMouseButton:
		if !event.pressed and event.button_index == MOUSE_BUTTON_LEFT and is_dragging:
			play_animation("Walking Forward Right")
			play_line(falling)
			is_dragging = false

# play a specific animation
func play_animation(animation_name: String) -> void:
	sprite.play(animation_name)
	# get the texture
	var texture = sprite.sprite_frames.get_frame_texture(animation_name,0)
	# set window size to size of texture
	DisplayServer.window_set_size(Vector2i(texture.get_width() * sprite.scale.x, texture.get_height() * sprite.scale.y))
	pass

# run every tick
func _physics_process(_delta: float) -> void:
	# if he's being dragged
	if is_dragging:
		# move him
		self.move_to(Vector2(DisplayServer.mouse_get_position()) - drag_offset)
		#self.velocity = Vector2(0,0)
	else:
		if will_collide():
			if sprite.animation != "Condescend":
				play_animation("Condescend")
			
		else:
			play_animation("Walking Forward Right")
			self.move_and_slide()

func play_line(line) -> void:
	var voice_line: AudioStreamPlayer = AudioStreamPlayer.new()
	add_child(voice_line)
	voice_line.stream = line
	voice_line.play()
	await voice_line.finished
	voice_line.queue_free()
	

func jarona_voice() -> void:
	var jarona_line = [jarona1, jarona2, jarona3, jarona4].pick_random()
	play_line(jarona_line)

func is_playing(player: AudioStreamPlayer) -> bool:
	return player.playing
