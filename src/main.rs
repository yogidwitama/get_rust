fn main() {
    println!("Hello, world!")
}

#[test]
fn hello_test(){
    println!("this unit test bro");
}

#[test]
fn test_variable(){
    let name="yogi dwitama";
    println!("hello {}", name);
}

#[test]
fn test_mutable(){
    let mut name="yogi uchiha";
    println!("Hi {}", name);

    name="yogi gigigi";
    println!("hihihi {}", name);
}

#[test]
fn static_typing(){
    let name = "Yog";
    println!("hello {}", name);

    //name = 10;
    println!("Hello {}", name)
}

#[test]
fn shadowing(){
    let name="hi";
    println!("hehe {}", name);

    let name = 10;
    println!("hehiy {}", name);
}

#[test]
fn excp(){
    let age:i32= 20;
    println!("{}", age);
}

#[test]
fn number(){
    let a:i8= 10;
    println!("{}", a);

    let b:f32= 10.5;
    println!("{}", b);
}

#[test]
fn number_conversion(){
    let a:i8=10;
    println!("{}", a);

    let b:i16= a as i16;
    println!("{}", b);

    let c:i32= a as i32;
    println!("{}", c);

    let d:i64 = 1000000000;
    let e:i8= d as i8;
    println!("{}", e); //integer overflow
}

#[test]
fn numeric_operation(){
    let a = 10;
    let b=12;
    let c= a*b;
    println!("{}", c);
    let d = a-b;
    println!("{}", d);
    let e=a/b;
    println!("{}", e);
}

#[test]
fn augmented_assignment(){
    let mut a = 10;
    println!("{}", a);

    a+=10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean(){
    let a = true;
    let b=false;
    println!("{} {}", a,b)
}

#[test]
fn comparison(){
    let a = 10;
    let b =32;
    let result:bool = a>b;
    println!("{}", result);
}

#[test]
fn operation_boolean(){
    let absen = 70;
    let nilai_akhir = 80;

    let lulus_absen= absen >= 75;
    let lulus_nilai_akhir = nilai_akhir>=70;

    let lulus = lulus_absen && lulus_nilai_akhir;
    println!("{}", lulus);
}

#[test]
fn char_type(){
    let char1='a';
    let char2='b';

    println!("{} {}", char1, char2)
}

#[test]
fn tuple_type(){
    let datas:(i32, f64, bool)= (10, 10.5, false);
    println!("{:?}", datas);
}

#[test]
fn tuple_type_access(){
    let datas:(i32, f64, bool)= (10, 10.5, false);
    println!("{:?}", datas);

    let a =datas.0;
    let b= datas.1;
    let c= datas.2;
    println!("{} {} {}", a,b,c);
}


#[test]
fn tuple_type_destructuring(){
    let datas:(i32, f64, bool)= (10, 10.5, false);
    println!("{:?}", datas);

    // let a =datas.0;
    // let b= datas.1;
    // let c= datas.2;
    let(a,b,c)=datas;
    println!("{} {} {}", a,b,c);
}

#[test]
fn tuple_type_mutable(){
    let datas:(i32, f64, bool)= (10, 10.5, false);
    println!("{:?}", datas);

    // let a =datas.0;
    // let b= datas.1;
    // let c= datas.2;
    let(a,b,c)=datas;
    println!("{} {} {}", a,b,c);

    // data.0= 20; // error karena data immutable
    let mut data_mut:(i32, f64, bool)=(3,5.5555, false);
    data_mut.0=10;
    data_mut.1=1.2;
    data_mut.2=true;
    let (d,e,f)= data_mut;
    println!("{} {} {}", d, e, f);
}
fn unit(){
    println!("hi")
}
#[test]
fn unit_test(){
    let result:() = unit();
    println!("{:?}", result);

    let test:()=();
    println!("{:?}", test);

}

#[test]
fn array(){
    let array:[i32;5]=[1,2,3,4,5];
    println!("{:?}", array)
}
#[test]
fn array_access(){
    let array:[i32;5]=[1,2,3,4,5];
    println!("{:?}",array);

    let a = array[1];
    let b= array[3];
    println!("{} {}", a,b);
}

#[test]
fn array_mutable(){
    let mut array:[i32;5]=[1,2,3,4,5];
    println!("{:?}", array);

    let a:i32= array[3];
    let b:i32= array[4];
    println!("{} {}", a, b);

    // array mutable
    array[0]=10;
    array[2]=200;
    println!("{:?}", array);

//     array length
    let length_array:usize= array.len();
    println!("{}", length_array);
}

#[test]
fn two_dimensional_array(){
    let matrix:[[i32;3];2]=[
        [1,2,3],
        [4,5,6]
];
    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1]);
}

const MINIMUM:i32= 0;
#[test]
fn constant_var(){
    // harus UPPERCASE semua dan type data harus dideklarasikan
    const  MAXIMUM:i32=100;
    println!("{} {}", MINIMUM,MAXIMUM);
}
#[test]
fn variable_scope(){
    println!("{}", MINIMUM);
    //variable outer scope
let yogi =10;
    {
        //akses variable outer
        println!("{}", yogi);
        //variable inner scope
        let dwitama=20;
        println!("{}", dwitama);
    }
    //println!("{}", dwitama); //error
}

#[test]
fn stack_heap(){
    function_a();
    function_b();
}

fn function_a(){
    let a =10;
    let b =String::from("Yogi");
    println!("{} {}", a,b)
}
fn function_b(){
    let a =32;
    let b = String::from("Dwitama");
    println!("{} {}", a, b)
}

#[test]
fn string_slice_trim(){
    let name:&str=" Yogi Dwitama ";
    let name_trim:&str= name.trim();
    println!("{}", name);
    println!("{}", name_trim);

    let mut username:&str="Yogi";
    username="Dwitama";
    println!("{}", username);

    // let mut values = 10;
    // values =22;
    // println!("{}", values)
}

#[test]
fn string_type(){
    let mut name:String=String::from("Yogi Dwitama");
    println!("{}", name);

    // String method push
    name.push_str(" Uchiha");
    println!("{}", name);

    let uzumaki =name.replace("Yogi", "Uzumakix");
    println!("{}", name);
    println!("{}", uzumaki);
}

#[test]
fn ownership_rules(){
    // a tidak bisa diakses disini, belum di deklarasikan
    let a =10; // a sudah bisa diakses mulai dari sini
    {//b tidak bisa diakses disini, belum dideklarasikan
        let b = 20; // bisa diakses mulai dari sini
        println!("{}", b)
    }//scope b selesai, b dihapus, b tidak bisa diakses lagi

    print!("{}", a);
}//scope a selesai, a dihapus, a tidak bisa diakses lagi

#[test]
fn data_copy(){
    let a = 10;
    let b=a;

    println!("{}", b);
}

#[test]
fn ownership_movement(){
    let name1= String::from("Yogi");
    // ownershiip dari name1 dipindahkan ke name2
    let name2= name1;
    //name1 tidak bisa diakses disini
    // println!("{}", name1);
    println!("{}", name2);
}

#[test]
fn ownership_clone(){
    let name1= String::from("Yogi");
    let name2= name1.clone();

    println!("{} {}", name2, name1)
}

#[test]
fn if_expression(){
    let value=4;
    if value >= 8{
        println!("Good")
    }else {
        println!("Almost..")
    }
}

#[test]
fn elseif_expression(){
    let value=6;
    if value >=8{
        println!("you are good")
    }else if value < 8 && value >= 7 {
        println!("good")
    }else if value <7 && value >=6 {
        println!("so so")
    }else {
        println!("you not passed")
    }
}

#[test]
fn let_elseif_expression_manual(){
    let value=6;
    let result:&str;

    if value >=8{
        result="you are good"
    }else if value < 8 && value >= 7 {
        result="good"
    }else if value <7 && value >=6 {
        result="so so"
    }else {
        result="you not passed"
    }
    println!("{}", result)
}

#[test]
fn let_elseif_expression(){
    let value=6;
    let result=if value >=8{
        "you are good"
    }else if value < 8 && value >= 7 {
        "good"
    }else if value <7 && value >=6 {
        "so so"
    }else {
        "you not passed"
    };

    println!("{}", result)
}

#[test]
fn loop_expression(){
let mut counter =10;
    loop{
        counter += 1;
        if counter >100{
            // println!("loop stop");
            break;
        }else if counter %2 ==0 {
            continue;
        }
        println!("Counter:{}", counter)
    }
}

#[test]
fn loop_return_value(){
    let mut counter =0;
    let result=loop{
        counter += 1;
        if counter >100{
            // println!("loop stop");
            break counter*2;
        }
    };
    println!("{:?}",result);
}

#[test]
fn loop_label(){
    let mut number =1;
    'outer: loop{
        let mut i =1;
        loop{
            if number > 10{
                break 'outer;
            }

            println!("{} x {}= {}", number, i, number +1 );
            i+=1;
            if i>10{
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop(){
    let mut number =1;
    while number <= 10{
        if number % 2 == 0{
            println!("counter :{}", number);
        }
        number += 1;
    }
}

#[test]
fn array_iteration(){
    let array:[&str;5]=["A", "B", "C", "D","E"];
    let mut index= 0;
    while index< array.len() {
        println!("Value :{}", array[index]);
        index += 1;
    }
}