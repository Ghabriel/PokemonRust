function chained_events(events)
    local chain = rust_create_chained_event()

    for _, event in pairs(events) do
        rust_add_event(chain, event)
    end

    rust_dispatch_event(chain)
end

function do_it()
    chained_events({
        rust_create_text_event("Hello, world!"),
        rust_create_warp_event("test_map", 10, 10)
    })
end
