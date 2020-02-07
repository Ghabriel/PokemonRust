Directions = {
    up = 0,
    down = 1,
    left = 2,
    right = 3,
}

NpcBuilder = {}
NpcBuilder.__index = NpcBuilder

function NpcBuilder:new(map, x, y, kind)
    local obj = { rust_create_npc(map, x, y, kind, Directions["down"]) }
    setmetatable(obj, self)
    return obj
end

function NpcBuilder:facing_towards(direction)
    rust_change_npc_direction(self[1], direction)
    return self
end

function NpcBuilder:event_driven(event_generator)
    self["event_generator"] = event_generator
    return self
end

function NpcBuilder:build()
    local character_id = rust_add_npc(self[1])

    if self["event_generator"] ~= nil then
        self["event_generator"](character_id):dispatch()
    end

    return character_id
end

NpcUtils = {}

function NpcUtils.rotate_towards_player(npc)
    rust_rotate_npc_towards_player(npc)
end
