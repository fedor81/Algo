use plotters::prelude::*;

pub fn draw_points() -> Result<(), Box<dyn std::error::Error>> {
    let input = "3.168070 1.752490
0.500730 6.436580
0.089300 0.112720
2.275440 7.508780
0.779230 4.377090
0.644400 1.381650
1.844920 1.430420
8.079870 5.225030
7.823270 5.317290
1.788400 5.426120";

    // Parse the input points
    let points: Vec<(f64, f64)> = input
        .lines()
        .map(|line| {
            let mut coords = line.split_whitespace();
            let x = coords.next().unwrap().parse().unwrap();
            let y = coords.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    // Create a drawing area with larger dimensions
    let root = BitMapBackend::new("points.png", (1200, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    // Find the min and max values for scaling
    let x_min = points
        .iter()
        .map(|(x, _)| x)
        .fold(f64::INFINITY, |a, &b| a.min(b));
    let x_max = points
        .iter()
        .map(|(x, _)| x)
        .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let y_min = points
        .iter()
        .map(|(_, y)| y)
        .fold(f64::INFINITY, |a, &b| a.min(b));
    let y_max = points
        .iter()
        .map(|(_, y)| y)
        .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    // Create the chart with padding
    let padding = 1.5;
    let mut chart = ChartBuilder::on(&root)
        .caption("Point Distribution", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            (x_min - padding)..(x_max + padding),
            (y_min - padding)..(y_max + padding),
        )?;

    // Configure the mesh
    chart.configure_mesh().draw()?;

    // Draw the points with circles and indices
    for (idx, (x, y)) in points.iter().enumerate() {
        // Draw the outer circle (60px diameter = ~30px radius)
        chart.draw_series(std::iter::once(Circle::new(
            (*x, *y),
            285,
            BLUE.mix(0.2).filled(),
        )))?;

        // Draw the center point (10px diameter)
        chart.draw_series(std::iter::once(Circle::new((*x, *y), 10, RED.filled())))?;

        // Draw the index number
        chart.draw_series(std::iter::once(Text::new(
            format!("{}", idx),
            (*x + 0.2, *y + 0.2),
            ("sans-serif", 20.0).into_font().color(&BLACK),
        )))?;
    }

    root.present()?;

    Ok(())
}
