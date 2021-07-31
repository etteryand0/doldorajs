#[test]
fn test_regex() {
  use regex::Regex;
  let expression = r"^([^a-z])|[^a-z_0-9-]+?";
  let project_name_re = Regex::new(expression).unwrap();
  assert_eq!(project_name_re.is_match("1fail_fail"), true);
  assert_eq!(project_name_re.is_match("5ыщь_some"), true);
  assert_eq!(project_name_re.is_match("ыодлоывад"), true);
  assert_eq!(project_name_re.is_match("ASsdfk_sdf"), true);
  assert_eq!(project_name_re.is_match("+project"), true);

  assert_eq!(project_name_re.is_match("project+new"), true);
  assert_eq!(project_name_re.is_match("myapp?s"), true);
  assert_eq!(project_name_re.is_match("mya&zs"), true);
  assert_eq!(project_name_re.is_match("myaыаZS"), true);

  assert_eq!(project_name_re.is_match("myapp"), false);
  assert_eq!(project_name_re.is_match("my-app"), false);
  assert_eq!(project_name_re.is_match("my-app_"), false);
  assert_eq!(project_name_re.is_match("hello_world"), false);
  assert_eq!(project_name_re.is_match("hello_my_world"), false);
}
