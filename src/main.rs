/*
fn main() {
    let score = 2; 
     
      let closure =  |add_2|{
        println!("My score  number is   {}",add_2);
    };
    
    closure (2+score);

    let mut closure;  
        for score in 1..11  
     {  
        closure=2 + score;    
        println!("2+{}={}",score,closure);  

}
}
*/
use std::thread;
use std::time::Duration;


fn main() {
    
    let goal = 2;
    let closure =  |multiply_2|{
        println!("My goal  number is   {}",multiply_2);
    };
    let _goal = thread::spawn(move|| {
        let mut closure;  
        for goal in 0..16  
     {  
       closure=2*goal;    
        println!("2*{}={}",goal,closure);  
     }    
       thread::sleep(Duration::from_millis(20));
        });
            closure (2*goal);

           _goal.join().unwrap();
}



/*
fn main (){
     let goal = 2;

      
     let Add_2 =|add:i32|
      {
        println!("My Anger loss {}",goal); 
    
        };
        Add_2(goal);
           
        
             
}
*/