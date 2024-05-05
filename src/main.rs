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
}