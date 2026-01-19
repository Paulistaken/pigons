extends Node3D

@export var spawn_node_out : Node3D
@export var spawn_node_in : Node3D
@export var human_node_out : PackedScene
@export var human_node_in : PackedScene
@export var bus_label : Label3D

func get_event(path : String) -> DataOutput:
	var file = FileAccess.open(path, FileAccess.READ)
	var data = file.get_as_text()
	var data_json = JSON.parse_string(data)
	
	print(data_json)

	var dataout = DataOutput.new()
	
	var date_of_event = data_json.date_of_event
	dataout.year = int(date_of_event.year)
	dataout.month = int(date_of_event.month)
	dataout.day = int(date_of_event.day)
	
	var time_of_event = data_json.time_of_event
	dataout.hour = int(time_of_event.hour)
	dataout.minute = int(time_of_event.minute)
	
	dataout.bus = int(data_json.id_of_the_bus.id_number)
	dataout.station = int(data_json.id_of_the_station.id_number)
	
	dataout.p_in = int(data_json.pasangers_coming_in)
	dataout.p_out = int(data_json.pasangers_coming_out)
	
	return dataout

var datain : DataOutput

func _init() -> void:
	var data = get_event("../simulation/simresults/pre/json/BusEVENTy2026m2d4h14m11b6s12.json")
	datain = data
	print(data)
	
var frame_a = 0.
var frame_c = 0.

func _process(delta: float) -> void:
	frame_a += delta
	if frame_a >= frame_c + 2. and frame_c < 200:
		frame_c += 1.
		frame_a = frame_c
		var img = get_viewport().get_texture().get_image()
		var path = "vid/e"+str(int(frame_c))+".png"
		img.save_png(path)


func _on_spawntimer_timeout() -> void:
	bus_label.text = str(datain.bus)
	print(datain.p_in)
	print(datain.p_out)
	if datain.p_out > 0:
		datain.p_out -= 1
		var obj = human_node_out.instantiate()
		obj.position = spawn_node_out.position
		add_child(obj)
		return
	if datain.p_in > 0:
		datain.p_in -= 1
		var obj = human_node_in.instantiate()
		obj.position = spawn_node_in.position
		add_child(obj)
