pub mod wind{
    use std::num;

    // 返回风向以及风速，风向（0-360）
    pub fn get_wind() ->i32{
        return 0;
    }

    // 判断对象是否在可航行区域内
    pub fn judge_zone(wind_ange:i32, obj_angle:i32) ->bool {
        let bigger = if wind_ange >= obj_angle{wind_ange} else {obj_angle};
        let smaller = if wind_ange < obj_angle {wind_ange} else {obj_angle};
        if (bigger-smaller <= 45) && (smaller+360-bigger<=45){
            false
        }else {
            true
        }
    }
}

pub mod sailing_body{
    pub enum BodyPosition {
        Upward,
        Downward
    }
    pub fn body_up_down() -> BodyPosition{
        // 需添加上下浪代码
        let position = BodyPosition::Downward;
        position
    }
}
