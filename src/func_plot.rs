use std::{cmp::Ordering, vec};

use eframe::{egui::{self, plot::{self, Text}, Grid, Window, TextEdit, TextBuffer}, epaint::Pos2};

use egui::plot::{Line, Plot, PlotPoints};
use indoc::indoc;
use egui_modal::{Modal, Icon};
use tinyfiledialogs::*;

use crate::{utils, errors::AppError, approximation};
const RANGE:f64 = 10.;
const STEP:f64 = 0.01;

use crate::utils::*;
type  FunctionWithDesc=(& 'static str,Function);
pub struct GraphApp  {
    funcs:  Vec<FunctionWithDesc>,
    points: Vec<[f64;2]>,
    pub range: (f64,f64),
    pub step:f64,
    pub out:String

}
/*pub struct  Point{
    x:f64, y:f64
}
impl  Point {
    pub fn new(X:f64,Y:f64)->Self{
        Self { x: X, y: Y}
    }
}*/
impl GraphApp {
    pub fn new  () -> Self {
        Self {
           funcs: vec![],
           points: vec![],
           range:(-RANGE,RANGE),
           step: STEP,
           out:String::new()
        }
    }
    pub fn get_functions(&mut self)->&mut Vec<FunctionWithDesc>{
        return self.funcs.as_mut();
    }
    pub fn get_points(&mut self)->&mut Vec<[f64;2]>{
        return self.points.as_mut();
    }

    pub fn sort(&mut self){
        self.points.sort_by(|a,b|{
            if a[0]>b[0] {
                Ordering::Greater
            } else if a[0]==b[0] {
                Ordering::Equal
            } else{
                Ordering::Less
            }
            
        });
    }
    pub fn approximate(&mut self)->Result<(), AppError>{
        self.sort();
        self.funcs.clear();
        match (self.points.first(), self.points.last()){
            (Some(s), Some(e)) => self.range = (s[0],e[0]),
            _=>()
        }
        self.funcs.push(("linear" , approximation::linear_approximation(&self.points)?));
        return  Ok(());
    }
    
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Enum {
    First,Second,Third
}
impl eframe::App for GraphApp {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
       
        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.heading("Function Graph");
            


            let mut data: Vec<Line> = vec![];
            let  range: (i32,i32)= ((self.range.0/self.step) as i32, (self.range.1/self.step) as i32);
            for func in self.funcs.iter_mut(){
                
                let d:PlotPoints  = (range.0..range.1+1).map(|i| {
                    let x = i as f64 * self.step;
                    [x, (func.1.0)(x)]
                }).collect();
                data.push(Line::new(d).name(&func.0));
            }   
            
            
            egui::Window::new("Data").show(ctx, |ui| {
                
                let mut modal = Modal::new(ctx, "ERROR!");
                modal.show_dialog();
                if(ui.button("approximate")).clicked(){
                    match self.approximate(){
                        Err(_)=>open_dialog_with_error(modal, "aaaa!"),
                        Ok(_)=>()
                    }
                }
                
                ui.vertical(|ui|{
                    
                    let mut selected =Enum::First;
                    egui::ComboBox::from_label("Select one!")
                        .selected_text(format!("{:?}", selected))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut selected, Enum::First, "First");
                            ui.selectable_value(&mut selected, Enum::Second, "Second");
                            ui.selectable_value(&mut selected, Enum::Third, "Third");
                        }
                    );
                });
                ui.end_row();
                egui::scroll_area::ScrollArea::new([false,true]).max_height(500.).show(ui, |ui|{
                    ui.vertical(|ui|{
                    
                        ui.label("таблица аппроксимации");
                        let mut i:usize = 0;
                        while i<self.points.len() {
                            ui.horizontal(|ui| {
                                
                                ui.add(egui::DragValue::new( &mut self.points[i][0]).speed(self.step));
                                ui.add(egui::DragValue::new( &mut self.points[i][1]).speed(self.step));
                                if ui.button("remove").clicked(){
                                    self.points.remove(i);
                                }
                                else {
                                    i+=1;
                                }
                            });
                            
                        }
                        if ui.button("add").clicked(){
                            let adding_point = [0.,0.];
                            if !self.points.contains(&adding_point){
                                self.points.push(adding_point);
                            }
                            
                        }
                        
                    });
                });
                
            });

            self.out = indoc!{"
                Вы любите розы?
                а я на них срал!
                стране нужны паровозы,
                нам нужен металл!
                товарищ!
                не охай,
                не ахай!
                не дёргай узду!
                коль выполнил план,
                посылай всех
                в п*зду
                не выполнил —
                сам
                иди
                на
                х*й.
            
            "}.to_string();
            egui::Window::new("Output").resizable(true).show(ctx, |ui| {
                ui.add(egui::TextEdit::multiline(&mut self.out).interactive(false));
            });
           
            
           
            
            
           
            egui::Window::new("Options").resizable(true).default_pos(Pos2{ x: 0., y: 0.}).show(ctx, |ui| {
                let mut modal = Modal::new(ctx, "ERROR!");
                modal.show_dialog();
                if ui.button("Open the modal").clicked() {
                    // Show the modal
                    
                    
                }
                if ui.button("open file").clicked(){
                    let open_file: String;
                    match tinyfiledialogs::open_file_dialog("Open", "points.txt", None) {
                        Some(file) => {
                            open_file = file;
                            match utils::read_points_from_file(&open_file){
                                Err(AppError::IOError) => open_dialog_with_error(modal,"something wrong with file"),
                                Err(AppError::InvalidFormat) => open_dialog_with_error(modal, "wrong format"),
                                Ok(points)=> self.points = points,
                                _=>()
                            }
                        },
                        None =>(),
                    }

                }
            });
            

            Plot::new("my_plot").legend(plot::Legend { text_style: egui::TextStyle::Monospace, background_alpha: 1., position: plot::Corner::LeftBottom }).data_aspect(0.1).show(ui, |plot_ui| {
                //plot_ui.set_plot_bounds(PlotBounds::from_min_max([-1 0.,-10.], [10.+1.,10.0+1.]));
                //plot_ui.line(Line::new(PlotPoints::new(vec![[1.,2.]])).width(5.));

                for line in data.into_iter(){
                    plot_ui.line(line);
                }
                
                plot_ui.points(plot::Points::new(self.points.clone()).radius(4.0).stems(0.).name("points"));
                
                
                
            });
        });
    }
}

fn open_dialog_with_error(modal:Modal, err:&str){
    modal.open_dialog(
        Some("ERROR!"), // title
        Some(err), // body
        Some(Icon::Warning) // icon
    );
}

pub fn run <T:eframe::App + 'static> (app: T){
    // Log to stdout (if you run with `RUST_LOG=debug`).
   

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 500.0)),
        ..Default::default()
    };

   
    eframe::run_native(
        "Function Graph",
        options,
        Box::new( |_cc| Box::new(app)),
    ).unwrap();

}
