pub fn lowest_price(books: &[u32]) -> u32 {
    let discounted = [0, 800, 1520, 2160, 2560, 3000];
    let mut baskets: Vec<Vec<u32>> = Vec::new();

    for book in books {
        match baskets
            .iter_mut()
            .filter(|basket| !basket.contains(book))
            .min_by_key(|basket| discounted[basket.len() + 1] - discounted[basket.len()])
        {
            Some(basket) => basket.push(*book),
            None => baskets.push(vec![*book]),
        }
    }
    
    baskets.iter().map(|basket| discounted[basket.len()]).sum()
}
