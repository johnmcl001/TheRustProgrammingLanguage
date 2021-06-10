pub fn public_function() {
    println!("called lib public function");
}

fn private_function() {
    println!("called lib private function");
}

pub fn indirect_access() {
    println!("called lib indirect access");
    private_function();
}
