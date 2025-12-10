use std::collections::HashMap;

use chrono::{DateTime, Local};
use ggegui::egui::ScrollArea;
use ggegui::{Gui, egui};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam};
use ggez::{Context, ContextBuilder, GameResult, glam};
use rust_decimal::prelude::*;

pub trait Detail {
    fn check_name(&self) -> &str;
    fn check_quantity(&self) -> u64;
}

pub trait Increment {}

pub trait Cost {
    fn check_cost(&self) -> Decimal;
    fn obtain_new_cost(&mut self);
}

//************************************************************** */
// Upgrade functionality
/**************************************************************** */
const ADD_UPG_RATE: Decimal = dec!(1.05);
const MULT_UPG_RATE: Decimal = dec!(1.9);
const EXP_UPG_RATE: Decimal = dec!(1.14);

enum UpgradeEffect {
    Additive,
    Multiplicative,
    Exponential,
}

pub struct Upgrade {
    generator_id: GeneratorID,
    name: String,
    effect: UpgradeEffect,
    quantity: u64,
    base_cost: Decimal,
    current_cost: Decimal,
}

impl Detail for Upgrade {
    fn check_name(&self) -> &str {
        &self.name
    }

    fn check_quantity(&self) -> u64 {
        self.quantity
    }
}

impl Cost for Upgrade {
    fn check_cost(&self) -> Decimal {
        self.current_cost
    }

    fn obtain_new_cost(&mut self) {
        let rate = match self.effect {
            UpgradeEffect::Additive => ADD_UPG_RATE,
            UpgradeEffect::Multiplicative => MULT_UPG_RATE,
            UpgradeEffect::Exponential => EXP_UPG_RATE,
        };

        self.current_cost = self.base_cost * rate.powu(self.quantity)
    }
}
//************************************************************************************************ */
// Resource Manager
//************************************************************************************************ */
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum GeneratorID {
    Clicker,
    AutoClicker,
}

struct Generator {
    generator_id: GeneratorID,
    name: String,
    quantity: u64,
    production_rate: Decimal,
}

impl Generator {
    fn check_generator_id(&self) -> GeneratorID {
        self.generator_id
    }

    fn check_name(&self) -> &str {
        &self.name
    }

    fn check_quantity(&self) -> u64 {
        self.quantity
    }

    fn increment_quantity(&mut self) {
        self.quantity += 1;
    }

    fn calculate_production(&self) -> Decimal {
        self.production_rate
    }
}

struct ResourceManager {
    electrons: Decimal,
    generators: Vec<Generator>,
    upgrades: HashMap<GeneratorID, Vec<Upgrade>>,
    time: DateTime<Local>,
}

impl ResourceManager {
    fn new() -> ResourceManager {
        let generator_vec = vec![
            Generator {
                generator_id: GeneratorID::Clicker,
                name: String::from("Scoop"),
                quantity: 1,
                production_rate: dec!(1),
            },
            Generator {
                generator_id: GeneratorID::AutoClicker,
                name: String::from("AutoScooper"),
                quantity: 0,
                production_rate: dec!(1),
            },
        ];

        let mut upgrade_hashmap = HashMap::new();

        upgrade_hashmap.insert(
            GeneratorID::Clicker,
            vec![Upgrade {
                generator_id: GeneratorID::Clicker,
                name: String::from("Even More"),
                effect: UpgradeEffect::Additive,
                quantity: 0,
                base_cost: dec!(15),
                current_cost: dec!(15),
            }],
        );

        upgrade_hashmap.insert(
            GeneratorID::Clicker,
            vec![
                Upgrade {
                    generator_id: GeneratorID::AutoClicker,
                    name: String::from("AutoScooper"),
                    effect: UpgradeEffect::Additive,
                    quantity: 0,
                    base_cost: dec!(15),
                    current_cost: dec!(15),
                },
                Upgrade {
                    generator_id: GeneratorID::AutoClicker,
                    name: String::from("Multiplier"),
                    effect: UpgradeEffect::Multiplicative,
                    quantity: 0,
                    base_cost: dec!(30),
                    current_cost: dec!(30),
                },
            ],
        );

        ResourceManager {
            electrons: dec!(0),
            generators: generator_vec,
            upgrades: upgrade_hashmap,
            time: chrono::Local::now(),
        }
    }

    fn electron_quantity(&self) -> Decimal {
        self.electrons
    }

    fn clicker_increment(&mut self) {
        self.electrons += self.generators[0].calculate_production()
    }

    fn update_time(&mut self) {
        self.time = chrono::Local::now();
    }

    fn calculate_seconds_passed(&mut self) -> u64 {
        let time = (self.time - chrono::Local::now())
            .num_seconds()
            .unsigned_abs();
        if time >= 1 {
            self.update();
        }
        time
    }

    pub fn can_buy_generator(&self) -> bool {
        todo!()
    }

    pub fn can_buy_upgrade(&self, index: usize) -> bool {
        todo!()
    }

    pub fn update(&mut self) {
        let generators = self
            .generators
            .iter()
            .filter(|&x| x.check_generator_id() != GeneratorID::Clicker && x.check_quantity() != 0);
        for generator in generators {
            self.electrons += generator.calculate_production()
        }
    }
}

struct Currency {
    balance: u64,
    click_rate: u64,
    auto_rate: u64,
    auto_gatherers: u32,
}

struct GameState {
    gui: Gui,
    resource_manager: ResourceManager,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameState {
        GameState {
            gui: Gui::new(ctx),
            resource_manager: ResourceManager::new(),
        }
    }

    pub fn purchase_upgrade(&mut self, index: usize) {
        todo!()
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let gui_ctx = self.gui.ctx();

        self.resource_manager.update();

        egui::TopBottomPanel::top("top_panel")
            .min_height(120.0)
            .show(&gui_ctx, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.heading("Collect Money");
                    ui.label(self.resource_manager.electron_quantity().to_string());
                    if ui.button("Click").clicked() {
                        self.resource_manager.clicker_increment();
                    }
                });
            });

        egui::CentralPanel::default().show(&gui_ctx, |ui| {
            ui.heading("The Elements are here");
        });

        self.gui.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::ZERO));
        canvas.finish(ctx)
    }

    fn mouse_wheel_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
    ) -> Result<(), ggez::GameError> {
        self.gui.input.mouse_wheel_event(x * 10.0, y * 10.0);
        Ok(())
    }

    fn resize_event(
        &mut self,
        _ctx: &mut Context,
        width: f32,
        height: f32,
    ) -> Result<(), ggez::GameError> {
        self.gui.input.resize_event(width, height);
        Ok(())
    }
}

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = GameState::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}
