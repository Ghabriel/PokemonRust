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
    FIRST_NPC = NpcBuilder
        :new("test_map", 30, 30, "example_npc")
        :facing_towards(Directions["right"])
        :build()

    SECOND_NPC = NpcBuilder
        :new("test_map", 34, 30, "example_npc2")
        :facing_towards(Directions["right"])
        :build()
end

function interact_with_npc(npc)
    if npc == FIRST_NPC then
        print("Interacted with the first NPC")
    elseif npc == SECOND_NPC then
        print("Interacted with the second NPC")
    end
end
