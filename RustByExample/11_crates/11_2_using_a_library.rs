fn main() {
    library::public_function();

    // Error because function is private
    // library::private_function();
    
    library::indirect_access();
}
