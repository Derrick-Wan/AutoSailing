mod sail;
mod phone;
mod sonar;
mod sensor;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]

    fn wind_obj_same_side_safe_zone_1(){
        let wind_dir = 30;
        let obj_dir = 80;
        assert_eq!(true, crate::sensor::wind::judge_zone(wind_dir,obj_dir));
    }

    #[test]
    fn wind_obj_same_side_safe_zone_2(){
        let wind_dir = 300;
        let obj_dir = 190;
        assert_eq!(true, crate::sensor::wind::judge_zone(wind_dir,obj_dir));
    }

    #[test]
    fn wind_obj_same_side_unsafe_zone_1(){
        let wind_dir = 30;
        let obj_dir = 40;
        assert_eq!(false, crate::sensor::wind::judge_zone(wind_dir,obj_dir));
    }

    #[test]
    fn wind_obj_same_side_unsafe_zone_2(){
        let wind_dir = 300;
        let obj_dir = 280;
        assert_eq!(false, crate::sensor::wind::judge_zone(wind_dir,obj_dir));
    }

    #[test]
    fn wind_obj_diff_side_safe_zone(){
        let wind_dir = 90;
        let obj_dir = 270;
        assert_eq!(true, crate::sensor::wind::judge_zone(wind_dir,obj_dir));
    }

    #[test]
    fn wind_obj_diff_side_unsafe_zone_1(){
        let wind_dir = 10;
        let obj_dir = 350;
        assert_eq!(false, crate::sensor::wind::judge_zone(wind_dir,obj_dir));
    }

    #[test]
    fn wind_obj_diff_side_unsafe_zone_2(){
        let wind_dir = 190;
        let obj_dir = 170;
        assert_eq!(false, crate::sensor::wind::judge_zone(wind_dir,obj_dir));
    }
}


