use json::object;

#[derive(Debug)]
pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    // println!("{:?}", foods.name);
    let mut cals: f64 = 0.0;
    let mut carbs: f64 = 0.0;
    let mut proteins: f64 = 0.0;
    let mut fats: f64 = 0.0;


    for f in foods {

    fats += f.fats * f.nbr_of_portions;
    carbs += f.carbs * f.nbr_of_portions;
    proteins += f.proteins * f.nbr_of_portions;

    if f.calories.1.contains("kcal") {
    cals += f.calories.1.replace("kcal", "").parse::<f64>().unwrap() * f.nbr_of_portions;
    }
}
    let instantiated = object!{
        "cals": ((cals * 100.0).round() / 100.0),
        "carbs": ((carbs * 100.0).round() / 100.0),
        "proteins": ((proteins * 100.0).round() / 100.0),
        "fats": ((fats * 100.0).round() / 100.0)
    };
return instantiated;
}
