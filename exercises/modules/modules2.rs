// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.



mod delicious_snacks {
    // TODO: Fix these use statements
    pub use self::fruits::PEAR as fruit;//这里的  self  就是  delicious_snacks  本身
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";//声明一个公开的常量PEAR 类型是静态字符切片引用  值是Pear
        pub const APPLE: &'static str = "Apple";//“静态”= 生命周期  'static ，保证这段字符串数据从程序生到死始终可用
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
