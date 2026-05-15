# Systems

## App Structure

Internally, every Bevy [`App`](https://docs.rs/bevy/latest/bevy/app/struct.App.html) is organized as a set of [`SubApps`](https://docs.rs/bevy/latest/bevy/app/struct.SubApps.html), with one main [`SubApp`](https://docs.rs/bevy/latest/bevy/app/struct.SubApp.html) and zero or more additional [`SubApp`](https://docs.rs/bevy/latest/bevy/app/struct.SubApp.html)s. In each run of the update loop, the app first runs the update schedule of the main [`SubApp`](https://docs.rs/bevy/latest/bevy/app/struct.SubApp.html). Then, for each additional [`SubApp`](https://docs.rs/bevy/latest/bevy/app/struct.SubApp.html) (in an arbitrary order), it runs the [`extract`](https://docs.rs/bevy/latest/bevy/app/struct.SubApp.html#method.extract) method for that [`SubApp`](https://docs.rs/bevy/latest/bevy/app/struct.SubApp.html) (allowing it to extract changes made to the main [`SubApp`](https://docs.rs/bevy/latest/bevy/app/struct.SubApp.html)) followed by the update schedule for that [`SubApp`](https://docs.rs/bevy/latest/bevy/app/struct.SubApp.html).

The relevant parts of the code look roughly like the following:

```rust
/// Every app is organized as a set of [`SubApps`]
struct App {
    sub_apps: SubApps
}

impl App {
    /// Runs the [`App`]
    /// 
    /// Loops the update method until the [`App`] receives the signal to exit.
    /// 
    /// In actuality, [`App`] can be customized with a "runner" that is
    /// responsible for this loop. By default, the runner is a function that
    /// only runs update once [`bevy::app::run_once`] (not public). When using
    /// the [`bevy::DefaultPlugins`] the runner is added by
    /// [`bevy::winit::WinitPlugin`], but when using [`bevy::MinimalPlugins`]
    /// this is added by [`bevy::app::ScheduleRunnerPlugin`].
    pub fn run(&mut self) -> AppExit {
        loop {
            self.update();

            if let Some(exit) = self.should_exit() {
                return exit;
            }
        }
    }

    pub fn update(&mut self) {
        self.sub_apps.update()
    }

    /// Returns an exit code if the [`App`] should exit.
    /// 
    /// In actuality, does some funkiness to read the [`AppExit`] message
    /// resource inside the world of the "main" [`SubApp`] and return that.
    fn should_exit(&self) -> Option<AppExit> {
        ...
    }
}

/// There is one "main" [`SubApp`] and zero or more additional [`SubApp`]s
struct SubApps {
    main: SubApp,
    additional: HashMap<Label, SubApp>
}

impl SubApps {
    /// Runs the update schedule for the "main" [`SubApp`]. Then, for each
    /// additional [`SubApp`], allows it to extract updates from the "main"
    /// [`SubApp`], before running its update schedule.
    pub fn update(&mut self) {
        self.main.run_update_schedule();

        for (_, sub_app) in self.additional.iter_mut() {
            sub_app.extract(&mut self.main.world);
            sub_app.run_update_schedule();
        }
    }
}

struct SubApp {
    world: World,
    update_schedule: Option<ScheduleLabel>
}

impl SubApp {
    /// Extract data from the main world
    /// 
    /// This is actually implemented as an optional closure stored on the
    /// [`SubApp`] for customizable behavior, but this is an example of
    /// roughly what a typical extract method looks like.
    pub fn extract(&mut self, world: &mut World) {
        self.world.foo = world.foo;
    }

    /// Runs the update schedule for the [`SubApp`]
    ///
    /// For `App::default()` the update schedule of the "main" [`SubApp`] is
    /// [`bevy::app::Main`], but for the rendering
    /// [`SubApp`] created by the rendering plugin, the update schedule is
    /// [`bevy::render::Render`].
    pub fn run_update_schedule(&mut self) {
        if let Some(label) = self.update_schedule {
            self.world.run_schedule(label);
        }
    }
}

```

For [`App::default()`](https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.default), the update schedule of the "main" [`SubApp`](https://docs.rs/bevy/latest/bevy/app/struct.SubApp.html) is [`Main`](https://docs.rs/bevy/latest/bevy/app/struct.Main.html), but for the rendering [`SubApp`](https://docs.rs/bevy/latest/bevy/app/struct.SubApp.html), the update schedule is [`Render`](https://docs.rs/bevy/latest/bevy/render/struct.Render.html).

### A Typical Update Loop

Altogether, this means the callstack of a typical update loop looks roughly like the following:
- [`Main`](https://docs.rs/bevy/latest/bevy/app/main_schedule/struct.Main.html) runs as the update schedule of the "main" [`SubApp`](https://docs.rs/bevy/latest/bevy/app/struct.SubApp.html)
    - If First Run
        - [`StateTransition`](https://docs.rs/bevy/latest/bevy/state/prelude/struct.StateTransition.html) (only on feature "bevy_state")
            - [`OnEnter`](https://docs.rs/bevy/latest/bevy/state/prelude/struct.OnEnter.html) will run here as each state enters its [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html) state
        - [`PreStartup`](https://docs.rs/bevy/latest/bevy/app/struct.PreStartup.html)
        - [`Startup`](https://docs.rs/bevy/latest/bevy/app/struct.Startup.html)
        - [`PostStartup`](https://docs.rs/bevy/latest/bevy/app/struct.PostStartup.html)
    - On All Runs
        - [`First`](https://docs.rs/bevy/latest/bevy/app/struct.First.html)
        - [`PreUpdate`](https://docs.rs/bevy/latest/bevy/app/struct.PreUpdate.html)
        - [`StateTransition`](https://docs.rs/bevy/latest/bevy/state/prelude/struct.StateTransition.html) (only on feature "bevy_state")
            - [`OnExit`](https://docs.rs/bevy/latest/bevy/state/prelude/struct.OnExit.html) will run for states in leaf to root order
            - [`OnTransition`](https://docs.rs/bevy/latest/bevy/state/prelude/struct.OnTransition.html) will run for states in an arbitrary order
            - [`OnEnter`](https://docs.rs/bevy/latest/bevy/state/prelude/struct.OnEnter.html) will run for states in root to leaf order
        - [`RunFixedMainLoop`](https://docs.rs/bevy/latest/bevy/app/struct.RunFixedMainLoop.html)
            - [`FixedMain`](https://docs.rs/bevy/latest/bevy/app/struct.FixedMain.html) a number of times proportional to the elapsed time
                - [`FixedFirst`](https://docs.rs/bevy/latest/bevy/app/main_schedule/struct.FixedFirst.html)
                - [`FixedPreUpdate`](https://docs.rs/bevy/latest/bevy/app/main_schedule/struct.FixedPreUpdate.html)
                - [`FixedUpdate`](https://docs.rs/bevy/latest/bevy/app/main_schedule/struct.FixedUpdate.html)
                - [`FixedLast`](https://docs.rs/bevy/latest/bevy/app/main_schedule/struct.FixedLast.html)
                - [`FixedPostUpdate`](https://docs.rs/bevy/latest/bevy/app/main_schedule/struct.FixedPostUpdate.html)
        - [`Update`](https://docs.rs/bevy/latest/bevy/app/struct.Update.html)
        - [`SpawnScene`](https://docs.rs/bevy/latest/bevy/app/struct.SpawnScene.html)
        - [`PostUpdate`](https://docs.rs/bevy/latest/bevy/app/struct.PostUpdate.html)
        - [`Last`](https://docs.rs/bevy/latest/bevy/app/struct.Last.html)
        - [`RemoteLast`](https://docs.rs/bevy/latest/bevy/remote/struct.RemoteLast.html) (only on feature "bevy_remote")
- The render [`SubApp`](https://docs.rs/bevy/latest/bevy/app/struct.SubApp.html) extracts updates from the "main" [`SubApp`](https://docs.rs/bevy/latest/bevy/app/struct.SubApp.html) (only on feature "bevy_render")
    - On First Run
        - [`RenderStartup`](https://docs.rs/bevy/latest/bevy/render/struct.RenderStartup.html)
    - On All Runs
        - [`ExtractSchedule`](https://docs.rs/bevy/latest/bevy/render/struct.ExtractSchedule.html)
- [`Render`](https://docs.rs/bevy/latest/bevy/render/struct.Render.html) runs as the update schedule fo the render [`SubApp`](https://docs.rs/bevy/latest/bevy/app/struct.SubApp.html) (only on feature "bevy_render")
