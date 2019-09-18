use automation;

#[test]
fn add_five_and_six(){
    assert_eq!(automation::add_five_and_six(1),12);
}

#[test]
fn add_six(){
    assert_eq!(automation::some_features::add_five(6),11)
}

#[test]
fn always_true(){
    assert_eq!(automation::some_features::always_true(),true)
}

#[test]
fn always_false(){
    assert_eq!(automation::some_features::always_false(),false)
}
