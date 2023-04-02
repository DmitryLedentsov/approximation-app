use crate::{utils::{Function, self, round}, gauss::{self, solve}, errors::AppError};
use expression_format::ex_format;
const E:f64 = std::f64::consts::E;
pub fn linear_approximation(points: &Vec<[f64;2]>)->Result<(f64,f64,Function, String, Vec<f64>, f64, Option<f64>),AppError>{
    let n = points.len();

    let mut summ_x:f64 = 0.;
    for p in points{
        summ_x += p[0]
    }
        

    let mut summ_x_sqd= 0.;
    for p in points{
        summ_x_sqd += p[0]*p[0];
    }
    let mut summ_y = 0.;
    for p in points{
        summ_y += p[1];
    }

    let mut summ_x_y = 0.;
    for p in points{
        summ_x_y += p[0] * p[1]
    }

    //коэффициент корреляции Пирсона
    let mid_x = summ_x / n as f64;
    let mid_y = summ_y / n as f64;
    //числитель
    let mut summ_1 = 0.;
    for p in points{
        summ_1 += (p[0] - mid_x) * (p[1] - mid_y);
    }
    //знаменатель (суммы 2 и 3)
   
    let mut summ_2 = 0.;
    for p in points{
        summ_2 += (p[0] - mid_x).powf(2.);
    }
   

    let mut summ_3 = 0.;
    for p in points{
        summ_3 += (p[1] - mid_y).powf(2.);
    }

    let mut r =  Some(summ_1/ ((summ_2*summ_3).sqrt()));
    if let Some(val) = r{
        if val.is_nan() {
            r = None;
        }
    }


    
    /*try:
        r = (summ_1)/(math.sqrt(summ_2*summ_3))
        print(f"Коэффициент корреляции Пирсона равен: {round(r, 3)}")
    except Exception:
        print("Не получилось посчитать коэффициент корреляции Пирсона")*/
    /*ans = calc_system([[summ_x_sqd, summ_x, summ_x_y],[summ_x, n, summ_y]], 2)

    result_func = lambda x: ans[0]*x + ans[1]

    str_result_func = f"{round(ans[0], 3)}x + {round(ans[1], 3)}"

    #среднеквадратичное отклонение
    errors = [(points[i][1] - result_func(points[i][0]))**2 for i in range(n)]
    mid_sqd_err = math.sqrt(sum(errors)/n)

    return result_func, str_result_func, errors, mid_sqd_err*/

    
    let ans:  Vec<f64> = solve(&mut vec![vec![summ_x_sqd,summ_x], vec![summ_x,n as f64]], &mut vec![summ_x_y,summ_y]).map_err(|_e| AppError::UnableApproximate)?;
    let x = round(ans[0], 3);
    let y = round(ans[1], 3);
    let str_func = format!("{x} x + {y}");
    let mut res_func = Function::new(move |x| ans[0]*x + ans[1]);
    let errors:Vec<f64> = (0..n).map(|i|{
        (points[i][1]-res_func.call(points[i][0])).powf(2.)
    }).collect();
    let sum:f64 = errors.iter().sum();
    let mid_err = (sum.abs() /n as f64).sqrt();

    return  Ok((x,y,res_func,str_func, errors, mid_err, r));
}

pub type StandartApproximator = fn(&Vec<[f64;2]>)->Result<(Function, String, Vec<f64>, f64),AppError>;
pub fn squad_approximate(points: &Vec<[f64;2]>)->Result<(Function, String, Vec<f64>, f64),AppError>{
    let n = points.len();
    let mut summ_x = 0.;
    for p in points{
        summ_x += p[0];
    }

    let mut summ_x_sqd = 0.;
    for p in points{
        summ_x_sqd += p[0].powf(2.);
    }


    let mut summ_x_qub = 0.;
    for p in points{
        summ_x_qub += p[0].powf(3.);
    }

    let mut summ_x_forth = 0.;
    for p in points{
        summ_x_forth += p[0].powf(4.);
    }

    let mut summ_y = 0.;
    for p in points{
        summ_y += p[1];
    }

    let mut summ_x_y = 0.;
    for p in points{
        summ_x_y += p[0]*p[1];
    }

    let mut summ_x_sqd_y = 0.;
    for p in points{
        summ_x_sqd_y += p[0].powf(2.)*p[1];
    }

    let ans:  Vec<f64>  = solve(&mut vec![
        vec![n as f64,summ_x,summ_x_sqd], 
        vec![summ_x, summ_x_sqd, summ_x_qub],
        vec![summ_x_sqd, summ_x_qub, summ_x_forth] ],
        &mut vec![summ_y,summ_x_y, summ_x_sqd_y]).map_err(|_e| AppError::UnableApproximate)?;

    
    for x in ans.iter(){
        if x.is_nan(){
            return  Err(AppError::UnableApproximate);
        }
    }
    let x_2 = round(ans[2], 3);
    let x = round(ans[1], 3);
    let a = round(ans[0], 3);
    let mut result_func = Function::new(move |x|  ans[2]*(x*x) + ans[1]*x + ans[0]);

    
    let str_func = format!("{x_2}x^2 + {x}x + {a}");
    
    let errors:Vec<f64> = (0..n).map(|i|{
        (points[i][1]-result_func.call(points[i][0])).powf(2.)
    }).collect();
    let sum:f64 = errors.iter().sum();
    let mid_err = (sum.abs() /n as f64).sqrt();
    //СКО
    Ok((result_func, str_func, errors, mid_err))
}


pub fn cub_approximate(points: &Vec<[f64;2]>)->Result<(Function, String, Vec<f64>, f64),AppError>{
    let n = points.len();
    let mut summ_x = 0.;
    for p in points{
        summ_x += p[0];
    }

    let mut summ_x_sqd = 0.;
    for p in points{
        summ_x_sqd += p[0].powf(2.);
    }


    let mut summ_x_qub = 0.;
    for p in points{
        summ_x_qub += p[0].powf(3.);
    }

    let mut summ_x_forth = 0.;
    for p in points{
        summ_x_forth += p[0].powf(4.);
    }

    let mut summ_x_fifth = 0.;
    for p in points{
        summ_x_fifth += p[0].powf(5.);
    }

    let mut summ_x_six = 0.;
    for p in points{
        summ_x_six += p[0].powf(6.);
    }

    let mut summ_y = 0.;
    for p in points{
        summ_y += p[1];
    }

    let mut summ_x_y = 0.;
    for p in points{
        summ_x_y += p[0]*p[1];
    }

    let mut summ_x_sqd_y = 0.;
    for p in points{
        summ_x_sqd_y += p[0].powf(2.)*p[1];
    }

    let mut summ_x_cub_y = 0.;
    for p in points{
        summ_x_cub_y += p[0].powf(3.)*p[1];
    }

    let ans:  Vec<f64>  = solve(&mut vec![
        vec![n as f64,summ_x,summ_x_sqd, summ_x_qub], 
        vec![summ_x, summ_x_sqd, summ_x_qub, summ_x_forth],
        vec![summ_x_sqd, summ_x_qub, summ_x_forth, summ_x_fifth],
        vec![summ_x_qub, summ_x_forth, summ_x_fifth, summ_x_six]
        ],
        &mut vec![summ_y,summ_x_y, summ_x_sqd_y, summ_x_cub_y]).map_err(|_e| AppError::UnableApproximate)?;

    for x in ans.iter(){
        if x.is_nan(){
            return  Err(AppError::UnableApproximate);
        }
    }
    let x_3 = round(ans[3], 3);
    let x_2 = round(ans[2], 3);
    let x = round(ans[1], 3);
    let a = round(ans[0], 3);
    let mut result_func = Function::new(move |x|  ans[3]*(x*x*x) + ans[2]*x*x + ans[1]*x + ans[0]);

    
    let str_func = format!("{x_3}x^3 + {x_2}x^2 + {x}x + {a}");
    
    let errors:Vec<f64> = (0..n).map(|i|{
        (points[i][1]-result_func.call(points[i][0])).powf(2.)
    }).collect();
    let sum:f64 = errors.iter().sum();
    let mid_err = (sum.abs() /n as f64).sqrt();
    //СКО
    Ok((result_func, str_func, errors, mid_err))
}


pub fn exp_approximate(input: &Vec<[f64;2]>)->Result<(Function, String, Vec<f64>, f64),AppError>{
    let  mut points = Vec::<[f64;2]>::new();
    for p in input.iter(){
        if p[1]>0.{
            points.push(*p);
        }
    }
    if points.len()!= input.len(){
        return Err(AppError::UnableApproximate);
    }
    let n = points.len();
    let mut summ_x = 0.;
    for p in points.iter(){
        summ_x += p[0];
    }

    let mut summ_x_sqd = 0.;
    for p in points.iter(){
        summ_x_sqd += p[0].powf(2.);
    }



   
   
    let mut summ_y = 0.;
    for p in points.iter(){
        summ_y += p[1].log(E);
    }

    let mut summ_x_y = 0.;
    for p in points.iter(){
        summ_x_y += p[0]*(p[1].log(E));
    }

    

    let ans:  Vec<f64>  = solve(&mut vec![
        vec![summ_x_sqd,summ_x], 
        vec![summ_x, n as f64],
        ],
        &mut vec![summ_x_y, summ_y]).map_err(|_e| AppError::UnableApproximate)?;

    for x in ans.iter(){
        if x.is_nan(){
            return  Err(AppError::UnableApproximate);
        }
    }
    let x = round(ans[1].exp(), 3);

    let a = round(ans[0], 3);
    let mut result_func = Function::new(move |x|  (ans[1].exp())*((x*ans[0]).exp()));

    
    let str_func = format!("{x}e^{a} * x");
    
    let errors:Vec<f64> = (0..n).map(|i|{
        (points[i][1]-result_func.call(points[i][0])).powf(2.)
    }).collect();
    let sum:f64 = errors.iter().sum();
    let mid_err = (sum.abs() /n as f64).sqrt();
    //СКО
    Ok((result_func, str_func, errors, mid_err))
}


pub fn ln_approximate(input: &Vec<[f64;2]>)->Result<(Function, String, Vec<f64>, f64),AppError>{
    let  mut points = Vec::<[f64;2]>::new();
    for p in input.iter(){
        if p[0]>0.{
            points.push(*p);
        }
    }
    if points.len()!= input.len(){
        return Err(AppError::UnableApproximate);
    }
    let n = points.len();
    let mut summ_x = 0.;
    for p in points.iter(){
        summ_x += p[0].log(E);
    }

    let mut summ_x_sqd = 0.;
    for p in points.iter(){
        summ_x_sqd += p[0].log(E).powf(2.);
    }



   
   
    let mut summ_y = 0.;
    for p in points.iter(){
        summ_y += p[1];
    }

    let mut summ_x_y = 0.;
    for p in points.iter(){
        summ_x_y += (p[0].log(E))*p[1];
    }

    

    let ans:  Vec<f64>  = solve(&mut vec![
        vec![summ_x_sqd,summ_x], 
        vec![summ_x, n as f64],
        ],
        &mut vec![summ_x_y, summ_y]).map_err(|_e| AppError::UnableApproximate)?;

    for x in ans.iter(){
        if x.is_nan(){
            return  Err(AppError::UnableApproximate);
        }
    }
    let x = round(ans[0],3);

    let a = round(ans[1], 3);
    let mut result_func = Function::new(move |x|  x.log(E)*ans[0] + ans[1]);

    
    let str_func = format!("{x} ln(x) + {a}");
    
    let errors:Vec<f64> = (0..n).map(|i|{
        (points[i][1]-result_func.call(points[i][0])).powf(2.)
    }).collect();
    let sum:f64 = errors.iter().sum();
    let mid_err = (sum.abs() /n as f64).sqrt();
    //СКО
    Ok((result_func, str_func, errors, mid_err))
}

pub fn pow_approximate(input: &Vec<[f64;2]>)->Result<(Function, String, Vec<f64>, f64),AppError>{
    let  mut points = Vec::<[f64;2]>::new();
    for p in input.iter(){
        if p[0]>0. && p[1]>0.{
            points.push(*p);
        }
    }
    if points.len()!= input.len(){
        return Err(AppError::UnableApproximate);
    }
    let n = points.len();
    let  mut lin = Vec::<[f64;2]>::new();
    for i in 0..n {
        lin.push([points[i][0].log(E), points[i][1].log(E)]);
    }
    let (lina,linb,..) = linear_approximation(&points)?;
    let a = linb.exp();
    let b = lina;
    let mut result_func = Function::new(move |x|  a*(x.powf(b)));


    let str_func = ex_format!("{round(a,3)} * x ^ {round(b,3)}");
    
    let errors:Vec<f64> = (0..n).map(|i|{
        (points[i][1]-result_func.call(points[i][0])).powf(2.)
    }).collect();
    let sum:f64 = errors.iter().sum();
    let mid_err = (sum.abs() /n as f64).sqrt();
    //СКО
    Ok((result_func, str_func, errors, mid_err))
}