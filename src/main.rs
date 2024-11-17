//! # DiceRoll
//!
//! Attempts to determine if rolling two dice yields the same result on average, as rolling one die and doubling and
//! measuring the difference.
//! Presumably, there is an elegant statistical method for determining exactly this, but it's easy enough to roll a
//! billion virtual dice and average the result.

mod dice_roller;
use eframe::egui;
use egui::Key;
use eframe::egui::text::LayoutJob;
use eframe::egui::Vec2;
use thousands::Separable;

use dice_roller::DiceRoller;

fn main() -> eframe::Result
{
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Critical Hit Odds",
        options,
        Box::new(|_| {
            Ok(Box::<DiceApp>::default())
        }),
    )
}

/// # DiceApp
/// Egui requires a struct the implements eframe::App
/// All events happen within the context of this struct, so I made the whole program live within it.
/// Ideally, this would be a separate module.
///
/// TODO: Make this a separate module.
struct DiceApp
{
    sides: String,
    times: String,
    roller: DiceRoller,
    /*value_a: Arc<Mutex<f64>>,
    value_b: Arc<Mutex<f64>>,
    results: String,
    handle_a: JoinHandle<()>,
    handle_b: JoinHandle<()>,
    rolling_a: bool,
    rolling_b: bool,
    ready_a: bool,
    ready_b: bool,*/
}

impl Default for DiceApp
{
    fn default() -> Self
    {
        Self
        {
            sides: "12".to_owned(),
            times: "100000000".to_owned(),
            roller: DiceRoller::new(),
        }
    }
}

impl eframe::App for DiceApp
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::CentralPanel::default().show(ctx, |ui| {
            let Self { sides, times, roller, .. } = self;

            // this code catches any non-integer values
            roller.sides =  sides.parse().map_or_else(|_|
                                                          {
                                                              roller.results = "Invalid Input".to_owned();
                                                              *sides = format!("{}", roller.sides).to_string();
                                                              roller.sides
                                                          },
                                                      |v|v);
            roller.times = times.parse().map_or_else(|_|
                                                         {
                                                             roller.results = "Invalid Input".to_owned();
                                                             *times = format!("{}", roller.times).to_string();
                                                             roller.times
                                                         }, |v|v);

            // START: UI construction
            ui.heading("Dice Roller");

            ui.horizontal(|ui| {
                let sides_label = ui.label("Sides on Dice: ");
                ui.text_edit_singleline(sides)
                  .labelled_by(sides_label.id);
            });

            ui.horizontal(|ui| {
                let times_label = ui.label("Times to Roll: ");
                ui.text_edit_singleline(times)
                  .labelled_by(times_label.id);
            });

            let btn = egui::Button::new("Roll <Enter>").min_size(Vec2::new(80.0, 30.0));
            let r = ui.add(btn);



            // when the 'run' button is clicked or the user presses 'Enter'
            if r.clicked() || ctx.input(|i| i.key_pressed(Key::Enter))
            {
                if roller.times > 1_000_000_000
                {
                    roller.results = "Too many times to roll.\nMax: 1 billion rolls.".to_owned();
                    return;
                }

                // Catch an edge case where both sides and times are zero.
                if roller.sides == 0 || roller.times == 0
                {
                    return;
                }
                // we only want to start rolling if no dice are rolling.
                if !roller.rolling_a && !roller.rolling_b
                {
                    // reset results
                    roller.reset();

                    // start rolling and store the handles
                    roller.handle_a = roller.start_rolling_a();
                    roller.handle_b = roller.start_rolling_b();
                }
            }

            // if Method A started rolling and then finished
            if roller.rolling_a &&roller.handle_a.is_finished()
            {
                roller.results.push_str(format!("Rolling (2d{}) {} times...\n",
                                                roller.sides, roller.times.separate_with_commas()).as_str());
                roller.results.push_str(format!("Average roll: {:.5}\n", roller.result_a()).as_str());

                roller.rolling_a = false;
                roller.ready_a = true;
                //println!("Method A - {}", self.result_a());
            }

            // if Method B started rolling and then finished
            if roller.rolling_b && roller.handle_b.is_finished()
            {
                roller.results.push_str(format!("Rolling (1d{} x 2) {} times...\n",
                                                roller.sides, roller.times.separate_with_commas()).as_str());
                roller.results.push_str(format!("Average roll: {:.5}\n", roller.result_b()).as_str());

                roller.rolling_b = false;
                roller.ready_b = true;
                //println!("Method B - {}", self.result_b());
            }

            // if both results are in, we can summarize.
            if  roller.ready_a && roller.ready_b
            {
                let a = roller.value_a.lock().unwrap().clone();
                let b = roller.value_b.lock().unwrap().clone();

                roller.results.push_str(
                    format!("\nPercent Difference: {:.5}%", (f64::max(a, b) - f64::min(a, b)) / f64::max(a, b) * 100f64)
                    .as_str()
                );

                // full reset.
                roller.ready_a = false;
                roller.ready_b = false;
            }

            let mut result = roller.results.clone();

            if roller.rolling_a || roller.rolling_b
            {
                result.push_str("Working...");
                //"Working...".to_string()
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut job = LayoutJob::single_section(
                    result,
                    egui::TextFormat::default(),
                );

                job.wrap = egui::text::TextWrapping::default();

                ui.label(job);
            });

            // repainting is slightly less efficient, but it's the only way the UI updates between job completions.
            ctx.request_repaint();
        });
    }
}