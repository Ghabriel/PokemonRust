Event = {}
Event.__index = Event

function Event:new()
    local obj = { 0 }
    setmetatable(obj, self)
    return obj
end

function Event:dispatch()
    rust_dispatch_event(self[1])
end


TextEvent = Event:new()
TextEvent.__index = TextEvent

function TextEvent:new(text)
    local obj = { rust_create_text_event(text) }
    setmetatable(obj, self)
    return obj
end


WarpEvent = Event:new()
WarpEvent.__index = WarpEvent

function WarpEvent:new(map, x, y)
    local obj = { rust_create_warp_event(map, x, y) }
    setmetatable(obj, self)
    return obj
end


ChainedEvents = Event:new()
ChainedEvents.__index = ChainedEvents

function ChainedEvents:new(events)
    local chain = rust_create_chained_event()

    for _, event in pairs(events) do
        rust_add_event(chain, event[1])
    end

    local obj = { chain }
    setmetatable(obj, self)
    return obj
end


function do_it()
    ChainedEvents:new({
        TextEvent:new("Hello, world!"),
        WarpEvent:new("test_map", 10, 10)
    }):dispatch()
end
