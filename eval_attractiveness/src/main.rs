static ATT_W: f32 = 10.0; // Speed of population increase
static DEC_W: f32 = 0.05; // amount of decreasement of attractiveness

fn main() {
    let mut pop_limit = 100_000_000;
    let mut pop = 0;
    let mut att = 50.0;
    for i in 0..20 {
        if i == 10 {
            att = -50.0
        }
        let (new_pop, new_att) = get_population(att, pop, pop_limit);
        println!("Newpop : {} Newatt : {}", new_pop, new_att);
        pop = new_pop;
        att = new_att;
    }
}

fn get_population(attractiveness: f32, population: i32, population_limit: i32) -> (i32, f32) {
    // returns new pop, att
    let mut pop_cal_int: i32 = unsafe { (attractiveness * ATT_W).floor().to_int_unchecked() };
    if pop_cal_int > population_limit - population {
        pop_cal_int = population_limit - population;
    }
    let mut new_pop: i32 = pop_cal_int + population;
    if new_pop < 0 {
        if population == 0 {
            pop_cal_int = 0;
        }
        new_pop = 0
    }
    let att_cal = DEC_W * pop_cal_int as f32;
    let new_att = attractiveness - att_cal;

    (new_pop, new_att)
}
