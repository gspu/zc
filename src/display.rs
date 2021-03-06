
use std::char;
use std::thread;
use std::time;

extern crate ncurses;
use ncurses::*;

use crate::command;

const BOTTOM_BORDER_SIZE: usize = 3;


pub struct Screen {

    max_y: i32,
    max_x: i32,

    left_content: Content,
    right_content: Content,
}

impl Screen {

    const KEY_ESC:   i32 = 0x1b;
    const KEY_ENTER: i32 = 0xa;

    pub fn new() -> Screen {

        initscr();
        cbreak();
        // raw();
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        keypad(stdscr(), true);
        // nodelay(stdscr(), true);

        let mut max_y: i32 = 0;
        let mut max_x: i32 = 0;
        
        getmaxyx(stdscr(), &mut max_y, &mut max_x);

        let left_content = Content::new(true, ContentType::Datasets);
        let right_content = Content::new(false, ContentType::Snapshots);

        Screen {
            max_y: 0,
            max_x: 0,

            left_content,
            right_content,
        }
    }

    pub fn run(&mut self) {

        self.update_content();

        loop {

            self.draw();

            let should_update = match self.handle_keys() {
                Err(_) => { break; },
                Ok(should_update) => should_update,
            };

            thread::sleep(time::Duration::from_millis(10));
            if should_update == true { self.update_content(); }
        }
    }

    fn handle_keys(&mut self) -> Result<bool,()> {

        const KEY_TAB: i32 = 0x9;
        const KEY_PUP: i32 = 0x153;
        const KEY_PDN: i32 = 0x152;

        let key = wgetch(stdscr());

        match key {
            KEY_F1    => { self.key_f1(); return Ok(true); },
            KEY_F2    => { self.key_f2(); return Ok(true); },
            KEY_F3    => { self.key_f3(); return Ok(true); },
            KEY_F4    => { self.key_f4(); return Ok(true); },
            KEY_F5    => { self.key_f5(); return Ok(true); },
            KEY_F6    => { self.key_f6(); return Ok(true); },
            KEY_F7    => { self.key_f7(); return Ok(true); },
            KEY_F8    => { self.key_f8(); return Ok(true); },
            KEY_F9    => { self.key_f9(); return Ok(true); },
            KEY_F10   => { return Err(()); },
            KEY_F11   => { self.key_f11(); },
            KEY_F12   => { self.test_windows(); },

            KEY_DL    => {},

            KEY_HOME  => { self.key_home(); },
            KEY_END   => { self.key_end(); },

            KEY_UP    => { self.key_up(); },
            KEY_DOWN  => { self.key_down(); },

            KEY_PUP   => { self.key_pgup(); }
            KEY_PDN   => { self.key_pgdown(); }

            KEY_LEFT  => { self.switch_window(); return Ok(true); },
            KEY_RIGHT => { self.switch_window(); return Ok(true); },

            KEY_IL    => {},
            KEY_TAB   => { self.switch_mode(); return Ok(true); },

            _ => {},
        }

        Ok(false)
    }

    fn key_home(&mut self) {

        if self.left_content.is_selected {
            self.left_content.position = 0;
        } else {
            self.right_content.position = 0;
        }
    }

    fn key_end(&mut self) {

        if self.left_content.is_selected {
            self.left_content.position = self.left_content.command_result.len()-1;
        } else {
            self.right_content.position = self.right_content.command_result.len()-1;
        }
    }

    fn key_up(&mut self) {

        if self.left_content.is_selected && self.left_content.position > 0 {
            self.left_content.position -= 1;
        } else if self.right_content.is_selected && self.right_content.position > 0{
            self.right_content.position -= 1;
        }
    }

    fn key_down(&mut self) {

        if self.left_content.is_selected && 
           self.left_content.position+2 < self.left_content.command_result.len() {
            self.left_content.position += 1;
    
        } else if self.right_content.is_selected && 
           self.right_content.position+2 < self.right_content.command_result.len() {
            self.right_content.position += 1;                
        }
    }

    fn key_pgup(&mut self) {

        if self.left_content.is_selected {
            if self.left_content.position > 10 {
                self.left_content.position -= 10;
            } else {
                self.left_content.position = 0;
            }

        } else if self.right_content.is_selected {
            if self.right_content.position > 10 {
                self.right_content.position -= 10;
            } else {
                self.right_content.position = 0;
            }
        }
    }

    fn key_pgdown(&mut self) {

        if self.left_content.is_selected {
            if self.left_content.position+11 < self.left_content.command_result.len() {
                self.left_content.position += 10;
            } else {
                self.left_content.position = self.left_content.command_result.len()-2;
            }

        } else if self.right_content.is_selected {
            if self.right_content.position+11 < self.right_content.command_result.len() {
                self.right_content.position += 10;
            } else {
                self.right_content.position = self.right_content.command_result.len()-2;
            }
        }
    }

    fn key_f1(&self) { 
        // TODO
    }

    fn key_f2(&self) { 

        let selected_elements = self.selected_elements();

        match self.content_type() {
            ContentType::Pools =>     { },
            ContentType::Datasets =>  { self.input_dataset_create(selected_elements); },
            ContentType::Volumes =>   { },
            ContentType::Snapshots => { },
        };
    }

    fn input_dataset_create(&self, selected_elements: Vec<String>) {

        let selected_string = self.seleted_string(&selected_elements);

        match self.input_dialog(" Create Dataset: ", "Enter the name of the new Dataset", selected_string.as_str()) {
            Ok(dataset_name) => { command::zfs_create(dataset_name); },
            Err(_)    => { },
        }

        wrefresh(stdscr());
    }

    fn key_f3(&self) { 
        // TODO
    }

    fn key_f4(&self) { 

        // let selected_elements = self.selected_elements();

        match self.content_type() {
            ContentType::Pools =>     { },
            ContentType::Datasets =>  { },
            ContentType::Volumes =>   { },
            ContentType::Snapshots => { },
        };
    }

    fn key_f5(&self) { 

        let selected_elements = self.selected_elements();

        match self.content_type() {
            ContentType::Pools =>     { },
            ContentType::Datasets =>  { self.input_snapshot_dataset(selected_elements); },
            ContentType::Volumes =>   { self.input_snapshot_dataset(selected_elements); },
            ContentType::Snapshots => { self.input_snapshot_clone(selected_elements);   },
        };
    }

    fn input_snapshot_clone(&self, selected_elements: Vec<String>) {

        let selected_string = self.seleted_string(&selected_elements);

        match self.input_dialog(" Clone Snapshot: ", "Enter the name of the new Snapshot", "") {
            Ok(dataset_name) => { command::zfs_clone(selected_string, dataset_name); },
            Err(_)    => { },
        }

        wrefresh(stdscr());
    }

    fn input_snapshot_dataset(&self, selected_elements: Vec<String>) {

        let selected_string = self.seleted_string(&selected_elements);

        let snapshot = format!("{}@", selected_string);

        match self.input_dialog(" Snapshot Dataset: ", "Enter the name of the new Snapshot", snapshot.as_str()) {
            Ok(dataset_name) => { command::zfs_snapshot(dataset_name); },
            Err(_)    => { },
        }

        wrefresh(stdscr());
    }

    fn key_f6(&self) { 

        let selected_elements = self.selected_elements();

        match self.content_type() {
            ContentType::Pools =>     { },
            ContentType::Datasets =>  { self.input_dataset_rename(selected_elements); },
            ContentType::Volumes =>   { self.input_dataset_rename(selected_elements); },
            ContentType::Snapshots => { self.input_dataset_rename(selected_elements); },
        };
    }

    fn input_dataset_rename(&self, selected_elements: Vec<String>) {

        let selected_string = self.seleted_string(&selected_elements);

        match self.input_dialog(" Rename Dataset: ", "Enter the new name for the Dataset", selected_string.as_str()) {
            Ok(new_dataset_name) => { command::zfs_rename(selected_string, new_dataset_name); },
            Err(_)    => { },
        }

        wrefresh(stdscr());
    }

    fn key_f7(&self) {

        let selected_elements = self.selected_elements();

        match self.content_type() {
            ContentType::Pools =>     { self.confirm_pool_scrub(selected_elements); },
            ContentType::Datasets =>  { },
            ContentType::Volumes =>   { },
            ContentType::Snapshots => { self.confirm_snapshot_rollback(selected_elements); },
        };
    }

    fn confirm_pool_scrub(&self, selected_elements: Vec<String>) {

        let selected_string = self.seleted_string(&selected_elements);

        let title = " Confirm Scrub: ";
        let prompt = "The following pools(s) will be scrubbed: ";

        if let Err(_) = self.confirm_dialog(title, prompt, selected_string.as_str()) {
            return;
        }

        command::zpool_scrub(selected_elements);
    }

    fn confirm_snapshot_rollback(&self, selected_elements: Vec<String>) {

        let selected_string = self.seleted_string(&selected_elements);

        let title = " Confirm Rollback: ";
        let prompt = "The Dataset(s) will be rolled back to the following snapshot(s): ";

        if let Err(_) = self.confirm_dialog(title, prompt, selected_string.as_str()) {
            return;
        }

        command::zfs_rollback(selected_elements);
    }

    fn key_f8(&self) {

        let selected_elements = self.selected_elements();
        let selected_string = self.seleted_string(&selected_elements);

        let title = " Confirm Destroy: ";
        let prompt = "The following element(s) will be destroyed: ";

        if let Err(_) = self.confirm_dialog(title, prompt, selected_string.as_str()) {
            return;
        }

        match self.content_type() {
            ContentType::Pools =>     { command::zpool_destroy(selected_elements) },
            ContentType::Datasets =>  { command::zfs_destroy(selected_elements) },
            ContentType::Volumes =>   { command::zfs_destroy(selected_elements) },
            ContentType::Snapshots => { command::zfs_destroy(selected_elements) },
        }
    }

    fn key_f9(&self) { 
        // TODO
    }

    fn key_f11(&self) { 
        // TODO
    }

    fn switch_window(&mut self) {

        if self.left_content.is_selected {
            self.left_content.is_selected = false;
            self.right_content.is_selected = true;
        } else {
            self.left_content.is_selected = true;
            self.right_content.is_selected = false;
        }
    }

    fn switch_mode(&mut self) {

        if self.left_content.is_selected {
            self.left_content = Content::new(true, self.left_content.next());

        } else {
            self.right_content = Content::new(true, self.right_content.next());
        }
    }

    fn scroll_window(content: &mut Content, height: i32) {

        if content.position < content.start_from {
            content.start_from = content.position

        } else if content.position - content.start_from > (height as usize - BOTTOM_BORDER_SIZE - 1) {
            content.start_from = content.position - (height as usize - BOTTOM_BORDER_SIZE - 1);
        }

        if content.command_result.len() > 2 && 
           content.position > content.command_result.len()-2 {
            content.position = content.command_result.len()-2;
        }
    }

    fn test_windows(&self) {

        let s = format!("  Left position: {} len: {}  ", self.left_content.position, self.left_content.command_result.len());
        mvwprintw(stdscr(), 1, 1, s.as_str());

        let s = format!(" Right position: {} len: {}  ", self.right_content.position, self.right_content.command_result.len());
        mvwprintw(stdscr(), 2, 1, s.as_str());

        let key = getch();
        let s = format!(" Keystroke: 0x{:x}     ", key);
        mvwprintw(stdscr(), 3, 1, s.as_str());

        getch();
    }

    fn draw(&mut self) {

        getmaxyx(stdscr(), &mut self.max_y, &mut self.max_x);

        self.draw_menu();
        self.draw_content();
    }

    fn draw_content(&mut self) {

        let left_start_y = 0;
        let left_start_x = 0;
        let left_height = self.max_y;
        let left_width  = self.max_x/2;
        let left_title = self.left_content.c_type.text();

        let left_window = Screen::draw_window(left_height-1, left_width, left_start_y, left_start_x, left_title.as_str());
        // self.left_content.update();
        Screen::scroll_window(&mut self.left_content, left_height);
        Screen::write_content(&self.left_content, left_window, left_height, left_width);

        let right_start_x = left_width;
        let right_start_y = 0;
        let right_height = self.max_y;
        let right_width  = self.max_x - right_start_x;
        let right_title = self.right_content.c_type.text();

        let right_window = Screen::draw_window(right_height-1, right_width, right_start_y, right_start_x, right_title.as_str());
        // self.right_content.update();
        Screen::scroll_window(&mut self.right_content, right_height);
        Screen::write_content(&self.right_content, right_window, right_height, right_width);

        wrefresh(stdscr());
        wrefresh(left_window);
        wrefresh(right_window);
    }

    fn draw_window(height: i32, width: i32, start_y: i32, start_x: i32, title: &str) -> WINDOW {

        let win = newwin(height, width, start_y, start_x);

        box_(win, 0, 0);
        wmove(win, 0, 1);
        wprintw(win, title);

        win
    }

    fn draw_menu(&mut self) {

        let pools_menu     = format!(" 1 _____ 2 _____ 3 _____ 4 _____ 5 _____ 6 _____ 7 Scrub 8 Destr 9 _____ 10 Exit ");
        let datasets_menu  = format!(" 1 _____ 2 Creat 3 _____ 4 _____ 5 Snaps 6 Renam 7 _____ 8 Destr 9 _____ 10 Exit ");
        let volumes_menu   = format!(" 1 _____ 2 _____ 3 _____ 4 _____ 5 Snaps 6 Renam 7 _____ 8 Destr 9 _____ 10 Exit ");
        let snapshots_menu = format!(" 1 _____ 2 _____ 3 _____ 4 _____ 5 Clone 6 Renam 7 RollB 8 Destr 9 _____ 10 Exit ");

        let mut selected_menu: String;

        match self.content_type() {
            ContentType::Pools =>     { selected_menu = pools_menu; },
            ContentType::Datasets =>  { selected_menu = datasets_menu; },
            ContentType::Volumes =>   { selected_menu = volumes_menu; },
            ContentType::Snapshots => { selected_menu = snapshots_menu; },
        };

        for _ in selected_menu.len()..self.max_x as usize {
            selected_menu.push_str(" ");
        }

        wattron(stdscr(), A_BOLD());
        mvwprintw(stdscr(), self.max_y-1, 0, selected_menu.as_str());
        wattroff(stdscr(), A_BOLD());
    }

    fn content_type(&self) -> &ContentType {

        if self.left_content.is_selected {
            &self.left_content.c_type
        } else {
            &self.right_content.c_type 
        }
    }

    fn write_content(content: &Content, window: WINDOW, height: i32, width: i32) {

        const TOP_CONTENT_Y: i32 = 1;
        const TOP_CONTENT_X: i32 = 1;

        for (i, result_line) in content.command_result.iter().enumerate() {

            if i < content.start_from { continue }
            if i >= height as usize + content.start_from - BOTTOM_BORDER_SIZE { break }
            if i == content.position && content.is_selected { wattron(window, A_REVERSE()); }

            let text = Screen::fit_to_window(result_line.name.as_str(), width as usize);

            let content_position = i as i32 - content.start_from as i32 + TOP_CONTENT_Y;

            mvwprintw(window, content_position, TOP_CONTENT_X, text.as_str());
            wattroff(window, A_REVERSE());
        }
    }

    fn update_content(&mut self) {
        self.left_content.update();
        self.right_content.update();
    }

    fn fit_to_window(result_name: &str, width: usize) -> String {

        let mut name = result_name.to_string();

        if name.len() > width-2 {
            name = name.get(0..width-2).unwrap().to_string();
        }

        for _ in name.len()..width-2 {
            name.push_str(" ");
        }

        format!("{}", name)
    }

    fn selected_elements(&self) -> Vec<String> {

        if self.left_content.is_selected {
            if self.left_content.selected_elements.len() > 0 {
                self.left_content.selected_elements.to_owned()
            } else {
                vec![self.left_content.command_result[self.left_content.position].name.to_owned()]
            }

        } else {
            if self.right_content.selected_elements.len() > 0 {
                self.right_content.selected_elements.to_owned()
            } else {
                vec![self.right_content.command_result[self.right_content.position].name.to_owned()]
            }
        }
    }

    fn input_dialog(&self, title: &str, prompt: &str, info: &str) -> Result<String,()> {

        let dialog_height = 8;
        let dialog_width = 70;

        let start_y = self.max_y/2 - dialog_height/2;
        let start_x = self.max_x/2 - dialog_width/2;

        let footnote = "ESC Cancel     ENTER Confirm";

        let dialog = Screen::draw_window(dialog_height, dialog_width, start_y, start_x, title);
        mvwprintw(dialog, 2, 3, prompt);
        wattroff(dialog, A_REVERSE());

        mvwprintw(dialog, 5, 3, "----------------------------------------------------------------");
        let foot_x = dialog_width/2 - footnote.len() as i32/2;
        mvwprintw(dialog, 6, foot_x, footnote);

        wattron(dialog, A_REVERSE());
        mvwprintw(dialog, 3, 3, "                                                                ");
        mvwprintw(dialog, 3, 3, info);

        wrefresh(dialog);

        let mut input = String::from(info);
        let mut _input_scr = String::new();

        loop {
            let key = getch();

            match key {
                Screen::KEY_ENTER => { return Ok(input.to_owned()) },
                Screen::KEY_ESC   => { return Err(())   },
                0x20..=0x7f       => { input.push(char::from_u32(key as u32).unwrap()); },
                KEY_BACKSPACE     => { input.pop(); }
                _                 => {},
            }

            let input_size = input.len();
            _input_scr = input.clone();
            for _ in input_size..64 {
                _input_scr.push(' ');
            }

            mvwprintw(dialog, 3, 3, _input_scr.as_str());
            wrefresh(dialog);
        }
    }
    
    fn confirm_dialog(&self, title: &str, prompt: &str, info: &str) -> Result<(),()> {

        let dialog_height = 8;
        let dialog_width = 70;

        let start_y = self.max_y/2 - dialog_height/2;
        let start_x = self.max_x/2 - dialog_width/2;

        let footnote = "ESC Cancel     ENTER Confirm";

        let dialog = Screen::draw_window(dialog_height, dialog_width, start_y, start_x, title);
        mvwprintw(dialog, 2, 3, prompt);
        mvwprintw(dialog, 3, 3, info);

        mvwprintw(dialog, 5, 3, "----------------------------------------------------------------");
        let foot_x = dialog_width/2 - footnote.len() as i32/2;
        mvwprintw(dialog, 6, foot_x, footnote);

        wrefresh(dialog);

        loop {
            let key = getch();

            if key == Screen::KEY_ENTER {
                return Ok(())
            }

            if key == Screen::KEY_ESC {
                return Err(())
            }
        }
    }

    fn seleted_string(&self, selected_elements: &Vec<String>) -> String {

        if selected_elements.len() == 1 {
            selected_elements.get(0).unwrap().to_string()
        } else {
            format!("{} elements", selected_elements.len())
        }
    }
}

impl Drop for Screen {

    fn drop(&mut self) {
        clear();
        endwin();
    }
}

#[allow(dead_code)]
enum ContentType {
    Datasets,
    Pools,
    Volumes,
    Snapshots,
}

impl ContentType {

    pub fn text(&self) -> String {
        match self {
            ContentType::Pools => " Pools: ".to_string(),
            ContentType::Datasets => " Datasets: ".to_string(),
            ContentType::Volumes => " Volumes: ".to_string(),
            ContentType::Snapshots => " Snapshots: ".to_string(),
        }
    }
}

struct Content {

    is_selected: bool,
    start_from: usize,
    position: usize,
    c_type: ContentType,
    command_result: Vec<command::CommandResult>,
    selected_elements: Vec<String>,
}

impl Content {

    pub fn new(is_selected: bool, c_type: ContentType) -> Content {

        Content {
            is_selected: is_selected,
            start_from: 0,
            position: 0,
            c_type: c_type,
            command_result: Vec::new(),
            selected_elements: Vec::new(),
        }
    }

    pub fn update(&mut self) {

        match self.c_type {
            ContentType::Pools     => { self.command_result = command::zfs_pools(); },
            ContentType::Datasets  => { self.command_result = command::zfs_dataset(); },
            ContentType::Volumes   => { self.command_result = command::zfs_volumes(); },
            ContentType::Snapshots => { self.command_result = command::zfs_snapshots(); },
        }
    } 

    pub fn next(&mut self) -> ContentType {

        match self.c_type {
            ContentType::Pools     => { ContentType::Datasets },
            ContentType::Datasets  => { ContentType::Volumes },
            ContentType::Volumes   => { ContentType::Snapshots },
            ContentType::Snapshots => { ContentType::Pools },
        }

    }
}