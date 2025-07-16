use chrono;
use serde::{Serialize, Deserialize};

// types and structs that are helpfull to use for Task
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority{
    High,
    Normal,
    Low
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Progress{
    Complete,
    InProgress,
    Unstarted
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TaskTitle(String);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TaskDescription(String);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PriorityLevel(Priority);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ProgressLevel(Progress);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Resources(Vec<String>);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ImportantFiles(Vec<std::path::PathBuf>);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Deadline(chrono::NaiveDateTime);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Created(chrono::NaiveDateTime);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Tags(Vec<String>);
#[derive(Serialize, Deserialize)]
pub struct Task{
    title: TaskTitle,
    description: TaskDescription,
    priority: PriorityLevel,
    progress: ProgressLevel,
    resources: Resources,
    important_files: ImportantFiles,
    deadline: Deadline,
    created_at: Created,
    tags: Tags
}


pub struct TaskBuilder{
    title: TaskTitle,
    description: TaskDescription,
    priority: PriorityLevel,
    progress: ProgressLevel,
    resources: Resources,
    important_files: ImportantFiles,
    deadline: Deadline,
    created_at: Created,
    tags: Tags
}

#[derive(Serialize, Deserialize)]
pub struct TaskList(pub Vec<Task>);

impl Default for TaskBuilder{
    fn default() -> Self {
        TaskBuilder { 
            title: TaskTitle(String::from("Empty")), 
            description: TaskDescription(String::from("Empty")),
            priority: PriorityLevel(Priority::Normal), 
            progress: ProgressLevel(Progress::Unstarted),
            resources: Resources(vec![]),
            important_files: ImportantFiles(vec![]), 
            deadline: Deadline(chrono::Local::now().naive_local()), 
            created_at: Created(chrono::Local::now().naive_local()), 
            tags: Tags(vec![]) 
        }
    }
}



// setters and builder for the real struct mostly for safety of changing tasks Logic still needs implementation
impl TaskBuilder{
    pub fn set_title(&mut self, title: String)->&mut Self{
        self.title = TaskTitle(title);
        self
    }
    pub fn set_description(&mut self, description: String)->&mut Self{
        self.description = TaskDescription(description);
        self
    }
    pub fn set_priority(&mut self, priority: Priority)->&mut Self{
        self.priority = PriorityLevel(priority);
        self
    }
    pub fn set_progress(&mut self,progress: Progress)->&mut Self{
        self.progress = ProgressLevel(progress);
        self
    }
    pub fn add_resource(&mut self, resource: String)->&mut Self{
        self.resources.0.push(resource);
        self
    }
    pub fn add_important_file(&mut self, file:std::path::PathBuf)->&mut Self{
        self.important_files.0.push(file);
        self
    }
    pub fn set_deadline(&mut self,date: chrono::NaiveDateTime  )->&mut Self{
        self.deadline = Deadline(date);
        self
    }
    pub fn add_tag(&mut self, tag: String)->&mut Self{
        self.tags.0.push(tag);
        self
    }
    pub fn build(self)->Task{
        Task {
            title: self.title, description: self.description,
            priority: self.priority, progress: self.progress,
            resources: self.resources, important_files: self.important_files, 
            deadline: self.deadline, created_at: self.created_at, tags: self.tags 
        }
    }
}
// operations you can do on the Task after its been built
impl Task{
    pub fn get_builder(self)->TaskBuilder{
        TaskBuilder {
            title: self.title, description: self.description,
            priority: self.priority, progress: self.progress,
            resources: self.resources, important_files: self.important_files, 
            deadline: self.deadline, created_at: self.created_at, tags: self.tags 
        }
    }   
    pub fn delete_task(self){
        
    }

 pub fn get_title(&self) -> &str {
        &self.title.0
    }

    pub fn get_description(&self) -> &str {
        &self.description.0
    }

    pub fn get_priority(&self) -> &Priority {
        &self.priority.0
    }

    pub fn get_progress(&self) -> &Progress {
        &self.progress.0
    }

    pub fn get_resources(&self) -> &Vec<String> {
        &self.resources.0
    }

    pub fn get_important_files(&self) -> &Vec<std::path::PathBuf> {
        &self.important_files.0
    }

    pub fn get_deadline(&self) -> &chrono::NaiveDateTime {
        &self.deadline.0
    }

    pub fn get_created_at(&self) -> &chrono::NaiveDateTime {
        &self.created_at.0
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags.0
    }
}

// im making this a trait because this is a crate and everyone can implement the open function the 
// way they want to im gonna write logic for this in the app im gonna build with this crate
pub trait OpenTask{
    fn open_task(&self){
    }
}


impl OpenTask for Task{
        fn open_task(&self) {
        println!("┌──────────────────── Task ────────────────────┐");
        println!("│ Title       : {}", self.title.0);
        println!("│ Description : {}", self.description.0);
        println!("│ Priority    : {:?}", self.priority.0);
        println!("│ Progress    : {:?}", self.progress.0);
        println!("│ Deadline    : {}", self.deadline.0);
        println!("│ Created At  : {}", self.created_at.0);
        println!("│ Resources   :");
        for res in &self.resources.0 {
            println!("│   - {}", res);
        }
        println!("│ Important Files:");
        for path in &self.important_files.0 {
            println!("│   - {}", path.display());
        }
        println!("│ Tags        : {:?}", self.tags.0);
        println!("└──────────────────────────────────────────────┘");
        }
}






impl<'a> IntoIterator for &'a TaskList{
    type Item = &'a Task;
    type IntoIter = std::slice::Iter<'a, Task>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}


