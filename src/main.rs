use bracket_lib::prelude::BError;

fn main() -> BError {
    use bracket_lib::prelude::*;

    // Let's add a GameMode enum;
    // The Game we're currently making is called flappy dragon;
    // The Game has 3 modes - Menu, Playing, and End
    enum GameMode {
        Menu,
        Playing,
        End,
    }

        // Game Constants;
    const SCREEN_WIDTH : i32 = 80;
    const SCREEN_HEIGHT : i32 = 50;
    const FRAME_DURATION : f32 = 65.0;

    // Define a Player;
    struct Player{
        x: i32, //The Player's `x` position;
        y: i32, // The Player's `y` position;
        velocity: f32, // The player's vertical velocity.
    }

    impl Player{
        fn new(x: i32 , y: i32) -> Self{
            Player {
                x,
                y, 
                velocity: 0.0
             }
        }

        fn render(&mut self, ctx: &mut BTerm){
            // Render the character as an '@' symbol;
            ctx.set(
                0, // Player always starts at the left of the screen;
                self.y, // Set y coordinate;
                YELLOW, // Set a foreground color;
                BLACK, // Set a background color;
                to_cp437('@') // Set a character;
            );

            // set() is a ‘bracket-lib‘ function that sets a single character on the screen.
            // The x coordinate on the screen at which to render the character.
            // The y coordinate on the screen at which to render the character.
            // bracket-lib includes a set of named colors for your use—derived from the
            // HTML named color list. Alternatively, you can use RGB::from_u8() to specify
            // red/green/blue values in the 0-254 range, or RGB::from_hex() to specify
            // HTML-style colors.
            // The character to render. The function to_cp437() converts a Unicode symbol
            // from your source code to the matching Codepage 437 character number
        }

        fn gravity_and_move(&mut self){
            // Check for terminal velocity: only apply gravity if the downward momentum
            // is less than two.
            if self.velocity < 2.0 {
                    self.velocity += 0.2; // Adding the current velocity moves the player up or down;
            }
            self.y += self.velocity as i32; // Add the velocity to the player's y position;
            self.x += 1;  // Even though you’re not moving the character on the screen, you need to
            //                 know how far it has progressed through the level. Incrementing x lets you
            //                 keep track of this.

            if self.y < 0{
                self.y = 0
            }

        }

        fn flap(&mut self){
            // The `flap` function sets the player's velocity to -2.0. It's a negative number, so this will move the character upward;
            self.velocity =  -2.0;
        }
    }

    struct Obstacle{
        x: i32,
        gap_y: i32,
        size: i32
    }
    // Obstacles have an x value, defining their position in world-space (to match
    // the player’s world-space x value). The gap_y variable defines the center of the
    // gap through which the dragon may pass. size defines the length of the gap in
    // the obstacle.

    impl Obstacle{
        fn new(x: i32, score: i32) -> Self{
            let mut random = RandomNumberGenerator::new();
            Obstacle{
                x,
                gap_y: random.range(10, 40),
                size: i32::max(2, 20 - score)
            }
        }

        fn render(&mut self, ctx: &mut BTerm, player_x: i32){
            let screen_x = self.x - player_x;
            let half_size = self.size / 2;

            // Draw the top half of the obstacle;
            for y in 0..self.gap_y - half_size {
                ctx.set(screen_x, 
                y, 
                RED, 
                BLACK, 
                to_cp437('|'));
            }

            // Draw the bottom half of the obstacle;
            for y in self.gap_y + half_size..SCREEN_HEIGHT {
                ctx.set(screen_x,
                y,
                RED, 
                BLACK, 
            to_cp437('|'));
            }
        }

        fn hit_obstacle(&self, player: &Player) -> bool {
            let half_size = self.size / 2;
            let does_x_match = player.x == self.x;
            let player_above_gap = player.y < self.gap_y - half_size;
            let player_below_gap = player.y > self.gap_y + half_size;
            does_x_match && (player_above_gap || player_below_gap)
        }
    }


    // Define the state structure
    struct State {
        player: Player, // Add a player to the state of the game;
        frame_time: f32, // This tracks the time accumulated between frames to control the game's speed;
        mode: GameMode, // The state of the game has a mode property which is an enum and has 3 modes;
        score: i32, // The player's score representing how many obstacles they've overcome;
        obstacle: Obstacle, // Obstacles for the player to judge;
    }

    // Create an associated function `new` whose default mode is the menu;
    impl State{
        fn new() -> Self{
            State { 
                player: Player::new(5, 25),
                frame_time: 0.0,
                mode: GameMode::Menu,
                score: 0,
                obstacle: Obstacle::new(SCREEN_WIDTH, 0),
             }
        }

        fn play(&mut self, ctx: &mut BTerm){
            ctx.cls_bg(NAVY); // Clear the console and set the navy background;
            self.frame_time += ctx.frame_time_ms;  // start counting frame time
            if self.frame_time > FRAME_DURATION{
                self.frame_time = 0.0;
                self.player.gravity_and_move();
            }

            // Flap dragons wings whenever the space button is clicked;
            if let Some(VirtualKeyCode::Space) = ctx.key{
                self.player.flap();
            }
            // render the player;
            self.player.render(ctx);
            // print  gameplay instructions;
            ctx.print(0, 0, "Press SPACE to flap");
            // Print current score;
            ctx.print(0, 1, &format!("Score {}", self.score));

            self.obstacle.render(ctx, self.player.x); // Render first obstacle;

            if self.player.x > self.obstacle.x {
                self.score += 1;
                self.obstacle = Obstacle::new(
                    self.player.x + SCREEN_WIDTH, self.score
                );
            }
            // If the player has fallen off the bottom of the screen or hit an obstacle, end the game;
            if self.player.y > SCREEN_HEIGHT ||
                self.obstacle.hit_obstacle(&self.player)
            {
                self.mode = GameMode::End;
            }
        }

        fn restart(&mut self){
            self.player = Player::new(5, 25);
            self.frame_time = 0.0;
            self.mode =  GameMode::Playing;
            self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
            self.score = 0;
        }

        fn main_menu(&mut self, ctx: &mut BTerm){
            ctx.cls(); // Clear The Screen
            ctx.print_centered(5, "Welcome to Flappy Dragon");
            ctx.print_centered(8, "(P) Play Game");
            ctx.print_centered(9, "(Q) Quit Game");

            if let Some(key) = ctx.key{ // This is equivalent to a match statement that checks if ctx.key is not none
                match key{ // If the key is not none, match the key that was pressed;
                    VirtualKeyCode::P => self.restart(), // If P was pressed, restart the game;
                    VirtualKeyCode::Q => ctx.quitting = true,  // if q was pressed, quit the game;
                    _ => {} // Ignore every other key;
                }
            }
        }
        // print_centered() is an extended version of print() that centers text on a line, given
        // only a y position.

        fn dead(&mut self, ctx: &mut BTerm){
            ctx.cls(); // Clear Screen;
            ctx.print_centered(5, "You are dead!");
            ctx.print_centered(8, "(P) Play Game");
            ctx.print_centered(9, "(Q) Quit Game");
            ctx.print_centered(6, &format!("You earned {} points", self.score));
            if let Some(key) = ctx.key{ // This is equivalent to a match statement that checks if ctx.key is not none
                match key{ // If the key is not none, match the key that was pressed;
                    VirtualKeyCode::P => self.restart(), // If P was pressed, restart the game;
                    VirtualKeyCode::Q => ctx.quitting = true,  // if q was pressed, quit the game;
                    _ => {} // Ignore every other key;
                }
            }            
        }



    }

    impl GameState for State {
        // Implement the tick function as required by the GameState trait
        fn tick(&mut self, ctx: &mut BTerm) {
            // With every tick, match the current game mode to it's proper function;
            match self.mode {
                GameMode::Menu => self.main_menu(ctx),
                GameMode::End => self.dead(ctx),
                GameMode::Playing => self.play(ctx)
            }
        }
    }

    // Create the game context and start the game loop
    let context = BTermBuilder::simple80x50() // Request an 80x50 terminal
                                             .with_title("Flappy Dragon") // Set the window title
                                             .build()?; // Build the context

    main_loop(context, State::new()) // Start the game loop
}
