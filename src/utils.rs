extern crate winapi;

use utils::winapi::um::tlhelp32::{Process32First, Process32Next, LPPROCESSENTRY32, CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, PROCESSENTRY32};
use utils::winapi::um::winnt::HANDLE;
use utils::winapi::um::handleapi::INVALID_HANDLE_VALUE;

pub struct ProcessInformation {
    pub pid: u32,
    pub name: String,
}

impl ProcessInformation {
    fn new(_pid: u32, _name: String) -> ProcessInformation {
        ProcessInformation { pid: _pid, name: _name }
    }
}

pub struct ProcessInformationIterator {
    process_information: ProcessInformation,
    index: usize,
    process_snapshot: HANDLE,
    process_entry: PROCESSENTRY32,

}

fn char_arr_to_string(chars  : &[i8]) -> String {
    chars.into_iter().map(|c| { *c as u8 as char }).collect()
}

impl ProcessInformationIterator {
    pub fn new() -> ProcessInformationIterator {
        let h_process_snapshot: HANDLE = unsafe {
            CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
        };
        if h_process_snapshot == INVALID_HANDLE_VALUE {
            panic!("Invalid handle value");
        }
        println!("Got process snapshot handle, moving on...");
        let mut pe: PROCESSENTRY32;
        unsafe {
            pe = ::std::mem::zeroed();
        }
        let a = ::std::mem::size_of::<PROCESSENTRY32>();

        let lppe: LPPROCESSENTRY32 = &mut pe;
        pe.dwSize = a as u32;
        let res = unsafe { Process32First(h_process_snapshot, lppe) };
        if res == 0 {
            panic!("Can't get process list");
        }

        let pid: u32 = pe.th32ProcessID;
        let process_name: String = char_arr_to_string(&pe.szExeFile);
        ProcessInformationIterator { process_information: ProcessInformation::new(pid, process_name), index: 0, process_snapshot: h_process_snapshot, process_entry: pe }
    }
}

impl Iterator for ProcessInformationIterator {
    type Item = ProcessInformation;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.index = self.index + 1;
        if self.index == 1 {
            return Some(ProcessInformation::new(self.process_information.pid, self.process_information.name.clone()));
        }

        let mut pe = self.process_entry;
        let lppe = &mut pe;
        let res;
        unsafe {
            (*lppe).szExeFile = ::std::mem::zeroed();
            res = Process32Next(self.process_snapshot, lppe);
        }
        if res != 1 { // No more processes, finish the iteration
            None
        } else {
            let pid: u32 = (*lppe).th32ProcessID;
            let process_name: String = char_arr_to_string(&(*lppe).szExeFile);
            Some(ProcessInformation::new(pid, process_name))
        }
    }
}