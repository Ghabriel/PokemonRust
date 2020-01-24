TextEvent = {}
TextEvent.__index = TextEvent

function TextEvent:new(text)
    local obj = { rust_create_text_event(text) }
    setmetatable(obj, self)
    return obj
end

function TextEvent:dispatch()
    rust_dispatch_event(self[1])
end


WarpEvent = {}
WarpEvent.__index = WarpEvent

function WarpEvent:new(map, x, y)
    local obj = { rust_create_warp_event(map, x, y) }
    setmetatable(obj, self)
    return obj
end

function WarpEvent:dispatch()
    rust_dispatch_event(self[1])
end


ChainedEvents = {}
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

function ChainedEvents:dispatch()
    rust_dispatch_event(self[1])
end


function chained_events(events)
    local chain = rust_create_chained_event()

    for _, event in pairs(events) do
        rust_add_event(chain, event[1])
    end

    rust_dispatch_event(chain)
end

function do_it()
    ChainedEvents:new({
        TextEvent:new("Hello, world!"),
        WarpEvent:new("test_map", 10, 10)
    }):dispatch()
end
