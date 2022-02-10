use crate::decorators;
use crate::protocols::{colormanager, PathType};
use crate::services;
use std::fs;

#[derive(Clone)]
pub struct FileStyle {
    pub positions: DisplayPositions,
    dir_name_color: String,
    file_name_color: String,
    file_time_color: String,
    file_size_color: String,
    file_owner_color: String,
    file_extension_color: String,
    file_size_style: String,
    file_owner_style: String,
    file_perms_style: String,
    file_time_style: String,
    file_extension_style: String,
    num_positions: i32,
    dir_num_positions: i32,
    pub dir_color: String,
    pub symlink_color: String,
    pub path_color: String,
    pub pipe_color: String,
    pub chard_color: String,
    pub blockd_color: String,
    pub socket_color: String,
    pub read_color: String,
    pub write_color: String,
    pub execute_color: String,
    pub dash_color: String,
}
// TODO: change perms
impl FileStyle {
    pub fn new(
        display_pos: DisplayPositions,
        dir_name_col: String,
        file_name_col: String,
        file_time_col: String,
        file_size_col: String,
        file_owner_col: String,
        file_ext_col: String,
        file_size_sty: String,
        file_owner_sty: String,
        file_perms_sty: String,
        file_time_sty: String,
        file_ext_sty: String,
        file_num_pos: i32,
        dir_num_pos: i32,
        dir_col: String,
        symlink_col: String,
        path_col: String,
        pipe_col: String,
        chard_col: String,
        blockd_col: String,
        socket_col: String,
        read_col: String,
        write_col: String,
        execute_col: String,
        dash_col: String,
    ) -> Self {
        Self {
            positions: display_pos,
            dir_name_color: dir_name_col.to_uppercase(),
            file_name_color: file_name_col.to_uppercase(),
            file_time_color: file_time_col.to_uppercase(),
            file_size_color: file_size_col.to_uppercase(),
            file_owner_color: file_owner_col.to_uppercase(),
            file_extension_color: file_ext_col.to_uppercase(),
            file_size_style: file_size_sty.to_uppercase(),
            file_owner_style: file_owner_sty.to_uppercase(),
            file_perms_style: file_perms_sty.to_uppercase(),
            file_time_style: file_time_sty.to_uppercase(),
            file_extension_style: file_ext_sty.to_uppercase(),
            num_positions: file_num_pos,
            dir_num_positions: dir_num_pos,
            dir_color: dir_col,
            symlink_color: symlink_col,
            path_color: path_col,
            pipe_color: pipe_col,
            chard_color: chard_col,
            blockd_color: blockd_col,
            socket_color: socket_col,
            read_color: read_col,
            write_color: write_col,
            execute_color: execute_col,
            dash_color: dash_col,
        }
    }
}

#[derive(Clone)]
pub struct File {
    pub path: std::path::PathBuf,
    pub file_type: Vec<PathType>,
    pub group: String,
    pub user: String,
    pub modified: String,
    pub accessed: String,
    pub created: String,
    pub size: String,
    pub perms: String,
    pub styles: FileStyle,
    pub file_time_type: String,
    pub show_extension: bool,
    pub display_positions: DisplayPositions,
    pub show_short: bool,
}

#[derive(Clone)]
pub struct DisplayPositions {
    file_size_position: i32,
    file_owner_position: i32,
    file_perms_position: i32,
    file_time_position: i32,
    file_extension_position: i32,
}

impl DisplayPositions {
    pub fn new(
        file_size_pos: i32,
        file_owner_pos: i32,
        file_perms_pos: i32,
        file_time_pos: i32,
        file_extension_pos: i32,
    ) -> Self {
        Self {
            file_extension_position: file_extension_pos,
            file_time_position: file_time_pos,
            file_perms_position: file_perms_pos,
            file_owner_position: file_owner_pos,
            file_size_position: file_size_pos,
        }
    }
}

impl File {
    // TODO: add diff time options
    pub fn new(
        path: std::path::PathBuf,
        time_format: &str,
        file_time_type: String,
        styles: FileStyle,
        show_extension: bool,
        display_positions: DisplayPositions,
        show_short: bool,
    ) -> anyhow::Result<Self> {
        let metadata = path.metadata()?;
        Ok(Self {
            group: services::group(&metadata),
            user: services::user(&metadata),
            modified: services::time::time_modified(&metadata, time_format),
            created: services::time::time_created(&metadata, time_format),
            accessed: services::time::time_accessed(&metadata, time_format),
            size: services::size::size(&metadata)?,
            perms: services::perms::perms(&metadata, styles.clone()),
            file_type: PathType::new(&metadata),
            path,
            styles,
            file_time_type,
            show_extension,
            display_positions,
            show_short,
        })
    }

    fn get_color_for(&self, typ: &str, input: String) -> String {
        if typ == "FILE_PERMS_POSITION" {
            return input;
        }
        match typ {
            "FILE_OWNER_POSITION" => {
                colormanager::colorize_string(&self.styles.file_owner_color, input)
            }
            "FILE_SIZE_POSITION" => {
                colormanager::colorize_string(&self.styles.file_size_color, input)
            }
            "DIR_NAME_POSITION" => {
                colormanager::colorize_string(&self.styles.dir_name_color, input)
            }
            "FILE_NAME_POSITION" => {
                colormanager::colorize_string(&self.styles.file_name_color, input)
            }
            "FILE_TIME_POSITION" => {
                colormanager::colorize_string(&self.styles.file_time_color, input)
            }
            "FILE_EXTENSION_POSITION" => {
                colormanager::colorize_string(&self.styles.file_extension_color, input)
            }
            _ => "".to_string(),
        }
    }

    pub fn display_format(&self) -> anyhow::Result<String> {
        let mut res = String::new();
        for (i, v) in self.file_type.iter().enumerate() {
            if i == 0 {
                res = v.get_text_traits_for_type(
                    &self
                        .path
                        .components()
                        .next_back()
                        .ok_or_else(|| anyhow::anyhow!(""))?
                        .as_os_str()
                        .to_string_lossy()
                        .to_string(),
                    &self.path,
                );
                res = format!("{}{}", v.get_color_for_type(self.styles.clone()), res);
                continue;
            }
            res = v.get_text_traits_for_type(&res, &self.path);
            res = format!("{}{}", v.get_color_for_type(self.styles.clone()), res);
        }

        if self.show_short {
            return Ok(res);
        }
        let metadata = fs::metadata(&self.path)?;
        let elements: String = (1..=5)
            .into_iter()
            .map::<anyhow::Result<String>, _>(|i| {
                Ok(self.get_styled_text(
                    &self.get_color_for(
                        &self.get_position_category(i),
                        self.get_result_for_position(&self.get_position_category(i))?,
                    ),
                    &self.get_style_for_position(&self.get_position_category(i)),
                    i == self.styles.dir_num_positions,
                ))
            })
            .fold(Ok(String::new()), |acc: anyhow::Result<String>, element| {
                Ok(acc? + &element?)
            })?;
        if metadata.is_dir() {
            let file_count = fs::read_dir(&self.path)?.count();
            Ok(format!("{} [{}] ({} items)", res, elements, file_count))
        } else {
            Ok(format!("{} [{}]", res, elements))
        }
    }

    pub fn get_styled_text(&self, text: &str, style: &str, is_end: bool) -> String {
        if text.is_empty() {
            return "".to_string();
        };
        let mut result = match style {
            "BOLD" => decorators::bold(text),
            "DIMMED" => decorators::dimmed(text),
            "ITALIC" => decorators::italic(text),
            "UNDERLINE" => decorators::underline(text),
            "BLINK" => decorators::blink(text),
            "REVERSE" => decorators::reverse(text),
            "HIDDEN" => decorators::hidden(text),
            "STRICKEN" => decorators::stricken(text),
            "NORMAL" => text.to_string(),
            _ => decorators::bold("INVALID FONT STYLE"),
        };
        if !is_end {
            result.push(' ');
        }
        result
    }

    pub fn get_name(&self) -> String {
        let mut res = String::new();
        for (i, v) in self.file_type.iter().enumerate() {
            if i == 0 {
                res = v.get_text_traits_for_type(
                    &self
                        .path
                        .components()
                        .next_back()
                        .unwrap()
                        .as_os_str()
                        .to_string_lossy()
                        .to_string(),
                    &self.path,
                );
                res = format!("{}{}", v.get_color_for_type(self.styles.clone()), res);
                continue;
            }
            res = v.get_text_traits_for_type(&res, &self.path);
            res = format!("{}{}", v.get_color_for_type(self.styles.clone()), res);
        }
        res
    }

    pub fn get_position_category(&self, index: i32) -> String {
        if self.display_positions.file_size_position == index {
            return "FILE_SIZE_POSITION".to_string();
        }

        if self.display_positions.file_owner_position == index {
            return "FILE_OWNER_POSITION".to_string();
        }

        if self.display_positions.file_perms_position == index {
            return "FILE_PERMS_POSITION".to_string();
        }

        if self.display_positions.file_time_position == index {
            return "FILE_TIME_POSITION".to_string();
        }

        if self.display_positions.file_extension_position == index {
            return "FILE_EXTENSION_POSITION".to_string();
        }

        "".to_string()
    }

    // TODO: do timing stuff (env check if mod or created or accessed)
    pub fn get_result_for_position(&self, position: &str) -> anyhow::Result<String> {
        match position {
            "FILE_SIZE_POSITION" => Ok(self.size.to_string()),
            "FILE_OWNER_POSITION" => Ok(self.user.to_string()),
            "FILE_PERMS_POSITION" => Ok(self.perms.to_string()),
            "FILE_TIME_POSITION" => {
                if self.file_time_type == "CREATED" {
                    Ok(self.created.to_string())
                } else {
                    Ok(self.modified.to_string())
                }
            }
            "FILE_EXTENSION_POSITION" => services::extension::extension(&self.path),
            _ => Ok("".to_string()),
        }
    }

    pub fn get_style_for_position(&self, position: &str) -> String {
        match position {
            "FILE_SIZE_POSITION" => self.styles.file_size_style.clone(),
            "FILE_OWNER_POSITION" => self.styles.file_owner_style.clone(),
            "FILE_PERMS_POSITION" => self.styles.file_perms_style.clone(),
            "FILE_TIME_POSITION" => self.styles.file_time_style.clone(),
            "FILE_EXTENSION_POSITION" => self.styles.file_extension_style.clone(),
            _ => "".to_string(),
        }
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = String::new();
        for (i, v) in self.file_type.iter().enumerate() {
            if i == 0 {
                res = format!(
                    "{}{}",
                    v.get_color_for_type(self.styles.clone()),
                    v.get_text_traits_for_type(
                        &self
                            .path
                            .components()
                            .next_back()
                            .unwrap()
                            .as_os_str()
                            .to_string_lossy()
                            .to_string(),
                        &self.path
                    )
                );
            } else {
                res = format!(
                    "{}{}",
                    v.get_color_for_type(self.styles.clone()),
                    v.get_text_traits_for_type(&res, &self.path)
                );
            }
        }
        write!(f, "{}", res)
    }
}

impl std::fmt::Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = String::new();
        for (i, v) in self.file_type.iter().enumerate() {
            if i == 0 {
                res = v.get_text_traits_for_type(
                    &self
                        .path
                        .components()
                        .next_back()
                        .unwrap()
                        .as_os_str()
                        .to_string_lossy()
                        .to_string(),
                    &self.path,
                );
                res = format!("{}{}", v.get_color_for_type(self.styles.clone()), res);
                continue;
            }

            res = v.get_text_traits_for_type(&res, &self.path);
            res = format!("{}{}", v.get_color_for_type(self.styles.clone()), res);
        }

        let metadata = fs::metadata(&self.path).unwrap();
        if metadata.is_dir() {
            let file_count = fs::read_dir(&self.path).unwrap().count();
            if self.show_short {
                return writeln!(f, "{}", res);
            }
            return writeln!(f, "{} [{element_one}{element_two}{element_three}{element_four}{element_five}] ({} items)",
        res, file_count,
         element_one = self.get_styled_text(&self.get_color_for(&self.get_position_category(1), self.get_result_for_position(&self.get_position_category(1)).unwrap()), &self.get_style_for_position(&self.get_position_category(1)), 1 == self.styles.dir_num_positions),
         element_two = self.get_styled_text(&self.get_color_for(&self.get_position_category(2), self.get_result_for_position(&self.get_position_category(2)).unwrap()), &self.get_style_for_position(&self.get_position_category(2)), 2 == self.styles.dir_num_positions),
         element_three = self.get_styled_text(&self.get_color_for(&self.get_position_category(3), self.get_result_for_position(&self.get_position_category(3)).unwrap()), &self.get_style_for_position(&self.get_position_category(3)), 3 == self.styles.dir_num_positions),
         element_four = self.get_styled_text(&self.get_color_for(&self.get_position_category(4), self.get_result_for_position(&self.get_position_category(4)).unwrap()), &self.get_style_for_position(&self.get_position_category(4)), 4 == self.styles.dir_num_positions),
         element_five = self.get_styled_text(&self.get_color_for(&self.get_position_category(5), self.get_result_for_position(&self.get_position_category(5)).unwrap()), &self.get_style_for_position(&self.get_position_category(5)), 5 >= self.styles.dir_num_positions)
       );
        } else {
            if self.show_short {
                return writeln!(f, "{}", res);
            }
            return writeln!(
                f,
                "{} [{element_one}{element_two}{element_three}{element_four}{element_five}]",
                res,
                element_one = self.get_styled_text(
                    &self.get_color_for(
                        &self.get_position_category(1),
                        self.get_result_for_position(&self.get_position_category(1))
                            .unwrap()
                    ),
                    &self.get_style_for_position(&self.get_position_category(1)),
                    1 == self.styles.num_positions
                ),
                element_two = self.get_styled_text(
                    &self.get_color_for(
                        &self.get_position_category(2),
                        self.get_result_for_position(&self.get_position_category(2))
                            .unwrap()
                    ),
                    &self.get_style_for_position(&self.get_position_category(2)),
                    2 == self.styles.num_positions
                ),
                element_three = self.get_styled_text(
                    &self.get_color_for(
                        &self.get_position_category(3),
                        self.get_result_for_position(&self.get_position_category(3))
                            .unwrap()
                    ),
                    &self.get_style_for_position(&self.get_position_category(3)),
                    3 == self.styles.num_positions
                ),
                element_four = self.get_styled_text(
                    &self.get_color_for(
                        &self.get_position_category(4),
                        self.get_result_for_position(&self.get_position_category(4))
                            .unwrap()
                    ),
                    &self.get_style_for_position(&self.get_position_category(4)),
                    4 == self.styles.num_positions
                ),
                element_five = self.get_styled_text(
                    &self.get_color_for(
                        &self.get_position_category(5),
                        self.get_result_for_position(&self.get_position_category(5))
                            .unwrap()
                    ),
                    &self.get_style_for_position(&self.get_position_category(5)),
                    5 == self.styles.num_positions
                )
            );
        }
    }
}
