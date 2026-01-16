use std::f64::consts::PI;
use plotters::prelude::*;
use radar_toolbox::antennas::phased_array::*;
pub fn solve(){
    println!("---- Solving problem 1 for chapter 9 ----");

    let arr = LinearArray::new(0.5, 5, 0.0);
    let step: f64 = 0.01;
    let count: usize = ((2.0*PI) / step).floor() as usize;
    let angles: Vec<f64> = (0..=count)
        .map(|i| (-1.0*PI + (i as f64) * step).to_degrees())
        .collect();
    let af_db: Vec<f64> = (0..=count)
            .map(|i| 10.0*(arr.antenna_factor(-1.0*PI + (i as f64)*step, 1.0).abs_sq().log10()))
            .collect();


    let root = BitMapBackend::new("plots/chapter_9_problem_8.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // 2. Configure the chart
    let mut chart = ChartBuilder::on(&root)
        .caption("Antenna Directivity", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        // Set limits: X (-90 to 90), Y (min dB to max dB)
        .build_cartesian_2d(-90.0..90.0, -40.0..30.0).unwrap(); 

    chart.configure_mesh()
        .x_desc("Angle (degrees)")
        .y_desc("Directivity (dB)")
        .draw().unwrap();

    // 3. Draw the Line Series
    chart.draw_series(LineSeries::new(
        angles.iter().zip(af_db.iter()).map(|(&x, &y)| (x, y)),
        &RED,
    )).unwrap()
    .label("Main Beam")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw().unwrap();

    root.present().unwrap();
}
