extends Node3D

func _on_mouse_tracker_position_changed(pos: Vector2) -> void:
	rotation.y = -pos.angle() - PI / 2
