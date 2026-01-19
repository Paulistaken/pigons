extends Node3D

@export var going_dir : float = 1.
@export var speed : float = 1.

func _process(delta: float) -> void:
	position.z += speed * going_dir * delta


func _on_timer_timeout() -> void:
	queue_free()
