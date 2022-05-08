use cargo_snippet::snippet;

#[snippet]
fn rle(s: Vec<char>) -> Vec<(char, usize)> {
    // ランレングス圧縮
    // ref -> https://www.yu2ta7ka-emdded.com/entry/2020/08/13/135134
    let mut res: Vec<(char, usize)> = Vec::new();
    let mut pre = ' ';
    let mut chain = 1;

    for c in s.into_iter() {
        if c == pre {
            chain += 1;
        } else {
            if pre != ' ' {
                res.push((pre, chain));
            }
            pre = c;
            chain = 1;
        }
    }
    res.push((pre, chain));
    res
}
