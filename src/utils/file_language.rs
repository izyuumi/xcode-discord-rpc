/// Represents the different file languages supported
#[derive(Debug)]
pub enum FileLanguage {
    /// Swift
    Swift,
    /// C++
    Cpp,
    /// C
    C,
    /// Ruby
    Ruby,
    /// Java
    Java,
    /// Json
    Json,
    /// Metal
    Metal,
    /// Objective-C
    ObjectiveC,
    /// Unknown or unsupported
    Unknown,
}

impl FileLanguage {
    /// Returns the asset key and text for the `FileLanguage` as (text, image)
    pub fn get_asset_keys(&self) -> (&'static str, &'static str) {
        match self {
            FileLanguage::Swift => ("Swift", "swift"),
            FileLanguage::Cpp => ("C++", "cpp"),
            FileLanguage::C => ("C", "c"),
            FileLanguage::Ruby => ("Ruby", "ruby"),
            FileLanguage::Java => ("Java", "java"),
            FileLanguage::Json => ("Json", "jSON"),
            FileLanguage::Metal => ("Metal", "metal"),
            FileLanguage::ObjectiveC => ("Objective-C", "objc"),
            FileLanguage::Unknown => ("Xcode", "xcode"),
        }
    }

    /// Returns the text for the `FileLanguage`
    pub fn get_text_asset_key(&self) -> &'static str {
        self.get_asset_keys().0
    }

    /// Returns the asset key for the `FileLanguage`
    pub fn get_image_asset_key(&self) -> &'static str {
        self.get_asset_keys().1
    }
}

/// Trait for converting types to `FileLanguage`
pub trait ToFileLanguage {
    /// Converts the implementing type to a `FileLanguage`
    fn to_file_language(&self) -> FileLanguage;
}

/// Implementation of `ToFileLanguage` for `str`
impl ToFileLanguage for str {
    /// Converts `str` to the corresponding `FileLanguage`
    fn to_file_language(&self) -> FileLanguage {
        match self {
            "swift" => FileLanguage::Swift,
            "cpp" | "cp" | "cxx" => FileLanguage::Cpp,
            "c" => FileLanguage::C,
            "m" | "mm" | "h" => FileLanguage::ObjectiveC,
            "ruby" => FileLanguage::Ruby,
            "java" => FileLanguage::Java,
            "json" => FileLanguage::Json,
            "metal" => FileLanguage::Metal,
            _ => FileLanguage::Unknown,
        }
    }
}

/// Trait for getting file extensions
pub trait FileExtention {
    fn get_file_extension(&self) -> String;
}

/// Implementation of `FileExtention` for `str`
impl FileExtention for str {
    /// Returns the file extension of the string
    fn get_file_extension(&self) -> String {
        self.split('.').next_back().unwrap_or("").trim().to_string()
    }
}
