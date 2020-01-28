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

function interact_with_npc(x, y)
    print("NPC interaction: (" .. x .. ", " .. y .. ")")
end
