use func_plot::{ run, GraphApp};
use utils::Function;
mod func_plot;
mod errors;
mod utils;
mod approximation;
mod gauss;
use gauss::solve;
fn main(){
    // Log to stdout (if you run with `RUST_LOG=debug`).



    let mut matrix = vec![
        vec![1.0, 1.0],
        vec![1.0, 2.0]
    ];
    let solution = gauss::solve(&mut matrix, &mut vec![5.,2.]).unwrap();
    for x in solution{
        println!("{x} ");
    }


    let mut binding = |x : f64| x.powi(2);
    let f =  Function::new( binding);
    
    let mut app = GraphApp::new();
    app.get_functions().push(("x^2",f));
   // app.get_functions().push(|x| 1./x);
    //app.get_points().push([3.,3.]);
    //app.get_points().push([4.,4.]);
    
    for i in 0..20{
        app.get_points().push([i as f64,(i as f64).powf(2.)]);
    }
    app.range = (-50.,50.);
    app.step = 0.01;
    run(app);

    

}



    /*let sin: PlotPoints = (0..1000).map(|i| {
        let x = i as f64 * 0.01;
        [x, x.sin()]
    }).collect();
    let line = Line::new(sin);
    Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));*/
