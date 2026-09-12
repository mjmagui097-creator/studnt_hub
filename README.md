==== STUDENT HUB ====
.This code is for an application with three menus: 
    -> the hub ( for timetables + to-dos );
    -> the timer ( with pomodoro timer );
    -> the record ( for keeping record of your grades as well as simulate future averages).

.This code will be produced in RUST; 
.This code will first be created for terminal I/O -> then converted to a file output ( database only ) -> then converted to an UI using websockets & HTML / CSS

This page will hold info on the files of the project, as well as functions, with their expected inputs and outputs, errors and goals.

Structs
    to-do list item - struct with Vec<To-do> of structs:to-do, title, to-do counter and dones counter;
    to-do items - struct with name, done/not done;

    subject item - struct with name, ects, s/t and grade;
    year item - struct with name, Vec<Subject> of structs:subjects, average;

    program - struct with name of menu it is currently in; 


Functions
    -- GRADES -- 
    create_year(name: String) -> Year ; 
        - Creates year with the name 'name', Vec<Subjects> = [], average = 0.00.

    delete_year(year: Year) -> () ; 
        - Deletes 'year';

    create_subject(year: Year, subject: String, credits: u8, sim: bool, grade: u8) -> Year ; 
        - Creates subject inside 'year' with name = 'subject', ects = 'credits', s/t = 'sim', grade = 'grade'.

    edit_grade(year: &mut Year, subject: String, grade: u8) -> () ;
        - Edits 'grade' from 'subject' in 'year' to the new value. 

    edit_sim(year: Year, subject: String, sim: bool) -> Year ; 
        - Edits simulation from 'subject' in 'year' to the new value. 

    delete_subject(year: Year, subject: String) -> Year ; 
        - Deletes 'subject' in 'year'.

    yearly_average(year: &Year) -> f32 ; 
        - Calculates yearly average from 'year'. 

    total_average(years: &[Year]) -> f32 ;
        - Calculates total average of 'years'. 

    print_year(year: Year) -> () ; 
        - Prints contents of 'year'.

    --

    end() -> () ;
        - Terminates Program 







    choose_menu(program: &mut Program, menu: String) -> () ;
        - Chooses what menu it is in ( To-do, Grades ); 

    show_menu(program: Program) -> () ; 
        - Displays the name of the menu it is currently in; 



    -- TODO --
    create_todo_list(list_name: String) -> To-do List ;
        - Creates a to-do list with the title 'list_name', progress counter = 0, to-do counter = 0, Vec<To-do> of to-dos = [].

    delete_todo_list(list: To-do List) -> () ;
        - Deletes 'list'; 

    add_todo(list: &mut To-do List, todo: String) -> () ; 
        - Creates a to-do item with the name = 'todo', done/not_done = 0 (false) in 'list'.

    delete_todo(list: &mut To-do List, todo: String) -> () ;
        - Deletes 'todo' item inside 'list'.

    check_todo(list: &mut To-do List, todo: String) -> () ;
        - Checks 'todo' item inside 'list'.

    uncheck_todo(list: &mut To-do List, todo: String) -> () ;
        - Unchecks 'todo' item inside 'list'.

    show_progress_list(list: &To-do List) -> f32 ;
        - Shows progress (percentage of checked) of 'list'.

    print_todo_list(list: To-do List) -> () ; 
        - Prints contents of 'list'.
    