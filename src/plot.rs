use plotters::prelude::*;

pub fn time_histogram(sorted_vec: &Vec<(i32, Vec<i32>)>) {
    let mut data: Vec<(i32, i32)> = Vec::new();

    for (time, games) in sorted_vec {
        for _ in games {
            data.push((*time, 1));
        }
    }

//    for (item, value) in &data {
//        println!("{item} {value}");
//    }
    let drawing_area = SVGBackend::new("histogram.svg", (900, 600)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut chart_builder = ChartBuilder::on(&drawing_area);
    chart_builder.margin(5).set_left_and_bottom_label_area_size(20);

    let mut chart_context = chart_builder
        .build_cartesian_2d((1..11000).into_segmented(), 0..130)
        .unwrap();
    chart_context.configure_mesh().draw().unwrap();
    chart_context.draw_series(
        Histogram::vertical(&chart_context)
            .style(BLUE.filled())
            .margin(10)
            .data(data)
    ).unwrap();
}
