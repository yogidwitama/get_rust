

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


// Trait
// Trait adalah definisi fungsional untuk tipe data lain
// Biasanya Trait digunakan untuk dasar implementasi beberapa tipe data
// Di bahasa pemrogramana lain seperti Java atau Golang, Trait mirip seperti Interface
// Trait berisi definisi method tanpa implementasi konkrit
// Untuk membuat Trait, kita bisa menggunakan kata kunci `trait`, diikuti dengan Trait nya
trait CanSayHello{
    fn say_Hello_Trait(&self)-> String;
    fn say_Hello_Trait_to(&self, name: &str)->String;
}

// Implementasi Trait
// Trait bisa digunakan sebagai tipe data, namun tetap perlu ada implementasi konkritnya, misal mengguanakan Struct atau Enum
// Untuk implementasi Trait, kita bisa gunakan:
// impl NamaTrait for NamaType{
//      isi method
// }
impl CanSayHello for Person{
    fn say_Hello_Trait(&self) -> String {
        format!("Hello im is {}", self.first_name)
    }
    fn say_Hello_Trait_to(&self, name: &str) -> String {
        format!("Hello, {} my name is {}", name, self.first_name)
    }
}

// Menggunakan Trait
// Trait tidak bisa dibuat instance-nya
// Untuk membuat instance dengan Tipe data Trait, maka kita harus gunakan implementasinya
#[test]
fn test_trait() {
    let person:Person=Person{
        first_name:String::from("Yogi"),
        middle_name:String::from("Ogiy"),
        last_name:String::from("Dwitama"),
        age:20
    };

    let result =person.say_Hello_Trait_to("Roger");
    println!("{}", result);
}


// Default Implementasi
// Sebelumnya kita hanya membuatmethod di Trait tanpa implementasi konkritnya
// Trait sebenarnya bisa juga digunakan untuk membuat Method dengan implementasi konkrit, atau kita sebut dengan Default implementation
// Secara otomatis Type yang nanti kita lakukan implementasi, akan mendapatkan default implementastion dari methdo tersebut
trait CanSayHelloDefault{
    fn hello(&self)->String{
        String::from("Hello")

    }
    fn say_Hello_Trait_default(&self)-> String;
    fn say_Hello_Trait_to_default(&self, name: &str)->String;
}

impl CanSayHelloDefault for Person{
    fn say_Hello_Trait_default(&self) -> String {
        format!("Hello im is {}", self.first_name)
    }
    fn say_Hello_Trait_to_default(&self, name: &str) -> String {
        format!("Hello, {} my name is {}", name, self.first_name)
    }
}

#[test]
fn test_trait_default() {
    let person:Person=Person{
        first_name:String::from("Yogi"),
        middle_name:String::from("Ogiy"),
        last_name:String::from("Dwitama"),
        age:20
    };

    let result =person.say_Hello_Trait_to_default("Roger");
    println!("{}", result);
    let result =person.hello();
    println!("{}", result);
}


// Trait sebagai Parameter
// Salah satu keuntungan mengggunakan Trait adalah ketika kita gunakan Trait sebagai parameter
// Saat gunakan Trait sebagai parameter, maka kita bisa gunakan value apapun yang merupakan implemntasi dari Trait tersebut sebagai value untuk parameternya
// Untuk menggunakan Trait  sebagai paramter, kita bisa gunakan  kata kunci impl NamaTrait pada parameternya
// Jika kita ingin tipe data reference, kita bisa gunakan  &impl NamataTrait pada parameter nya
// Jika kita ingin tipe data reference, kita bisa gunakan  `&impl NamaTrait`

fn say_hello_trait_parameter(person: &impl CanSayHelloDefault){
    println!("{}", person.say_Hello_Trait_default())
}

#[test]
fn test_trait_parameter() {
    let person:Person=Person{
        first_name:String::from("Yogi"),
        middle_name:String::from("Ogiy"),
        last_name:String::from("Dwitama"),
        age:20
    };

    say_hello_trait_parameter(&person);

    let result =person.say_Hello_Trait_to_default("Roger");
    println!("{}", result);
    let result =person.hello();
    println!("{}", result);
}

// Multiple Trait
// Typ itu bisa mengimplementasikan lebih dari satu Trait
// Oleh karena itu, saat kita membuat parameter juga,kita bisa buat satu parameter untuk beberapa tipe Trait
// Kita bisa gunakan tanda `+ (plus)` jika ingin membuat paramter dengan tipe Multiple Trait, misal (impl Trait1 + Trait2 +Trait3)
trait CanSayGoodByeMultipleTrait{
    fn good_bye(&self)-> String;
    fn good_bye_to(&self, name:&str)->String;

}

impl CanSayGoodByeMultipleTrait for Person{
    fn good_bye(&self) -> String {
        format!("Good bye, my name is {}", self.first_name)
    }
    fn good_bye_to(&self, name: &str) -> String {
        format!("Good bye, {} my name is {}", name, self.first_name)
    }

}

fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodByeMultipleTrait)){
    println!("{}", value. say_Hello_Trait());
    println!("{}", value.good_bye());
}

#[test]
fn test_trait_parameter_multiple_trait() {
    let person:Person=Person{
        first_name:String::from("Yogi"),
        middle_name:String::from("Ogiy"),
        last_name:String::from("Dwitama"),
        age:20
    };

    say_hello_trait_parameter(&person);

    let result =person.say_Hello_Trait_to_default("Roger");
    println!("{}", result);
    let result =person.hello();
    println!("{}", result);

    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("Cia"));

    hello_and_goodbye(&person);
}

// Return Trait
// Selain untuk Paramter, Trait juga bisa digunakan sebagai Return Value di function
// Namun seperti yang dijelaskan di awal, karena Trait tidak bisa dibuat instance-nya secara langsung, maka value yang kita kembalikan juga harus dalam bentuk implementasi Type nya
// Untuk membuat Trait sebagai return value, kita perlu sebutakn seprti Paramater, yaitu impl NamaTrait nya
struct SimplePerson{
    name:String
}

impl CanSayGoodByeMultipleTrait for SimplePerson{
    fn good_bye(&self) -> String {
        format!("Good bye, my name is {}", self.name)
    }
    fn good_bye_to(&self, name: &str) -> String {
        format!("Good bye, {} my name is {}", name, self.name)
    }

}
fn create_person(name:String)-> impl CanSayGoodByeMultipleTrait{
    SimplePerson{name} //return value
}

#[test]
fn test_return_trait() {
    let person=create_person(String::from("Yogi"));
    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("Yogi"));
}

// Conflict Method Name
// Salah satu problem ketika menggunakan beberapa `Trait` adalah, kadang nama method di Trait bentrok atau konflik dengan method di Trait lainnya
// Atau bahkan bisa bentrokd engan method di Type nya sendiri
// Contoh sebelumnya, kita membuta method say_hello() di Trait CansayHello, dan Person juga sudah memiliki method say_hello()
// Saat kita buat implementasi dari trait, Rust tidak akan menjadikan itu sebagai error, namun masalahnya terjadi ketika kita memanggil method nya
// Rust akan menjadikan itu error karena methodnya ambigu, Rust akan komplen karena ada beberapa method dengan nama yang sama
// Cara agar bisa menentukan method yang ingin kita panggil, kita bisa sebutkan Type::nama_methodd(instance)
#[test]
fn test_trait_parameter_Conflict_Method() {
    let person:Person=Person{
        first_name:String::from("Yogi"),
        middle_name:String::from("Ogiy"),
        last_name:String::from("Dwitama"),
        age:20
    };

    say_hello_trait_parameter(&person);

    let result =person.say_Hello_Trait_to_default("Roger");
    println!("{}", result);
    let result =person.hello();
    println!("{}", result);

    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("Cia"));

    CanSayHelloDefault::say_Hello_Trait_default(&person);
    Person::say_hello(&person, "RogerD");
}

// Super Trait
// Trait bisa digunakan dengan konsep mirip pewarisan, dimana satu Trait bisa memiliki beberapa Trait dibawahnya
// Trait yang ada diatasnya bisa kita sebut Super Trait
// Misal kita punya Trait A, lalu kita buat Trait B dan Trait C, Trait A kita jadikan sebagai Super Trait dari Trait B dan Trait C.
// Aritnya sekaranag jika kita implementasi Trait B atau Trait C, secara otomatis kita harus implementasi juga Trait A
// Trait boleh memiliki lebih dari satu Super Trait, caranya kita bisa gunakan tanda  `+ (plus)`

impl CanSayHelloDefault for SimplePerson{
    fn say_Hello_Trait_default(&self) -> String {
        format!("Hello im is {}", self.name)
    }
    fn say_Hello_Trait_to_default(&self, name: &str) -> String {
        format!("Hello, {} my name is {}", name, self.name)
    }
}
trait CanSay: CanSayHelloDefault +CanSayGoodByeMultipleTrait{
    fn say(&self){
        println!("{}", self.say_Hello_Trait_default());
        println!("{}",self.good_bye_to("hehehe"));
    }
}

impl CanSay for SimplePerson {

}

// Generic
// Generic merupakan fitur dimana kita bisa membuat function, struct, enum, mtehod, dan trait yang tipe datanya bisa diubah ketika digunakan
// Fitur ini sangat berguna ketika memang kita ingin membuat sebuah kode yang generic/ general untuk berbagai tipe data, sehingga kita tidak perlu tentukan dari awal tipe data yang ingin kita gunakan
// Kita akan coba fitur generic ini di berbagai lokasi yang bisa dilakukan di rust

// Generic di Struct
// Ketika membuat generic di Struct, kita bisa tambahkan tipe data generic setelah nam Struct menggunakna tanda <> (diamond), dimana didalam tanda diamond tersebut, kita sebutkan nam-nama tipe data genericnya
// Tipe data generic bisa lebih dari satu, tinggal gunakan, (koma) sebagai pemisah tipe data generic nya
// Biasanya nama tipe data generic hanya menggunakan satu huruf kapital
struct Point<T>{
    x:T,
    y:T,
}

#[test]
fn test_generic_struct() {
    let integer:Point<i32> = Point::<i32>{x:5, y:10};
    let float: Point<f64>= Point::<f64>{x:1.0, y:4.0};

    println!("integer x: {} y:{}", integer.x, integer.y);
    println!("float x:{} y:{}", float.x, float.y);
}

// Generic Enum
enum Value<T>{
    NONE,
    VALUE(T)
}

#[test]
fn test_generic_enum() {
    let value:Value<i32>=Value::<i32>::VALUE(10);

    match value {
        Value::NONE=>{
            println!("none")
        }
        Value::VALUE(value)=>{
            println!("value")
        }
    }

}

// Generic Type Bound
// Saat kita membuat generic type, kita bisa memberi batasan type yang diperbolehkan
// Caranya kita bisa gunakan:(titik dua) diikuti dengan Trait
// Artinya, genric type yang diperbolehkan hanyalah implementasi dari Trait tersebut
// Jika ingin menggunakan multiple Trait, seperti biasa kita bisa gunakan +(plus)
struct Hi<T: CanSayGoodByeMultipleTrait>{
    value: T,
}

#[test]
fn test_generic_struct_with_trait() {
    let hi =Hi::<SimplePerson>{ //bisa ditambahdengan +CanSayHello
        value:SimplePerson{
            name:String::from("Yogi")
        }
    };
    println!("{}", hi.value.good_bye_to("batman"))
}

// Generic Function

fn min<T:PartialOrd>(value1:T, value2:T)->T{
    if value1<value2{
        value1
    }else{
        value2
    }
}

#[test]
fn generic_in_function(){
    let result =min::<i32>(10,20);
    println!("{}", result);

    let result =min(17,5);
    println!("{}", result);
}

// Generic di Method
// Ketika membuat generic di method, kita bisa tambahkan tiped ata generic setelah kata kunci impl, yang secara otomatis bisa digunakan di semua method
// Ataua jika hanya khusus untuk method tertentu, kita bisa tambahkan generic type seperti pada function
impl <T>Point<T>{
    fn get_x(&self)->&T{
        &self.x
    }
    fn get_y(&self)-> &T{
        &self.y
    }
}

#[test]
fn test_generic_method() {
    let point:Point<i32>= Point{x:5, y:10};
    println!("{}", point.get_x());
    println!("{}", point.get_y());
}

// Generic Trait
// Saat kita membuat Trait, kita juga bisa menambahkan generic type
// Saat ktia membuat generic type di Trait, secara otomatis kita kan memaksa implementasi Trait tersebut harus menggunakan generic type di implementasi
trait GetValue<T>{
    fn get_value(&self)-> &T;
}

impl <T>GetValue<T> for Point<T> {
    fn get_value(&self) -> &T {
        &self.x
    }
}
// Where Clause
// Sebelumnya saat menggunakan ty[e bound, kita akan menggunakan :(titik dua) diikuti dengan Trait
// Ada cara yang lain untuk menambahkan type bound, caranya menggunakan kata kunci where
// Ini akan lebih mudah dibaca ketika type bound sangat banyak
trait  GetValues<T> where T: PartialOrd{ //bisa ditambakan  hanya dengan `,` koma , X:PartialOrd
    fn get_value(&self)->&T;
}

impl <T> GetValues<T> for Point<T> where  T:PartialOrd{
    fn get_value(&self) -> &T {
        &self.x
    }
}

// Default Generic Type
// Saat kita menggunakan generic type, kita bisa menambakan default type menggunakan tanda `=` (sama dengan)
// Artinya jika kita tidak menentukan tipe genericnya, secara otomatis akan menggunakan tipe data tersebut
struct  PointBlank<T=i32>{
    x:T,
    y:T,
}

#[test]
fn test_generic_default_value() {
    let pointlank=PointBlank{x:10, y:20};
    println!("x:{}, y:{}", pointlank.x, pointlank.y);
    let pointlank=PointBlank::<f32>{x:10.5, y:20.5};
    println!("x:{}, y:{}", pointlank.x, pointlank.y);
}
// Overloadable
// Sebelumnya kita pernah belajar operator matematika untuk tipe data number
// Apakah tipe data selain number mendukung operator matematika seperti `+`, `-`, dan lainnya
// Secara default tidak, namun Rust memiliki fitur dimana kita bisa mengimplementasikan oprator dalam bentuk method, sehingga bisa menggunakan operator matematika
// Semua operator direprensentasikan dalam bentuk Trait yang bisa kita implementasikan
// Semua Traitnya berada di Module/ Crate core::ops
// https://doc.rust-lang.org/core/ops/index.html

use core::ops::Add;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

//usually on the top
struct Apple{
    quantity:i32,
}

// INI AKAN ERROR

impl Add for Apple{
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple{
            quantity:self.quantity + rhs.quantity
        }
    }
}

#[test]
fn test_operator_add() {
    let apple1= Apple{quantity:10};
    let apple2= Apple{quantity:10};

   let apple23= apple1+apple2;
    println!("{}", apple23.quantity);
}
// Optional Value
// Null atau Undefined
// Jika sebelumnya kita pernah belajar bahasa pemrograman sepertiu java, JavaScript atau Undefined
// Yaitu nilai kosong(tidak ada) pada variable
// Rust tidak mengenal hal itu, saat membuat variable maka kita wajib mengisi value pada variable tersebut, akan aman karena sudah dipastikan bahwa variable tersebut berisi data
// Lantas bagaiaman jika kita ingin membuat variable yang memang datanya tidak wajib kita isi? Maka kita bisa menggunakan Option Enum

// Rust menyediakan Optional Enum, yang merupakan reperesentasi dari optional value (value yang tidak wajib diisi)
// Sederhananya, Option Enum menyediakan dua opsi, None untuk opsi nilai kosong, dan some(T) untuk opsi value tidak kosong
// Kelebihannya menggunakan Enum adalah, kita bisa menggunakan Pattern Matching ketika melakukan pengecekan nilai pada Enum Option tersebut
// Enum Option terdapat terdapat di Module/ Create core ::option
// https://doc.rust-lang.org/stable/core/option/

fn double(x:Option<i32>)-> Option<i32>{
    match x{
        None=>None,
        Some(i)=>Some(i*2),
    }
}

#[test]
fn test_optional_values() {
    let result =double(Some(3));
    println!("{:?}", result);

    let result =double(None);
    println!("{:?}", result);
}

// Comparing
// Selain operator matematika, di Rust juga juga bisa digunakan untuk pembuatan operator perbandingan menggunakan Module/Crate core:cmp
// https://doc.rust-lang.org/core/cmp/index.html
// Penggunaanny sama, kita tinggal implementasi Trait yang sesuai dengan operator yang ingin kita buat
impl PartialEq for Apple{
    fn eq(&self, other: &Self) -> bool {
        self.quantity== other.quantity
    }
}

impl  PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

// VS
// Apple Implementation (Manual)
// impl PartialOrd for Apple{
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         if self.quantity<other.quantity{
//             Some(Ordering::Less)
//         }else if self.quantity> other.quantity{
//             Some(Ordering::Greater)
//         }else {
//             Some(Ordering::Equal)
//         }
//     }
// }

#[test]
fn test_comparing() {
    let apple1= Apple{quantity:10};
    let apple2= Apple{quantity:20};

    println!("Apple 1 == Apple 2: {}", apple1==apple2);
    println!("Apple 1 < Apple 2: {}", apple1 < apple2);
    println!("Apple 1 > Apple 2 :{}", apple1>apple2)
}

// String Manipulation
// https://doc.rust-lang.org/std/string/struct.String.html#
#[test]
pub fn test_string_manipulation () {
    let s =String::from("Yogi Dwitama Uchiha");

    println!("{}", s.to_ascii_uppercase());
    println!("{}", s.to_uppercase());
    println!("{}", s.len());
    println!("{}", s.replace("Yogi", "Allicio"));
    println!("{}", s.contains("Yogi"));
    println!("{}", s.starts_with("Yogi"));
    println!("{}", s.trim());
    println!("{:?}",&s[0..3]);
    println!("{:?}", s.get(0..5));
}

// Formatting
// Sebelumnya kita asering menggunakan `println!`
// `println!` adalah macro, bukan function
// Saat menggunakan macro println!, kita sering menambahkan parameter tambahan untuk menampilkan data
//  secara default, data tidak bisa ditampilkan dalam macro `println!`, yang bisa ditampilkan hanyalah data yang sudah implementasi Module core:fmt
// https://doc.rust-lang.org/std/fmt/
#[test]
fn test_format() {
    let person = SimplePerson{
        name:String::from("Yogi")
    };

    // println!("{:?}", person);
}

// Display vs Debug
// Saat kita menggunakan formatiing, kita sering menggunakan  `{}` (Display), atau `{:?}` (Debug), Pertanyaannya, lebih baik pilih yang mana ?
// Sebenernya kalo diperhatikan, kebanyakan tipe data yang primitive menggunakan Display, sedangkan tipe data kompleks sperti Array, Slice itu banyak menggunakan Debug
// Tapi sebenarnya kita juga bisa implementasi Display dan Debug secara bersamaan.
struct Category{
    id:String,
    name:String
}

use std::fmt::{Debug, Formatter};
use std::io::read_to_string;
use std::ops::Deref;
use std::panic::Location;
use log::error;

impl Debug for Category{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("author",&"yogs")
            .finish()
    }
}

#[test]
fn test_formatter_impl() {
    let catergory= Category{
        name:String::from("Gatget"),
        id:String::from("gatget")
    };

    println!("{:?}", catergory)
}

// Closure
// Closure adalah function tanpa nama yang biasanya disimpan ddi variable atau digunakan di parameter atau bis disebut anonymous function
// Kita bisa membuat Closure dan memanggil Closure ketika membutuhkannya
// Untuk membuat Closure, kita bisa gunakan tipe data `fn(paramType)->returnType`
// Dan untuk memanggil Closure, kita bisa panggil menggunakan nama variable atau paramternya secara langsung
#[test]
fn test_closure() {
    let sum:fn(i32, i32)-> i32 =|value1:i32, value2:i32|->i32{
        value1+value2
    };
    let result =sum(2,8);
    println!("{}", result);
}

// Closure sebagai parameter
fn print_with_filter(value:String, filter:fn(String)-> String){
    let result =filter(value);
    println!("Result :{}", result);
}

#[test]
fn test_closure_as_aparameter() {
    let name = String::from("Yogi Dwitama");
    print_with_filter(name, |value:String| -> String{value.to_uppercase()});
}

// Closure dari function
fn to_uppercase(value:String)->String{
    value.to_uppercase()
}

#[test]
fn test_print_to_upper() {
    let name=String::from("Dwitama");
    print_with_filter(name, to_uppercase);
}

// Closure Scope
#[test]
fn test_closure_Scope() {
    let mut counter =0;

    let mut increment =||{
        counter +=1;
        println!("Incremennt")
    };

    increment();
    increment();
    increment();

    println!("Counter :{}", counter)
}

// Alternative
struct  Counter{
    counter:i32,
}

impl Counter{
    fn icnrement(&mut self){
        self.counter +=1;
        println!("Increment")
    }
}

#[test]
fn test_print_closure_scope_with_struct() {
    let mut counter= Counter{counter:0};
    counter.icnrement();
    counter.icnrement();
    counter.icnrement();

    println!("Counter :{}", counter.counter)
}

// Sequence
// Sequence adalah tipe data Collection mirip seperti array, memiliki index
// Rust menyediakan beberapa tipe data Sequence, dan bisa digunakan sesuai kebutuhan kita,
// Vec(Vector)
// VecDeque
// LinkedList

// Vector
// Vector merupakan sequence yang urutannya sesuai dengan yang kita inginkan
// Menambahkan data ke Vector dilakukan dibagian belakang
// Cocok untuk implementasi Stact(tumpukan), Lasar In First Out
// https://doc.rust-lang.org/std/vec/struct.Vec.html
#[test]
fn test_vector() {
    let mut names: Vec<String>= Vec::<String>::new();
    names.push(String::from("Yogi"));
    names.push(String::from("Dwitama"));
    names.push(String::from("Uchiha"));

    for nama in &names{ // gunakan `&` pointer
        println!("{}", nama)
    }

    println!("{}", names[0]);

//     Bandingkan dengan Array yang disimpan dalam `Stack`
    let arrayPrim=["Yogi", "Dwitama", "Uchiha"];
    for value in arrayPrim{
        println!("{}", value)
    };


//     kemudian kita panggil lagi functionnya, maka functionnya masih bisa digunakan
    println!("{:?}", arrayPrim)
}

// VecDeque
// VecDeque sebenarnya mirip seperti Vector
// Yang membedakan adalah dia memiliki kemampuan menambahkan data di depan(head) dan dibelakang(end)
// Sehingga VecDeque juga cocok digunakan untuk implementasi Queue(Antrian), First In First Out
// https://doc.rust-lang.org/std/collections/struct.VecDeque.html
#[test]
fn test_vec_deque() {
    let mut names: VecDeque<String>=VecDeque::new();
    names.push_back(String::from("Yogi"));
    names.push_back(String::from("Dwitama"));
    names.push_front(String::from("Uchiha"));
    for name in &names{
        println!("{}", name)
    }
    // Sifatnya sama seperti array
    println!("{}", names[0]);
}

// LinkedList
// LinkedList merupakan implementasi Sequence menggunakan struktur data Linked List
// Struktur data LinkedList sangat efisien untuk menambahkan dan pengurangan data, oleh karena itu sangat cocok ketikakita butuh Sequence yang tidak terprediksi ukurannya
// Namun perlu diperhatikan, performa Linked List tidak sebaik Vector ketiak mengakses data megnggunakan index, oleh karena itu Linked List tidak memiliki fitur untuk mengakses data menggunakan index
// https://doc.rust-lang.org/std/collections/struct.LinkedList.html
#[test]
fn test_LinkedList() {
    let mut names: LinkedList<String>=LinkedList::<String>::new();
    names.push_back(String::from("Yogi"));
    names.push_back(String::from("Dwitama"));
    names.push_front(String::from("Uchiha"));
    for name in &names{
        println!("{}", name)
    }
    // Sifatnya tidak sama seperti array
    // println!("{}", names[0]);
}

// HashMap dan BTreeMap
// Rust memiliki dua implementasi Map, yaitu HashMap dan urutan BTreeMap
// Perbedaan yang mencolok adalah pada BTreeMap, key akan diurutkan
// Dan karena pada HashMap tidak akan diurutkan, oleh karena itu operasi untuk memasukann data di HashMap lebih cepat dibandingkan BTreeMap, namun urutan key tidak bisa dijamin urutannya sama sekali
// https://doc.rust-lang.org/std/collections/struct.HashMap.html
// https://doc.rust-lang.org/std/collections/btree_map/index.html

// HashMap
#[test]
fn test_hash_map() {
    let mut map: HashMap<String, String>=HashMap::new();
    map.insert(String::from("name"), String::from("Yogi"));
    map.insert(String::from("age"), String::from("25"));

    let name=map.get("name");
    let age= map.get("age");

    println!("Name: {}, age:{}", name.unwrap(), age.unwrap())
}

// BTreeMap
#[test]
fn test_btree_map() {
    let mut map: BTreeMap<String, String>= BTreeMap::new();
    map.insert(String::from("name"), String::from("Yogi"));
    map.insert(String::from("age"), String::from("34"));
    map.insert(String::from("country"), String::from("Ausi"));

    for entry in map{
        println!("{}:{}", entry.0, entry.1 );
    }
}

// Set
// `Set` merupakan tipe data collection dimana data didalam Set tidak boelh duplikat (tidak boleh sama)
// Jika kita memsukan data ke dalam `Set` dengan data yang sudah ada didalam  `Set`, secara otomatis data tidak akan diterima
// `Set` tidak sperti Sequence,data di `Set` tidak bisa diakases menggunakan index tapi menggunakan perulangan(loop)

// HashSet dan BTreeSet
// Sama seperti Map, Set memiliki dua implementai di Rust, yaitu HashSet dan BTreeSet
// HashSet tidak menjamin urutan data, karena tujuan HashSet adalah memastiakn tidak ada data duplikat secara cepat
// BTeeSet selain memastikan tidak ada data duplikat, juga mengurutkan data di dalam Set, oleh karena itu performanya lebih lambat dari HashSet karena perlu mengurutkan data setiap kita menambahkan data ke BTreeSet
// https://doc.rust-lang.org/std/collections/struct.HashSet.html
// https://doc.rust-lang.org/std/collections/struct.BTreeSet.html

// HashSet
#[test]
fn test_hash_set() {
    let mut set : HashSet<String>=HashSet::new();
    set.insert(String::from("Yogi"));
    set.insert(String::from("Yogi"));
    set.insert(String::from("Dwitama"));
    set.insert(String::from("Dwitama"));
    set.insert(String::from("Uchiha"));
    set.insert(String::from("Uchiha"));
    for setsat in set{
        println!("{}", setsat)
    }
}
// BtreeSet
#[test]
fn test_btree_set() {
    let mut set : BTreeSet<String>=BTreeSet::new();
    set.insert(String::from("Yogi"));
    set.insert(String::from("Yogi"));
    set.insert(String::from("Dwitama"));
    set.insert(String::from("Dwitama"));
    set.insert(String::from("Uchiha"));
    set.insert(String::from("Uchiha"));
    for setsat in set{
        println!("{}", setsat)
    }
}

// Iterator
// Rust memiliki medoule Iterator, yang digunakan sebagai mekanisme untuk melakukan operasi terhadap ururtan dari data
// Semua tipe data yang multiple seperti Array, Slice dan Collection memiliki fitur Iterator
// Dengan menggunakan Iterator, secara otomatis kita bisa melakukan iterasi data menggunakan For Loop terhadap value tersebut
// https://doc.rust-lang.org/std/iter/trait.Iterator.html
#[test]
fn test_btree_set_into_iter() {
    let mut set : BTreeSet<String>=BTreeSet::new();
    set.insert(String::from("Yogi"));
    set.insert(String::from("Yogi"));
    set.insert(String::from("Dwitama"));
    set.insert(String::from("Dwitama"));
    set.insert(String::from("Uchiha"));
    set.insert(String::from("Uchiha"));
    for setsat in set.into_iter(){ //tidak harus eksplisit `.into_iter`
        println!("{}", setsat)
    }
}

#[test]
fn test_terator() {
    let array:[i32;5]=[1,2,3,4,5];
    let mut iterator= array.iter();

    while let Some (value)= iterator.next(){
        println!("{}", value);

    }

    for value in iterator{
        println!("{}", value)
    }
}

// Iterator Method
#[test]
fn test_iterator_method() {
    let vector:Vec<i32>= vec![1,2,3,4,5,6,7,8,9,10,202];
    println!("Vector:{:?}", vector);
    let sum:i32= vector.iter().sum();
    println!("Sum:{}", sum);
    let count:usize=vector.iter().count();
    println!("Count:{}", count);
    let doubled:Vec<i32>=vector.iter().map(|x| x*2).collect();
    println!("Doubled:{:?}", doubled);
    let odd: Vec<&i32>=vector.iter().filter(|x| *x % 2 !=0).collect();
    println!("Odd:{:?}", odd)
}

// Error Handling
// Rust membagi error menjadi dua jenis recoverable( dapat dipulihkan) dan unrecoverable(tidak dapat dipulihkan)
// Rust tidak punya data Exception, seperti Java, PHP, JavaScript
// Rust menggunakan pedakatan lain untuk Error Handling

// Unrecoverable Error
// Jika terdapat jenis error yang menurut kita tidak bisa dipulihkan, maka kita bisa jenis Unrecoverable Error
// Rust menggunakan macro `panic!` untuk melakukan ini
// Misalnya saat aplikasi yang kita buat perjalan, ternyata tidak ada konfigurasi untuk terkoneksi di database. Tidak ada gunanya mengecek ulang pada kasus ini misalnya, lebih baik matikan aplikasi dan sebutkan error konfigurasi tidak ada. Pada kasus ini, kita bisa gunakan Unrecovarable Error
// Beberapa hal di Rust juga menggunakan Unrecoverable Error, contoh ketiak mengakses index di array/ vector diluar jangkauan index nya

fn connect_database(host:Option<String>){
    match host {
        Some(host)=>{
            println!("Connect to database at: {}", host);
        }
        None=>{
            panic!("No database host provided")
        }
    }
}
#[test]
fn test_uncrecoverable_error() {
    connect_database(Some(String::from("localhost")));
    // connect_database(None);
}

// Recoverable Error
// Seperti yang dijelaskan diawal, Rust tidak memilki tipe data Exception, Lantas bagaimana untuk jenis Recoverable Error ? Sama seperti Enum `Option`, Rust menyediakan Enum Result untuk ini
// Jadi ketiak misalnya kita membuta function yang bisa mengemabalikan sukses atau gagal, kita bisa buat function dengan return value Enum Result
// Enum Result hanya memilki dau nilai, `OK(T)`  dan `Err(E)`
// https://doc.rust-lang.org/std/result/
// https://doc.rust-lang.org/std/result/enum.Result.html
fn connect_cache(host:Option<String>)->Result<String, String>{
    match host{
        Some(host)=>Ok(host),
        None=>Err("No cache host provided".to_string())
    }
}

#[test]
fn test_recoverable_error() {
    // let cache= connect_cache(None);
    let cache= connect_cache(Some("Localhost:8090".to_string()));
    match cache {
        Ok(host)=>{
            println!("Connect to cache at {}", host)
        }
        Err(error)=>{
            println!("Error connection to cache {}", error)
        }
    }
}

// `?Operator`
// Saat menggunakan Recoverable Error, kadang sering memanggil beberapa jenis function yang mengahasilkan `Result`, lalu ingin mengecek, jika Errmaka kita ingin langsung mengambalikan error itu secara langsung
// Jika melakukan manual menggunakan Pattern Matching, kadang menyulitkan
//  Kita bisa menggunakan `? Opertaor`, yang secara otomatis bisa mengembalikan `Result` jika memang Err

// Tanpa `?Operator`
fn connect_email(host:Option<String>)->Result<String, String>{
    match host{
        Some(host)=>Ok(host),
        None=>Err("No connect email".to_string())
    }
}
fn connect_without_application(host:Option<String>)->Result<String, String>{
    // cek untuk cache result
    let cache_result= connect_cache(host.clone());
    match cache_result {
        Ok(_)=>{}
        Err(err)=>{
            return Err(err)
        }
    }
    // cek untuk cache email
    let email_result= connect_email(host.clone());
    match email_result {
        Ok(_)=>{}
        Err(err)=>{
            return Err(err)
        }
    }
    Ok("Connected aplication".to_string())
}

// Using `?Operator`
fn connect_application(host:Option<String>)->Result<String, String>{
    connect_cache(host.clone())?;
    connect_email(host.clone())?;
    Ok("Connected to application". to_string())
}

#[test]
fn test_application_error_operator_question() {
    let result = connect_application(None);
    match result {
        Ok(host)=>{println!("Success connect with message :{}", host)}
        Err(err)=>{println!("Erro with message:{}", err)}
    }
}

// Lifetime
// Di materi ownership dan reference, kita sudah tau bahwa tiap data/ reference memiliki lifetime (alur hidup) yang sudah ditentukan
// Secara default, lifetime di rust sudah ditentukan mengikuti scope variable, sehingga aman dan Rust juga melakukan borrow check pda saat melakukan kompilasi untuk memastikan tidak ada Dangling Reference.
#[test]
fn test_dangling_reference() {
    let r:&i32;
    {
        let x=5;
        // r=&x; //will error "x does not live long enough"(Dangling Reference) because r borrowed in this scope
    }
    r=&40;
    println!("{}", r)
}

// Lifetime di Function
// Salah satu yang membingungkan lainnya adalah ketika kita mengguanakan reference sebagai parameter, seklaigus sebagai return value
// Misal kita akan membuat function dengan dua parameter reference, lalu kita bandingkan dana mengambalikan salah satu paramter reference sebagai return value
// Pada kasus ini, Rust akan bingung karena harus melakukan borrow parameter pertama atau parameter kedua, karena kondisinya bisa berbeda

// fn longest(value1: &str, value2:&str)-> &str{ //`-> &str` error `missing lifetime` rust will be confused because this function don't know which one to borrowed
//     if value1.len()>value2.len(){
//         value1
//     }else{
//         value2
//     }
// }

// Lifetime Annotation Syntax
// Pada kasus Lifetime di Parameter sebelumnya, Rust menyediakan fitur bernama Lifetime Annotation, dimana kita bisa menyebutkan yang mana kemungkinan akan di borrow
// Cara menambahkan Lifetime Annotation sama seperti Generic, hanya saja Typenya diawali dengan `'`(petik satu)
// Selanjutnya pada variable yang kita ingin tandai Lifetime Annotation Type, kita bisa tambahkan juga sbelum Type aslinya

// This code will fix upper code `missing lifetime`
fn longest<'a>(value1:&'a str, value2:&'a str)-> &'a str{
    if value1.len()> value2.len(){
        value1
    }else {
        value2
    }
}

#[test]
fn test_longest_lifetime_annotation() {
    let value1="Yogi";
    let value2= "Dwitama";
    let result=longest(value1, value2);
    println!("The longest is:{}", result)
}

// Lifetime Annotation tidak Mengubah Waktu Hidup
// Lifetime Annotation tidak akan mengubah waktu alur hidup, hanya penanda untuk membantu `Rust Borrow Checker`
// Oleh karena itu pada kasus jika ternyata alur hidup variable sudah selesai, maka bisa aja terjadi error seperti diawal, yaitu Dangling Reference
// Example
#[test]
fn test_lifetime_annotation_dangling_reference() {
    let string1= String::from("Eko");
    let result;
    let string2= String::from("Dwitama");
    {
        // let string2= String::from("Dwitama"); //borrowed value does not live long enough
        result=longest(string1.as_str(), string2.as_str());
    }

    println!("The longest string is: {}", result)
}

// Lifetime Annotation di Struct
// Lifetime Annotation mirip seperti Generic, kita bisa gunakan juga diStruct
// Dengan menggunakan Lifetime Annotation di Struct, kita bsai menandai field dengan tipe Reference
// Dengan begitu, kita bisa menggunakan Lifetime Annotation ketiak nanti menggunakan Struct tersebut
struct Student<'a, 'b>{
    name:&'a str,
    last_name:&'b str
}
// #[test]
// fn test_student() {
//     let student=Student{
//         name:"Yogi",
//         last_name:"Dwitama"
//     };
//     println!("{}", student.name)
// }

fn longest_student_name<'a,'b>(student1:&Student<'a, 'b>, student2:&Student<'a, 'b>)->&'a str {
    if student1.name.len()>student2.name.len(){
        student1.name
    }else {
        student2.name
    }
}
#[test]
fn test_longest_student() {
    let student=Student{
        name:"Yogi",
        last_name:"Dwitama"
    };
    // println!("{}", student.name);
    let student2=Student{
        name:"Naruto",
        last_name:"Uzumaki"
    };
    let result=longest_student_name(&student, &student2);
    println!("The longest is: {}", result)

}

// Lifetime Annotation di Method
// Lifetime Annotation selain di Struct dan Function, juga bisa guankan di Method
// Caranya pun sama seperti membuat Generic Type biasanya
struct StudentMethod<'a>{
    name:&'a str
}

impl <'a> StudentMethod<'a>{
    fn longest_name(&self, student: &StudentMethod<'a>)-> &'a str{
        if self.name.len() > student.name.len(){
            self.name
        }else { 
            student.name
        }
    }
}

#[test]
fn test_lifetime_annotation_method() {
    let student1=StudentMethod{
        name:"Yogi"
    };
    let student2=StudentMethod{
        name:"Dwitama"
    };
    let result=student1.longest_name(&student2);
    println!("The longest is: {}", result)
}

// Lifetime Annotation dan Generic Type
// Saaat menggunakan Lifetime Annotation, kita bisa gabungkan bersama Generic type
struct Teacher<'a, ID>
    where
        ID:Ord,
{
    id:ID,
    name:&'a str,
}
#[test]
fn test_lifetime_annotation_generic_struct() {
    let teacher:Teacher<i32>= Teacher{id:1, name:"Yogi"};
    println!("teacher:{}- {}", teacher.id, teacher.name)
}

// Attribute
// Attribute merupakan cara menambahkan metadata(infromasi tambahan) ke code yang kita buat
// Syntax Attribute di Rust mirip dengan bahasa C# menggunakan tanda  #[NamaAttribute]
// Di bahasa lain, ada juga yang menyebutnya sebagai Decorator atau Annotation
// https://doc.rust-lang.org/reference/attributes.html

// Derive Attribute
// Salah satu Attribute yang sering digunakan adalah Derive Attribute
// Derive Attribute adalah Attribute yang digunakan untuk membuat implementasi Trait secara otomatis
// Tidak semua Trait bisa otomatis dibuat implementasinya, hanya yang sudah ditentukan
// https://doc.rust-lang.org/reference/attributes/derive.html
#[derive(Debug, PartialEq, PartialOrd)]
struct Company{
    name:String,
    location: String,
    website: String
}

#[test]
fn test_attribute_debug(){
    let company=Company{
        name:"Rust".to_string(),
        location:"USA".to_string(),
        website:"https://www.rust-lang.org".to_string()
    };
    println!("{:?}", company)
}

// Melihat Hasil Derive
// Tidak ada hal Magic di Rust, sebenarnya ketika kita gunakan Derive Attribute, ketika proses kompilasi Rust akan membuat kode yang dibutuhkan sebelum dikompilasi
// Unutk melihat hasil kode yang dibuat , kita bisagunakan cargo-expand
// https://github.com/dtolnay/cargo-expand
// Install : cargo install cargo-expand
// Lalu untuk melihat hasil kode yang dibuat , kita bisa gunakan perintah: cargo expand --tests nama_module

// Smart Pointer
// Pointer adalah konsep yang umum dimana sebuah variable berisi almat lokasi di memory
// Di Rust, reference merupakan pointer
// Smart Pointer adalah tipe data pointer namun memilki metadata(informasi tambahan) dan kemampuan lain selain sebagai penunjuk ke lokasi data
// Di Rust yang menggunakan konsep ownership (pemilik) dan borrowimg(meminjam), pada kebanyakan kasus, reference hanya meminjam data, sedangkan pointer merpkana pemilik darid ata yang ditunjuk

// `Box<T>` untuk menunjukan data di Heap
// Menggunakan Box<T>, mengizinkan kita membuat data di Heap sedangkan pointernya disimpan di Stack
// https://doc.rust-lang.org/std/boxed/struct.Box.html

#[test]
fn test_box() {
    let value:Box<i32>=Box::new(10);
    println!("value : {}", value)
}

// Recursive Data Type
// Single data dari Box mungkin terlihat tidak begitu menarik, namun Box akan sangat berguna ketika kita menemui tipe data yang recursive
// Misal kita punya tipe data Category, dimana di dalamny bisa terdapat Category lagi. kIta sering melihat jenis data seperti ini, contohnya Toko Online
#[derive(Debug)]
enum Categories{
    Of(String, Box<Categories>),
    End,
}

#[test]
fn test_box_enum() {
    let category =Categories::Of(
        "Laptop".to_string(),
        Box::new(Categories::Of("Dell".to_string(), Box::new(Categories::End)))
    );
    println!("{:?}", category)
}

// Dereference
// Saat kita menggunakan Reference, kadang kita ingin melakukan manipulasi data langsung ke Valuenya
// Kita bisa melakukan Dereference untuk mengakases langsung Valuenya, bukan lagi Referencenya
// Untuk melakukan Dereference, kita bisa menggunakan operator `*` (bintang)
#[test]
fn test_dereference() {
    let value1=Box::new(10);
    let value2=Box::new(20);
    let result:i32=*value1 * *value2;
    println!("result:{}", result)
}

// Deref Trait
// Saat kita menggunakan Reference atau Box<T>, kita bisa mengguanakn `*` Operator untuk melakukan Dereference
// Bagaimana jika kita mengguankan tipe data lain? Misal Struct yang kita buat sendiri? Secara default kita tidak bisa mengguankan Dereference
// Namun jika kita ingin membuat Struct yang kita buat memilki kemampuan Dereference, kita bisa mengguanakn `Deref Trait`
// https://doc.rust-lang.org/std/ops/trait.Deref.html
// Khusus Deref Trait Mut kita bisa menggunakan `Deref Trait Mut`
// https://doc.rust-lang.org/std/ops/trait.DerefMut.html
struct MyValue<T>{
    value:T
}

impl <T> Deref for MyValue<T> {
    type Target = T;
    fn deref(&self)->&Self::Target{
        &self.value
    }
}
#[test]
fn test_deref() {
    let value=MyValue{value:10};
    let real_value:i32=*value;
    println!("value :{}", real_value)
}

fn say_hello_reference(name:&String){
    println!("Hello : {}", name)
}

#[test]
fn test_deref_reference() {
    let name= MyValue{
        value:"Yogi".to_string()
    };
    say_hello_reference(&name.value)
}

// Drop Trait
// Saat kita membuat value,ketika value tersebut keluar dari scope, secara otomatis value akan di drop(hapus) oleh Rust
// Drop Trait merupakan Trait yang bisa kita implementasikan, untuk membuat code yang akan dieksekusi sebelum value di drop
// Misal menutup koneksi, resource dan lain lain
struct Book{
    title:String
}

impl Drop for Book{
    fn drop(&mut self) {
        println!("Dropping Book:{}", self.title)
    }
}

#[test]
fn test_drop() {
    let book = Book{title:"Rust".to_string()};
    println!("{}", book.title)
}

// Multiple Ownership
// Pada umumnya, value biasanya hanya dimiliki oleh satu variable
// Namun, mungkin akan ada kasus dimana satu value dimiliki oleh beberapa variable, contoh  misal pada struktur data Graph, diamna satu titik bisa berasal dari beberapa titik
// Seperti yang kita tahu, bahwa defaultnya di Rust satu value hanya bisa dimiliki oleh satu variable
// Jika kita ingin membuat satu value bisa dimiliki oleh beberapa variable, kita harus mengguanakan type `Rc<T>` (Reference Counted)

// Rc<T>
//
// `Rc<T>` atau Reference Counted adalah tipe data Smart Pointer yang bisa digunakan untuk lebih dari satu variable owner
// Penggunaa `Rc<T>` mirip seperti `Box<T>`
// https://doc.rust-lang.org/alloc/rc/
// https://doc.rust-lang.org/stable/alloc/rc/struct.Rc.html
use std::rc::Rc;

enum Brand{
    Of(String, Rc<Brand>),
    End
}

#[test]
fn test_multiple_ownership() {
    let apple:Rc<Brand>= Rc::new(Brand::Of("Apple". to_string(), Rc::new(Brand::End)));
    println!("Apple reference count:{}",Rc::strong_count(&apple));
    let laptop:Brand=Brand::Of("Laptop". to_string(), Rc::clone(&apple));
    println!("Apple reference count:{}", Rc::strong_count(&apple));
    {
        let smartphone:Brand=Brand::Of("Smartphone".to_string(), Rc::clone(&apple));
        println!("Apple reference count: {}", Rc::strong_count(&apple))
    }
    println!("Apple reference count:{}", Rc::strong_count(&apple))
}







