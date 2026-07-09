extends Flowery

enum Status { IDLE, FALLING, FLYING, OUT_OF_BOUNDS, MID_ANIMATION, GRABBED}

const IGNORE_FLIP_WHEN = [Status.OUT_OF_BOUNDS, Status.GRABBED]

const gravity = Vector2(0, 1281)
const walking_speed = 100

# preload audio files we might need
# misc lines
var flesh = preload("res://voicelines/flesh.wav")
var sustingus = preload("res://voicelines/sustingus.wav")
var frandisco = preload("res://voicelines/frandisco.wav")
var mysterious_wind = preload("res://voicelines/mysterious_wind.wav")
var mini_peppers = preload("res://voicelines/mini_peppers.wav")
# mid-falling
var falling = preload("res://voicelines/falling.wav")
var great_style = preload("res://voicelines/great_style.wav")
# landing
var forget_it = preload("res://voicelines/forget_it.wav")
var get_a_chance_1 = preload("res://voicelines/get_a_chance_1.wav")
var get_a_chance_2 = preload("res://voicelines/get_a_chance_2.wav")
var hoo = preload("res://voicelines/hoo.wav")
var huh = preload("res://voicelines/huh.wav")
# kept you waiting
var sorry_to_keep_you_waiting = preload("res://voicelines/sorry_to_keep_you_waiting.wav")
var sorry_to_keep_you_waiting_2 = preload("res://voicelines/sorry_to_keep_you_waiting_2.wav")
var sorry_again = preload("res://voicelines/sorry_again.wav")
var sorry_king = preload("res://voicelines/sorry_king.wav")
var sorry_again_king = preload("res://voicelines/sorry_again_king.wav")
var sorry_lady = preload("res://voicelines/sorry_lady.wav")
var sorry_again_lady = preload("res://voicelines/sorry_again_lady.wav")
# my x
var my_human = preload("res://voicelines/my_human.wav")
var my_lady = preload("res://voicelines/my_lady.wav")
var my_king = preload("res://voicelines/my_king.wav")
# jarona
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
var said_falling = false
var kept_you_waiting = false

@export
var lady = true
@export
var king = true

var drag_offset = Vector2()
var fall_variant = 1
var land_variant = 1

var time_of_next_action: int = range(10,20).pick_random()
@export
var animation_offsets = {
	"Fall" = Vector2(-10,1),
	"L Fall" = Vector2(0,1),
	"Standing" = Vector2(0,2),
	"L Standing" = Vector2(2,2),
	"Condescend" = Vector2(4,0),
	"Grabbed" = Vector2(0,6),
	"Walking" = Vector2(4,2),
	"Crouch" = Vector2(0, 12),
	"L Crouch" = Vector2(0, 12)
	
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
			said_falling = false
			if velocity.length() > 1500:
				play_line(mini_peppers, true)
			else:
				var grab_line = [1,2].pick_random()
				match grab_line:
					1:
						play_line(jarona_voice(), true)
					2:
						var myGuy = [my_human]
						if king:
							myGuy.append(my_king)
						if lady:
							myGuy.append(my_lady)
						play_line(myGuy.pick_random(), true)
			acceleration = Vector2()
			play_animation("Grabbed")
			velocity = Vector2()
			var mouse_pos = Vector2(DisplayServer.mouse_get_position())
			var win_pos = Vector2(DisplayServer.window_get_position())
			drag_offset = mouse_pos - win_pos

# drop him when you let go
func _unhandled_input(event):
	if event is InputEventMouseButton:
		if !event.pressed and event.button_index == MOUSE_BUTTON_LEFT and status == Status.GRABBED:
			velocity = Vector2(DisplayServer.mouse_get_position() - last_mouse_pos) * Vector2(25, 25)
			acceleration = gravity
			status = Status.FALLING
			if !touching_bottom_side():
				play_fall_animation()
			idle_timer = 0
			is_dragging = false

func play_fall_animation() -> void:
	fall_variant = [1, 2].pick_random()
	if fall_variant == 1:
		play_animation("Fall")
	else:
		play_animation("Fall Spinning")
		if velocity.x < 0:
			sprite.play_backwards()


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
	elif status == Status.FALLING and fall_variant == 2:
		if velocity.x >= 0:
			sprite.play()
		elif velocity.x < 0:
			sprite.play_backwards()
	if sprite.animation.begins_with("L ") and velocity.x >= 0:
		sprite.animation = sprite.animation.remove_chars("L ")
	elif !sprite.animation.begins_with("L ") and velocity.x < 0 and "L "+sprite.animation in sprite.sprite_frames.get_animation_names():
		sprite.animation = "L "+sprite.animation
		

func set_offset() -> void:
	if sprite.animation in animation_offsets.keys():
		print("Adjusting offset to ", animation_offsets[sprite.animation], " for ", sprite.animation)
		sprite.offset = animation_offsets[sprite.animation]
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
			
		if !frozen:
			self.move_and_slide(delta)

func decide_next_action() -> void:
	pass

func return_from_offscreen() -> void:
	velocity = Vector2()
	acceleration = Vector2()
	status = Status.MID_ANIMATION

	move_to_taskbar()
	var size = get_shape().size
	
	await get_tree().process_frame

	move_to(Vector2i(0 - size.x - 100, int(get_position().y)))

	await get_tree().process_frame
	await get_tree().create_timer(2).timeout
	
	play_animation("Walking")

	
	print(get_shape().position)
	acceleration = Vector2()
	velocity = Vector2(100,0)
	await get_tree().create_timer(2).timeout
	waiting_voice()
	await get_tree().create_timer(2).timeout
	velocity = Vector2()
	status = Status.IDLE
	said_falling = false
	play_animation("Standing")
	idle_timer = 0
	
#Handles logic for which 'waiting' voice clips to play
func waiting_voice() -> void:
	var waiting_line_options = [sorry_to_keep_you_waiting, sorry_to_keep_you_waiting_2]
	if king:
		waiting_line_options.append(sorry_king)
		if kept_you_waiting:
			waiting_line_options.append(sorry_again_king)
	if lady:
		waiting_line_options.append(sorry_lady)
		if kept_you_waiting:
			waiting_line_options.append(sorry_again_lady)
	if kept_you_waiting:
		waiting_line_options.append(sorry_again)
	kept_you_waiting = true
	var waiting_line = waiting_line_options.pick_random()
	if waiting_line in [sorry_again_king, sorry_again_lady, sorry_again]:
		kept_you_waiting = false
	play_line(waiting_line, true)

func play_line(line, override_playing: bool = false) -> void:
	var voice_line: AudioStreamPlayer = AudioStreamPlayer.new()
	if !(get_children().filter(func(x: Node): return x is AudioStreamPlayer and x.playing).size() > 0) or override_playing:
		add_child(voice_line)
		voice_line.stream = line
		voice_line.play()
		await voice_line.finished
		voice_line.queue_free()

func jarona_voice() -> Resource:
	return [jarona1, jarona2, jarona3, jarona4].pick_random()

func move_to_taskbar() -> void:
	play_animation("Standing")
	await get_tree().process_frame
	move_to(Vector2i(int(get_shape().position.x), DisplayServer.screen_get_usable_rect().end.y - int(get_shape().size.y)))


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
			#velocity = Vector2()
			move_to_taskbar()
			await get_tree().process_frame
			#land_variant = [1,2].pick_random()
			match fall_variant:
				1:
					play_animation("Crouch")
					velocity = Vector2()
					var randomLine = [hoo, huh].pick_random()
					play_line(randomLine)
					await get_tree().create_timer(0.5).timeout
					
				2:
					play_animation("Condescend")
					velocity = Vector2()
					var randomLine = [forget_it, get_a_chance_1, get_a_chance_2].pick_random()
					play_line(randomLine)
					await sprite.animation_finished
			idle_timer = 0
			status = Status.IDLE
			said_falling = false
			play_animation("Standing")
		
	if (right or left) and status != Status.MID_ANIMATION and !up:
		velocity = velocity * Vector2(-1,1)
		check_animation_swap()
	elif right or left and status != Status.MID_ANIMATION and up:
		return_from_offscreen()
		

func _on_action_timer_timeout() -> void:
	if status == Status.FALLING and idle_timer > 0.3 and velocity.y >= 0 and !said_falling:
		if fall_variant == 1:
			play_line(falling)
		else:
			play_line(great_style)
		said_falling = true
			
	if range(200).pick_random() == 5:
		play_line([flesh, sustingus, mysterious_wind].pick_random())
	if idle_timer > 20:
		decide_next_action()
		
	pass # Replace with function body.
