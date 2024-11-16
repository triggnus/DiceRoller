use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
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
    sides: i32,
    times: i32,
    value_a: Arc<Mutex<f64>>,
    value_b: Arc<Mutex<f64>>,
    results: String,
    done_a: bool,
    done_b: bool,
    handle_a: JoinHandle<()>,
    handle_b: JoinHandle<()>,
}

impl Default for DiceApp
{
    fn default() -> Self
    {
        Self
        {
            sides: 12,
            times: 100_000_000,
            value_a: Arc::new(Mutex::new(0.0)),
            value_b: Arc::new(Mutex::new(0.0)),
            results: "".to_owned(),
            done_a: true,
            done_b: true,
            handle_a: thread::spawn(move || {}),
            handle_b: thread::spawn(move || {}),
        }
    }
}

impl DiceApp
{

    fn start_rolling_a(&self) -> JoinHandle<()>
    {
        let value = Arc::clone(&self.value_a);
        let sides = self.sides.clone();
        let times = self.times.clone();

        let handle = thread::spawn(move || {
            let mut a = value.lock().unwrap();

            for _ in 0..times
            {
                *a += roll(sides) + roll(sides);
            }
            println!("Method A - Done.");
        });

        println!("Method A - Loaded.");
        handle
    }

    fn start_rolling_b(&self) -> JoinHandle<()>
    {
        let value = Arc::clone(&self.value_b);
        let sides = self.sides.clone();
        let times = self.times.clone();

        let handle = thread::spawn(move || {
            //self.working_b.store(true, std::sync::atomic::Ordering::Relaxed);
            let mut a = value.lock().unwrap();

            for _ in 0..times
            {
                *a += 2.0 * roll(sides);
            }
            println!("Method B - Done.");
            //self.working_b.store(false, std::sync::atomic::Ordering::Relaxed);
        });

        println!("Method B - Loaded.");
        handle
    }

    fn result_a(&self) -> f64
    {
        let total = self.value_a.lock().unwrap().clone();
        return total / self.times as f64;
    }

    fn result_b(&self) -> f64
    {
        let total = self.value_b.lock().unwrap().clone();
        return total / self.times as f64;
    }
}

impl eframe::App for DiceApp
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::CentralPanel::default().show(ctx, |ui| {
            let Self { sides, times, .. } = self;

            let sides: i32 = *sides;
            let times: i32 = *times;

            let sides_str = &mut format!("sides: {}", sides);
            let times_str = &mut format!("times: {}", times);

            ui.heading("Dice Roller");

            ui.horizontal(|ui| {
                let sides_label = ui.label("Sides on Dice: ");
                ui.text_edit_singleline(sides_str)
                  .labelled_by(sides_label.id);
            });

            ui.horizontal(|ui| {
                let times_label = ui.label("Times to Roll: ");
                ui.text_edit_singleline(times_str)
                  .labelled_by(times_label.id);
            });

            //ui.label(format!("Hello '{}', age {}", self.name, self.age));


            let btn = egui::Button::new("Roll <Enter>").min_size(Vec2::new(80.0, 30.0));
            let r = ui.add(btn);

            if sides == 0 || times == 0
            {
                return;
            }

            if r.clicked() || ctx.input(|i| i.key_pressed(Key::Enter))
            {
                self.done_a = false;
                self.done_b = false;
                self.results = "".to_owned();
               /* let handle_a = self.start_rolling_a();
                let handle_b = self.start_rolling_b();*/

                self.handle_a = self.start_rolling_a();
                self.handle_b = self.start_rolling_b();

                /* *results = format!("Rolling (2d{}) {} times...\n", sides, times.separate_with_commas());

                let mut a = 0.0;
                for _ in 0..times
                {
                    a = a + roll(sides) + roll(sides);
                }

                results.push_str(format!("Average roll: {:.5}\n", a / (times as f64)).as_str());
                results.push_str(format!("Rolling (1d{} x 2) {} times...\n", sides, times.separate_with_commas()).as_str());

                let mut b:f64 = 0.0;
                for _ in 0..times
                {
                    b = b + (2.0 * roll(sides));
                }

                results.push_str(format!("Average roll: {:.5}\n", b / (times as f64)).as_str());
                results.push_str(format!("Percent Difference: {:.5}%", (f64::max(a, b) - f64::min(a, b)) / f64::max(a, b) * 100f64).as_str());*/
            }

            if self.handle_a.is_finished() && !self.done_a
            {
                self.done_a = true;

                self.results.push_str(format!("Rolling (2d{}) {} times...\n", sides, times.separate_with_commas()).as_str());
                self.results.push_str(format!("Average roll: {:.5}\n", self.value_a.lock().unwrap().clone() / (times as f64)).as_str());

                println!("Method A - {}", self.result_a());
            }

            if self.handle_b.is_finished() && !self.done_b
            {
                self.done_b = true;

                self.results.push_str(format!("Rolling (1d{} x 2) {} times...\n", sides, times.separate_with_commas()).as_str());
                self.results.push_str(format!("Average roll: {:.5}\n", self.value_b.lock().unwrap().clone() / (times as f64)).as_str());

                println!("Method B - {}", self.result_b());
            }

           /* ui.centered_and_justified(move |ui| {
                ui.disable();
                ui.text_edit_multiline(&mut self.results);
            });*/
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut job = LayoutJob::single_section(
                    self.results.clone(),
                    egui::TextFormat::default(),
                );

                job.wrap = egui::text::TextWrapping::default();

                // NOTE: `Label` overrides some of the wrapping settings, e.g. wrap width
                ui.label(job);
            });
            ctx.request_repaint();
        });
    }
}