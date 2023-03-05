use clap::Parser;
use crabe_filter::{FilterConfig, FilterPipeline};
use crabe_framework::component::{Component, FilterComponent, InputComponent, OutputComponent, ToolComponent};
use crabe_framework::config::CommonConfig;
use crabe_framework::data::output::FeedbackMap;
use crabe_framework::data::world::World;
use crabe_io::pipeline::input::{InputConfig, InputPipeline};
use env_logger::Env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crabe_framework::data::tool::ToolData;
use crabe_io::pipeline::output::{OutputConfig, OutputPipeline};
use crabe_io::tool::ToolConfig;
use crabe_io::tool::ToolServer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    #[command(next_help_heading = "Common")]
    pub common: CommonConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Input")]
    pub input_config: InputConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Filter")]
    pub filter_config: FilterConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Tool")]
    pub tool_config: ToolConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Output")]
    pub output_config: OutputConfig,
}

pub struct System {
    input_component: Box<dyn InputComponent>,
    filter_component: Box<dyn FilterComponent>,
    tool_component: Box<dyn ToolComponent>,
    output_component: Box<dyn OutputComponent>,
    running: Arc<AtomicBool>,
    world: World
}

impl System {
    // TODO: Builder
    pub fn new(
        world: World,
        input_component: impl InputComponent + 'static,
        filter_component: impl FilterComponent + 'static,
        tool_component: impl ToolComponent + 'static,
        output_component: impl OutputComponent + 'static,
    ) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let running_ctrlc = Arc::clone(&running);

        ctrlc::set_handler(move || {
            running_ctrlc.store(false, Ordering::Relaxed);
        })
        .expect("Failed to set Ctrl-C handler");

        Self {
            input_component: Box::new(input_component),
            filter_component: Box::new(filter_component),
            tool_component: Box::new(tool_component),
            output_component: Box::new(output_component),
            running,
            world,
        }
    }

    // TODO: Use refresh rate
    pub fn run(&mut self, _refresh_rate: Duration) {
        let mut feedback: FeedbackMap = Default::default();


        while self.running.load(Ordering::SeqCst) {
            let receive_data = self.input_component.step(&mut feedback);
            self.filter_component.step(receive_data, &mut self.world);
            //dbg!(&world);
            let mut tool_data = ToolData {};
            self.tool_component.step(&mut self.world, &mut tool_data);
            thread::sleep(_refresh_rate);
        }
    }

    pub fn close(self) {
        self.input_component.close();
        self.filter_component.close();
        self.tool_component.close();
    }
}

fn main() {
    let cli = Cli::parse();
    let env = Env::default()
        .filter_or("CRABE_LOG_LEVEL", "debug")
        .write_style_or("CRABE_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    // DecisionPipeline
    // ToolsPipeline
    // GuardPipeline
    // OutputPipeline

    let mut system = System::new(
        World::with_config(&cli.common),
        InputPipeline::with_config(cli.input_config, &cli.common),
        FilterPipeline::with_config(cli.filter_config, &cli.common),
        ToolServer::with_config(cli.tool_config, &cli.common),
        OutputPipeline::with_config(cli.output_config, &cli.common)
    );

    system.run(Duration::from_millis(16));
    system.close();
}
