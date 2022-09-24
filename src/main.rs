use sycamore::prelude::*;
use sycamore::suspense::*;
use sycamore::futures::spawn_local_scoped;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use url::Url;
use chrono::{Utc, DateTime, Duration};
// use wasm_bindgen::prelude::*;
//use chrono::{NaiveDate, NaiveDateTime};
//use bigdecimal::BigDecimal;


#[derive(Clone, Copy, PartialEq, Eq)]
struct CanvasParams(i32);

#[derive(Clone, PartialEq, PartialOrd)]
struct DemandCurveInput {
    timestamp: DateTime<Utc>,
    value: f64,
}

// impl CanvasParams {
//     fn get_power_value(self) -> i32 {
//         self.0
//     }

//     fn increase_power_value(mut self) {
//         self.0 = self.0 + 1;
//     }
// }

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

    
    /*
    if request_result.is_err() {
        return format!("Got HTTP status {}", request_result.err().unwrap().status().unwrap());
    } else {
        request_result.unwrap().text().await.unwrap()
    }
    */
}

// #[wasm_bindgen]
// extern { fn initDatePicker(); }

#[component]
async fn DemandCurve<G: Html>(cx: Scope<'_>) -> View<G> {
    let canvas_params = create_signal(cx, CanvasParams(2));
    provide_context_ref(cx, canvas_params);
    let power_value = create_signal(cx, 2);

    let output = create_signal(cx, String::from("nothing"));
    //let text:String = String::from("/test_data.json");
    let demand_curve_data_endpoint = create_signal(cx, String::from("test_data.json"));
    //let datetime_picker_format = "%Y-%m-%d %H:%M";
    // let current_hour = Utc::now()
    // .with_minute(0).unwrap()
    // .with_second(0).unwrap()
    // .with_nanosecond(0).unwrap();
    //let demand_curve_data_from_node_ref = create_node_ref(cx);
    //current_hour.format(datetime_picker_format).to_string()
    let demand_curve_data_from = create_signal(cx, String::new());
    //(current_hour + Duration::hours(1)).format(datetime_picker_format).to_string()
    let demand_curve_data_to = create_signal(cx, String::new());

    // sycamore::web::on_mount(cx, move || {
    //     web_sys::console::log_1(&"Starting initDatePicker call timer".into());
    //     spawn_local_scoped(cx, async {
    //         futures_timer::Delay::new(std::time::Duration::from_secs(5)).await;
    //         web_sys::console::log_1(&"Calling initDatePicker".into());
    //         initDatePicker();
    //     });
    // });

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
        //let request_url_str = request_url.to_string();
        //output.set(request_url.to_string());
        demand_curve_data_from.track();
        demand_curve_data_to.track();
        if request_url.is_err() {
            output.set(format!("Could not parse url {}", demand_curve_data_endpoint_text));
        } else {
            spawn_local_scoped(cx, async move {
                let from_val = &demand_curve_data_from.get();
                let to_val = &demand_curve_data_to.get();
                if from_val.to_string().is_empty() || to_val.to_string().is_empty() {
                    //web_sys::console::log_1(&format!("From val at empty check {}", from_val.to_string()).into());
                    //web_sys::console::log_1(&format!("To val at empty check {}", to_val.to_string()).into());
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
                                // let from = DateTime::parse_from_rfc3339("2022-09-17T20:00:00.000Z")
                                // .unwrap()
                                // .with_timezone(&Utc);
                                // web_sys::console::log_1(&"Parsing test value 2022-09-21 20:00".into());
                                // match DateTime::parse_from_str("2022-09-21 20:00", datetime_picker_format) {
                                //     Err(e) => web_sys::console::log_1(&format!("Error parsing test value:  {}", e).into()),
                                //     Ok(v) => web_sys::console::log_1(&format!("Parsed test value to {}", v).into()),
                                // }
                                //web_sys::console::log_1(&format!("Parsed test value to {}", test).into());
                                web_sys::console::log_1(&format!("Trying to parse from value {}", from_val).into());
                                // let from_naive = NaiveDateTime::parse_from_str(from_val, datetime_picker_format)
                                // .unwrap();
                                let from_offset = DateTime::parse_from_rfc3339(from_val).unwrap();
                                web_sys::console::log_1(&format!("Parsed from value to fixed offset datetime: {}", from_offset).into());
                                let from_naive = from_offset.naive_utc();
                                web_sys::console::log_1(&format!("Parsed from value to naive datetime: {}", from_naive).into());
                                let from = DateTime::<Utc>::from_utc(from_naive, Utc);
                                // let from_naive = NaiveDateTime::parse_from(from_val, datetime_picker_format)
                                // .unwrap();
                                // web_sys::console::log_1(&format!("Parsed from to NaiveDateTime {}", from_naive).into());
                                // let from = DateTime::<Utc>::from_utc(from_naive, Utc);
                                web_sys::console::log_1(&format!("Parsed from to DateTime<Utc> {}", from).into());
                                //.with_timezone(&Utc);
                                // web_sys::console::log_1(&format!("Parsed from value to {}", from).into());
                                // let to = DateTime::parse_from_str(&demand_curve_data_to.get(), "%Y-%m-%d %H:%M")
                                // .unwrap()
                                // .with_timezone(&Utc);
                                //let from = Utc.ymd(2022, 9, 17).and_hms(20, 0, 0);
                                // let mut demand_curve_inputs: Vec<DemandCurveInput> = Vec::new();
                                // demand_curve_inputs.push(DemandCurveInput {
                                //                 timestamp: from,
                                //                 value: 10.0,
                                //             });
                                // demand_curve_inputs.push(DemandCurveInput {
                                //     timestamp: from + Duration::minutes(15),
                                //     value: 20.0,
                                // });

                                // let to_naive = NaiveDateTime::parse_from_str(to_val, datetime_picker_format)
                                // .unwrap();
                                web_sys::console::log_1(&format!("Trying to parse to value {}", to_val).into());
                                let to_offset = DateTime::parse_from_rfc3339(to_val).unwrap();
                                web_sys::console::log_1(&format!("Parsed to value to fixed offset datetime: {}", to_offset).into());
                                let to_naive = to_offset.naive_utc();
                                web_sys::console::log_1(&format!("Parsed to value to naive datetime: {}", to_naive).into());
                                let to = DateTime::<Utc>::from_utc(to_naive, Utc);
                                
                                if from < to {
                                    //let to = from + Duration::hours(1);
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
                //output.set(response);
            });
        }
        
        /*
        let request_result = reqwest::get(&request_url).await?;
        if request_result.is_err() {
            output.set(request_result.err().unwrap().to_string());
        } else {
            output.set(request_result.unwrap().text().unwrap());
        }
        */
        //demand_curve_data_endpoint.track();
        //output.set((*demand_curve_data_endpoint.get()).clone());
    });
    //2010-01-01T00:00

    // create_effect(cx, || {
    //     output.set(demand_curve_data_from.get().to_string());
    //     //DateTime::format(demand_curve_data_from.get(), "%Y-%m-%d %H:%M");
    // });
    
    //value="/test_data.json"
    // limit
    // create_effect(cx , {
    //     let to_time_string = demand_curve_data_to.get();
    //     match DateTime::parse_from_str(to_time_string, datetime_picker_format) {
    //         Ok(DateTime<Utc>) => {
    //             if 
    //         },
    //     }
    // });

    // sycamore::web::on_mount(cx, move||{
    //     //let demand_curve_data_from_max = create_signal(cx, "".to_string());
    //     create_effect(cx, ||{
    //             let max_from_value = match DateTime::parse_from_str(&demand_curve_data_to.get(), datetime_picker_format) {
    //                 Ok(d) => d.format(datetime_picker_format).to_string(),
    //                 _ => "".to_string(),
    //             };
    //             //demand_curve_data_from_node_ref.get::<DomNode>().set_attribute("max", &max_from_value);
    //         }
    //     );
    //     }
    // );

    view! { cx,
        // div(id="main_div" , style="position: relative;display: flex;flex-flow: column wrap;align-items: center;") {
            label(for="demand_curve_data_endpoint") {
                "Data Source URL"
            }
            input(id="demand_curve_data_endpoint", type="text", bind:value=demand_curve_data_endpoint)
            // label(for="demand_curve_data_from") {
            //     "From"
            // }
            //bind:value=demand_curve_data_from
            input(id="demand_curve_data_from", type="hidden", bind:value=demand_curve_data_from
            // on:input=|_| {
            //     let window = web_sys::window().expect("no global `window` exists");
            //     let document = window.document().expect("should have a document on window");
            //     let node_value = web_sys::Document::get_element_by_id(&document, "demand_curve_data_from_hidden").unwrap()
            //     .node_value()
            //     .unwrap();
            //     web_sys::console::log_1(&format!("Using from node value {}", node_value).into());
            //     demand_curve_data_from.set(node_value);
            // }
            )
            //input(ref=demand_curve_data_from_node_ref, id="demand_curve_data_from", type="datetime-local", bind:value=demand_curve_data_from)
            // label(for="demand_curve_data_to") {
            //     "To"
            // }
            input(id="demand_curve_data_to", type="hidden", bind:value=demand_curve_data_to)
            // p(id="power_value") {
            //     //(canvas_params.get().get_power_value())
            //     (power_value.get())
            // }
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
        // }
        
        canvas(id="canvas", width="800", height="600", style="padding-left: 0;padding-right: 0;margin-left: auto;margin-right: auto;display: block;")
    }
}

fn main() {
    // env_logger::Builder::new()
    // .init();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let main_div = document.get_element_by_id("main_div").unwrap();
    sycamore::render_to(|cx| {
        
        //draw("canvas", canvas_params.get().get_power_value());
        let view_model = view! { cx,
            Suspense(fallback=view! {cx,
                "Loading DemandCurve..."
            }) {
                DemandCurve {}
            }
        };

        //draw("canvas", *power_value.get());
        return view_model;
        
    }, &main_div);
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

fn fit_demand_curve_inputs_into_datetime_range(from: DateTime<Utc>, to: DateTime<Utc>, demand_curve_inputs: Vec<DemandCurveInput>) -> Vec<DemandCurveInput> {
    // let mut dci = demand_curve_inputs.clone();
    // dci.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    // let filtered_dci = Vec::from_iter(dci.iter()
    // .filter(|i| i.timestamp >= from && i.timestamp < to)
    // );
    if demand_curve_inputs.is_empty() {
        demand_curve_inputs
    } else {
        let mut last_demand_before_from = Option::None;
        let mut fitted_demand_curve_inputs = Vec::new();
        //let mut last_input;
        let mut found_input_with_from_timestamp = false;
        for i in 0..demand_curve_inputs.len() {
            let demand_curve_input = demand_curve_inputs.get(i).unwrap();
            if demand_curve_input.timestamp < from {
                last_demand_before_from = Some(demand_curve_input);
            }
            if demand_curve_input.timestamp >= from && demand_curve_input.timestamp < to {
                if demand_curve_input.timestamp == from {
                    // this does not seem to work
                    found_input_with_from_timestamp = true;
                }
                // if !found_first_input {
                //     found_first_input = true;
                //     if demand_curve_input.timestamp > from && last_input.timestamp < from {
                //         fitted_demand_curve_inputs.push(DemandCurveInput { timestamp: from, value: last_input.value });
                //     }
                // }
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

        // if only one demand_curve_input 
        // if !found_first_input && fitted_demand_curve_inputs.is_empty() {
        //     fitted_demand_curve_inputs.push(DemandCurveInput { timestamp: from, value: last_input.value });
        // }

        // let first_input = dci.iter()
        // .filter(|i| i.timestamp >= from && i.timestamp < to)
        // .find(predicate) demand_curve_inputs.first().unwrap();
        // if first_input.timestamp <= from {
        //     fitted_demand_curve_inputs.push(DemandCurveInput { timestamp: from, value: first_input.value });
        // } else {
        //     fitted_demand_curve_inputs.push(*first_input);
        // }
        // for i in 1..demand_curve_inputs.len() {

        // }
        fitted_demand_curve_inputs
    }
}

async fn draw_demand_curve_time_series(canvas_id: &str, from: DateTime<Utc>, to: DateTime<Utc>, demand_curve_inputs: Vec<DemandCurveInput>) {
    // -> DrawResult<impl Fn((i32, i32)) -> Option<(DateTime<Utc>, f32)>>
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 30.0).into();

    root.fill(&WHITE).unwrap();

    //let start = Utc.ymd(2022, 9, 13).and_hms(12, 0, 0);
    //let start = Utc.from_utc_datetime(&from);
    //let end = Utc.from_utc_datetime(&to);
    //let end = start + Duration::hours(1);

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

    // let mut data_points:Vec<(DateTime<Utc>, f64)> = Vec::new();
    // let first_input = demand_curve_inputs.first().unwrap();
    // data_points.push((first_input.timestamp, first_input.value));
    // let mut previous_input = first_input;
    // for i in 0..fitted_inputs.len() {
    //     let input = demand_curve_inputs.get(i).unwrap();
    //     data_points.push((input.timestamp, previous_input.value));
    //     data_points.push((input.timestamp, input.value));
    //     previous_input = input;
    // }
    // data_points.push((to, previous_input.value));

    // let mut current = start;

    // let mut dates:Vec<(DateTime<Utc>, f64)> = Vec::new();
    // dates.push((current, 0.0));
    // current = current + Duration::minutes(15);
    // dates.push((current, 0.0));
    // dates.push((current, 10.0));
    // current = current + Duration::minutes(10);
    // dates.push((current, 10.0));
    // dates.push((current, 20.0));
    // current = current + Duration::minutes(20);
    // dates.push((current, 20.0));
    // dates.push((current, 10.0));
    // current = current + Duration::minutes(15);
    // dates.push((current, 10.0));

    let zero_line = [(from, 0.0), (to, 0.0)];

    chart.draw_series(LineSeries::new(
        zero_line.iter().map(|d| (d.0, d.1)),
        &BLACK,
    ))
    .unwrap();
        //.build_cartesian_2d(
  //          (Utc.ymd(2010, 1, 1)..Utc.ymd(2018, 12, 1)).monthly(),
    //        30.0..50.0,
      //  );
//(Utc.ymd(2022, 9, 13).and_hms(12, 0, 0)..Utc.ymd(2022, 9, 13).and_hms(13, 0, 0)).monthly(),

//        let coord_spec = self.drawing_area.into_coord_spec();
//move |coord| coord_spec.reverse_translate(coord)

    root.present().unwrap();
    //coord_spec = chart.draw.into_coord_spec();
    //return Ok(chart.into_coord_trans());
}