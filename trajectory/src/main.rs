use log::info;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};
use ratatui::{
    style::{Style},
    symbols,
    widgets::{Axis, Block, Chart, Dataset, GraphType},
};
use std::{
    io,
    ops::{Add, Mul, Sub},
};
// Const definitions

const ACCELERATION_GRAVITY: Vector2 = Vector2 { x: 0.0, y: -9.81 };
const CONTACT_EFFIENENCY: f64 = 0.25; // Bounciness

#[derive(Debug, Clone, Default)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct Projectile {
    pub position: Vector2,
    pub velocity: Vector2,
    pub acceleration: Vector2,
}

impl Projectile {
    pub fn new(initial_pos: &Vector2) -> Self {
        Self {
            position: initial_pos.clone(),
            velocity: Vector2::default(),
            acceleration: Vector2::default(),
        }
    }

    pub fn fire(&mut self, velocity: &Vector2) {
        self.velocity = velocity.clone();
    }

    // Returns current position
    pub fn update(&mut self, dt: &f64) -> Vector2 {
        self.update_velocity(dt);
        self.update_position(dt);
        self.position.clone()
    }

    fn update_position(&mut self, dt: &f64) {
        let movement = self.velocity.clone() * (dt.clone());
        let mut translation = self.position.clone() + movement;

        // Ground Check
        // If the translation (proposed new position) is below or at 0,
        // Set the y component to 0 (ground) and reverse the current velocity
        // minus some lost in energy

        if translation.y <= 0. {
            translation.y = 0.;
            let new_velocity = self.velocity.clone() * -1.0 * CONTACT_EFFIENENCY;
            self.velocity.y = new_velocity.y;
        }

        self.position = translation;

        self.acceleration = Vector2::default() + ACCELERATION_GRAVITY;
    }

    fn update_velocity(&mut self, dt: &f64) {
        let delta_velocity = self.acceleration.clone() * dt.clone();
        self.velocity = self.velocity.clone() + delta_velocity;
    }
}
fn make_chart<'a>(data: &'a Vec<(f64, f64)>) -> Chart<'a> {
    // Create the datasets to fill the chart with
    let datasets = vec![

        // Line chart
        Dataset::default()
            .name("Projectile Position")
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Line)
            .style(Style::default().red())
            .data(data),
    ];

    // Create the X axis and define its properties
    let x_axis = Axis::default()
        .title("X Axis".red())
        .style(Style::default().white())
        .bounds([-1.0, 15.0])
        .labels(["-1.0", "0.0", "15.0"]);

    // Create the Y axis and define its properties
    let y_axis = Axis::default()
        .title("Y Axis".red())
        .style(Style::default().white())
        .bounds([-1.0,30.0])
        .labels(["-1.0", "GROUND", "30.0"]);

    // Create the chart and link all the parts together
    let chart = Chart::new(datasets)
        .block(Block::new().title("Chart"))
        .x_axis(x_axis)
        .y_axis(y_axis);

    return chart;
}
fn run<'a>(mut terminal: DefaultTerminal, points: &'a Vec<Vector2>) -> io::Result<()> {

    loop {
        terminal.draw(|frame| {
            let data = points.iter().map(|vec| return (vec.x, vec.y)).collect();
            let chart = make_chart(&data);
            frame.render_widget(chart, frame.area());

        })?;
        

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
pub fn main() -> Result<(), anyhow::Error> {
    pretty_env_logger::init();

    let init_pos = Vector2::new(1.0, 10.);
    let mut projectile = Projectile::new(&init_pos);

    projectile.fire(&Vector2::new(2.0, 2.0));

    let dt = 0.01;
    let mut t = 10.0;
    info!("Staring: {:#?}", projectile);
    let mut points = vec![];
    while t > 0.0 {
        t = t - dt;
        let result_pos = projectile.update(&dt);
        points.push(result_pos);
    }
    info!("Ending: {:#?}", projectile);

    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal, &points);
    ratatui::restore();

    Ok(())
}
