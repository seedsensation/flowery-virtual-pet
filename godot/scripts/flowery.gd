extends Flowery

enum Status { IDLE, FALLING, FLYING, OUT_OF_BOUNDS, MID_ANIMATION, GRABBED}

const IGNORE_FLIP_WHEN = [Status.OUT_OF_BOUNDS, Status.GRABBED]

const gravity = Vector2(0, 981)
const walking_speed = 100

# preload audio files we might need
var falling = preload("res://voicelines/falling.wav")
var flesh = preload("res://voicelines/flesh.wav")
var sustingus = preload("res://voicelines/sustingus.wav")
var frandisco = preload("res://voicelines/frandisco.wav")
var forget_it = preload("res://voicelines/forget_it.wav")
var mysterious_wind = preload("res://voicelines/mysterious_wind.wav")
var mini_peppers = preload("res://voicelines/mini_peppers.wav")
var sorry_to_keep_you_waiting = preload("res://voicelines/sorry_to_keep_you_waiting.wav")

var jarona1 = preload("res://voicelines/jarona1.wav")
var jarona2 = preload("res://voicelines/jarona2.wav")
var jarona3 = preload("res://voicelines/jarona3.wav")
var jarona4 = preload("res://voicelines/jarona4.wav")

var status: Status = Status.FALLING
var idle_timer: float = 0
var last_mouse_pos = Vector2()

# set basic things
var facing_left = false
var is_dragging = false
var frozen: bool = false
var drag_offset = Vector2()

var time_of_next_action: int = range(10,20).pick_random()
@export
var animations = {
	"Fall" = Vector2(-10,1),
	"L Fall" = Vector2(0,1),
	"Standing" = Vector2(0,6),
	"L Standing" = Vector2(2,6),
	"Condescend" = Vector2(3,1),
	"Grabbed" = Vector2(0,6)
	
}

# run as soon as the game launches
func _ready() -> void:
	area = $Sprite/Area
	sprite = $Sprite
	velocity = Vector2i(0,0)
	acceleration = gravity
	play_animation("Standing")
	area.input_event.connect(_on_area_input)

# run whenever you interact with the collision box
func _on_area_input(_viewport, event, _shape_idx):
	if event is InputEventMouseButton and event.button_index == MOUSE_BUTTON_LEFT:
		facing_left = false
		if event.pressed:
			status = Status.GRABBED
			is_dragging = true
			if velocity.length() > 1500:
				play_line(mini_peppers)
			else:
				jarona_voice()
			acceleration = Vector2()
			play_animation("Grabbed")
			velocity = Vector2()
			var mouse_pos = Vector2(DisplayServer.mouse_get_position())
			var win_pos = Vector2(DisplayServer.window_get_position())
			drag_offset = mouse_pos - win_pos

func _unhandled_input(event):
	if event is InputEventMouseButton:
		if !event.pressed and event.button_index == MOUSE_BUTTON_LEFT and status == Status.GRABBED:
			velocity = Vector2(DisplayServer.mouse_get_position() - last_mouse_pos) * Vector2(50, 45)
			acceleration = gravity
			status = Status.FALLING
			if !touching_bottom_side():
				play_animation("Fall")
			idle_timer = 0
			is_dragging = false


# play a specific animation
func play_animation(animation_name: String) -> void:
	if (velocity.x < 0 or facing_left) and not status in IGNORE_FLIP_WHEN and "L "+animation_name in sprite.sprite_frames.get_animation_names():
		sprite.play("L "+animation_name)
	else:
		sprite.play(animation_name)
	set_offset()
	readjust_window_size()
	
func check_animation_swap():
	if status in IGNORE_FLIP_WHEN:
		return
	if sprite.animation.begins_with("L ") and velocity.x >= 0:
		sprite.animation = sprite.animation.remove_chars("L ")
	elif !sprite.animation.begins_with("L ") and velocity.x < 0:
		sprite.animation = "L "+sprite.animation
		

func set_offset() -> void:
	if sprite.animation in animations.keys():
		print("Adjusting offset to ", animations[sprite.animation], " for ", sprite.animation)
		sprite.offset = animations[sprite.animation]
	else:
		print("Resetting offset for ",sprite.animation)
		sprite.offset = Vector2()
	


# run every tick
func _physics_process(delta: float) -> void:
	self.velocity += self.acceleration * delta
	idle_timer += delta
	# if he's being dragged
	if status == Status.GRABBED:
		# move him
		self.move_to(Vector2(DisplayServer.mouse_get_position()) - drag_offset)
		last_mouse_pos = DisplayServer.mouse_get_position()

	else:
		if touching_top_side() and idle_timer > 4 and status == Status.FALLING:

			return_from_offscreen()
			

		elif status == Status.FALLING and idle_timer > 2 and velocity.y >= 0:
			idle_timer = 0
			play_line(falling)
		if !frozen:
			self.move_and_slide(delta)

func decide_next_action() -> void:
	pass

func return_from_offscreen() -> void:
	velocity = Vector2()
	acceleration = Vector2()
	status = Status.MID_ANIMATION
	await get_tree().create_timer(2).timeout
	var size = get_shape().size
	move_to(Vector2i(0 - size.x, DisplayServer.screen_get_usable_rect().end.y - size.y))
	print(get_shape().position)
	play_animation("Walking")
	acceleration = Vector2()
	velocity = Vector2(100,0)
	await get_tree().create_timer(2).timeout
	play_line(sorry_to_keep_you_waiting)
	await get_tree().create_timer(2).timeout
	velocity = Vector2()
	status = Status.IDLE
	play_animation("Standing")
	idle_timer = 0

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


func _on_screen_border_collision(up: bool, right: bool, down: bool, left: bool) -> void:
	if down and status == Status.FALLING:
		if velocity.length() > 300 and velocity.normalized().y > 0:
			# bounce off the bottom of the screen
			velocity = velocity * Vector2(0.75, -0.5)
		# for some reason he kept clipping offscreen
		# so this is a catcher so that if he ends up out of bounds, we're able
		# to find him
		elif DisplayServer.screen_get_usable_rect().intersects(self.get_shape()):
			status = Status.MID_ANIMATION
			facing_left = velocity.x < 0
			acceleration = Vector2()
			velocity = Vector2()
			play_animation("Condescend")
			# not entirely sure why but 
			# we need to wait for the next tick
			# in order for it to work
			await get_tree().process_frame
			
			move_to(Vector2i(get_shape().position.x, DisplayServer.screen_get_usable_rect().end.y - get_shape().size.y))





			velocity = Vector2()

			
			play_line(forget_it)
			
			await sprite.animation_finished
			idle_timer = 0
			status = Status.IDLE

			play_animation("Standing")
		
	if right or left and status != Status.MID_ANIMATION:
		velocity = velocity * Vector2(-1,1)
		check_animation_swap()


func _on_action_timer_timeout() -> void:
	if idle_timer > 20:
		decide_next_action()
		
	pass # Replace with function body.
