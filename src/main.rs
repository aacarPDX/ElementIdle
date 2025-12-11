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

impl Upgrade {
    fn check_intended_generator(&self) -> GeneratorID {
        self.generator_id
    }
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

    fn increment_quantity(&mut self) {
        self.quantity += 1;
    }

    fn calculate_production(&self) -> Decimal {
        self.production_rate
    }
}

impl Detail for Generator {
    fn check_name(&self) -> &str {
        &self.name
    }

    fn check_quantity(&self) -> u64 {
        self.quantity
    }
}

struct ResourceManager {
    electrons: Decimal,
    generators: Vec<Generator>,
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

        ResourceManager {
            electrons: dec!(0),
            generators: generator_vec,
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

struct GameState {
    gui: Gui,
    upgrades: Vec<Upgrade>,
    resource_manager: ResourceManager,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameState {
        let upgrade_vector = vec![
            Upgrade {
                generator_id: GeneratorID::Clicker,
                name: String::from("Even More: Increase Electrons Per Click"),
                effect: UpgradeEffect::Additive,
                quantity: 0,
                base_cost: dec!(15),
                current_cost: dec!(15),
            },
            Upgrade {
                generator_id: GeneratorID::AutoClicker,
                name: String::from("AutoScooper: Add AutoScooper"),
                effect: UpgradeEffect::Additive,
                quantity: 0,
                base_cost: dec!(15),
                current_cost: dec!(15),
            },
            Upgrade {
                generator_id: GeneratorID::AutoClicker,
                name: String::from("AutoScooper: Increase Scooped Amount"),
                effect: UpgradeEffect::Multiplicative,
                quantity: 0,
                base_cost: dec!(30),
                current_cost: dec!(30),
            },
        ];

        GameState {
            gui: Gui::new(ctx),
            resource_manager: ResourceManager::new(),
            upgrades: upgrade_vector,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let gui_ctx = self.gui.ctx();

        self.resource_manager.update();

        egui::SidePanel::right("Upgrades panel")
            .min_width(300.0)
            .show(&gui_ctx, |ui| {
                ui.heading("Upgrades");
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    for i in &self.upgrades {
                        ui.label(i.check_name());
                        ui.horizontal(|ui| {
                            ui.label("Cost:");
                            ui.label(i.check_cost().to_string());
                            ui.label("Electrons")
                        });
                    }
                });
            });

        egui::CentralPanel::default().show(&gui_ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Electrons");
                ui.label(self.resource_manager.electron_quantity().to_string());
                if ui.button("Click").clicked() {
                    self.resource_manager.clicker_increment();
                }
            });
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
    let (mut ctx, event_loop) = ContextBuilder::new("ElementIdle", "AcarPDX")
        .build()
        .expect("Could not create a game instant (context)");

    let my_game = GameState::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}
