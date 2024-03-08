fn main() {
    let datapoints : Vec<(f32,f32)> = vec![(0.0,0.0),(33.0,33.0),(44.0,44.0)];
    let result = linear_regression_f32(datapoints);
    println!("{:#?}",result);
}
/*
  LinearRegression Struct is used to for holding the results of the linear_regression function 
  It is a generic Struct can hold all types of data types
 */
#[derive(Debug)]
struct LinearRegression<T>{
    pearson_constant : T,
    slope : T,
    y_intercept : T
}
/*
    @params:
        data_points : Vec<(f32,f32)> =  It is the Vector which holds the X and Y coords as a tuple pair type
    @return:
        return a option<LinearRegression<f32>>
            Case 1: Returns a None type if the data_points is empty 
            Case 2: Returns a Some type of LinearRegression<f32> once the value if calculated 
 */
fn linear_regression_f32(data_points : Vec<(f32,f32)>) -> Option<LinearRegression<f32>> 
{
    let length : f32 = data_points.len() as f32;
    if length  < 0.0 {
        return None;
    }
    let x_mean : f32 = data_points.iter().fold(0.0 , |sum ,x| sum + x.0) / length;
    let y_mean : f32 = data_points.iter().fold(0.0 , |sum ,y| sum + y.1) / length;
    //It is not a correct covariance as it is used to calculate the PCC - Pearson's co-relation
    //constant it does not require the formulae : ((x-x`)*(y-y`))/N-1 where in PCC N-1 is not
    //required
    let mut covariance : f32 = 0.0;
    let mut std_dev_sqr_x : f32 = 0.0;
    let mut std_dev_sqr_y : f32 = 0.0;
    for dp in data_points {
        covariance += (dp.0 - x_mean)*(dp.1 - y_mean);
        std_dev_sqr_x += (dp.0 - x_mean).powi(2);
        std_dev_sqr_y += (dp.1 - y_mean).powi(2);
    }
    let std_dev_x = std_dev_sqr_x.sqrt();
    let std_dev_y = std_dev_sqr_y.sqrt();
    let std_dev_prod = std_dev_x * std_dev_y;
    println!("{:?} {:?} {:?}", std_dev_x,std_dev_y,covariance);
    let pearson_constant = covariance / std_dev_prod;
    let slope = pearson_constant * (std_dev_y / std_dev_x);
    let y_intercept = y_mean - slope * x_mean;
    let result = LinearRegression::<f32>{
        pearson_constant,
        slope,
        y_intercept
    };
    Some(result)
}
