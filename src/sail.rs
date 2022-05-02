pub mod finding_mod{
    use crate::sonar;
    use crate::phone;
    pub fn free_sail() -> bool{
        while !sonar::finding_mod::find_fish(10) {
        //     这里需要改成并发操作，不能用轮询。
        }
        phone::found_fish();
        true
    }
}

pub mod tracing_mod{
    use crate::sonar::trace_fish::get_location;
    use crate::sail::sail_base;
    use crate::sail::sail_base::{change_direction, sail_in_unsail_zone_prepare};
    use crate::sensor;
    use crate::sensor::wind;
    pub fn trace_fish(){
    //     判断鱼群现在所处方位
        let fish_position = get_location();
    //     判断风向
        let wind_position = wind::get_wind();

    //判断鱼群是否在可航行区域内
        if wind::judge_zone(wind_position,fish_position){
        //     如果在可航行区域内
            sail_base::sail_with_angle_in_sail_zone(fish_position);
        }else {
            // 调整船身至与风的夹角为45度
            sail_in_unsail_zone_prepare(wind_position);
            let fish_new_position = get_location();
            if fish_new_position<=45&&(360-fish_new_position)<=45{
                return
            }else {
                change_direction();
                return
            }
        }
    }
}

pub mod sail_base{
    use std::num;
    use crate::Steering_engine;
    use crate::sensor;
    pub fn sail_with_angle_in_sail_zone(angle: i32){
        // 调整角度较小的时候
        if num::abs(angle-180) >= 90 {
            Steering_engine::turn_steering(if angle<180 {angle*2/3} else { (angle-360)*2/3 });
        }else {
            Steering_engine::turn_steering(if angle<180 {60} else {-60})
        }
    }

    pub fn sail_in_unsail_zone_prepare(wind_angle:i32){
        // 如果风是从右侧吹来
        if wind_angle<180{
            Steering_engine::turn_steering(30);
            while num::abs(sensor::wind::get_wind()-45)>=5 {
            //     要修改成并发操作
            }
            Steering_engine::turn_steering(0);
        }else {
            // 如果风从左侧吹来
            Steering_engine::turn_steering(-30);
            while num::abs(sensor::wind::get_wind()-(360-45)>=5) {
            //     要修改成并发操作
            }
            Steering_engine::turn_steering(0);
        }
    }

    pub fn change_direction(){

    }

    pub fn upward(){

    }

    pub fn downward(){

    }
}