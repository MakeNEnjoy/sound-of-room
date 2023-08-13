pub struct TransparentNode {

}

impl TransparentNode {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process(&mut self, input: &mut [f32], output: &mut [f32]) -> bool {
        output.copy_from_slice(input);
        true
    }
}