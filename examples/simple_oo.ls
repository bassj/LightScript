class Emitter {
    // Properties of an object prefixed with an underscore can only be accessed by the this keyword. 
    prop _handlers = {}; // Property of the class.

    let _type = "Emitter"; // Static field.

    func on(event_name, event_handler) {
        if event_name in this._hanlders {
            this._handlers[event_name] += event_handler;
        }
    }

    func _emit(event_name, event_handler) {
        if event_name in this._handlers {
            this._handlers[event_name]();
        }
    }

}