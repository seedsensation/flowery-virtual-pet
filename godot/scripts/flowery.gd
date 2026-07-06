extends Flowery

enum Status { IDLE, FALLING, FLYING }


# preload audio files we might need
var falling = preload("res://voicelines/falling.wav")
var flesh = preload("res://voicelines/flesh.wav")
var sustingus = preload("res://voicelines/sustingus.wav")
var frandisco = preload("res://voicelines/frandisco.wav")
var forget_it = preload("res://voicelines/forget_it.wav")
var mysterious_wind = preload("res://voicelines/mysterious_wind.wav")

var jarona1 = preload("res://voicelines/jarona1.wav")
var jarona2 = preload("res://voicelines/jarona2.wav")
var jarona3 = preload("res://voicelines/jarona3.wav")
var jarona4 = preload("res://voicelines/jarona4.wav")

var status: Status = Status.IDLE

# set basic things
var is_dragging = false
var drag_offset = Vector2()

var time_of_next_action: int = range(10,20).pick_random()

# easier to remember definitions of our specific nodes
@onready var sprite = $Sprite
@onready var area = $Sprite/Area

# run as soon as the game launches
func _ready() -> void:
	velocity = Vector2i(0,0)
	acceleration = Vector2(0,9.81)

	play_animation("Standing")
	area.input_event.connect(_on_area_input)

# run whenever you interact with the collision box
func _on_area_input(_viewport, event, _shape_idx):
	if event is InputEventMouseButton and event.button_index == MOUSE_BUTTON_LEFT:
		if event.pressed:
			is_dragging = true
			acceleration = Vector2()
			play_animation("Grabbed")
			velocity = Vector2()
			jarona_voice()
			var mouse_pos = Vector2(DisplayServer.mouse_get_position())
			var win_pos = Vector2(DisplayServer.window_get_position())
			drag_offset = mouse_pos - win_pos

func _unhandled_input(event):
	if event is InputEventMouseButton:
		if !event.pressed and event.button_index == MOUSE_BUTTON_LEFT and is_dragging:
			velocity = Vector2(0,0)
			acceleration = Vector2(0,9.81)
			play_animation("Fall")
			is_dragging = false
			status = Status.FALLING
			await sprite.animation_finished;
			play_animation("Standing")

# play a specific animation
func play_animation(animation_name: String) -> void:
	sprite.play(animation_name)
	readjust_size()

# readjust the window size based on the current animation
func readjust_size() -> void:
	var texture = sprite.sprite_frames.get_frame_texture(sprite.animation,0)
	# set window size to size of texture
	DisplayServer.window_set_size(Vector2i(texture.get_width() * sprite.scale.x, texture.get_height() * sprite.scale.y))

# run every tick
func _physics_process(delta: float) -> void:
	self.velocity += self.acceleration * delta
	# if he's being dragged
	if is_dragging:
		# move him
		self.move_to(Vector2(DisplayServer.mouse_get_position()) - drag_offset)
		#self.velocity = Vector2(0,0)
	else:
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


func _on_screen_border_collision() -> void:
	if status == Status.FALLING:
		status = Status.IDLE
		velocity = Vector2()
		acceleration = Vector2()
		play_animation("Condescend")
		play_line(forget_it)
