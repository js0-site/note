fn main() {
    // This will cause a compilation error due to missing send_guard feature
    error::attempt_send_guard_without_feature();

    // This shows a working alternative approach
    error::working_alternative_without_send_guard();
}
