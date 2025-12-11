use chrono::{DateTime, Local};
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

enum UpgradeEffect {
    Additive(Decimal),
    Multiplicative(Decimal),
}

#[derive(Copy, Clone)]
enum UpgradeType {
    AutoClickerQuantity,
    ProductionBoost,
}

pub struct Upgrade {
    generator_id: GeneratorID,
    name: String,
    effect: UpgradeEffect,
    upgrade_type: UpgradeType,
    tier: u64,
    base_cost: Decimal,
    current_cost: Decimal,
}

impl Upgrade {
    fn check_intended_generator(&self) -> GeneratorID {
        self.generator_id
    }

    fn check_type_and_effect(&self) -> (UpgradeType, &UpgradeEffect) {
        (self.upgrade_type, &self.effect)
    }
}

impl Detail for Upgrade {
    fn check_name(&self) -> &str {
        &self.name
    }

    fn check_quantity(&self) -> u64 {
        self.tier
    }
}

impl Cost for Upgrade {
    fn check_cost(&self) -> Decimal {
        self.current_cost
    }

    fn obtain_new_cost(&mut self) {
        let rate = match self.effect {
            UpgradeEffect::Additive(_) => ADD_UPG_RATE,
            UpgradeEffect::Multiplicative(_) => MULT_UPG_RATE,
        };

        self.current_cost = self.base_cost * rate.powu(self.tier)
    }
}
//************************************************************************************************ */
// Resource Manager
//************************************************************************************************ */
#[derive(Copy, Clone, PartialEq, Eq)]
enum GeneratorID {
    Clicker,
    AutoClicker,
}

struct Generator {
    name: String,
    quantity: u64,
    production_rate: Decimal,
    multiplier: Decimal,
}

impl Generator {
    fn increment_quantity(&mut self) {
        self.quantity += 1;
    }

    fn calculate_production(&self) -> Decimal {
        self.production_rate * Decimal::from(self.quantity) * self.multiplier
    }

    fn apply_upgrade(&mut self, upgrade_effect: &UpgradeEffect) {
        match upgrade_effect {
            UpgradeEffect::Additive(value) => {
                self.production_rate += value;
            }
            UpgradeEffect::Multiplicative(value) => {
                self.multiplier *= value;
            }
        }
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
                name: String::from("Scoop"),
                quantity: 1,
                production_rate: dec!(1),
                multiplier: dec!(1),
            },
            Generator {
                name: String::from("AutoScooper"),
                quantity: 0,
                production_rate: dec!(1),
                multiplier: dec!(1),
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
        self.electrons += self.generators[0].calculate_production();
    }

    fn update_time(&mut self) {
        self.time = chrono::Local::now();
    }

    fn calculate_seconds_passed(&mut self) -> u64 {
        let time = (self.time - chrono::Local::now())
            .num_seconds()
            .unsigned_abs();
        if time >= 1 {
            self.update_time();
        }
        time
    }

    fn can_purchase(&self, cost: Decimal) -> bool {
        self.electrons >= cost
    }

    fn purchase_upgrade(
        &mut self,
        cost: Decimal,
        generator_id: GeneratorID,
        type_and_effect: (UpgradeType, &UpgradeEffect),
    ) {
        if self.electrons >= cost {
            self.electrons -= cost;

            let index = match generator_id {
                GeneratorID::Clicker => 0,
                GeneratorID::AutoClicker => 1,
            };

            match type_and_effect.0 {
                UpgradeType::AutoClickerQuantity => {
                    if self.generators[index].check_quantity() == 0 {
                        self.update_time();
                    }
                    self.generators[index].increment_quantity();
                }
                UpgradeType::ProductionBoost => {
                    self.generators[index].apply_upgrade(type_and_effect.1)
                }
            }
        }
    }

    pub fn update(&mut self) {
        let time_passed = self.calculate_seconds_passed();
        self.electrons += Decimal::from(time_passed) * self.generators[1].calculate_production();
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
                effect: UpgradeEffect::Additive(dec!(1)),
                upgrade_type: UpgradeType::ProductionBoost,
                tier: 0,
                base_cost: dec!(15),
                current_cost: dec!(15),
            },
            Upgrade {
                generator_id: GeneratorID::AutoClicker,
                name: String::from("AutoScooper: Add AutoScooper"),
                effect: UpgradeEffect::Additive(dec!(1)),
                upgrade_type: UpgradeType::AutoClickerQuantity,
                tier: 0,
                base_cost: dec!(15),
                current_cost: dec!(15),
            },
            Upgrade {
                generator_id: GeneratorID::AutoClicker,
                name: String::from("AutoScooper: Increase Scooped Amount"),
                effect: UpgradeEffect::Multiplicative(dec!(1)),
                upgrade_type: UpgradeType::ProductionBoost,
                tier: 0,
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

    fn is_purchase_possible(&self, cost: Decimal) -> bool {
        self.resource_manager.can_purchase(cost)
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
                        ui.horizontal(|ui| {
                            ui.label(i.check_name());
                            ui.label("Cost:");
                            ui.label(i.check_cost().to_string());
                            ui.label("Electrons");
                        });
                        ui.add_enabled_ui(self.is_purchase_possible(i.check_cost()), |ui| {
                            if ui.button("Purchase").clicked() {
                                self.resource_manager.purchase_upgrade(
                                    i.check_cost(),
                                    i.check_intended_generator(),
                                    i.check_type_and_effect(),
                                );
                            }
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
