require("pokemon_rust.lua.events");

function on_map_enter()
    BgmChangeEvent:new("route228.mp3"):dispatch()
    print("Welcome to Test Map 2!")
end

function enter_cave()
    WarpEvent:new("test_map3", 5, 10):dispatch()
end
