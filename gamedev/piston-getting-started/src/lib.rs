// Handle rendering any objects on screen.
pub fn render(&mut self, args: &RenderArgs) {
    // Get the location of the "player" we want to render.
    let rotation = self.rotation;
    let x = self.x;
    let y = self.y;

    // Create a little square and render it on screen.
    let square = rectangle::square(0.0, 0.0, 50.0);
    self.window.gl.draw(args.viewport(), |c, gl| {
        // Clear the screen.
        clear(BLACK, gl);
        // Place object on screen
        let transform = c.transform.trans(x, y)
            // Handle any rotation
            .rot_rad(rotation)
            // Center object on coordinate.
            .trans(-25.0, -25.0);
        // Draw a box rotating around the middle of the screen.
        rectangle(RED, square, transform, gl);
    });
}

// Update any animation, etc.
pub fn update(&mut self, args: &UpdateArgs) {
    // Rotate 2 radians per second.
    self.rotation += 2.0 * args.dt;
}
