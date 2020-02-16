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


BgmChangeEvent = Event:new()
BgmChangeEvent.__index = BgmChangeEvent

function BgmChangeEvent:new(filename)
    local obj = { rust_create_bgm_change_event(filename) }
    setmetatable(obj, self)
    return obj
end

function BgmChangeEvent.preload(filename)
    rust_preload_bgm(filename)
end


CyclicEvent = Event:new()
CyclicEvent.__index = CyclicEvent

function CyclicEvent:new(event)
    local obj = { rust_create_cyclic_event(event[1]) }
    setmetatable(obj, self)
    return obj
end


NpcMoveEvent = Event:new()
NpcMoveEvent.__index = NpcMoveEvent

function NpcMoveEvent:new(npc, num_tiles)
    local obj = { rust_create_npc_move_event(npc, num_tiles) }
    setmetatable(obj, self)
    return obj
end


NpcRotateEvent = Event:new()
NpcRotateEvent.__index = NpcRotateEvent

function NpcRotateEvent:new(npc, direction)
    local obj = { rust_create_npc_rotate_event(npc, direction) }
    setmetatable(obj, self)
    return obj
end

function NpcRotateEvent:towards_player(npc)
    local obj = { rust_create_npc_rotate_towards_player_event(npc) }
    setmetatable(obj, self)
    return obj
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
