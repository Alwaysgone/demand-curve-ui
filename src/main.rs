use sycamore::prelude::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
//use env_logger::Builder;

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

#[component]
fn DemandCurve<G: Html>(cx: Scope) -> View<G> {
    let canvas_params = create_signal(cx, CanvasParams(2));
    provide_context_ref(cx, canvas_params);
    let power_value = create_signal(cx, 2);

    let output = create_signal(cx, String::from("nothing"));
    //let text:String = String::from("/test_data.json");
    let demand_curve_data_endpoint = create_signal(cx, String::from("/test_data.json"));
    let update = create_effect(cx, || {
        let request_url = (*demand_curve_data_endpoint.get()).clone();
        /*
        let request_result = reqwest::get(&request_url);
        if request_result.is_err() {
            output.set(request_result.err().unwrap().to_string());
        } else {
            output.set(request_result.unwrap().text().unwrap());
        }
        */
        //demand_curve_data_endpoint.track();
        output.set((*demand_curve_data_endpoint.get()).clone());
    });
        //value="/test_data.json"
    view! { cx,
        div(style="position: relative;display: flex;flex-flow: column wrap;align-items: center;") {
            input(id="demand_curve_data_endpoint", type="text", bind:value=demand_curve_data_endpoint)
            p(id="power_value") {
                //(canvas_params.get().get_power_value())
                (power_value.get())
            }
            p(id="output") {
                (output.get())
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
        }
        
        canvas(id="canvas", width="800", height="600", style="padding-left: 0;padding-right: 0;margin-left: auto;margin-right: auto;display: block;")
    }
}

fn main() {
    // env_logger::Builder::new()
    // .init();
    
    sycamore::render(|cx| {
        
        //draw("canvas", canvas_params.get().get_power_value());
        let view_model = view! { cx,
            // Suspense(fallback=view! {cx,
            //     "Loading DemandCurve..."
            // }) {
                DemandCurve {}
            // }
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