pub trait GPU<InputData, OutputTarget> {
    fn draw_primitive(&self, input: &InputData, output: &mut OutputTarget);
}
