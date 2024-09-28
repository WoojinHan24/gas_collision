use crate::object_log::Log;

pub trait ObjectTrait {
    fn calculate_interaction(&mut self, other: &mut Self);
    fn time_propagate(&mut self, time: f64);
    fn get_log(self) -> Log;
}
