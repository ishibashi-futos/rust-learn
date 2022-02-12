use crate::defining_modules::front_of_house::hosting;
pub use crate::defining_modules::front_of_house::hosting as Hosting; // pub useとすることで、このモジュールを別のスコープに公開できる

pub fn using_defined_modules() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // asキーワードでモジュールを別名で参照する
    Hosting::add_to_waitlist();
    Hosting::add_to_waitlist();
    Hosting::add_to_waitlist();
}