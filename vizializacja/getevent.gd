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
var cpath = ""
var events : Array[String]

func _init() -> void:
	var events_folder = DirAccess.open("../simulation/simresults/pre/json/")
	events_folder.list_dir_begin()
	var flpath = events_folder.get_next()
	print(flpath)
	while flpath != "":
		print(flpath)
		if events_folder.current_is_dir():
			flpath = events_folder.get_next()
			continue
		events.append("../simulation/simresults/pre/json/"+flpath)
		flpath = events_folder.get_next()
	events_folder.list_dir_end()
	if not events.is_empty():
		cpath = events.pop_back()
		datain = get_event(cpath)
		print(datain)
	
var frame_a = 0.
var frame_c = 0.

func _process(delta: float) -> void:
	frame_a += delta
	if frame_a >= frame_c + 2. and frame_c < 200:
		frame_c += 1.
		frame_a = frame_c
		var img = get_viewport().get_texture().get_image()
		var path = "b"+str(datain.bus)+"s"+str(datain.station)+"vid/e"+str(int(frame_c))+".png"
		img.save_png(path)

var freetiks = 10
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
		return
	if freetiks > 0:
		freetiks -= 1
		return
	freetiks = 10
	if not events.is_empty():
		cpath = events.pop_back()
		datain = get_event(cpath)
		print(datain)
