use sycamore::prelude::*;
use sycamore::suspense::*;
use sycamore::futures::spawn_local_scoped;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use url::Url;
use chrono::{Utc, DateTime, Duration};


#[derive(Clone, Copy, PartialEq, Eq)]
struct CanvasParams(i32);

#[derive(Clone, PartialEq, PartialOrd)]
struct DemandCurveInput {
    timestamp: DateTime<Utc>,
    value: f64,
}

async fn get_demand_curve_data_response(url: Url) -> std::result::Result<serde_json::Value, String> {
    let url_string = url.to_string();
    let request_result = reqwest::get(url).await;

    if request_result.is_err() {
        //let errorMessage = request_result.as_ref().err().unwrap().to_string();
        Err(format!("An error occurred while requesting {}: {}", url_string, request_result.as_ref().err().unwrap()))
    } else {
        let parse_result = request_result.unwrap().json::<serde_json::Value>().await;
        match parse_result {
            Ok(r) => Ok(r),
            Err(e) => Err(format!("Response is not a JSON: {}", e)),
        }
    }
}

#[component]
async fn DemandCurve<G: Html>(cx: Scope<'_>) -> View<G> {
    let canvas_params = create_signal(cx, CanvasParams(2));
    provide_context_ref(cx, canvas_params);

    let output = create_signal(cx, String::from("nothing"));
    let demand_curve_data_endpoint = create_signal(cx, String::from("test_data.json"));
    let demand_curve_data_from = create_signal(cx, String::new());
    let demand_curve_data_to = create_signal(cx, String::new());

    create_effect(cx, move || {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let demand_curve_data_endpoint_text = (*demand_curve_data_endpoint.get()).clone();
        let request_url;
        if demand_curve_data_endpoint_text.starts_with("http") {
            request_url = Url::parse(demand_curve_data_endpoint_text.as_str());
        } else {
            request_url = Url::parse(document.url().unwrap().as_str())
            .unwrap()
            .join(demand_curve_data_endpoint_text.as_str());
        }
        demand_curve_data_from.track();
        demand_curve_data_to.track();
        if request_url.is_err() {
            output.set(format!("Could not parse url {}", demand_curve_data_endpoint_text));
        } else {
            spawn_local_scoped(cx, async move {
                let from_val = &demand_curve_data_from.get();
                let to_val = &demand_curve_data_to.get();
                if from_val.to_string().is_empty() || to_val.to_string().is_empty() {
                    output.set("Set date range parameter".to_string());
                } else {
                    let response = get_demand_curve_data_response(request_url.unwrap()).await;
                    match response {
                        Ok(j) => output.set(match j.as_array() {
                            Some(json_array) => {
                                let demand_curve_inputs: Vec<DemandCurveInput> = json_array.iter().map(|v| {
                                        let timestamp = DateTime::parse_from_rfc3339(v.get("timestamp").unwrap().as_str().unwrap())
                                        .unwrap()
                                        .with_timezone(&Utc);
                                        let value: f64 = v.get("value").unwrap().as_f64().unwrap();
                                        return DemandCurveInput {
                                            timestamp: timestamp,
                                            value: value,
                                        };        
                                    })
                                    .collect();
                                web_sys::console::log_1(&format!("Trying to parse from value {}", from_val).into());
                                let from_offset = DateTime::parse_from_rfc3339(from_val).unwrap();
                                web_sys::console::log_1(&format!("Parsed from value to fixed offset datetime: {}", from_offset).into());
                                let from = DateTime::<Utc>::from_utc(from_offset.naive_utc(), Utc);
                                web_sys::console::log_1(&format!("Parsed from to DateTime<Utc> {}", from).into());
                                web_sys::console::log_1(&format!("Trying to parse to value {}", to_val).into());
                                let to_offset = DateTime::parse_from_rfc3339(to_val).unwrap();
                                web_sys::console::log_1(&format!("Parsed to value to fixed offset datetime: {}", to_offset).into());
                                let to = DateTime::<Utc>::from_utc(to_offset.naive_utc(), Utc);
                                web_sys::console::log_1(&format!("Parsed to to DateTime<Utc> {}", to).into());

                                if from < to {
                                    draw_demand_curve_time_series("canvas", from, to, demand_curve_inputs).await;
                                    format!("Found JSON array with size {}", json_array.len().to_string())
                                } else {
                                    format!("From parameter needs to be before To parameter")
                                }
                            }
                            None => "JSON is not an array".to_string(),
                        }),
                        Err(m) => output.set(m),
                    }
                }
            });
        }

    });

    view! { cx,
            label(for="demand_curve_data_endpoint") {
                "Data Source URL"
            }
            input(id="demand_curve_data_endpoint", type="text", bind:value=demand_curve_data_endpoint)
            input(id="demand_curve_data_from", type="hidden", bind:value=demand_curve_data_from)
            input(id="demand_curve_data_to", type="hidden", bind:value=demand_curve_data_to)
            p(id="output") {
                (output.get())
            }
            canvas(id="canvas", width="800", height="600", style="padding-left: 0;padding-right: 0;margin-left: auto;margin-right: auto;display: block;")
    }
}

fn main() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let main_div = document.get_element_by_id("main_div").unwrap();
    sycamore::render_to(|cx| {
        view! { cx,
            Suspense(fallback=view! {cx,
                "Loading DemandCurve..."
            }) {
                DemandCurve {}
            }
        }
    }, &main_div);
}

fn fit_demand_curve_inputs_into_datetime_range(from: DateTime<Utc>, to: DateTime<Utc>, demand_curve_inputs: Vec<DemandCurveInput>) -> Vec<DemandCurveInput> {
    if demand_curve_inputs.is_empty() {
        demand_curve_inputs
    } else {
        let mut last_demand_before_from = Option::None;
        let mut fitted_demand_curve_inputs = Vec::new();
        let mut found_input_with_from_timestamp = false;
        for i in 0..demand_curve_inputs.len() {
            let demand_curve_input = demand_curve_inputs.get(i).unwrap();
            if demand_curve_input.timestamp < from {
                last_demand_before_from = Some(demand_curve_input);
            }
            if demand_curve_input.timestamp >= from && demand_curve_input.timestamp < to {
                if demand_curve_input.timestamp == from {
                    found_input_with_from_timestamp = true;
                }
                web_sys::console::log_1(&format!("Including DemandCurveInput with timestamp {}, from {}, to {}", demand_curve_input.timestamp, from, to).into());
                fitted_demand_curve_inputs.push(DemandCurveInput { timestamp: demand_curve_input.timestamp, value: demand_curve_input.value });
            } else {
                web_sys::console::log_1(&format!("Not including DemandCurveInput with timestamp {}, from {}, to {}", demand_curve_input.timestamp, from, to).into());
            }
        }

        if fitted_demand_curve_inputs.is_empty() || !found_input_with_from_timestamp {
            web_sys::console::log_1(&"Checking if empty demand curve inputs can be filled with previously starting DemandCurveInput ...".into());
            match last_demand_before_from {
                Some(d) => {
                    web_sys::console::log_1(&format!("Filling emtpy demand curve inputs with DemandCurveInput with timestamp {} and value {}", d.timestamp, d.value).into());
                    fitted_demand_curve_inputs.insert(0, DemandCurveInput { timestamp: from, value: d.value });
                },
                None => {},
            }
        }
        fitted_demand_curve_inputs
    }
}

async fn draw_demand_curve_time_series(canvas_id: &str, from: DateTime<Utc>, to: DateTime<Utc>, demand_curve_inputs: Vec<DemandCurveInput>) {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 30.0).into();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(40)
        .caption(
            "Demand",
            font,
        )
        .x_label_area_size(30u32)
        .y_label_area_size(50u32)
        // adding another minute because range is end exclusive
        .build_cartesian_2d(from..(to + Duration::minutes(1)),
        -50.0..50.0,
        )
        .unwrap();
    
    chart.configure_mesh()
    .disable_y_mesh()
    .x_labels(5)
    .max_light_lines(15)
    .y_desc("MW")
    .draw()
    .unwrap();

    let fitted_inputs = fit_demand_curve_inputs_into_datetime_range(from, to, demand_curve_inputs);

    if !fitted_inputs.is_empty() {
        let mut data_points:Vec<(DateTime<Utc>, f64)> = Vec::new();
        let mut previous_input = fitted_inputs.first().unwrap();
        data_points.push((previous_input.timestamp, previous_input.value));
        for i in 1..fitted_inputs.len() {
            let input = fitted_inputs.get(i).unwrap();
            data_points.push((input.timestamp, previous_input.value));
            data_points.push((input.timestamp, input.value));
            previous_input = input;
        }
        data_points.push((to, previous_input.value));

        chart.draw_series(LineSeries::new(
            data_points.iter().map(|d| (d.0, d.1)),
            &RED,
        ))
        .unwrap();
    }

    let zero_line = [(from, 0.0), (to, 0.0)];

    chart.draw_series(LineSeries::new(
        zero_line.iter().map(|d| (d.0, d.1)),
        &BLACK,
    ))
    .unwrap();

    root.present().unwrap();
}