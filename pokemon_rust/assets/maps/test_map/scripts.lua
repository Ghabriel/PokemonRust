require("pokemon_rust.lua.events");

function on_map_enter()
    print("Welcome to Test Map!")
end

function do_it()
    ChainedEvents:new({
        TextEvent:new("Hello, world!"),
        WarpEvent:new("test_map", 10, 10)
    }):dispatch()
end

function interact_with_tree()
    TextEvent:new(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
    ):dispatch()
end

Directions = {
    up = 0,
    down = 1,
    left = 2,
    right = 3,
}

NpcBuilder = {}
NpcBuilder.__index = NpcBuilder

function NpcBuilder:new(x, y, kind)
    local obj = { rust_create_npc(x, y, kind, Directions["down"]) }
    setmetatable(obj, self)
    return obj
end

function NpcBuilder:facing_towards(direction)
    rust_change_npc_direction(self[1], direction)
    return self
end

function NpcBuilder:build()
    return rust_add_npc(self[1])
end

function on_map_load()
    NpcBuilder
        :new(30, 30, "example_npc")
        :facing_towards(Directions["right"])
        :build()

    NpcBuilder
        :new(34, 30, "example_npc2")
        :facing_towards(Directions["right"])
        :build()
end
