use chrono::{DateTime, Local};
use ggegui::egui::ScrollArea;
use ggegui::{Gui, egui};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam};
use ggez::{Context, ContextBuilder, GameResult, glam};

struct Upgrade {}

//Add rate and upgrades
struct Element {
    name: String,
    number: u8,
    amount: u64,
}

struct Currency {
    money: u64,
    click_rate: u64,
    auto_rate: u64,
    auto_gatherer: u32,
}

// add a field called upgrades: [Upgrade; 3]
impl Currency {
    fn new() -> Currency {
        Currency {
            money: 0,
            click_rate: 1,
            auto_rate: 1,
            auto_gatherer: 0,
        }
    }

    fn num_money(&self) -> &u64 {
        &self.money
    }

    fn collect_money(&mut self) {
        self.money += self.click_rate;
    }

    fn update_money_auto(&mut self, time_passed: u64) {
        self.money += self.click_rate * time_passed;
    }

    fn auto_coll(&self) -> bool {
        self.auto_gatherer != 0
    }

    fn can_buy_auto(&self) -> bool {
        self.money > 20
    }

    fn auto_gatherer_buy(&mut self) {
        self.money -= 20;
        self.auto_gatherer += 1;
    }
}

struct GameState {
    gui: Gui,
    currency: Currency,
    time: DateTime<Local>,
    elements: Vec<Element>,
    elements_unlckd: usize,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameState {
        //obtain the names of the elements and atomic numbers.
        let iter = (1..40).map(|x| Element {
            name: format!("Element {}", x),
            number: x,
            amount: 0,
        });
        // Load/create resources such as images here.
        GameState {
            gui: Gui::new(ctx),
            currency: Currency::new(),
            time: chrono::Local::now(),
            elements: Vec::from_iter(iter),
            elements_unlckd: 10,
        }
    }

    pub fn calculate_seconds_passed(&mut self) -> u64 {
        let time = (self.time - chrono::Local::now())
            .num_seconds()
            .unsigned_abs();
        if time >= 1 {
            self.time = chrono::Local::now();
        }
        time
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let gui_ctx = self.gui.ctx();

        if self.currency.auto_coll() {
            let time_passed = self.calculate_seconds_passed();
            self.currency.update_money_auto(time_passed);
        }

        egui::TopBottomPanel::top("top_panel")
            .min_height(120.0)
            .show(&gui_ctx, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.heading("Collect Money");
                    ui.label(self.currency.num_money().to_string());
                    if ui.button("Click").clicked() {
                        self.currency.collect_money();
                    }
                    ui.add_enabled_ui(self.currency.can_buy_auto(), |ui| {
                        if ui.button("Buy auto collector").clicked() {
                            self.currency.auto_gatherer_buy();
                        }
                    });
                });
            });

        egui::CentralPanel::default().show(&gui_ctx, |ui| {
            ui.heading("The Elements are here");
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::TOP).with_cross_justify(true),
                |ui| {
                    ScrollArea::vertical()
                        .stick_to_right(true)
                        .max_width(300.0)
                        .show(ui, |ui| {
                            ui.separator();
                            for elem in &self.elements[0..self.elements_unlckd] {
                                ui.horizontal(|ui| {
                                    ui.label(&elem.name);
                                    ui.label(elem.amount.to_string());
                                });
                                ui.horizontal(|ui| {
                                    if ui.button("Button 1").clicked() {}
                                    if ui.button("Button 2").clicked() {}
                                });
                                ui.separator();
                            }
                        });
                },
            );
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
