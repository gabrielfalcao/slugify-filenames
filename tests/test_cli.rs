#![allow(unused)]
use iocore::Path;
use iocore::{walk_dir, NoopProgressHandler};
use iocore_test::{folder_path, path_to_test_file};
use slugify::cli::SlugifyFilenames;
use slugify::errors::Result;

fn setup() {
    let folders = vec![
        folder_path!("recursive"),
        folder_path!("recursive files"),
        folder_path!("recursive-files"),
    ];

    for folder in folders {
        match folder.delete() {
            Ok(_) | Err(_) => {}
        }
    }
}
#[test]
fn test_slugify_filenames_recursive_files() -> Result<()> {
    setup();
    path_to_test_file!("recursive/ filename with spaces ").write_unchecked(&[]);
    path_to_test_file!("recursive/filename_with-Special!CharacteR$").write_unchecked(&[]);
    path_to_test_file!("recursive/ filename with spaces and Special # CharacteR$ ").write_unchecked(&[]);

    let test_path = folder_path!().name();
    assert_eq!(&test_path, "tests");

    let file_list = path_to_test_file!("recursive").parent().unwrap().list()?;
    assert!(file_list.len() > 0);
    assert_eq!(
        file_list[0],
        Path::new("tests/recursive/ filename with spaces and Special # CharacteR$ . .txt "),
    );
    assert_eq!(
        file_list[1],
        Path::new("tests/recursive/filename_with-Special!CharacteR$.txt"),
    );
    assert_eq!(
        file_list[2],
        Path::new("tests/recursive/ filename with spaces .txt "),
    );

    SlugifyFilenames::execute(vec![String::from("slugify-filenames"), test_path])?;

    let file_list = folder_path!("recursive").list()?;
    assert_eq!(
        file_list[0],
        Path::new("tests/recursive/filename-with-spaces-and-special-character.txt"),
    );
    assert_eq!(
        file_list[1],
        Path::new("tests/recursive/filename_with-special-character.txt"),
    );
    assert_eq!(
        file_list[2],
        Path::new("tests/recursive/filename-with-spaces.txt"),
    );
    Ok(())
}

#[test]
fn test_slugify_filenames_recursive_files_and_folders() -> Result<()> {
    setup();
    path_to_test_file!("recursive files/ and folders/ filename with spaces ").write_unchecked(&[]);
    path_to_test_file!("recursive files/ and folders/filename_with-Special!CharacteR$").write_unchecked(&[]);
    path_to_test_file!(
        "recursive files/ and folders/ filename with spaces and Special # CharacteR$ "
    ).write_unchecked(&[]);

    let test_path = folder_path!().name();
    assert_eq!(&test_path, "tests");

    assert!(
        folder_path!("recursive files").exists(),
        "the path \"recursive files\" should exist"
    );
    assert!(
        folder_path!("recursive files/ and folders").exists(),
        "the path \"recursive files/ and folders\" should exist"
    );
    let file_list = path_to_test_file!("recursive files/ and folders/").parent().unwrap().list()?;


    assert!(file_list.len() > 0);
    assert_eq!(
        file_list[2],
        Path::new("tests/recursive files/ and folders/ filename with spaces .txt "),
    );
    assert_eq!(
        file_list[0],
        Path::new("tests/recursive files/ and folders/ filename with spaces and Special # CharacteR$ . .txt "),
    );
    assert_eq!(
        file_list[1],
        Path::new("tests/recursive files/ and folders/filename_with-Special!CharacteR$.txt"),
    );

    SlugifyFilenames::execute(vec![String::from("slugify-filenames"), test_path])?;

    assert!(
        folder_path!("recursive-files").exists(),
        "the path \"recursive-files\" should exist"
    );
    assert!(
        folder_path!("recursive-files/and-folders").exists(),
        "the path \"recursive-files/and-folders\" should exist"
    );
    let file_list = folder_path!("recursive-files/and-folders").list()?;
    assert_eq!(
        file_list[0],
        Path::new(
            "tests/recursive-files/and-folders/filename-with-spaces-and-special-character.txt"
        ),
    );
    assert_eq!(
        file_list[1],
        Path::new("tests/recursive-files/and-folders/filename_with-special-character.txt")
    );
    assert_eq!(
        file_list[2],
        Path::new("tests/recursive-files/and-folders/filename-with-spaces.txt")
    );
    Ok(())
}
