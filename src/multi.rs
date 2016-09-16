use std::cmp::Ordering;

fn bucket_distance(fi: f64, hi: f64, delta: i32, W: f64) -> f64 {
    if delta == -1 {
        W*(fi - hi)
    } else if delta == 1 {
        W - W*(fi - hi)
    } else {
        0 as f64
    }
}
fn compute_sorted_i_delta<T>(q: &[T], f_sig: &[f64], h_sig: &[f64], W: f64) -> Vec<(usize, i32)> {
    let mut intermediate_vec: Vec<((usize,i32), f64)> = f_sig.iter().zip(h_sig.iter()).enumerate().flat_map(|(i, (fi, hi))| {
        vec![((i, 1), bucket_distance(*fi, *hi, 1, W)),
         ((i, -1), bucket_distance(*fi, *hi, -1, W))].into_iter()
    }).collect();
    intermediate_vec.sort_by(|a, b| {
        if a.1 > b.1 {
            Ordering::Greater
        } else if a.1 < b.1 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
    intermediate_vec.iter().map(|a| {a.0}).collect()
}
#[test]
fn sorted_delta_test() {
    let test_q = vec![1.0,2.0,3.0,4.0,5.0];
    let f_sig = vec![1.5,1.2,2.2];
    let h_sig = vec![1.0,1.0,2.0];
    let W = 10.0;
    compute_sorted_i_delta(&test_q, &f_sig, &h_sig, W);
}
fn score_set(perturbation_set: &[usize], square_zj_list: &[f64]) -> f64 {
    perturbation_set.iter().map(|ind| {square_zj_list[*ind]}).sum()
}

#[derive(PartialEq)]
struct PerturbationSet<'a> {
    data: Vec<usize>,
    zj_list: &'a Vec<f64>
}

impl<'a> Eq for PerturbationSet<'a> {}

impl<'a> Ord for PerturbationSet<'a> {
    fn cmp(&self, other: &PerturbationSet) -> Ordering {
        let self_score = score_set(&(self.data), self.zj_list);
        let other_score = score_set(&(other.data), other.zj_list);
        if (self_score - other_score) <= 1e-6 {
            Ordering::Equal
        } else if self_score > other_score {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl<'a> PartialOrd for PerturbationSet<'a> {
    fn partial_cmp(&self, other: &PerturbationSet) -> Option<Ordering> {
        let self_score = score_set(&(self.data), self.zj_list);
        let other_score = score_set(&(other.data), other.zj_list);
        self_score.partial_cmp(&other_score)
    }
}
impl<'a> PerturbationSet<'a> {
    fn shift(&self) -> PerturbationSet {
        let mut new_data = Vec::new();
        let max_val = self.data.iter().max().unwrap();
        for x in self.data.iter() {
            if *x == *max_val {
                new_data.push(*x+1);
            } else {
                new_data.push(*x);
            }
        }
        PerturbationSet {
            data: new_data,
            zj_list: self.zj_list
        }
    }
    fn expand(&self) -> PerturbationSet {
        let mut new_data = Vec::new();
        let max_val = self.data.iter().max().unwrap();
        for x in self.data.iter() {
            new_data.push(*x);
        }
        new_data.push(*max_val + 1);
        PerturbationSet {
            data: new_data,
            zj_list: self.zj_list
        }
    } 
}