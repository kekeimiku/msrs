use core::{any::Any, cmp::min, iter::Sum, slice::Iter, time::Duration};
use std::thread;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    JoinError(Box<dyn Any + Send>),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<Box<dyn Any + Send>> for Error {
    fn from(e: Box<dyn Any + Send>) -> Self {
        Error::JoinError(e)
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

trait MySum {
    type Item;
    fn par_sum(&self) -> Result<Self::Item>;
}

// 其实应该像rayon那样搞一个 MyIterator trait 然后 my_iter().map(xxx).sum() 之类的
// 不过懒得搞了...
impl<T> MySum for Iter<'_, T>
where
    T: Send + Sync + Sum + for<'a> Sum<&'a T>,
{
    type Item = T;

    fn par_sum(&self) -> Result<T> {
        let thread_num = thread::available_parallelism()?.get();
        let per = (self.len() - 1) / thread_num + 1;
        Ok(thread::scope(|sc| {
            (0..thread_num)
                .map(|num| {
                    sc.spawn(move || {
                        // 每个线程sleep一秒假装这个计算很耗时
                        thread::sleep(Duration::from_secs(1));
                        let st = num * per;
                        let ed = min(st + per, self.len());
                        self.as_slice()[st..ed].iter().sum::<T>()
                    })
                })
                // 初学者可能的误区: 这里一定要 collect 后再join
                // 如果直接join那就变成了原本只要sleep 1s 的时间变为 1s * thread_num
                .collect::<Vec<_>>()
                .into_iter()
                .map(|t| t.join())
                .sum::<Result<T, _>>()
        })?)
    }
}

fn main() {
    let vec = (0..1000000).collect::<Vec<_>>();
    let sum: u64 = vec.iter().sum();
    let par_sum = vec.iter().par_sum().unwrap();
    assert_eq!(sum, par_sum)
}
