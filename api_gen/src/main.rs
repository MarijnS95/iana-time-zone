use windows_bindgen::bindgen;

fn main() {
    // Intentionally ignore warnings.  We only need the GetTimeZone method on Calendar, but bindgen
    // will complain that it can't (fully) generate a few other methods because of lacking other
    // types.  We're not interested in generating (more bloated / unused) bindings for those.
    let _ = bindgen(["--etc", "bindings.txt"]);
}
