
# Flappy Dragon

Flappy Dragon is a simple game developed in Rust using the `bracket-lib` library. The game is inspired by the classic Flappy Bird, where the player navigates a dragon through obstacles.

## Getting Started

### Prerequisites

To run this project, you'll need to have Rust and Cargo installed. You can install Rust and Cargo by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/Parado-xy/flappyDragon.git
    cd flappyDragon
    ```

2. Build the project:
    ```sh
    cargo build
    ```

3. Run the game:
    ```sh
    cargo run
    ```

## Gameplay

- Use the `Space` key to make the dragon flap its wings and avoid obstacles.
- The game has three modes:
  - **Menu**: The starting screen where you can choose to play or quit.
  - **Playing**: The main gameplay mode.
  - **End**: The game over screen showing your score.

## Project Structure

- `src/main.rs`: The main source code file containing the game logic.
- `Cargo.toml`: The configuration file for Cargo, specifying dependencies and metadata.

### Dependencies

The project relies on the following dependencies:
- `bracket-lib`: A Rust library for building roguelike games.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes.

## License

This project is licensed under the MIT License.

