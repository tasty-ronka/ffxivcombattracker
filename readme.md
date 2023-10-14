### FFXIVCombatTracker
#### Description
This is a fullstack application that will use Rust to parse memory operations for FFXIV. That data is exposed to a React Native frontend through Foreign Function Interfaces (FFI) and Event Emitters to allow for asyncronous processes.

#### Purpose
- Modernize Advanced Combat Tracker
- Be Operating System (OS) agnostic (Linux vs Windows vs MacOS)
- Handle memory processes in a safe and efficient manner
- Provide analytic insights into logged data

#### Minimal Viable Products (MVPs)
Rollout of this effort will follow an MVP roadmap. Each MVP is sequential to the prior MVP.
- MVP 1: create combat logs from memory processes pertaining to FFXIV  
    - Parse those logs into a GUI to represent the equivalent of a damage meter in World of Warcraft
- MVP 2: provide indicators for incurring mechanics
- MVP 3: add trigger support to handle event based indicators (a cooldown is coming available)
- MVP 4: add analytical dashboarding from parsed logs
- MVP 5: add customizable sounds
- MVP 6: add support for community added extensibility (example: community added triggers)
- MVP 7: interface with fflogs.com to allow for logs to be uploaded