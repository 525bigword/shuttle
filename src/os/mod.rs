mod win;
mod linux;


pub enum OS {
    Win,
    Linux,
    Mac
}

impl OS {
    pub fn get_system_metrics(&self)-> (i32,i32){
        match self {
            OS::Win => win::get_system_metrics(),
            OS::Linux => todo!(),
            OS::Mac => todo!(),
        }
    }
}