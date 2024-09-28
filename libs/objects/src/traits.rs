
pub trait ObjectTrait<const D: usize> {
    fn calculate_interaction(&mut self, other: &mut Self);
    fn time_propagate(&mut self, time: f64);
}
