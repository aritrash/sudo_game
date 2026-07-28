# S.U.D.O. (System User Deception Override)

A multiplayer system-level survival game built in Rust using the Bevy engine. 

Players are spawned as background Daemons in a brutalist, bare-metal Data Center. The Admins must manually route critical Data Packets to the Uplink, while one player—the Rootkit—uses environmental camouflage to infiltrate, infect, and terminate the Admins before the system stabilizes.

## The Tech Stack
* **Engine:** [Bevy](https://bevyengine.org/) (Data-driven ECS framework)
* **Language:** Rust (Strict ownership, memory safety, and concurrency)
* **Networking:** `bevy_renet` (Client/Server) & `bevy_steamworks` (P2P Lobby routing)
* **UI:** `bevy_egui` (Immediate-mode GUI for terminals and menus)
* **Assets:** Custom low-poly `.glb` models generated in Blender.

## Core Mechanics
* **The Admins:** Must retrieve glowing Data Packets from Storage and physically carry them through dangerous, dark corridors to the Uplink room. 
* **The Rootkit:** Can spoof their visual identity to match environmental props (server racks, cooling fans). If they infect an Admin, a fatal Kernel Panic countdown begins for that player.
* **The Syslog:** If a terminated process (dead player) is found, the engine pauses and drops players into a terminal interface to isolate and quarantine the suspected Rootkit.

## The Studio
* **Aritrash Sarkar:** ECS Architecture, Game State Management, and Network Sync.
* **Roheet Purkayastha:** Gameplay Physics, Infection Logic, and the Network Pipeline.


## Getting Started

Ensure you have the latest Rust toolchain installed.

1. **Clone the repository:**
   ```bash
   git clone https://github.com/aritrash/sudo_game.git
   cd sudo_game
   ```

2. **Run the Engine:**
    ```bash
    cargo run
    ```
    __[Note] S.U.D.O. uses dynamic linking for rapid development. The first compile will take several minutes to build the Bevy engine and OS-level dependencies. Subsequent iterative builds will take 1-3 seconds.__
