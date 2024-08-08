

use std::collections::hash_map::Values;
use std::fmt::format;
mod fiveth;

mod models{
    pub struct User{
        pub first_name:String,
        pub last_name:String,
        pub usernme:String,
        pub email:String,
        pub age:u8
    }

    impl User {
        fn sayHello(&self, name:&str){
            println!("Hello {}, my name is {}", name, self.first_name);
        }
    }
}

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

#[test]
fn array_iteration_for(){
    let array:[&str;5]=["A","B", "C", "D", "E"];
    for value in array{
        println!("Value :{}", value)
    }
}

#[test]
fn for_loop_range(){
    let array:[&str;5]=["A", "B", "C", "D", "E"];
    let range=0..5;
    println!("Start: {}", range.start);
    println!("End :{}", range.end);

    for i in range{
        println!("{}", array[i])
    }
}

#[test]
fn for_loop_range_inclusif(){
    let array:[&str;5]=["A", "B", "C", "D", "E"];
    let range=0..=4;
    println!("Start: {}", range.start());
    println!("End :{}", range.end());

    for i in range{
        println!("{}", array[i])
    }
}

#[test]
fn say_hello(){
    println!("Say hello")
}

#[test]
fn call_say_hello(){
    say_hello();
    say_hello();
}

fn say_goodbye(first_name:&str, last_name:&str) {
    println!("Good bye {} {}", first_name, last_name);
}
#[test]
fn call_say_goodbye(){
    say_goodbye("Yogi", "Dwitama");
    say_goodbye("Joko", "Bambang");
}

fn factorial_loop(n:i32)->i32{
    if n<1{
        return  0;
    };
    let mut result =1;
    for i in 1..=n{
        result*=i;
    };
    result
}

#[test]
fn test_factorial_loop(){
    let result:i32 =factorial_loop(5);
    println!("{}", result)
}


fn print_text(values: String, times:u32){
    if times == 0{
        return;
    }else {
        println!("{}", values)
    };

    print_text(values, times-1);
}

#[test]
fn test_print_text(){
    print_text(String::from("Yogi"), 10)
}

fn factorial_recursive(n:u32)->u32{
    if n<=1{
        return 1;
    }
    n*factorial_recursive(n-1)
}

#[test]
fn test_factorial_recursive(){
    let result= factorial_recursive(5);
    println!("{}", result)
}

fn function_ownership_print_number(number:i32){
    println!("number: {}", number);
}

fn function_ownership_hi(name:String){
    println!("hi :{}", name);
}

#[test]
fn function_ownership_test_hi(){
    let number =10;
    function_ownership_print_number(number); //print_number(10)
    println!("{}", number);

    let name=String::from("Yogi");
    function_ownership_hi(name);
    // println!("{}", name); //owner ship sudah dipindahkan ke parameter name
}

fn return_value_ownership_full_name(first_name:String, last_name:String)->String{
    format!("{} {}", first_name, last_name)
}

#[test]
fn return_value_ownership_test_full_name(){
    let first_name= String::from("Yogi");
    let last_name= String::from("Dwitama");
    let name= return_value_ownership_full_name(first_name, last_name);
    println!("{}", name);
    // println!("{}", first_name);  // error
    // println!("{}", last_name); //error
}

fn return_value_without_take_ownership_full_name(first_name:String, last_name:String)->(String, String, String){
   let full_name= format!("{} {}", first_name, last_name);
    (first_name, last_name, full_name)

}

#[test]
fn return_value_without_take_ownership_test_full_name(){
    let first_name= String::from("Yogi");
    let last_name= String::from("Dwitama");
    let (first_name, last_name, full_name)= return_value_without_take_ownership_full_name(first_name, last_name);
    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}
// Masalah dengan return Value Ownership
// jika tidak ingin mengambil ownership dari parameter, maka jika tiap membuat function kita harus membuat value tuple, maka lama-lama akan sangat menyulitkan.
//  BAHLAN JIKA DIBACA, FUNCTION YANG KITA BUA AKAN SULIT dimengerti
//  Untungnya Rust menyediakan fitur untuk menggunakan value, tanpa harus melakukab tansfer ownership, namanya adalah Reference
//

//Reference and Borrowing
//References
//Reference adalah pointer(penunjuk) yang bisa kita ikuti ke lokasi data aslinya di Heap. Datanya sendiri dimiliki oleh variable lain. bukan si references
//Reference akan dijamin menunjuk value yang valid selama alur hidup reference tersebut, jika alur hidup selesai, maka reference akan dihapus, namun tidak dengan data yang ditunjuknya, karena data yang ditunjuk mengikuti alur hidup variable ownernya
// Untuk membuat reference di Rust, kita bisa gunakan tanda &(and) sebelum tipe datanya, dan dalam satu waktu kita bisa membuat banyak reference
//Sebelum kita tahu bahwa tipe data text str selalu kita buat dalam bentuk &str, hal ini karena defaultnya adalah references ke str


fn references_full_name(first_name:&String, last_name:&String)->String{
    format!("{} {}", first_name, last_name)
}

#[test]
fn references_test_full_name(){
    let first_name= String::from("Yogi");
    let last_name=String::from("Dwitama");

    let name=references_full_name(&first_name, &last_name);
    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name);
}

//Borrowing
//Ketika membuat reference, aksi itu disebut borrowing(meminjam)
//Kalo diibaratkan, kita bisa meminjam barang, tapi jika sudah selesai menggunakan barangnya, kita harus mengembalikan ke owner(pemilik) barangnya
// Saat mencoba memodifikasi value dari reference, maka secara default, hal itu tidak bisa dilakukan, jika secara default reference adalah immutable, waluapun variable ownernya sendiri adalah mutable

// fn reference_change_value(value:&String){
//     value.push_str("Push"); // tidak bisa value di ganti karena borrowing konsep
// }
//
// #[test]
// fn reference_test_change_value(){
//     let mut value=String::from("Yogi");
//     reference_change_value(&value);
//     println!("{}", value);
// }

//Jadi sebenarnya bisa mutable namun secara default immutable
//Mutable Reference
// Pada kasusu jika memang diperbolehkan memodifikasi value dari reference, maka kita bisa gunakan mutable reference
// Mutable reference adalah dengan tanda %mut, dimana artinya kita bisa memodfikasi value dari reference tersebut
//Namun ada ketentuan jika bisa menggunakan mutable reference, variable owner juga harus mutable, jika variable owner adalah immutable, maka mutable reference tidak bisa dilakukan
// selain itu, untuk menjamin keamanan, dalam satu waktu, hanya diperbolehkan membuat `SATU` mutable reference, dan tidak ada reference lainnya

fn reference_mut_change_value(value:&mut String){
    value.push_str("Push"); // tidak bisa value di ganti karena borrowing konsep
}

#[test]
fn reference_mut_test_change_value(){
    let mut value=String::from("Yogi");

    //let valueBorrow1= &mut value; // boleh
    //let valueBorrow2= &mut value; // tidak boleh dilakukan  karena life cycle berada di satu fungsi yang sama yaitu reference_mut_test_cahneg_value

    //let valueBorrowing1= &value; //boleh
    //let valueBorr0wing2= &value;// boleh namun akan ada satu variable yang tidak digunakan

    crate::reference_mut_change_value(&mut value);
    reference_mut_change_value(&mut value); //Boleh dilakukan karena aturan life cycle scope
    println!("{}", value);
}


//Dangling Pointer
// Dangling pointer adalah kondisi diamna ada reference (pointer) yang menunjuk ke value yang tidak ada di memory
// Di Rust, hal ini tidak diperbolehkan, contoh ketika kita mengembalikan reference dalam function, maka secara otomatis value akan dihapus kerena sudah keluar dari scope function
// Pada kasus seperti ini, Rust akan menganggap hel ini error, akan berpotensi terjadi dangling pointer
// Biasanya programmer Golang sering sekali membuat function yang mengembalikan pointer

// fn dangling_full_name(first_name:&String, last_name:&String)->&String{ // memberikan pointer
//    let name= format!("{} {}", first_name, last_name);
//     return &name;
// }
//
// #[test]
// fn dangling_test_full_name(){
//     let first_name= String::from("Yogi");
//     let last_name=String::from("Dwitama");
//
//     let name= dangling_full_name(&first_name, &last_name);
//     println!("{}", name);
//     println!("{}", first_name);
//     println!("{}", last_name);
// }


//Solusi Dangling Pointer
//Jika memang data yang yang dikembalikan dibuat di dalam function, maka kita harus kembalikan dalam bentuk value langsung, bukan reference
// atau kita bisa mengeluarkan varable owner dari value diluar function, agar masuk variable scope, sehingga Rust tidak menghapus variable dan value tersebut setelah function selesai di eksekusi
//Solusi 1
fn dangling_full_name(first_name:&String, last_name:&String)->String{ // memberikan pointer
   let name= format!("{} {}", first_name, last_name);
    return name;
}

#[test]
fn dangling_test_full_name(){
    let first_name= String::from("Yogi");
    let last_name=String::from("Dwitama");

    let name= dangling_full_name(&first_name, &last_name);
    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name);
}


//Slice
//Slice adalah reference ke sebagaian elemen dari data collection(misalnya array)
//karena slice adalah reference, jadi dia tidak punya ownership
//Contoh misal kita punya array dengan total data 10, kita mau ambil 5 data terdepan, maka kita bisa membuat Slice sebagai reference data dari data ke-1 sampai ke-5

//Range
// Saat kita ingin mengambil sebagian data Collection, kita butuh menentukan range untuk Slice yang akan kita ambil
// Rust sendiri memiliki banyak jenis range, sebelumnya kida sudah bahas tentang range(exclusive) dan Ranga Inclusive, selain itu masih ada yang lain

#[test]
fn slice_reference(){
    let array:[i32;10]= [1,2,3,4,5,6,7,8,9,10];
    let slice1:&[i32]= &array[..]; //full range
    println!("{:?}", slice1);
    let slice2:&[i32]= &array[0..5];
    println!("{:?}", slice2);

    let slice3:&[i32]=&array[5..];
    println!("{:?}", slice3)
}

//String slice(&str)
// Sebelumnya, saat kita menggunakan tipe data text, kita selalu buat dalam bentuk &str(String Slice), hal ini sebenarnya berarti &str itu adalah reference ke sebagian atau seluruh data str
// Saat kita menggunakan tipe data String, kita juga mengambil sebagian karakter di String, hasil dari sebaguian data itu adalah &str( StringSlice)
// Karena &str itu adalah Slice, yang artinya adalah reference, maka sebenarnya tidak memiliki ownership, dan oleh karena itu ketiak kita assign ke variable lain atau ke function, yang di copy sebenarnya adalah referencenya, datnaya tetap menggunakan data yang sama
#[test]
fn string_slice(){
    let name:String= String::from("Yogi Dwitama");
    let first_name:&str= &name[0..3];
    println!("{}", first_name);

    let last_name:&str=&name[5..];
    println!("{}", last_name)
}

//Struct
//Struct adalah tipe data yang mirip tuple yang bisa digunakan untuk menampung beberapa data dengan tipe yang berbeda
// Yang membedakan dengan Tuple, pdada Struct, kita bisa member nama untuk tiap datanya, atau dibilang filed, sehingga lebih jelas dibanding Tuple yang hanya menggunakan number
// Dengan menambahkan nama pada data di Struct, secara otomatis kita tidak perlu harus menentukan urutan posisi data yang selalu sama , bisa berubah-ubah seiring pembuatan kode
// Untuk membuat Struct, kita bisa gunakan kata kunci struct
// Jika terdiri dari duakata maka tidak boleh memakai underscore (_)

struct Person{
    first_name:String,
    middle_name:String,
    last_name:String,
    age:u8,
}

// Membuat Instance dari Struct
// Jadi Struct sebelum digunakan kita harus buat terlebih dahulu definisi dari Structnya
// Setelah kita membuat definisi Struct-nya, selanjutnya selanjutnya kita bisa memebuat instance/ value dari Struct yang sudah kita buat
// Saat membuat isntance dari struct, kita wajib menentukan semua value untuk field dari Struct-nya
// Posisi tidak terlalu penting
#[test]
fn struct_person(){
    let person:Person=Person{
        first_name:String::from("Yogi"),
        middle_name:String::from("Dwitama"),
        last_name:String::from("Uchiha"),
        age:22
    };

    println!("{}", person.first_name);
    println!("{}", person.middle_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

// Menggunakan Struct di Function
// Struct sama seperti tipe data lainnya, kita bisa gunakan dimananpun
// kita bisa gunakan Struct sebagai parameter di function, atau return value di function

fn print_person(person:&Person){
    println!("First Name: {}", person.first_name);
    println!("Middle Name :{}", person.middle_name);
    println!("Last Name: {}", person.last_name);
    println!("Age:{}", person.age);
}

#[test]
fn struct_print_person(){
    let person:Person=Person{
        first_name:String::from("Yogi"),
        middle_name:String::from("Dwitama"),
        last_name:String::from("Uchiha"),
        age:22
    };
    print_person(&person);
}

// Init Shorthand
// Kadang ada kasus ingin membuat value untuk field di Struct dari variable yang sudah ada
// Jika misalnya nama variable sama dengan filed, kita tidak perlu sebutkan nam field secara eksplisit
// Fitur ini bernama `Init Shorthand`

#[test]
fn struct_init_shorthand_person(){
    let first_name =String::from("Yogi");
    let last_name=String::from("Uchiha");

    let person:Person=Person{
        first_name,
        middle_name:String::from("Dwitama"),
        last_name,
        age:20
    };
    // println!("{}", first_name) // will error here because ownershipnya sudah dipindahkan
    print_person(&person);
}

// Struct Update Syntax
// Sama seperti tipe data lainnya, saat kita buat istance Struct sebagai immutable, maka semua field di instance tersebut tidak bisa diubah
// Jika kita ingin mengubahnya, kita harus buat istance Struct dalam bentuk mutable variable
// Struct memiliki fitur bernama Struct Update Syntax, ini digunakan untuk membuat instance baru dari instance yang sudah ada
// Bahkan, kita bisa membuat instance baru sekaligus mengubah beberapa field yang kita mau.
#[test]
fn struct_update_syntax_person(){
    let first_name =String::from("Yogi");
    let last_name=String::from("Uchiha");

    let person:Person=Person{
        first_name,
        middle_name:String::from("Dwitama"),
        last_name,
        age:20
    };
    // println!("{}", first_name) // will error here because ownershipnya sudah dipindahkan
    print_person(&person);

    //syntax update here
    let person2=Person{..person};
    print_person(&person2)
}

//Masalah dengan Struct Update Syntax
//Namun saat menggunakan struct update syntax, hati-hati dengan field yang memiliki value di Heap, karena ownershipnya secara otomatis akan dipindahkan ske field di isntance baru
// Oleh karena itu, secara otomatis instance lama tidak bisa digunakan karena value di fieldnya sudah dipindahkan ownershipnya ke instance baru
// Atau kita bisa melakukan clone data fieldnya, jika memang tidak mau memindahkan ownershipnya
#[test]
fn struct_partial_update_syntax_manual_person(){
    let first_name =String::from("Yogi");
    let last_name=String::from("Uchiha");

    let person:Person=Person{
        first_name,
        middle_name:String::from("Dwitama"),
        last_name,
        age:20
    };
    // println!("{}", first_name) // will error here because ownershipnya sudah dipindahkan
    print_person(&person);

    //syntax update here
    let person2=Person{
        first_name:person.first_name.clone(),
        middle_name:person.middle_name.clone(),
        last_name:person.last_name.clone(),
        ..person

    };
    print_person(&person2);
    println!("{}", person.first_name);

}

// Tupek Struct
//Seperti di awal dijelaskan, bahwa Struct mirip seperti Tuple
// Seandainya jika kita ingin membuat Struct seperti Tuple, kita juga bisa buat Struct tanpa menyebutkan nama fieldny
// Namun ketika kita membuat Struct jenis ini, maka cara mengakses fieldnya sama seperti ketika kita membuat Tuple
// Ini cocok ketika kita mau membuat Tuple dengan data banyak, agar lebih sederhana, dibuat dalam bentuk Struct

struct Geopoint(f64, f64);

#[test]
fn tuple_struct(){
    let geo_point=Geopoint(-6.200000, 106.816666);
    println!("Longitude : {}", geo_point.0);
    println!("Latitude :{}", geo_point.1);
}

// Struct tanpa Field
// Field di Struct tidak wajib, artinya jika kita buat Struct tanpa field sama sekali, hal itu diperbolehkan
// Struct tanpa Field itu sama saja dengan tipe data Unit()
// Apa gunanya Struct tanpa Field? Saat di Trait, mungkin akan sering membuat Struct tanpa field.

struct Nothing;

#[test]
fn test_nothing(){
    let _nothing1:Nothing=Nothing;
    let _nothing2:Nothing=Nothing{};
}

// Reference Field di Struct
// Sebelumnya kita menggunakan  tipe data String yang disimpan di Heap, bagaimana jika kita menggunakan tipe data &str(String slice) yang merupakan tipe data reference ?
// Struct Field bisa bertipe data reference, namun untuk melakukan itu kita harus menggunakan Lifetime, dan untuk ini kita akan bahas di materi Lifetime

// Method
impl Person {
    fn say_hello(&self, name:&str){
        println!("Hello, {} my name is {}", name, self.first_name);
    }
}

#[test]
fn test_method(){
    let person =Person{
        first_name:String::from("Yogi"),
        middle_name:String::from("Yoga"),
        last_name:String::from("Yoman"),
        age:20

    };
    person.say_hello("Yogo")
}

// Associated Fucntions
// Setiap function yang dibuat dalam impl ktia sebut dengan Associated Functions, karena terkait dengan tipe data yang kita tentukan di dalam impl
// Associated Functions yang memiliki parameter self artinya adalah `Method`, dan dipanggil setelah kita membuat instance
// Namun, kita juga bisa membuat function tanpa parameter self, yang artinya function tersebut tidak terhubung dengan instancenya
// Untuk memanggil Assiciated Functions yang bukan Method, kita bisa langsung gunakan NamaType.nama_Function()
// Biasanya Associated Function bukan Method, digunakan untuk membuat isntance dan Type-nya

impl Geopoint{
    fn new(long:f64, lat:f64)->Geopoint{
        Geopoint(long, lat)
    }
}

#[test]
fn test_associated_function(){
    let geo_point:Geopoint=Geopoint::new(-6.200000, 106.8166666);
    println!("Long : {}", geo_point.0);
    println!("Lat: {}", geo_point.1);
}

// Enum
// Enum atau Enumeration, merupakan tipe data yang dibuat untuk mengumpulakan beberapa kemungkinan value
// Contoh misal, kita akan membuat tipe data Level, dimana ada bebertapa kemungkinan, value, misalnya Regular, Premium dan Platinum. Maka kita bisa gunakan tipe data Enum untuk menampung jenis data ini
// untuk mebuat Enum, kita bisa gunakan kata kunci Enum lalu diikuti dengan nama Enum
// Di dalam Enumnya, kita tentukan kemungkinan value yang diperbolehkan

enum Level{
    Regular,
    Premium,
    Platinum
}

#[test]
fn test_enum(){
    let _level:Level= Level::Platinum;
}

// Enum Data
enum Payment{
    // card number
    CreditCart(String),
    //Bank name, ewallet, number
    BankTransfer(String, String),
    // ewallet name, ewallet number
    Ewallet(String, String)
}

#[test]
fn test_payment_enum(){
    let _payment3:Payment=Payment::Ewallet(String::from("Dana"), String::from("89898989"));
    let _payment2:Payment =Payment::CreditCart(String::from("673827382"));
    let _payment1:Payment=Payment::BankTransfer(String::from("BCA"), String::from("212121"));
}

//Enum Method
// Enum Juga mirip seperti Struct, kita bisa tambahan Method
// Cara menambahkan Method di Enum caranya sama seperti nemambahkan Method di Struct

impl Payment{
    fn pay(&self, amount:u32){
        println!("Paying amount {}", amount)
    }
}

#[test]
fn test_payment_reference(){
    let _payment4 :Payment=Payment::BankTransfer(String::from("BCA"), String::from("324242"));
    _payment4.pay(600000)
}

// Mengakses Data Enum
// Secara default, kita tidak bisa mengakses data Enum
// Bahkan kita juga tidak bisa lakukan pengecekan menggunakan If Else dengan Enum
// Untuk melakukan itu semua, kita harus belajar Pattern Matching




// Pattern Matching
// Selain menggunakan If, untuk percabangan di Rust mendukung fitur bernama Pattern Matching menggunakan match
// Pattern Matching di Rust sebenarnya sangat kompleks, bisa digunakan untuk melakukan pengecekan value, variable, dan banyak hal

#[test]
fn test_enum_pattern_matching() {
    let level:Level =Level::Premium;
    match level{
        Level::Regular=>{
            println!("Regular");
        }
        Level::Premium=>{
            println!("Actived Premium")
        }
        Level::Platinum=>{
            println!("Platinum")
        }
    }
}

// Destructuring Enum Data Patterns
// Pattern matching juga bisa digunakan untuk mengambil data yang terdapat di Enum, atau disebut dengan destructuring
impl Payment{
    fn payit(&self, amount:u32){
        match self {
            Payment::BankTransfer(bank, number)=>{
                println!("Paying with bank transfer {} {} amount {}", bank, number, amount);
            }
            Payment::Ewallet(wallet, number)=>{
                println!("Paying with Ewallet {} {} amount {}", wallet, number, amount)
            }
            Payment::CreditCart(number)=>{ // jika tidak dibutuhkan maka berikan under score
                println!("Paying with credit card {} amount {}", number, amount)
            }
        }
    }
}

#[test]
fn test_payment_enum_des(){
    let _payment3:Payment=Payment::Ewallet(String::from("Dana"), String::from("89898989"));
    _payment3.payit(8900);
    let _payment2:Payment =Payment::CreditCart(String::from("673827382"));
    _payment2.payit(7000);
    let _payment1:Payment=Payment::BankTransfer(String::from("BCA"), String::from("212121"));
    _payment1.payit(6600);
}
// Patetrn Matching untuk Value
// Pattern Matching juga bisa digunakan untuk mengecek value  misalnya number atau Strign
// Namun untuk kasus itu, pasti dimungkinkan ada kombinasi yang tidak bisa dicakupi, anggap saja bagian Else nya
// Untuk bagian Else nya, gunakan nama variable, yang secara otomatis akan diisi dengan value yang kita match

#[test]
fn test_pattern_matching_value() {
    let name:&str="Yogi";

    match name {
        "Joko"=>{
            println!("Hello Yogi");
        }
        "Arman"=>{
            println!("Hello Arman");
        }
        other=>{
            println!("Hello {}", other)
        }
    }

}

// Multiple Patterns
//  Pattern matching bisa menggunakna beberapa kondisi menggunakan  | (pipe)
// Misal jika kita buat kode: "Yogi"|"Brook"|"Zoro"
// Artinya value boleh Eko, Budi  atau Joko


#[test]
fn multiple_pattern_matching() {
    let name:&str= "Yogs";

    match name {
        "Yogi" | "Brook"=>{
            println!("Shappp boss")
        }
        other=>{
            println!("Ohayo {}", other);
        }
    }
}

// Range Patterns
// Multiple Patterns sangat cocok untuk match value lebih dari satu, tapia bagaimana jika kita butuh multiple value dalam bentuk range, misal dari 0 sampai 10
// Maka akan sangat menyulitkan jika harus dibuat dalam bentuk multiple patterns
// Untungnya, Pattern Matching juga mendukung Range Patterns
// Jadi kita cukup gunakan tipe data Range(seperti yang pernah bahas di materi Slice)
// Namun saat ini, Range yang bisa digunakan adalah tipe data Inclusive Range
#[test]
fn range_pattern_matching() {
    let value =101;
    match value {
        75..=100=>{
            println!("Great");
        }
        50..=74=>{
            println!("Good");
        }
        25..=49=>{
            println!("BAD");
        }
        0..=24=>{
            println!("U st")
        }
        other=>{
            println!("Invalid value {}", other);
        }
    }
}

// Destructuring Struct Patterns
// Selain Enum. Pattern Matching juga bisa digunakan untuk melakukan destructuring terhadap Struct Field
// Namun untuk nama harus sama dengan nam fieldnya
// Kecuali untuk tipe Tuple Struct, kita bisa gunakan nama variable lain
// Jika kita butuh fieldnya untuk digunakan, kita bisa gunakan  ..(titik sebanyak dua kali)
#[test]
fn test_struct_patterns_matching() {
    let point= Geopoint::new(2.0, 11.0);
    match point {
        Geopoint(long, 0.0)=>{
            println!("Long: {}", long);
        }
        Geopoint(0.0, lat)=>{
            println!("Lat:{}", lat);
        }
        Geopoint(long, lat)=>{
            println!("long:{} lat:{}", long, lat);
        }
    }
}

#[test]
fn test_struct_patterns_matching2() {
    let person=Person{
        first_name:String::from("Yogi"),
        middle_name:String::from("Dwitama"),
        last_name:String::from("Uchiha"),
        age:54
    };
    match person {
        Person{first_name, last_name,..}=>{
            println!("{} {}", first_name, last_name)
        }
    }
}

// Ignoring
// Sebelumnya di Struct jika kita tidak fieldnya, kita bisa gunakan  ...(titik sebanyak dua kali)
// Namun pada kasus Tuple Struct, Enum, kita tidak bisa melakukan hal itu, karena posisi fieldnya sudah diatur sesuai dengan posisinya
// Jika kita tidak butuh field tersebut, kita bisa gunakan menjadi _(garis bawah)
// Atau jika tidak butuh data apapun, kita juga bisa gunakan _(ggaris bawah) seluruhnya

#[test]
fn test_ignoring() {
    let point =Geopoint(0.0, 11.0);
    match point {
        Geopoint(long, _)=>{
            println!("Long: {}", long)
        }
    }
}

#[test]
fn range_pattern_matching_ignoring() {
    let value =101;
    match value {
        75..=100=>{
            println!("Great");
        }
        50..=74=>{
            println!("Good");
        }
        25..=49=>{
            println!("BAD");
        }
        0..=24=>{
            println!("U st")
        }
        _ =>{
            println!("Invalid value ");
        }
    }
}

// Match Expression
// Sama seperti If, Loop, dan While, Match juga dianggap sebagai expression, artiny bisa menghasilkan value
#[test]
fn test_pattern_match_expression() {
    let value:i32=2;
    let result:&str=match value {
         0=>"nol",
        1=>"more than nol",
        2=>"2x than nol",
        _=>"invalid"
    };
    println!("{}", result);
}

// Type Alias
// Kadang kita butuh membuta tipe dat alias (nama lain) dari tipe data yang sudah ada
// Ini biasanya dilakukan agar lebih mendeskripsikan data yang digunakan
// Misal kita tau bahwa umur biasanya menggunakan nilai integer, kita bisa buat type alia Age misal untuk umur
// Atau kita bisa buat type alis untuk nomor identitas dari String
type  Age= u8;
type IdentityNumber= String;

struct  Customer{
    id: IdentityNumber,
    name:String,
    age:Age
}

#[test]
fn test_customer() {
    let customer=Customer{
        id:String::from("8954324829"),
        name:String::from("Yogi"),
        age:27
    };
    println!("id {}, name {}, age {}", customer.id, customer.name, customer.age)
}

// Atau bisa juga diassign ulang
// type Pelanggan =Customer;


// Module
// Saat membuat aplikasi yang semakin kompleks, maka kode program kita akan semakin banyak
// Agar kode yang kita buat semakin rapih, kita bisa organisi kode kita dalam bentuk Module
// Sehingga kdoe-kode yang dalam fitur yang sama, bisa kita simpan dalam Module yang sama agar lebih rapih
// Untuk membuat module, kita bisa gunakan kata kunci mod lalu diikuti dengan nama Module nya
// Di dalam Module tersebut, kita bisa simpan semua kodeprogram yang ingin kita tempatkan
mod model{
    struct User{
        first_name:String,
        last_name:String,
        usernme:String,
        email:String,
        age:u8
    }

    impl User{
        fn sayHello(&self, name:&str){
            println!("Hello {}, my name is {}", name, self.first_name);
        }
    }
}


// Visibility
// Secara default, kode di dalam Module seperti Type, Function dan Method, itu hanya bisa diakases di Module yang sama, atau bisa dibilang private
// Jika kita ingin mengakses isi dari Module tersebut di luar Module nya, kita harus ubah akases dari private menajdi public. Kita bisa menggunakan kata kunci pub diawal Type, function atau Method
mod modelpub{
   pub  struct User{
        pub first_name:String,
        pub last_name:String,
        pub username:String,
        pub email:String,
        pub age:u8
    }

    impl User {
       pub fn sayHello(&self, name:&str){
            println!("Hello {}, my name is {}", name, self.first_name);
        }
    }
}

// Mengakases Module pub
// Untuk mengkases Type atau Function di Module, kita bisa gunakna nama Module, lalu diikuti dengan  :: (titik dua sebanyak dua kali), lalu diikuit dengan nam Type atau Funtionnya
#[test]
fn test_module() {
    let user = modelpub::User{
        first_name:String::from("Yogi"),
        last_name:String::from("Dwitama"),
        username:String::from("yogi.dwitama"),
        email:String::from("yogi@test.com"),
        age:20
    };
    user.sayHello("Yogi");
}

// Use Keyword
// Kadang agak teerlalu sulit jika harus menerus menulis nama mdoule ketika ingin meenggunakan Type atau Function disebuah module
// Kita bisa menggunakan use untuk mengambil member dari sebuah module masuk ke scope module yang melakukan use, sehingga kita tidak perlu lagi menyebutkan nama module sehingga menggunakan member tersebut
// Jika kita melakukan sue beberapa member di module yang berbeda, tapi ternyata nama memberny sama, maka kita bisa gunakan kata kunci as untuk membuat alias agar nama member tidak bentrok
mod first {
    pub fn say_hellow(){
        println!("Hello i'm first module");
    }
}

mod second{
    pub fn say_hello(){
        println!("Hello i'm second module")
    }
}

// Menggunakan Use
use first::say_hellow;
use second::say_hello as say_hello_Second;

#[test]
fn test_use() {
    say_hellow();
    say_hello_Second();
}

// Module di File Terpisah
// Walaupun  kita sudah organisir kode program kita dalam Module, namun jika disimpan di dalam satu file, lama-lama kode program akan terlalu panjnag dan sulit untuk di maintain
// Kita bisa memisahkan Module ke file terpisah, secara otomatis nama file akan menjadi nama Module, jadi kita tidak wajib menambahkan kode `mod` lagi
// Kecuali jika kita ingin menambahkan sub Module, kita bisa tambahkan mod lagi di dalam file Module yang sudah dipisah
// Secara default, file Module yang sudah dipisah tidak akan diinclude dalam program, jika kita ingin menggunakan Module file tersebut, kita harus include menggunakan mod namafile(tanpa extension.rs)
mod third; // usually this on first line of code
mod fourth;

// usually this on first line of code

use fourth::say_helloempat;
use third::say_hellotiga as say_hello_third;
#[test]
fn test_module_file() {
    say_helloempat();
    say_hello_third();
}

mod modelfilemodule;
use modelfilemodule::User;

#[test]
fn test_module_file_fn() {
    let user:User =User{
        first_name:String::from("Yogi"),
        last_name:String::from("Dwitama"),
        username:String::from("yogi.dwitama"),
        email:String::from("yogi@test.com"),
        age:20
    };
    user.sayHello("Joe");
}


// Penggunaan Use Lainnya
// Kadang ketika kdoe program kita sudah banyak, dan kita melakukan `use` banyak member di satu Module, maka kdeo use kita akan sangat banyak
// Ada beberapa cara lain untuk melakukan use, jika kita ingin mengambil semua member di module, kita bisa gunakan tanda *(bintang): use module::*;
// Atau jika ingin mengambil beberapa saja, bisa sebutkan dalam tanda {}(kurung kurawal): use module::{A,B,C}

// Crate
// Crate adalah kode yang dijalankan oleh Rust Compiler untuk membuat alikasi atau library
// Contoh jika di project yang sbelumnya kita buat, bentuknya adalah aplikasi, maka Carte nya adalah src/main.rs
// Kita tidak bisa ubah itu, karena itu sudah jadi ketentuan jika ingin membuat aplikasi, maka kita perlur membuat main file yang berisi main function
// Di dalam Crate, kita harus definisiakan file-file yang ingin kita gunakan sebagai Module
// Dan ketika melakukan use di file yang bukan main.rs, kita harus gunakan crate:: diawal, untuk mengacu ke main.rs
// Penggunaan `Crate::` hanya pada file/module salin main.rs.

// Kita akan gunakan Module Fiveth
// Misal kita akan menggunakan module Fiveth di module third
// Walaupun yang menggunakan adalah module first, namun include module file tersbut harus dilakukan dari main.rs
// Dan jika bukan dari main.rs, jika kita ingin melakukan use, maka diawal use kita harus gunakan `crate::` diikuti nama modulenya
#[test]
fn test_module_file_again() {
    say_helloempat();
    say_hello_third(); //this will get function from fiveth .rs module
}

// Super Keyword
// Saat kita membuat Nested Module, kadang kita ingin mengakses Module yang ada diatasnya
// Untuk kasus seperti ini, kita bisa sebutkan nama module nya dari awal menggunakan crate, atau kita bisa sebutkan module parent nya dengan kata kunci `super`
#[test]
fn test_module_file_super() {
    say_helloempat();
    say_hello_third(); //this will get function from fiveth .rs module
    // how to access nested module using `super`
    crate::third::second::third::say_hello();
}

