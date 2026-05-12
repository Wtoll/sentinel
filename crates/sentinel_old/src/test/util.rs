//! Utilities for writing tests for Bevy

use bevy::prelude::*;


/// A plugin that is a simple testing environment
pub trait SimpleTest {
    /// The number of loops to do before quitting
    const CYCLES: usize;

    /// Startup system
    fn startup(world: &mut World);

    /// Update system
    fn update(world: &mut World);
}

/// Runs a [`SimpleTest`]
#[macro_export]
macro_rules! simple_test {
    ($implementation:ident) => {

        use bevy::prelude::*;
        use std::time::Duration;
        use bevy::app::ScheduleRunnerPlugin;

        App::new()
            .add_plugins(MinimalPlugins
                .set(ScheduleRunnerPlugin::run_loop(Duration::ZERO)))
            .add_systems(Startup, $implementation::startup)
            .add_systems(Update, $implementation::update)
            .add_systems(PostUpdate, quit_after)
            .run();

        fn quit_after(
            mut writer: MessageWriter<AppExit>,
            mut counter: Local<usize>
        ) {
            *counter += 1;

            if *counter > $implementation::CYCLES {
                writer.write(AppExit::Success);
            }
        }
        
    }
}

#[cfg(test)]
mod test {
    
    #[test]
    fn instantiation() {
        use crate::test::util::SimpleTest;

        struct Implementation;

        impl SimpleTest for Implementation {
            const CYCLES: usize = 30;

            fn startup(world: &mut World) {
                println!("Startup");
            }

            fn update(world: &mut World) {
                println!("Update");
            }
        }

        simple_test!(Implementation);
    }
}