use std::{cmp::Ordering, vec, iter::Map};

use eframe::{egui::{self, plot::{self, Text}, Grid, Window, TextEdit, TextBuffer, Layout, Button, WidgetText}, epaint::{Pos2, Color32}};

use egui::plot::{Line, Plot, PlotPoints};
use indoc::indoc;
use egui_modal::{Modal, Icon};
use tinyfiledialogs::*;
use itertools::Itertools;
use crate::{utils, errors::AppError, approximation::{self, StandartApproximator, squad_approximate, cub_approximate, exp_approximate, ln_approximate, pow_approximate}};
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
        self.points.dedup_by(|a,b| a[0].partial_cmp(&b[0]).unwrap_or(Ordering::Greater).is_eq());
        self.sort();

        self.out.clear();
        self.funcs.clear();
        match (self.points.first(), self.points.last()){
            (Some(s), Some(e)) => self.range = (s[0],e[0]),
            _=>()
        }

        if self.points.len()<2 {
            return  Err(AppError::OnePoint);
        }
       
        let mut map = std::collections::HashMap::<String, f64>::new();
        {
            let method = "linear";
            let (_,_,func, str_func, errors, mid_err, k) = approximation::linear_approximation(&self.points)?;
            self.funcs.push((method , func));
            let sum :f64= round(errors.iter().sum(),3);
            self.out += &format!("{method} approximation returned function: {str_func}, S = {sum}, sigma = {mid_err}");
            if let Some(n) = k{
                self.out += &format!(", Pirson = {n}");
            }
            self.out += "\n\n";
            map.insert(method.to_string(), mid_err);
        }
        
        let mut add_method  =  |method:&'static str, approximate: StandartApproximator| -> Result<_, AppError>{
            
            let res = approximate(&self.points);
            if res.is_err(){
                self.out+=&format!("unable approximate with {method} method \n\n");
            }else{
                let (func, str_func, errors, mid_err) = res?;
                self.funcs.push((method , func));
                let sum :f64= round(errors.iter().sum(),3);
                self.out += &format!("{method} approximation returned function: {str_func}, S = {sum}, sigma = {mid_err}");
                self.out += "\n\n";
                map.insert(method.to_string(), mid_err);
            }
            Ok(())
        };
        add_method("square", squad_approximate);
        add_method("cubic", cub_approximate);
        add_method("exponental", exp_approximate);
        add_method("log", ln_approximate);
        //add_method("pow", pow_approximate);
        if let Some((name,err)) = map.iter().max_by(|a,b| b.1.partial_cmp(a.1).unwrap_or(Ordering::Greater)){
            self.out += &format!("minimal squad error: {err}, best approximation: {name} \n");
        }
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
                if ui.add(Button::new(WidgetText::RichText("approximate".into()).heading().color(Color32::YELLOW)).fill(Color32::DARK_GREEN)).clicked(){
                
                    match self.approximate(){
                        Err(AppError::OnePoint) =>open_dialog_with_error(modal, "need at least 2 points!"),
                        Err(_)=>open_dialog_with_error(modal, "something went wrong!"),
                        _=>()
                    }
                }
                /*
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
                });*/
                ui.end_row();
                egui::scroll_area::ScrollArea::new([false,true]).max_height(500.).show(ui, |ui|{
                    ui.vertical(|ui|{
                        ui.add_space(20.);
                        ui.label("table of points");
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
                            let mut adding_point:[f64;2] =*self.points.last().unwrap_or(&[0.0,0.0]);

                            adding_point[0] +=1.;
                            adding_point[1] += 1.;
                            if !self.points.contains(&adding_point){
                                self.points.push(adding_point);
                            }
                            
                        }
                        
                    });
                });
                
            });

            /*self.out = indoc!{"
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
            
            "}.to_string();*/
            egui::Window::new("Output").resizable(true).vscroll(true).show(ctx, |ui| {
                ui.add(egui::TextEdit::multiline(&mut self.out).interactive(false).desired_width(400.));
            });
           
            
           
            
            
           
            egui::Window::new("Options").resizable(true).default_pos(Pos2{ x: 0., y: 0.}).show(ctx, |ui| {
                let mut modal = Modal::new(ctx, "ERROR!");
                modal.show_dialog();
                /*ui.allocate_ui_with_layout(eframe::epaint::Vec2{ x:  200., y:  200.}, Layout::top_down(eframe::emath::Align::LEFT), |ui|{
                    if ui.button("Open the modal").clicked() {
                        // Show the modal
                        
                        
                    }
                    ui.add_sized([100.,100.], Button::)
                });*/

                /*if ui.add(Button::new("afasfj").min_size(eframe::epaint::Vec2 { x: 50., y: 100. })).clicked(){

                }*/
                
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
            

            Plot::new("my_plot").legend(plot::Legend { text_style: egui::TextStyle::Monospace, background_alpha: 1., position: plot::Corner::LeftBottom }).data_aspect(1.).show(ui, |plot_ui| {
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
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

   
    eframe::run_native(
        "Function Graph",
        options,
        Box::new( |_cc| Box::new(app)),
    ).unwrap();

}
