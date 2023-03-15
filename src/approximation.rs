use crate::{utils::{Function, self, round}, gauss::{self, solve}, errors::AppError};

pub fn linear_approximation(points: &Vec<[f64;2]>)->Result<(Function, String, Vec<f64>, f64, Option<f64>),AppError>{
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
    if let None = r{
        r = None
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

    return  Ok((res_func,str_func, errors, mid_err, r));
}