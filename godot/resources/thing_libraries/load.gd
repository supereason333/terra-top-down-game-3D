extends Node

# this literaly just lazy loads the singletons initialize

func _ready() -> void:
	GlobalSingleton.initialize()
