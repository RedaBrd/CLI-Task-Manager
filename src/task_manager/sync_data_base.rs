use std::{error::Error, fs::OpenOptions, io::Write, path::PathBuf};
use serde::{Deserialize, Serialize};
use crate::task_manager::core_features::{Task, TaskList};
use serde_json::{Serializer};


#[derive(Debug,Serialize, Deserialize)]
pub struct DataBase{path: std::path::PathBuf}



impl DataBase{
    pub fn set_data_base(data_base: &str) -> Self{
        DataBase{path : PathBuf::from(data_base)}
    }
    pub fn get_path(&self)->PathBuf{
        self.path.clone()
    }

    pub fn refresh_data(&self, list: &TaskList) -> Result<(), Box<dyn Error>>{
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.path.clone())?;
        let mut serializer = Serializer::new(&file);
        list.serialize(&mut serializer)?;
        Ok(())
    }

    pub fn reset_data(&self) -> Result<(), Box<dyn Error>>{
        std::fs::remove_file(self.path.clone())?;
        Ok(())
    }
    // not getting used might get deleted later if there is no clear purpose for this function to exist.
    fn update_data(&mut self,task: Task)-> Result<(), Box<dyn Error>>{
        let f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.path.clone())?;
            
        let mut serializer = Serializer::new(&f);
        task.serialize(&mut serializer).expect("Failed to serialize task");
        writeln!(&f)?;

        Ok(())
    }


}