use onmi_ledger;

#[test]
fn test_my_bubble_sort() {
    let mut data = [5, 3, 8, 1];
    onmi_ledger::bubble_sort(&mut data);
    assert_eq!(data, [1, 3, 5, 8]);
}