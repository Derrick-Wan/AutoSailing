pub mod finding_mod{
    pub fn free_sail(){

    }
}

pub mod tracing_mod{
    use crate::sonar::trace_fish::get_location;
    use crate::sail::sail_base;
    use crate::sensor;
    use crate::sensor::wind;
    pub fn trace_fish(period:i32){
    //     判断鱼群现在所处方位
        let fish_position = get_location();
    //     判断风向
        let wind_position = wind::get_wind();

    //判断鱼群是否在可航行区域内
        if wind::judge_zone(wind_position,fish_position){
        //     如果在可航行区域内
            sail_base::sail_with_angle_in_safe_zone(fish_position);
        }else { sail_base::sail_in_unsafe_zone(fish_position); }
    }
}

pub mod sail_base{
    pub fn sail(angle:i32, speed:i32){
    //     向某方向，以某速度航行
    }
    pub fn sail_with_angle_in_safe_zone(angle: i32){

    }
    pub fn sail_in_unsafe_zone(angle: i32){

    }
}