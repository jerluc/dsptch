use pipewire::{self, prelude::*, properties, Context, MainLoop};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mainloop = MainLoop::new().expect("Failed to create Pipewire Mainloop");
    let context = Context::new(&mainloop).expect("Failed to create Pipewire Context");
    let core = context
        .connect(None)
        .expect("Failed to connect to Pipewire Core");
    let registry = core.get_registry().unwrap();

    let output = core
        .create_object::<pipewire::node::Node, _>(
            "spa-node-factory",
            &properties! {
                *pipewire::keys::MEDIA_TYPE => "Midi",
                *pipewire::keys::MEDIA_CATEGORY => "Playback",
                *pipewire::keys::MEDIA_ROLE => "Production",
            },
        )
        .expect("Failed to create object");

    let _listener = output
        .add_listener_local()
        .param(|_, _, _, _| println!("param()"))
        .info(|node_info| println!("Node info: {:?}", node_info))
        .register();

    // Register a callback to the `global` event on the registry, which notifies of any new global objects
    // appearing on the remote.
    // The callback will only get called as long as we keep the returned listener alive.
    // let _listener = registry
    //     .add_listener_local()
    //     .global(|global| println!("New global: {:?}", global))
    //     .register();

    // let mut midi = pipewire::stream::Stream::new(
    //     &core,
    //     "dsptch_midi",
    // )?;

    // Calling the `destroy_global` method on the registry will destroy the object with the specified id on the remote.
    // We don't have a specific object to destroy now, so this is commented out.
    // registry.destroy_global(313).into_result()?;

    mainloop.run();

    Ok(())
}
