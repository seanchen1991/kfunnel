use kfunnel::Funnel;

use std::collections::VecDeque;

#[test]
fn test_creation() {
    let mut lbuf: VecDeque<i32> = VecDeque::new();
    let mut rbuf: VecDeque<i32> = VecDeque::new();

    lbuf.push_back(1);
    rbuf.push_back(2);

    let funnel = Funnel::new_with_buffers(lbuf, rbuf);

    assert_eq!(funnel.lbuf.front().unwrap(), &1);
    assert_eq!(funnel.rbuf.front().unwrap(), &2);
}

#[test]
fn test_merge_to_completion() {
    let mut lbuf: VecDeque<i32> = VecDeque::new();
    let mut rbuf: VecDeque<i32> = VecDeque::new();

    lbuf.push_back(1);
    lbuf.push_back(3);
    lbuf.push_back(5);

    rbuf.push_back(2);
    rbuf.push_back(4);
    rbuf.push_back(6);

    let mut funnel = Funnel::new_with_buffers(lbuf, rbuf);
    funnel.merge_to_completion();
    
    let b: &[_] = &[&1, &2, &3, &4, &5, &6];
    let c: Vec<&i32> = funnel.output.iter().collect();

    assert_eq!(&c[..], b);
}
