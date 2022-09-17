use sycamore::prelude::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

#[derive(Clone, Copy, PartialEq, Eq)]
struct CanvasParams(i32);

impl CanvasParams {
    fn get_power_value(self) -> i32 {
        self.0
    }

    fn increase_power_value(mut self) {
        self.0 = self.0 + 1;
    }
}

fn main() {
    sycamore::render(|cx| {
        let canvas_params = create_signal(cx, CanvasParams(2));
        provide_context_ref(cx, canvas_params);
        let power_value = create_signal(cx, 2);
        //draw("canvas", canvas_params.get().get_power_value());
        let view_model = view! { cx,
            p(id="power_value") {
                //(canvas_params.get().get_power_value())
                (power_value.get())
            }
            //p { "Hello, World!" }
            button(on:click=|_| {   
                                    //let params = use_context::<Signal<CanvasParams>>(cx);
                                    power_value.set(*power_value.get() + 1);
                                    //canvas_params.set(CanvasParams(canvas_params.get().get_power_value() + 1));
                                    //draw("canvas", canvas_params.get().get_power_value());
                                    draw("canvas", *power_value.get());
                                }) {
                "Increase power"
            }
            canvas(id="canvas", width="600", height="400")
        };

        //draw("canvas", *power_value.get());
        return view_model;
        
    });
}

pub fn draw(canvas_id: &str, power: i32) {
    //-> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>>
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)
    .unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption(format!("y=x^{}", power), font)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(-1f32..1f32, -1.2f32..1.2f32)
        .unwrap();

    chart.configure_mesh().x_labels(3).y_labels(3).draw()
    .unwrap();

    chart.draw_series(LineSeries::new(
        (-50..=50)
            .map(|x| x as f32 / 50.0)
            .map(|x| (x, x.powf(power as f32))),
        &RED,
    ))
    .unwrap();

    root.present()
    .unwrap();
    //return Ok(chart.into_coord_trans());
}