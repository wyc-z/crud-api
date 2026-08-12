pub fn description(desc: Option<String>) -> Option<String> {
    desc.and_then(|d| {
        let d = String::from(d.trim());
        (!d.is_empty()).then_some(d)
    })
}
pub fn ids(id: Option<u32>) -> Option<u32> {
    id.and_then(|i| (i != 0).then_some(i))
}
#[cfg(test)]
mod test {
    #[test]
    fn test_description() {
        use super::description;
        assert_eq!(description(Some(String::from("   "))), None);
        assert_eq!(description(Some(String::from(" "))), None);
        assert_eq!(
            description(Some(String::from("Hello"))),
            Some(String::from("Hello"))
        );
        assert_eq!(description(Some(String::from(""))), None);
        assert_eq!(description(None), None)
    }
    #[test]
    fn test_ids() {
        use super::ids;
        assert_eq!(ids(Some(0)), None);
        assert_eq!(ids(None), None)
    }
}
