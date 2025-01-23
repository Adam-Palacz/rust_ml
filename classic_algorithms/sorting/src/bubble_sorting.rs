pub fn bubble_sorting(v: &mut Vec<f64>) -> &Vec<f64> {
    while true {
        let mut swapped = false;
        for i in 1..v.len() {
            if v[i - 1] > v[i] {
                v.swap(i - 1, i);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    return v;
}