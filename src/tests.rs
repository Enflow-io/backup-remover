
#[cfg(test)]
mod tests {
    use crate::{file_generator::{cleanup_files, generate_daily_files}, get_checkers, get_files_list, remove_files, store::Store, Config};
    use crate::{check_files};
    use crate::file_generator::generate_weekly_files;

    #[test]
    fn test_daily_files() {
        let configs: Vec<Config> = vec![
            Config {
                period: "1d",
                qnt: 4,
            }
        ];
        let mut store = Store::new();
        let _ = cleanup_files();
        let _ = generate_daily_files(10);
        let files_list = get_files_list().unwrap();
        let checkers = get_checkers(configs);
        let _ = check_files(&files_list, &checkers, &mut store);

        let _ = remove_files(&store.files_to_delete);
        let file_in_folder = std::fs::read_dir("test-data").unwrap().count();

        assert_eq!(file_in_folder, 4);
    }

    #[test]
    fn test_weekly_files() {
        let configs: Vec<Config> = vec![
            Config {
                period: "1w",
                qnt: 7,
            }
        ];

        let mut store = Store::new();

        let _ = cleanup_files();
        let _ = generate_weekly_files(10);
        let files_list = get_files_list().unwrap();
        let checkers = get_checkers(configs);
        let _ = check_files(&files_list, &checkers, &mut store);


        let _ = remove_files(&store.files_to_delete);

        let file_in_folder = std::fs::read_dir("test-data").unwrap().count();
        assert_eq!(file_in_folder, 7);

    }

    #[test]
    fn test_daily_and_weekly_files() {
        let configs: Vec<Config> = vec![
            Config {
                period: "1d",
                qnt: 3,
            },
            Config {
                period: "1w",
                qnt: 3,
            }
        ];
        let _ = cleanup_files();
        let mut store = Store::new();

        let _ = cleanup_files();
        let _ = generate_daily_files(27);
        let files_list = get_files_list().unwrap();
        let checkers = get_checkers(configs);
        let _ = check_files(&files_list, &checkers, &mut store);
        println!("F: {:?}", store.files_to_delete);
        let _ = remove_files(&store.files_to_delete);
        let file_in_folder = std::fs::read_dir("test-data").unwrap().count();
        assert_eq!(file_in_folder, 6);
    }
}