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

var speed = 100
var direction = Vector2(1,0)
var screen_size = Vector2()
var window_size = Vector2(300,300)
var is_dragging = false
var drag_offset = Vector2()
var idle_timer = 0.0
var is_idling = false

var target_speed = Vector2(0.0, 0.0)

func _ready() -> void:
	pass
	

func _process(_delta: float) -> void:
	# movement stuff goes here
	

	pass
