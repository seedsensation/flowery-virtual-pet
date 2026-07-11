extends Window

var explosion = preload("res://voicelines/explosion.wav")

@export
var skateboard_speed: int = 50

var internal_position = Vector2()
var velocity = Vector2()
var acceleration = Vector2()

var time_since_launch: float = 0
var exploding = false

var launched = false

var starting_position = Vector2()

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	position = Vector2()
	reset()
func reset() -> void:
	visible = false
	$SkateboardSprite.frame = 0
	$SkateboardSprite.visible = true
	$SkateboardSprite.rotation_degrees = 0
	$Explosion.frame = 0
	$Explosion.stop()
	$Explosion.visible = false
	visible = false
	pass # Replace with function body.

func activate(current_position: Vector2):
	visible = true
	var target_point = Vector2(DisplayServer.screen_get_usable_rect().size.x - current_position.x, DisplayServer.screen_get_usable_rect().get_center().y + range(-500,500).pick_random())
	position = current_position
	starting_position = current_position
	velocity = Vector2.from_angle(current_position.angle_to_point(target_point)).normalized() * skateboard_speed
	
	

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	time_since_launch += delta
	if visible:
		$SkateboardSprite.rotation_degrees += 180 * delta
		$SkateboardSprite.play()
		velocity += acceleration
		internal_position += velocity * delta
		position = Vector2i(internal_position)
		
		if time_since_launch >= 1 and !exploding:
			exploding = true
			velocity = Vector2()
			acceleration = Vector2()
			$Explosion.visible = true
			exploding = true
			$AudioStreamPlayer.play()

			$Explosion.play()
			while $Explosion.is_playing() and $SkateboardSprite.visible:
				await $Explosion.frame_changed
				if $Explosion.frame >= 5 and $SkateboardSprite.visible:
					print("Destroying")
					$SkateboardSprite.visible = false
			await $Explosion.animation_finished
			reset()

			


func _on_skateboard_screen_border_collision(up: bool, right: bool, down: bool, left: bool) -> void:
	#if launched and not exploding and visible:
		
	pass # Replace with function body.
