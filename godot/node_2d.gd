extends Node2D

enum FloweryState {
	FLYING, 
	WALKING, 
	IDLE, 
	AURAFARM,
	FLYTOIDLE
	}

var current_state = FloweryState.IDLE

var falling = preload("res://voicelines/falling.wav")
var flesh = preload("res://voicelines/flesh.wav")
var sustingus = preload("res://voicelines/sustingus.wav")
var frandisco = preload("res://voicelines/frandisco.wav")

var speed_x = 0.0
var speed_y = 0.0

var pause_for_animation: bool = false

func _ready() -> void:
	pause_for_animation = false
	make_fly()
	pass
	

func _process(delta: float) -> void:
	# movement stuff goes here
	
	self.position + Vector2(speed_x, speed_y) * delta
	
	# everything past this point will only play
	# after the current animation is finished
	if pause_for_animation:
		if !$Flowery/Sprite.is_playing():
			pause_for_animation = false
			match current_state:
				FloweryState.FLYING:
					speed_y = -100
					make_idle_from_fly()
				FloweryState.FLYTOIDLE:
					speed_y = 0
					$Flowery/Sprite.offset = Vector2(17,3)
					make_idle()
		else:
			return
	

	
	pass
	
func make_fly() -> void:
	play_line(falling)
	$Flowery/Sprite.animation = "Fly"
	$Flowery/Sprite.play()
	pause_for_animation = true
	current_state = FloweryState.FLYING
	
func make_idle_from_fly() -> void:
	await get_tree().create_timer(2).timeout

	$Flowery/Sprite.animation = "Fly To Idle"
	$Flowery/Sprite.play()
	pause_for_animation = true
	current_state = FloweryState.FLYTOIDLE

func make_idle() -> void:
	$Flowery/Sprite.animation = "Idle Aurafarm"
	$Flowery/Sprite.play()
	current_state = FloweryState.IDLE

func play_line(line) -> void:
	if !$Flowery/Voice.is_playing():
		$Flowery/Voice.stream = line
		$Flowery/Voice.play()
	
