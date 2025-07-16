use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::process::exit;
use crate::task_manager::core_features::*;
use crate::task_manager::sync_data_base::*;


pub struct App{
    data_base:DataBase,
    task_list: TaskList
}





impl App{
    pub fn new()->Self{
        App { data_base: DataBase::set_data_base("Logger.txt") , task_list: TaskList(Vec::new())}
    }

    pub fn show_data(&self){
        let mut i = 1;
        for task in &self.task_list{
            println!("{}: {}", i,task.get_title());
            i += 1;
        }
    }

    pub fn sync_task_list(&mut self)-> Result<(), Box<dyn Error>>{
        let file = std::fs::File::open(self.data_base.get_path())?;
        let list: Vec<Task> = serde_json::from_reader(file)?;
        self.task_list.0 = list;
        Ok(())

    }


    pub fn exit(self) -> Result<(), Box<dyn Error>> {
        println!("exiting program");
        self.data_base.refresh_data(&self.task_list)?;
        exit(0);
    }

    pub fn get_data_base(&self) -> &DataBase {
        &self.data_base
    }
    pub fn get_task_list_mut(&mut self) -> &mut TaskList {
        &mut self.task_list
    }
    pub fn get_task_list(&self) -> &TaskList {
        &self.task_list
    }



    pub fn edit_task(&mut self, mut task: TaskBuilder){
            println!("title: ");
            let mut title = String::new();
            io::stdin().read_line(&mut title).expect("couldn't read line");
            let title = title.trim();

            println!("description: ");
            let mut description = String::new();
            io::stdin().read_line(&mut description).expect("couldn't read line");
            let description = description.trim();



            println!("resources: ");
            let mut resources = String::new();
            io::stdin().read_line(&mut resources).expect("couldn't read line");
            let resources = resources.trim();

            println!("priority: ");
            println!("1. High");
            println!("2. Normal");
            println!("3. Low");
            let mut priority: String = String::new();
            io::stdin().read_line(&mut priority).expect("couldn't read line");
            let priority: usize = priority.trim().parse().unwrap();
            let priority = match priority{
                1 => Priority::High,
                2 => Priority::Normal,
                3 => Priority::Low,
                _ => Priority::Normal
            };

            println!("progress: ");
            println!("1. Complete");
            println!("2. InProgress");
            println!("3. Unstarted");
            let mut progress: String = String::new();
            io::stdin().read_line(&mut progress).expect("couldn't read line");
            let progress: usize = progress.trim().parse().unwrap();
            let progress = match progress{
                1 => Progress::Complete,
                2 => Progress::InProgress,
                3 => Progress::Unstarted,
                _ => Progress::InProgress
            };

            println!("important_files: (file path)");
            let mut important_files = String::new();
            io::stdin().read_line(&mut important_files).expect("couldn't read line");
            let important_files = important_files.trim();

            println!("deadline: format(yyyy-mm-ddTHH:MM:SS)");
            let mut deadline = String::new();
            io::stdin().read_line(&mut deadline).expect("couldn't read line");
            let deadline = deadline.trim();

            println!("tags: ");
            let mut tags = String::new();
            io::stdin().read_line(&mut tags).expect("couldn't read line");
            let tags = tags.trim();
            

            if title != "."{
                task.set_title(title.to_string());
            }
            if description != "."{
                task.set_description(description.to_string());
            }
            if important_files != "."{
                task.add_important_file(PathBuf::from(important_files.to_string()));
            }
            if deadline != "." {
                task.set_deadline(
                chrono::NaiveDateTime::parse_from_str(deadline, "%Y-%m-%dT%H:%M:%S")
                    .unwrap_or_else(|_| chrono::Local::now().naive_local()));
            }
            if tags != "."{
                task.add_tag(tags.to_string());
            }
            if resources != "."{
                task.add_resource(resources.to_string());
            }
            task.set_priority(priority);
            task.set_progress(progress);
            
            let task = task.build();
            self.task_list.0.push(task);
            match self.data_base.refresh_data(&self.task_list){
                Ok(_) => return,
                Err(e) => println!("{}",e),
            }





}
}