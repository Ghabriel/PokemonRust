function do_it()
    -- print("Hello, world!")
    local event = rust_create_chained_event()
    rust_add_text_event(event, rust_create_text_event("Hello, world!"))
    rust_add_warp_event(event, rust_create_warp_event("test_map", 10, 10))
    rust_dispatch_event(event)
end
