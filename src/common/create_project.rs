use super::{Language, ProjectConfig, ProjectType};
use crate::features::{download_file, unzip};
use anyhow::Error;
use std::{env, fs};
pub(crate) fn create_project(config: &ProjectConfig) -> Result<String, Error> {
    let project_path = config.path.join(&config.name);
    fs::create_dir_all(&project_path)?;
    config.vcs.init_vcs_repo(&config.name, &config.path)?;
    let mut main_file_to_edit = "";
    match config.project_type {
        ProjectType::SpringBoot => {
            let params = [
                ("type", "maven-project"),
                ("language", &config.language.to_string().to_lowercase()),
                ("javaVersion", &config.language_version),
                ("bootVersion", &config.project_version),
                ("baseDir", &config.name),
            ];
            let temp_zip_file = env::temp_dir().join("starter.zip");
            download_file(
                "https://start.spring.io/starter.zip",
                &params,
                &temp_zip_file,
            )?;
            unzip(&temp_zip_file, &config.path)?;
            fs::remove_file(&temp_zip_file)?;
            main_file_to_edit = "src/main/java/com/example/demo/DemoApplication.java";
        }
        ProjectType::CMake => {
            let cmake_lists = format!(
                "\
                cmake_minimum_required(VERSION {})\n\
                project({})\n\
                \n\
                set(CMAKE_{}_STANDARD {})\n\
                \n\
                add_executable(${{PROJECT_NAME}} {})\n",
                config.project_version,
                config.name,
                if config.language == Language::C {
                    "C"
                } else {
                    "CXX"
                },
                config.language_version,
                if config.language == Language::C {
                    "main.c"
                } else {
                    "main.cpp"
                }
            );
            let main_c = "\
                #include <stdio.h>\n\
                \n\
                int main() {\n\
                \tprintf(\"Hello, World!\");\n\
                \treturn 0;\n\
                }\n";
            let main_cpp = "\
                #include <iostream>\n\
                \n\
                int main() {\n\
                \tstd::cout << \"Hello, World!\" << std::endl;\n\
                \treturn 0;\n\
                }\n";
            fs::write(project_path.join("CMakeLists.txt"), cmake_lists)?;
            if config.language == Language::C {
                fs::write(project_path.join("main.c"), main_c)?;
                main_file_to_edit = "main.c";
            } else {
                fs::write(project_path.join("main.cpp"), main_cpp)?;
                main_file_to_edit = "main.cpp";
            }
        }
        _ => {
            println!(
                "Created {} project directory at {}",
                config.project_type,
                project_path.display()
            );
        }
    }
    Ok(main_file_to_edit.to_string())
}
