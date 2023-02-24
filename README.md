CRAbE - Central AI of NAMeC

Some ideas : 

/// Qu'est ce qu'on veut ?
///
/// On veut pouvoir :
/// - Pouvoir ajouter facilement des tâches
/// - Lancer l'ensemble du système sans que les étudiants soient consicent des différents modules ou pipeline.
///
///
/// DataReceiverPipeline : () -> DataReceiver
/// FilterPipeline : (DataReceiver, FeedbackRobot) -> (SSLWorld, DebugData)
/// DecisionPipeline : (SSLWorld, ToolsResponse) -> (Control, DebugData)
/// ToolsPipeline : (SSLWorld, DebugData) -> Control
/// GuardPipeline : (SSLWorld, Control) -> (Control, DebugData)
/// DataTransmitterPipeline : (Control) -> (Feedback, DebugData)
///
/// SSLWorld can contains debuggingData ?
///
/// How to pass some parameters in another task?
/// SSLWorld needs to have a settings systems.
/// Pattern Observer ?
///
/// For annotations, name class :
/// Annotator
///
/// Control ->
/// - Commands robot
/// - Teleport robot
///
///
/// System::new(input_pipeline, filter_pipeline, decision_pipeline, guard_pipeline, output_pipeline);
/// System.init(); // Initialize the cli & logger and other things
/// System.run(); // Loop all project
/// System.close(); // Close all things
///
///

Modules :
- crabe
- crabe_filters
- crabe_framework
- crabe_guard
- crabe_io
- crabe_navigation
- crabe_protocol
