use std::{fs, io};

use super::{Language, ProjectConfig, ProjectType};
use anyhow::{Context, Error};
use reqwest::blocking::Client;
use zip::ZipArchive;

pub(crate) fn create_project(config: &ProjectConfig) -> Result<String, Error> {
    let project_path = config.path.join(&config.name);
    fs::create_dir_all(&project_path)?;
    config.vcs.init_vcs_repo(&config.name, &config.path)?;
    let mut main_file_to_edit = "";

    match config.project_type {
        ProjectType::SpringBoot => {
            let client = Client::new();
            let params = [
                ("type", "maven-project"),
                ("language", &config.language.to_string().to_lowercase()),
                ("javaVersion", &config.language_version),
                ("bootVersion", &config.project_version),
                ("baseDir", &config.name),
            ];
            let bytes = client
                .post("https://start.spring.io/starter.zip")
                .form(&params)
                .send()
                .context("Failed to send request to Spring Boot starter")?
                .bytes()
                .context("Failed to read response bytes")?;

            // 直接在内存中解压 ZIP
            let mut archive =
                ZipArchive::new(io::Cursor::new(bytes)).context("Failed to parse ZIP archive")?;

            // 确保目标目录存在
            fs::create_dir_all(&config.path).context("Failed to create project directory")?;

            // 解压所有文件到目标目录
            archive
                .extract(&config.path)
                .context("Failed to extract ZIP archive")?;
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
