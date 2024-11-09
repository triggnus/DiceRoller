use std::str::FromStr;
use eframe::egui;
use egui::Key;
use eframe::egui::text::LayoutJob;
use eframe::egui::Vec2;
use rand::Rng;
use thousands::Separable;

fn roll(sides: i32) -> f64
{
    rand::thread_rng().gen_range(1..=sides) as f64
}

fn main() -> eframe::Result
{
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0]),
        centered: true,
        ..Default::default()
    };

    //let die = 6;
    //let die:i32 = get_from_stdin("How Many Sides: ");
    //let times = 100_000_000;
    //let times:i32 = get_from_stdin("How Many Times to Roll: ");

    /*println!("Critical hit odds. Rolling twice versus rolling once and multiplying by 2.");
    println!("\tRolling (2d{}) {} times...", die, times.separate_with_commas());

    let mut a:f64 = 0.0;
    for _ in 0..times
    {
        a = a + roll(die) + roll(die);
    }

    println!("\tAverage roll: {:.5}\n", a / (times as f64));
    println!("\tRolling (1d{} x 2) {} times...", die, times.separate_with_commas());

    let mut b:f64 = 0.0;
    for _ in 0..times
    {
        b = b + (2.0 * roll(die));
    }

    println!("\tAverage roll: {:.5}\n", b / (times as f64));
    println!("Percent Difference: {:.5}%", (f64::max(a, b) - f64::min(a, b)) / f64::max(a, b) * 100f64)*/

    eframe::run_native(
        "Critical Hit Odds",
        options,
        Box::new(|_| {
            Ok(Box::<DiceApp>::default())
        }),
    )
}

struct DiceApp
{
    sides: String,
    times: String,
    results: String,
}

impl Default for DiceApp
{
    fn default() -> Self
    {
        Self
        {
            sides: "12".to_owned(),
            times: "1000000".to_owned(),
            results: "".to_owned(),
        }
    }
}

impl eframe::App for DiceApp
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::CentralPanel::default().show(ctx, |ui| {
            let Self { sides, times, results } = self;

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

            //ui.label(format!("Hello '{}', age {}", self.name, self.age));


            let s = i32::from_str(sides).unwrap_or(0);
            let t = i32::from_str(times).unwrap_or(0);

            let btn = egui::Button::new("Roll <Enter>").min_size(Vec2::new(80.0, 30.0));
            let r = ui.add(btn);

            if s == 0 || t == 0
            {
                return;
            }

            if r.clicked() || ctx.input(|i| i.key_pressed(Key::Enter))
            {
                *results = format!("Rolling (2d{}) {} times...\n", sides, times.separate_with_commas());

                let mut a = 0.0;
                for _ in 0..t
                {
                    a = a + roll(s) + roll(s);
                }

                results.push_str(format!("Average roll: {:.5}\n", a / (t as f64)).as_str());
                results.push_str(format!("Rolling (1d{} x 2) {} times...\n", sides, times.separate_with_commas()).as_str());

                let mut b:f64 = 0.0;
                for _ in 0..t
                {
                    b = b + (2.0 * roll(s));
                }

                results.push_str(format!("Average roll: {:.5}\n", b / (t as f64)).as_str());
                results.push_str(format!("Percent Difference: {:.5}%", (f64::max(a, b) - f64::min(a, b)) / f64::max(a, b) * 100f64).as_str());
            }

           /* ui.centered_and_justified(move |ui| {
                ui.disable();
                ui.text_edit_multiline(&mut self.results);
            });*/
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut job = LayoutJob::single_section(
                    self.results.to_owned(),
                    egui::TextFormat::default(),
                );

                job.wrap = egui::text::TextWrapping::default();

                // NOTE: `Label` overrides some of the wrapping settings, e.g. wrap width
                ui.label(job);
            });
        });
    }
}