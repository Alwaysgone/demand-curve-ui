mod utils;

use chrono::{TimeZone, Utc, DateTime, Duration};

use wasm_bindgen::prelude::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

use plotters::coord::ReverseCoordTranslate;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

/// Type alias for the result of a drawing function.
pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Type used on the JS side to convert screen coordinates to chart
/// coordinates.
#[wasm_bindgen]
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}

/// Result of screen to chart coordinates conversion.
#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Chart {

    /// Draw provided power function on the canvas element using it's id.
    /// Return `Chart` struct suitable for coordinate conversion.
    pub fn power(canvas_id: &str, power: i32) -> Result<Chart, JsValue> {
        let map_coord = draw(canvas_id, power).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    pub fn demandCurve(canvas_id: &str) -> Result<Chart, JsValue> {
        let map_coord = drawDemandCurve(canvas_id).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    pub fn demand_curve_time_series(canvas_id: &str, start: &str, end: &str) -> Result<Chart, JsValue> {
        //let startTime = Utc.datetime_from_str(start, "%Y-%m-%dT%H:%M").unwrap();
        //let a = (startTime - startTime).num_minutes() / 15;
        /*
        let map_coord = draw_demand_curve_time_series(canvas_id);
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (((x - startTime).num_minutes() / 15) as f64, y.into()))),
        })
        */
        draw_demand_curve_time_series(canvas_id);
        Ok(Chart {
            convert: Box::new(move |coord| Option::None),
        })
    }

    /// This function can be used to convert screen coordinates to
    /// chart coordinates.
    pub fn coord(&self, x: i32, y: i32) -> Option<Point> {
        (self.convert)((x, y)).map(|(x, y)| Point { x, y })
    }
}

pub fn draw(canvas_id: &str, power: i32) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption(format!("y=x^{}", power), font)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(-1f32..1f32, -1.2f32..1.2f32)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    chart.draw_series(LineSeries::new(
        (-50..=50)
            .map(|x| x as f32 / 50.0)
            .map(|x| (x, x.powf(power as f32))),
        &RED,
    ))?;

    root.present()?;
    return Ok(chart.into_coord_trans());
}

pub fn drawDemandCurve(canvas_id: &str) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
    .margin(20u32)
    .caption("Demand", font)
    .x_label_area_size(30u32)
    .y_label_area_size(30u32)
    .build_cartesian_2d(-1f32..8f32, -1f32..8f32)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    let mut points: Vec<(f32, f32)> = Vec::new();
    points.push((-1.0, 0.0));
    points.push((1.0, 0.0));
    points.push((1.0, 5.0));
    points.push((5.0, 5.0));
    points.push((5.0, 3.0));
    points.push((7.0, 3.0));
    points.push((7.0, 0.0));
    points.push((8.0, 0.0));

    chart.draw_series(LineSeries::new(
        points.iter().map(|i| (i.0, i.1)),
        &RED,
    ))?;

    root.present()?;
    return Ok(chart.into_coord_trans());
}

pub fn draw_demand_curve_time_series(canvas_id: &str) {
    // -> DrawResult<impl Fn((i32, i32)) -> Option<(DateTime<Utc>, f32)>>
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 30.0).into();

    root.fill(&WHITE).unwrap();

    let start = Utc.ymd(2022, 9, 13).and_hms(12, 0, 0);
    
    let end = start + Duration::hours(1);

    let mut chart = ChartBuilder::on(&root)
        .margin(40)
        .caption(
            "Demand",
            font,
        )
        .x_label_area_size(30u32)
        .y_label_area_size(50u32)
        // adding another minute because range is end exclusive
        .build_cartesian_2d(start..(end + Duration::minutes(1)),
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

    let mut current = start;

    let mut dates:Vec<(DateTime<Utc>, f64)> = Vec::new();
    dates.push((current, 0.0));
    current = current + Duration::minutes(15);
    dates.push((current, 0.0));
    dates.push((current, 10.0));
    current = current + Duration::minutes(10);
    dates.push((current, 10.0));
    dates.push((current, 20.0));
    current = current + Duration::minutes(20);
    dates.push((current, 20.0));
    dates.push((current, 10.0));
    current = current + Duration::minutes(15);
    dates.push((current, 10.0));

    chart.draw_series(LineSeries::new(
        dates.iter().map(|d| (d.0, d.1)),
        &RED,
    ))
    .unwrap();

    let zero_line = [(start, 0.0), (end, 0.0)];

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

/*
impl ReverseCoordTranslate for Shift {
    fn reverse_translate(&self, input: BackendCoord) -> Option<BackendCoord> {
        Some((input.0 - (self.0).0, input.1 - (self.0).1))
    }
}
*/