initSidebarItems({"mod":[["bgm_change_event","Changes the current background music."],["chained_events","Generic event. Chains multiple events so that they happen sequentially."],["character_move_event","Moves a character forward for a given number of tiles. See the CharacterMovementSystem for details on which situations this event can \"hang\"."],["character_rotate_event","Rotates a character towards a direction."],["character_single_move_event","Moves a character one tile forward. See the CharacterMovementSystem for details on which situations this event can \"hang\"."],["cyclic_event","Generic event. Repeats an event infinitely and sequentially."],["event_executor","An event executor. This is stored inside each state and is responsible for executing events in parallel while handling the addition of new events, typically coming from the Event Queue."],["event_queue","A simple event queue. This is a resource that stays inside the World and is used to store events before they're processed by the game state."],["fade_in_event","Undoes a fade out, revealing the contents of the screen beneath it. Affected by `GameConfig::fade_duration`."],["fade_out_event","Hides the contents of the screen with an animation. Affected by `GameConfig::fade_duration`."],["map_change_event","Announces a map change, displaying the name of the reached map."],["map_interaction_event","Signals the game that the interaction button was pressed. If there's an interaction event on the tile in front of the player, it is added to the Event Queue."],["parallel_events","Generic event. Orchestrates multiple events so that they happen in parallel."],["repeated_event","Generic event. Repeats an event for a given number of times sequentially."],["script_event","Runs a `GameScript` from the script repository of a map, given its corresponding `MapId` and the index of the script."],["switch_map_event","Immediately switches the map that the player is in, without any animations. To fade the screen while the switch is happening, use a `WarpEvent`. This event only finishes when the target map finishes loading."],["text_event","Displays a text box. Automatically wraps lines and splits the text in pages if needed. Affected by `GameConfig::text_delay`."],["warp_event","Fades out the screen, switches the location of the player and then fades the screen back in. This is the preferred event for doing any kind of warp between maps. The fade in will only occur when the map finishes loading."]],"struct":[["ExecutionConditions","Represents the conditions that a `GameEvent` must fulfill in order to be executed."]],"trait":[["GameEvent","A trait representing a game event."]],"type":[["BoxedGameEvent","A box containing a generic `GameEvent` that is guaranteed to be `Sync` and `Send`."]]});