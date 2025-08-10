extends Node

@onready var world_renderer: WorldRenderer = get_tree().current_scene.get_node("WorldRenderer")
@onready var label_button: Button = $label

func _ready() -> void:
	label_button.text = str(world_renderer.height)
	
func increase() -> void:
	world_renderer.height += 1;
	label_button.text = str(world_renderer.height)

func decrease() -> void:
	world_renderer.height -= 1;
	label_button.text = str(world_renderer.height)
