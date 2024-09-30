pub fn diverges_within_max_iterations(re_c: f64, im_c: f64, max_iterations: usize) -> Option<usize> {
    let (mut re_zn, mut im_zn) = (0f64, 0f64);

    for iter in 0..max_iterations {
        let a2 = re_zn * re_zn;
        let b2 = im_zn * im_zn;

        if a2 + b2 > 4.0 {
            // |zn| > 2
            return Some(iter);
        }

        let ab = re_zn * im_zn;

        re_zn = a2 - b2 + re_c;
        im_zn = 2.0 * ab + im_c;
    }

    None
}
