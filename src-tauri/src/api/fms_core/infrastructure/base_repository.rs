use crate::api::fms_core::AbstractModel;

pub trait BaseRepository<T>{
    // fn get_all(schema: T) -> Vec<dyn AbstractModel>{
    //     let connection = &mut establish_connection();
    //     let tank_profiles = FilterDsl::filter(
    //         schema, schema..eq(profile_tank_id)
    //     // ).select(TankProfile::as_select())
    //     //     .load(connection).expect("Error during selecting tank profiles");
    //     // tank_profiles
    // }
}