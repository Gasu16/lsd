use crate::color::{ColoredString, Colors, Elem};
use std::fs::Metadata;


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Count {
    index: Option<u64>,
    indice: i64,
}


impl<'a> From<&'a Metadata> for Count {

    #[cfg(unix)]
    fn from(meta: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt; 
        //println!("Sono in count.rs");
        let index = meta.ino();
        static mut INDICE: i64 = -1;
        unsafe {INDICE += 1};
        Self { index: Some(index), indice: unsafe {INDICE} }
    }

    #[cfg(windows)]
    fn from(_: &Metadata) -> Self {
        Self { index: None }
    }
}

        
impl Count {

    pub fn render(&self, colors: &Colors) -> ColoredString {
        
        match self.index {
            Some(_i) => colors.colorize(self.indice.to_string(), &Elem::Count { valid: true }),
            None => colors.colorize(String::from("-"), &Elem::Count { valid: false }),
        }
    }
}

#[cfg(test)]
#[cfg(unix)]
mod tests {
    use super::Count;
    use std::env;
    use std::io;
    use std::path::Path;
    use std::process::{Command, ExitStatus};

    fn cross_platform_touch(path: &Path) -> io::Result<ExitStatus> {
        Command::new("touch").arg(&path).status()
    }

    #[test]
    fn test_count_no_zero() {
        let mut file_path = env::temp_dir();
        file_path.push("count.tmp");

        let success = cross_platform_touch(&file_path).unwrap().success();
        assert!(success, "failed to exec touch");

        let count = Count::from(&file_path.metadata().unwrap());

        #[cfg(unix)]
        assert!(count.index.is_some());
        #[cfg(windows)]
        assert!(count.index.is_none());
    }
}
