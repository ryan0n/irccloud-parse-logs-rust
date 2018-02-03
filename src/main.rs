//use std::io::{ Result, Read , Seek};
//pub use read::ZipArchive;

fn main() {
    println!("Hello, world!");
}

// pub struct ZipFiles<'a, R: Read + Seek + 'a> {
//     i: usize,
//     archive: &'a mut ZipArchive<R>,
// }
//
// impl<'a, R: Read + Seek + 'a> Iterator for ZipFiles<'a, R> {
//     type Item = ZipResult<Vec<u8>>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let i = self.i;
//         if i < self.archive.len() {
//             self.i = i + 1;
//             Some(self.archive.by_index(i).and_then(|mut file| {
//                 let mut content = vec![];
//                 try!(file.read_to_end(&mut content));
//                 Ok(content)
//             }))
//         } else {
//             None
//         }
//     }
// }
