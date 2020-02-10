require("pokemon_rust.lua.events");
require("pokemon_rust.lua.npc");

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

function on_map_load()
    local square_movement = function(id)
        return CyclicEvent:new(
            ChainedEvents:new({
                NpcRotateEvent:new(id, Directions["right"]),
                NpcMoveEvent:new(id, 4),
                NpcRotateEvent:new(id, Directions["down"]),
                NpcMoveEvent:new(id, 4),
                NpcRotateEvent:new(id, Directions["left"]),
                NpcMoveEvent:new(id, 4),
                NpcRotateEvent:new(id, Directions["up"]),
                NpcMoveEvent:new(id, 4),
            })
        )
    end

    FIRST_NPC = NpcBuilder
        :new("test_map", 30, 30, "example_npc")
        :facing_towards(Directions["right"])
        :event_driven(square_movement)
        :build()

    SECOND_NPC = NpcBuilder
        :new("test_map", 35, 30, "example_npc")
        :facing_towards(Directions["down"])
        :event_driven(square_movement)
        :build()
end

function interact_with_npc(npc)
    -- NpcUtils.rotate_towards_player(npc)
    -- NpcMoveEvent:new(npc, 5):dispatch()

    if npc == FIRST_NPC then
        print("Interacted with the first NPC")
        TextEvent:new("Hello, world!"):dispatch()
    elseif npc == SECOND_NPC then
        print("Interacted with the second NPC")
    end
end
