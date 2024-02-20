use crate::FilesInTrash;
use std::fs::{create_dir_all, remove_dir_all};

#[test]
fn test_files_in_trash() {
    let test_dir = "test";

    create_dir_all(test_dir).unwrap();

    let test_json = "test/test.json";

    FilesInTrash::new(vec![]).write(test_json).unwrap();

    assert_eq!(
        FilesInTrash::read(test_json).unwrap(),
        FilesInTrash::new(vec![])
    );

    remove_dir_all(test_dir).unwrap();
}
