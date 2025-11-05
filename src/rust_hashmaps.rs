use  std::collections::HashMap;
pub fn hash_maps(){
    let mut scores:HashMap<String,u32>= HashMap::new();
     scores.insert(String::from("Victor"),  20);
     scores.insert(String::from("James"),  30);

     println!("{:?}", scores);


     let james_scores=scores.get("James");

     match  james_scores {
         Some(value) => println!("James Scores is {:?}", value),
         None => println!("No scores was recoreded")
     }

    for (key, value) in scores{
        println!("{},{}", key, value)
    }
    // println!("{:?}", scores)
}