use std::time::Instant;

fn mul_by_cols(m_len: usize) {
    let m_a: Vec<Vec<f32>> = vec![vec![1.0; m_len]; m_len];
    let mut m_b: Vec<Vec<f32>> = vec![];
    let mut m_c: Vec<Vec<f32>> = vec![vec![0.0; m_len]; m_len];

    for i in 0..m_len {
        m_b.insert(i, vec![(i+1) as f32; m_len]);
    }

    let begin = Instant::now();

    for i in 0..m_len {
        for j in 0..m_len {
            let mut tmp: f32 = 0.0;
            for k in 0..m_len {
                tmp += m_a[i][k] * m_b[k][j]
            }
            m_c[i][j] = tmp;
        }
    }

    let elapsed = begin.elapsed();
    println!("elapsed time [mul_by_cols]: {:.3?}", elapsed);

    let elem : usize = m_len * (m_len+1)/2;
    assert_eq!(m_c[0][0], elem as f32);
    println!("passed arithmetic progression test: {}", elem);
}

fn main() {
    mul_by_cols(1000);
}
