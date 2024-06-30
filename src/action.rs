pub trait Action
{
    fn usage(&self, name: &str, code: i32);
    fn handle(&self, args: &Vec<String>) -> Result<(), &str>;
}
