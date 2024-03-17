use std::pin::Pin;

pub trait StateSubscriber<S, SF, CP> {
    fn on_path_change(
        &mut self,
        changed_path: CP,
        callback: Pin<Box<dyn Fn(&S) -> () + Send>>,
    ) -> u32;
    fn on_paths_change(
        &mut self,
        changed_paths: Vec<CP>,
        callback: Pin<Box<dyn Fn(&S) -> () + Send>>,
    ) -> u32;
    fn unsubscribe(&mut self, id: u32);
    fn set_state(&mut self, new_state: S);
    fn get_state(&self) -> S;
}
