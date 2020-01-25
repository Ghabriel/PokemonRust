require("pokemon_rust.lua.events");

function enter_cave()
    WarpEvent:new("test_map3", 5, 10):dispatch()
end
