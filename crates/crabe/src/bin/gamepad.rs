use clap::{Args, Parser};
use crabe_decision::pipeline::{DecisionConfig, DecisionPipeline};
use crabe_filter::{FilterConfig, FilterPipeline};
use crabe_framework::component::{
    Component, DecisionComponent, FilterComponent, GuardComponent, InputComponent, OutputComponent,
    ToolComponent,
};
use crabe_framework::config::CommonConfig;
use crabe_framework::data::output::FeedbackMap;
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::World;
use crabe_guard::pipeline::{GuardConfig, GuardPipeline};
use crabe_io::pipeline::input::{InputConfig, InputPipeline};
use crabe_io::pipeline::output::{OutputConfig, OutputPipeline};
use crabe_io::tool::ToolConfig;
use crabe_io::tool::ToolServer;
use env_logger::Env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crabe::gamepad_pipeline::GamepadPipeline;
use crabe_io::gamepad::GamepadRobotIdConfig;

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
    #[command(next_help_heading = "Decision")]
    pub decision_config: DecisionConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Tool")]
    pub tool_config: ToolConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Guard")]
    pub guard_config: GuardConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Output")]
    pub output_config: OutputConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Control")]
    pub gamepad_config: GamepadRobotIdConfig,
}

#[derive(Default)]
pub struct SystemBuilder {
    input_component: Option<Box<dyn InputComponent>>,
    filter_component: Option<Box<dyn FilterComponent>>,
    decision_component: Option<Box<dyn DecisionComponent>>,
    tool_component: Option<Box<dyn ToolComponent>>,
    guard_component: Option<Box<dyn GuardComponent>>,
    output_component: Option<Box<dyn OutputComponent>>,
    world: Option<World>,
}

impl SystemBuilder {
    fn decision_component(mut self, decision: impl DecisionComponent + 'static) -> Self {
        self.decision_component = Some(Box::new(decision));
        self
    }

    fn tool_component(mut self, tool: impl ToolComponent + 'static) -> Self {
        self.tool_component = Some(Box::new(tool));
        self
    }

    fn guard_component(mut self, guard: impl GuardComponent + 'static) -> Self {
        self.guard_component = Some(Box::new(guard));
        self
    }

    fn output_component(mut self, output: impl OutputComponent + 'static) -> Self {
        self.output_component = Some(Box::new(output));
        self
    }

    fn world(mut self, world: World) -> Self {
        self.world = Some(world);
        self
    }

    fn build(self) -> System {
        let running = Arc::new(AtomicBool::new(true));
        let running_ctrlc = Arc::clone(&running);
        ctrlc::set_handler(move || {
            running_ctrlc.store(false, Ordering::Relaxed);
        })
            .expect("Failed to set Ctrl-C handler");

        System {
            decision_component: self.decision_component.expect("missing decision component"),
            tool_component: self.tool_component.expect("missing tool component"),
            guard_component: self.guard_component.expect("missing guard component"),
            output_component: self.output_component.expect("missing output component"),
            running,
            world: self.world.expect("missing world"),
        }
    }
}

pub struct System {
    decision_component: Box<dyn DecisionComponent>,
    tool_component: Box<dyn ToolComponent>,
    guard_component: Box<dyn GuardComponent>,
    output_component: Box<dyn OutputComponent>,
    running: Arc<AtomicBool>,
    world: World,
}

impl System {
    // TODO: Use refresh rate
    pub fn run(&mut self, _refresh_rate: Duration) {
        let mut feedback: FeedbackMap = Default::default();

        while self.running.load(Ordering::SeqCst) {
            let (mut command_map, mut tool_data) = self.decision_component.step(&self.world);
            self.tool_component
                .step(&mut self.world, &mut tool_data, &mut command_map);
            self.guard_component
                .step(&mut self.world, &mut command_map, &mut ToolCommands);
            feedback = self.output_component.step(command_map, ToolCommands);
            thread::sleep(_refresh_rate);
        }
    }

    pub fn close(self) {
        self.decision_component.close();
        self.guard_component.close();
        self.output_component.close();
        self.tool_component.close();
    }
}

fn main() {
    let cli = Cli::parse();
    let env = Env::default()
        .filter_or("CRABE_LOG_LEVEL", "info")
        .write_style_or("CRABE_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let mut system = SystemBuilder::default()
        .world(World::with_config(&cli.common))
        .decision_component(GamepadPipeline::with_config(cli.decision_config, &cli.common, &cli.gamepad_config))
        .tool_component(ToolServer::with_config(cli.tool_config, &cli.common))
        .guard_component(GuardPipeline::with_config(cli.guard_config, &cli.common))
        .output_component(OutputPipeline::with_config(cli.output_config, &cli.common))
        .build();

    system.run(Duration::from_millis(16));
    system.close();
}
